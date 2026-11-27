//! Build an internal shader container from native compute pipeline blocks.

use super::{
    ABSENT, CS_RSRC2, CS_THREADS, ComputeIo, ENTRY_SIZE, FSSM_HEADER, FSSM_VERSION,
    GRAPHICS_SLOT_COUNT, HwStage, KIND_NATIVE, MAX_UNPACKED, NativeProgram, RECORD_COUNTS,
    RECORD_FLAGS, RECORD_LAYOUT, RECORD_REGS, RECORD_SIZE, RECORD_USED, ResourceType, RootParam,
    RootType, SLOTS, StageIo, UsedResource, XBCH_HEADER, XBCH_VERSION, XDAT_OFFSET,
};
use crate::ShaderError::{self, Malformed};
use crate::dxbc::u32_at;
use crate::{FourCC, dxbc};

const S_CODE_END: u32 = 0xBF9F_0000;

const WAVE32_DEPTH_BIT: u32 = 0x10 << 16;

fn resources(table: &[u8]) -> Result<(Vec<RootParam>, Vec<UsedResource>), ShaderError> {
    let kind = u32_at(table, 0)?;
    let header = if kind & 0x400 != 0 { 0x18 } else { 0x10 };
    let size = u32_at(table, 4)? as usize;
    if table.len().checked_sub(header - 4) != Some(size) {
        return Err(Malformed("resource table size disagrees"));
    }
    let bytes = table
        .get(header..)
        .ok_or(Malformed("resource table header too short"))?;
    let (entries, tail) = bytes.as_chunks::<0x20>();
    let Some((hash, entries)) = entries.split_last() else {
        return Err(Malformed("missing resource table hash"));
    };
    if !tail.is_empty() || u32_at(hash, 0)? != 0x43 {
        return Err(Malformed("incomplete resource table"));
    }
    let count = entries
        .iter()
        .take_while(|entry| super::le32(entry.as_slice(), 0) == 0x41)
        .count();
    let (descriptors, ranges) = entries.split_at(count);
    if ranges.iter().any(|entry| super::le32(entry, 0) != 0x42) {
        return Err(Malformed("unexpected resource range"));
    }
    let (mut layout, mut used) = (Vec::new(), Vec::new());
    for entry in descriptors {
        let word = |k: usize| u32::from_le_bytes(entry[4 * k..4 * k + 4].try_into().unwrap());
        let (types, offset, register, stride, packed) =
            (word(1), word(2), word(3), word(4), word(5));
        let root = RootType::from_raw(types >> 8);
        let slot = u8::try_from(packed >> 16).map_err(|_| Malformed("implausible root slot"))?;
        let user_data = (packed >> 8) as u8;
        if root == RootType::Constants {
            if packed & 0xFF != 4 {
                return Err(Malformed("implausible root constants entry"));
            }
            // The first constant: the count is the root signature's.
            layout.push(RootParam {
                root,
                table_offset_dwords: 0,
                sgprs: 1,
                base_register: register,
                user_data,
                slot,
            });
            continue;
        }
        used.push(UsedResource {
            kind: match types & 0xFF {
                0 => ResourceType::Srv,
                1 => ResourceType::Sampler,
                2 => ResourceType::Cbv,
                3 => ResourceType::Uav,
                n => ResourceType::Other(n as u8),
            },
            root,
            table_offset_dwords: offset,
            lower_bound: register,
            stride,
            dwords: packed as u8,
            user_data,
            slot,
        });
        let sgprs = match root {
            RootType::Table => 1,
            RootType::Descriptor => 2,
            RootType::ImmediateSampler => 0,
            _ => {
                return Err(Malformed(
                    "a console resource table entry of a root type not seen yet",
                ));
            }
        };
        layout.push(RootParam {
            root,
            table_offset_dwords: offset,
            sgprs,
            base_register: register,
            user_data,
            slot,
        });
    }
    Ok((layout, used))
}

fn trimmed(code: &[u8]) -> &[u8] {
    let mut end = code.len();
    while end >= 4 && u32_at(code, end - 4).ok() == Some(S_CODE_END) {
        end -= 4;
    }
    &code[..end]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SerializedParam {
    root: RootType,
    dwords: u32,
    register: u32,

    srv: bool,
}

const HEAPS_INDEXED: u32 = 0xC00;

fn root_signature(blob: &[u8]) -> Result<(Vec<SerializedParam>, u32), ShaderError> {
    let rts0 = match crate::Container::parse(blob) {
        Ok(c) => {
            c.part(FourCC::RTS0)
                .ok_or(Malformed("a root signature container without RTS0"))?
                .data
        }
        Err(_) => blob,
    };
    if !(1..=3).contains(&u32_at(rts0, 0)?) {
        return Err(Malformed("unknown root signature version"));
    }
    let (count, at, flags) = (
        u32_at(rts0, 4)?,
        u32_at(rts0, 8)? as usize,
        u32_at(rts0, 20)?,
    );
    if count > 64 {
        return Err(Malformed("implausible root parameter count"));
    }
    let params = (0..count as usize)
        .map(|k| {
            let entry = at + 12 * k;
            let payload = u32_at(rts0, entry + 8)? as usize;
            let kind = u32_at(rts0, entry)?;
            let (root, dwords) = match kind {
                0 => (RootType::Table, 1),
                1 => (RootType::Constants, u32_at(rts0, payload + 8)?),
                2 | 3 => (RootType::Descriptor, 2),
                4 => {
                    return Err(Malformed("a root UAV, whose user data no capture places"));
                }
                _ => return Err(Malformed("unknown root parameter type")),
            };
            let register = if root == RootType::Table {
                0
            } else {
                u32_at(rts0, payload)?
            };
            Ok(SerializedParam {
                root,
                dwords,
                register,
                srv: kind == 3,
            })
        })
        .collect::<Result<_, _>>()?;
    Ok((params, flags))
}

fn completed(
    layout: &[RootParam],
    params: &[SerializedParam],
    flags: u32,
    user_sgprs: u32,
) -> Result<Vec<RootParam>, ShaderError> {
    let mut at = Vec::with_capacity(params.len());
    let mut end = 4u32;
    for p in params {
        at.push(end);
        end = end
            .checked_add(p.dwords)
            .ok_or(Malformed("implausible root constants size"))?;
    }
    let named = |k: usize| {
        layout
            .iter()
            .filter(|l| l.root != RootType::ImmediateSampler)
            .any(|l| usize::from(l.slot) == k)
    };
    let agrees = layout
        .iter()
        .filter(|l| l.root != RootType::ImmediateSampler)
        .all(|l| {
            let k = usize::from(l.slot);
            params.get(k).is_some_and(|p| p.root == l.root) && at[k] == u32::from(l.user_data)
        });
    if !agrees {
        return Err(Malformed(
            "a resource table places a root parameter where its root signature does not",
        ));
    }
    let mut out = layout.to_vec();
    // Root constants the table names: it gives the first only, the
    // signature their count.
    for l in out.iter_mut().filter(|l| l.root == RootType::Constants) {
        let p = &params[usize::from(l.slot)];
        if p.dwords > user_sgprs - u32::from(l.user_data) {
            return Err(Malformed(
                "a root parameter cut by the end of the program's user SGPRs",
            ));
        }
        l.sgprs = p.dwords;
    }
    for (k, p) in params.iter().enumerate() {
        let (first, past) = (at[k], at[k] + p.dwords);
        if named(k) || first >= user_sgprs {
            continue;
        }
        if past > user_sgprs {
            return Err(Malformed(
                "a root parameter cut by the end of the program's user SGPRs",
            ));
        }
        if p.srv {
            // A root SRV the table does not name: what it is read as (and
            // so its used resource) is not known.
            return Err(Malformed("a root SRV the resource table does not place"));
        }
        out.push(RootParam {
            root: p.root,
            table_offset_dwords: 0,
            sgprs: p.dwords,
            base_register: p.register,
            user_data: first as u8,
            slot: k as u8,
        });
    }
    // Name the heap SGPRs only when their count agrees with the register mask.
    let heaps: Vec<RootType> = [
        (0x400, RootType::ResourceHeap),
        (0x800, RootType::SamplerHeap),
    ]
    .into_iter()
    .filter(|&(bit, _)| flags & HEAPS_INDEXED & bit != 0)
    .map(|(_, root)| root)
    .collect();
    if end.checked_add(heaps.len() as u32) == Some(user_sgprs) {
        for (k, root) in heaps.into_iter().enumerate() {
            out.push(RootParam {
                root,
                table_offset_dwords: 0,
                sgprs: 1,
                base_register: 0,
                user_data: (end + k as u32) as u8,
                slot: (params.len() + k) as u8,
            });
        }
    }
    Ok(out)
}

fn console_compute(
    registers: &[u8],
    table: &[u8],
    code: &[u8],
) -> Result<NativeProgram, ShaderError> {
    if registers.len() < 0x1c {
        return Err(Malformed("compute registers too short"));
    }
    let flags = u32_at(registers, 0)?;
    let size = u32_at(registers, 0x0c)?;
    // The depth is the low half of its word; the high half says 32 lanes.
    let depth = u32_at(registers, 0x10)?;
    let threads = [size & 0xFFFF, size >> 16, depth & 0xFFFF];
    let rsrc1 = u32_at(registers, 0x14)?;
    let rsrc2 = u32_at(registers, 0x18)?;
    if threads.contains(&0)
        || threads[2] > 64
        || threads
            .into_iter()
            .try_fold(1u32, u32::checked_mul)
            .is_none_or(|lanes| lanes > 1024)
    {
        return Err(Malformed("implausible thread-group size"));
    }
    let wave64 = flags & super::FLAG_WAVE32 == 0 && depth & WAVE32_DEPTH_BIT == 0;
    // COMPUTE_PGM_RSRC1.VGPRS: granules of 4 (64 lanes) or 8 (32 lanes), less one.
    let vgprs = ((rsrc1 & 0x3F) + 1) * if wave64 { 4 } else { 8 };
    if vgprs > 256 || !code.len().is_multiple_of(4) {
        return Err(Malformed("invalid compute register count or code length"));
    }
    let (layout, used) = resources(table)?;
    // COMPUTE_PGM_RSRC2.USER_SGPR: what the root parameters can name.
    let user_sgprs = rsrc2 >> 1 & 0x1F;
    if layout
        .iter()
        .any(|p| u32::from(p.user_data) + p.sgprs > user_sgprs)
    {
        return Err(Malformed(
            "a resource table names user data the program does not have",
        ));
    }
    let code = trimmed(code);
    if code.is_empty() {
        return Err(Malformed("no code"));
    }
    Ok(NativeProgram {
        stage: HwStage::Cs,
        wave64,
        vgprs: vgprs as u16,
        layout,
        used,
        io: StageIo::Cs(ComputeIo { rsrc2, threads }),
        code: code.to_vec(),
    })
}

fn root_raw(root: RootType) -> u32 {
    match root {
        RootType::Constants => 0,
        RootType::Descriptor => 1,
        RootType::Table => 3,
        RootType::ImmediateSampler => 4,
        RootType::ResourceHeap => 5,
        RootType::SamplerHeap => 6,
        RootType::Other(n) => n,
    }
}

fn kind_raw(kind: ResourceType) -> u8 {
    match kind {
        ResourceType::Srv => 0,
        ResourceType::Sampler => 1,
        ResourceType::Cbv => 2,
        ResourceType::Uav => 3,
        ResourceType::Other(n) => n,
    }
}

pub fn console_compute_container(
    registers: &[u8],
    table: &[u8],
    code: &[u8],
    root: Option<&[u8]>,
) -> Result<Vec<u8>, ShaderError> {
    let mut program = console_compute(registers, table, code)?;
    if let Some(root) = root
        && let StageIo::Cs(cs) = &program.io
    {
        let (params, flags) = root_signature(root)?;
        program.layout = completed(&program.layout, &params, flags, cs.rsrc2 >> 1 & 0x1F)?;
    }
    compute_container(&program)
}

fn compute_container(program: &NativeProgram) -> Result<Vec<u8>, ShaderError> {
    let StageIo::Cs(cs) = &program.io else {
        return Err(Malformed("not a compute program"));
    };
    let put32 =
        |v: &mut Vec<u8>, at: usize, x: u32| v[at..at + 4].copy_from_slice(&x.to_le_bytes());
    let put64 =
        |v: &mut Vec<u8>, at: usize, x: u64| v[at..at + 8].copy_from_slice(&x.to_le_bytes());
    // payload: slot count and record offsets, the record, its stage data, code, tables
    let mut payload = vec![0u8; SLOTS.iter().map(|s| s.1).max().unwrap() + 8];
    put64(&mut payload, 0, GRAPHICS_SLOT_COUNT);
    for (_, at) in SLOTS {
        put64(&mut payload, at, ABSENT);
    }
    let record = payload.len();
    put64(
        &mut payload,
        super::SLOTS.iter().find(|s| s.0 == HwStage::Cs).unwrap().1,
        record as u64,
    );
    payload.resize(record + RECORD_SIZE as usize + CS_THREADS + 12, 0);
    let stage = record + RECORD_SIZE as usize;
    put32(&mut payload, stage + CS_RSRC2, cs.rsrc2);
    for (k, n) in cs.threads.iter().enumerate() {
        put32(&mut payload, stage + CS_THREADS + 4 * k, *n);
    }
    let code_at = payload.len();
    payload.extend_from_slice(&program.code);
    let layout_at = payload.len();
    for p in &program.layout {
        let mut e = [0u8; ENTRY_SIZE as usize];
        e[0..4].copy_from_slice(&root_raw(p.root).to_le_bytes());
        e[4..8].copy_from_slice(&p.table_offset_dwords.to_le_bytes());
        e[8..12].copy_from_slice(&p.sgprs.to_le_bytes());
        e[12..16].copy_from_slice(&p.base_register.to_le_bytes());
        e[16] = p.user_data;
        e[17] = p.slot;
        payload.extend_from_slice(&e);
    }
    let used_at = payload.len();
    for u in &program.used {
        let mut e = [0u8; ENTRY_SIZE as usize];
        e[0] = kind_raw(u.kind);
        e[1] = root_raw(u.root) as u8;
        e[4..8].copy_from_slice(&u.table_offset_dwords.to_le_bytes());
        e[8..12].copy_from_slice(&u.lower_bound.to_le_bytes());
        e[12..16].copy_from_slice(&u.stride.to_le_bytes());
        e[16] = u.dwords;
        e[17] = u.user_data;
        e[18] = u.slot;
        payload.extend_from_slice(&e);
    }
    put64(&mut payload, record, code_at as u64);
    put64(&mut payload, record + 8, program.code.len() as u64);
    put32(
        &mut payload,
        record + RECORD_FLAGS,
        if program.wave64 {
            0
        } else {
            super::FLAG_WAVE32
        },
    );
    put64(&mut payload, record + RECORD_LAYOUT, layout_at as u64);
    put64(&mut payload, record + RECORD_USED, used_at as u64);
    put32(
        &mut payload,
        record + RECORD_COUNTS,
        program.layout.len() as u32,
    );
    put32(
        &mut payload,
        record + RECORD_COUNTS + 4,
        program.used.len() as u32,
    );
    put32(&mut payload, record + RECORD_REGS, u32::from(program.vgprs));
    // FSSM, then XBCH around its zlib stream
    let mut fssm = b"FSSM".to_vec();
    fssm.extend(FSSM_VERSION.to_le_bytes());
    fssm.extend((payload.len() as u64).to_le_bytes());
    debug_assert_eq!(fssm.len(), FSSM_HEADER);
    fssm.extend(payload);
    if fssm.len() as u64 > MAX_UNPACKED {
        return Err(Malformed(
            "native compute program exceeds the container size limit",
        ));
    }
    let mut compressed = vec![0; zlib_rs::compress_bound(fssm.len())];
    let config = zlib_rs::DeflateConfig {
        level: 0,
        ..Default::default()
    };
    let (stream, status) = zlib_rs::compress_slice(&mut compressed, &fssm, config);
    if status != zlib_rs::ReturnCode::Ok {
        return Err(Malformed("could not encode native compute program"));
    }
    let mut xbch = b"XBCH".to_vec();
    xbch.extend(XBCH_VERSION.to_le_bytes());
    xbch.extend((stream.len() as u64).to_le_bytes());
    xbch.extend((fssm.len() as u64).to_le_bytes());
    debug_assert_eq!(xbch.len(), XBCH_HEADER);
    xbch.extend_from_slice(stream);
    // the DXIL part: a header up to the index, the index of one entry, the blob
    let mut dxil = vec![0u8; XDAT_OFFSET];
    dxil.extend_from_slice(b"XDAT");
    dxil.extend(1u32.to_le_bytes());
    dxil.extend(0x24u32.to_le_bytes());
    let blob_at = dxil.len() + 12;
    dxil.extend(KIND_NATIVE.to_le_bytes());
    dxil.extend((blob_at as u32).to_le_bytes());
    dxil.extend((xbch.len() as u32).to_le_bytes());
    dxil.extend(xbch);
    Ok(dxbc::build(&[(FourCC::DXIL, &dxil)]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    #[test]
    fn native_compute_round_trip_checks_layout_and_block_bounds() {
        let registers = words(&[0, 0, 0, (8 << 16) | 8, 1, 3, 6 << 1]);
        let code = words(&[0xBF81_0000, S_CODE_END]);
        let mut table = words(&[0x403, 68, 0, 0, 0, 0]);
        table.extend(words(&[0x41, 0, 0, 7, 0, 0x404, 0, 0]));
        table.extend(words(&[0x43, 0, 0, 0, 0, 0, 0, 0]));
        // One root constant parameter with two dwords at b7.
        let root = words(&[1, 1, 24, 0, 0, 0, 1, 0, 36, 7, 0, 2]);
        let shader = console_compute_container(&registers, &table, &code, Some(&root)).unwrap();
        let container = crate::Container::parse(&shader).unwrap();
        let back =
            super::super::native_programs(container.part(FourCC::DXIL).unwrap().data).unwrap();
        let [program] = back.as_slice() else {
            panic!("expected one compute program")
        };
        assert!(program.wave64);
        assert_eq!(program.vgprs, 16);
        assert_eq!(program.code, code[..4]);
        let StageIo::Cs(cs) = &program.io else {
            panic!("expected compute registers")
        };
        assert_eq!(cs.threads, [8, 8, 1]);
        assert_eq!(program.layout[0].sgprs, 2);
        assert_eq!(program.layout[0].base_register, 7);
        assert!(program.used.is_empty());

        for end in 0..table.len() {
            let mut short = table[..end].to_vec();
            if end >= 20 {
                short[4..8].copy_from_slice(&((end - 20) as u32).to_le_bytes());
            }
            assert!(console_compute_container(&registers, &short, &code, Some(&root)).is_err());
        }
        assert!(console_compute_container(&registers, &table, &code[..3], Some(&root)).is_err());
        let oversized = words(&[1, 1, 24, 0, 0, 0, 1, 0, 36, 7, 0, u32::MAX]);
        assert!(console_compute_container(&registers, &table, &code, Some(&oversized)).is_err());
    }
}
