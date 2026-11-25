//! Synthetic lane regressions from Scarlight #103 (project-flamingo #684/#697).
use super::*;
#[cfg(test)]
mod dpp_fi_tests {
    use super::*;
    use crate::native::{ComputeIo, StageIo};

    /// A VOP1 DPP16 `v_mov_b32 vdst, vsrc row_shr:1` with `bound_ctrl` and
    /// `fi`.
    const fn vop1_dpp16_row_shr1(vdst: u8, vsrc: u8, bound_ctrl: bool, fi: bool) -> [u32; 2] {
        [
            0x7E00_0000 | (vdst as u32) << 17 | 0x01 << 9 | 0xFA,
            (0xFu32) << 28
                | 0xF << 24
                | (bound_ctrl as u32) << 19
                | (fi as u32) << 18
                | 0x111 << 8
                | vsrc as u32,
        ]
    }

    /// A 32-thread compute program whose only use of the DPP result is an
    /// LDS write (so it is not dropped), lowered.
    fn lowered(fi: bool) -> lower::Lowered {
        let mut words = vec![
            0x7E02_0300, // v_mov_b32 v1, v0: a per-lane value
            0x7E04_0280, // v_mov_b32 v2, 0: the LDS address
        ];
        words.extend_from_slice(&vop1_dpp16_row_shr1(3, 1, true, fi));
        words.extend_from_slice(&[
            0xD834_0000,
            0x0000_0302, // ds_write_b32 v2, v3
            0xBF81_0000, // s_endpgm
        ]);
        let native = NativeProgram {
            stage: HwStage::Cs,
            wave64: false,
            vgprs: 4,
            layout: vec![],
            used: vec![],
            io: StageIo::Cs(ComputeIo {
                rsrc2: 1 << 15, // one 512-byte block of LDS
                threads: [32, 1, 1],
            }),
            code: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
        };
        let p = ir::build(&native.code, false, native.vgprs).unwrap();
        let st = structure::structure(&p).unwrap();
        let io = io::raise(&p, &native).unwrap();
        let res = resource::track(&p, &native.layout, native.stage, Some(&st)).unwrap();
        lower::lower(&lower::Parts {
            native: &native,
            p: &p,
            st: &st,
            io: &io,
            res: &res,
            inputs: vec![],
            outputs: vec![],
            prims: vec![],
            mesh_stride: 0,
        })
        .expect("lower")
    }

    /// project-flamingo #684 (`19dda7a54`): with `fi` clear an inactive DPP
    /// source lane is out of bounds like a row-bound miss, so the lowering
    /// ballots the wave for the source lane's EXEC bit; with `fi` set it
    /// reads straight through.
    #[test]
    fn dpp_fetch_inactive_ballots_the_source_lane_mask() {
        let fi0 = lowered(false);
        let fi1 = lowered(true);
        assert_eq!(fi0.exec_ballots, 1, "fi:0 ballots the source mask");
        assert_eq!(fi1.exec_ballots, 0, "fi:1 does not ballot");
        assert_ne!(fi0.dxil, fi1.dxil, "the two modes lower differently");
    }
}

#[cfg(test)]
mod swizzle_exec_tests {
    use super::*;
    use crate::native::{ComputeIo, StageIo};

    /// A 32-thread compute program of `words`, lowered.
    fn lowered(words: &[u32]) -> lower::Lowered {
        let native = NativeProgram {
            stage: HwStage::Cs,
            wave64: false,
            vgprs: 4,
            layout: vec![],
            used: vec![],
            io: StageIo::Cs(ComputeIo {
                rsrc2: 1 << 15, // one 512-byte block of LDS
                threads: [32, 1, 1],
            }),
            code: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
        };
        let p = ir::build(&native.code, false, native.vgprs).unwrap();
        let st = structure::structure(&p).unwrap();
        let io = io::raise(&p, &native).unwrap();
        let res = resource::track(&p, &native.layout, native.stage, Some(&st)).unwrap();
        lower::lower(&lower::Parts {
            native: &native,
            p: &p,
            st: &st,
            io: &io,
            res: &res,
            inputs: vec![],
            outputs: vec![],
            prims: vec![],
            mesh_stride: 0,
        })
        .expect("lower")
    }

    /// project-flamingo #697 (`7b3b66859`): the console's `ds_swizzle_b32`
    /// reads 0 from a source lane outside EXEC; `waveReadLaneAt` alone would
    /// forward that lane's stale VGPR.
    #[test]
    fn ds_swizzle_b32_zeros_inactive_source_lanes() {
        // s_mov_b32 exec_lo, 0x5555 (odd lanes inactive), v_mov_b32 v1, v0,
        // ds_swizzle_b32 v1, v1 (and:0x1f or:0 xor:1: the neighbour), then
        // v1 is stored to LDS so the swizzle is not dropped.
        let words = [
            0xBEFE_03FF,
            0x0000_5555, // s_mov_b32 exec_lo, 0x5555
            0x7E02_0300, // v_mov_b32 v1, v0
            0xD8D4_0101,
            0x0100_0000, // ds_swizzle_b32 v1, v1 offset:0x41f
            0x7E04_0280, // v_mov_b32 v2, 0
            0xD834_0000,
            0x0000_0102, // ds_write_b32 v2, v1
            0xBF81_0000, // s_endpgm
        ];
        let lowered = lowered(&words);
        assert_eq!(
            lowered.exec_ballots, 1,
            "ds_swizzle_b32 ballots inactive source lanes to 0"
        );
        assert!(lowered.wave_ops, "ds_swizzle_b32 uses wave operations");
    }
}
