use super::HostShader;
use crate::ShaderError;
use crate::native::{
    ComputeIo, HwStage, NativeProgram, ResourceType, RootParam, RootType, StageIo, UsedResource,
};

// Synthetic GFX10 dimension queries adapted from codec-d/cool_fox.
fn build(words: &[u32], srv: (u8, u32)) -> Result<HostShader, ShaderError> {
    let param = |root, user_data| RootParam {
        root,
        table_offset_dwords: 0,
        sgprs: 1,
        base_register: 0,
        user_data,
        slot: 0,
    };
    let range = |kind, table_offset_dwords, dwords, stride| UsedResource {
        kind,
        root: RootType::Table,
        table_offset_dwords,
        lower_bound: 0,
        stride,
        dwords,
        user_data: 0,
        slot: 0,
    };
    let native = NativeProgram {
        stage: HwStage::Cs,
        wave64: false,
        vgprs: 32,
        layout: vec![param(RootType::Table, 0), param(RootType::ResourceHeap, 1)],
        used: vec![
            range(ResourceType::Srv, 0, srv.0, srv.1),
            range(ResourceType::Uav, 16, 4, 0),
        ],
        io: StageIo::Cs(ComputeIo {
            rsrc2: 2 << 1,
            threads: [1, 1, 1],
        }),
        code: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
    };
    let p = crate::ir::build(&native.code, false, native.vgprs)?;
    let st = crate::structure::structure(&p)?;
    let res = crate::resource::track(&p, &native.layout, native.stage, Some(&st))?;
    let io = crate::io::raise(&p, &native)?;
    let bytes = crate::dxbc::build(&[]);
    let container = crate::Container::parse(&bytes)?;
    super::emit(&container, &native, &p, &st, &io, &res)
}
const TABLE: &[u32] = &[0xBE82_0300, 0xBE83_0380, 0xF408_0501, 0xFA00_0040];

const STORE: &[u32] = &[
    0x7E08_0204,
    0x7E0A_0205,
    0xE070_0000,
    0x8005_0400,
    0xE070_0004,
    0x8005_0500,
    0xBF81_0000,
];

const SIZES: &[u32] = &[
    0x9004_9E09,
    0x8F05_820A,
    0x8804_0504,
    0x8704_FF04,
    0x0000_FFFF,
    0x9385_FF0A,
    0x0010_000E,
    0x9386_FF0B,
    0x0004_000C,
    0x8104_8104,
    0x8105_8105,
    0x9004_0604,
    0x9005_0605,
];

const FROM_HEAP: &[u32] = &[
    0xBE86_0301,
    0xBE87_0380,
    0xBE8C_03A0,
    0xF40C_0203,
    0x1800_0000,
];

const COUNT: &[u32] = &[0xF408_0201, 0xFA00_0000, 0xBE84_030A, 0xBE85_0380];

#[test]
fn descriptor_dimensions_translate_without_image_reads() {
    let words = [TABLE, FROM_HEAP, SIZES, STORE].concat();
    let shader = build(&words, (8, 0)).expect("dimension query");
    assert!(!shader.dxil.is_empty());
    #[cfg(windows)]
    {
        use super::host::View;
        let one = [[0.0; 4]; 1];
        let texels = [[0.0; 4]; 15];
        assert_eq!(
            readback(
                &shader,
                &[
                    View::Texture(1, 1, &one),
                    View::Texture(5, 3, &texels),
                    View::RawUav(2),
                ]
            ),
            [5, 3]
        );
    }
}

#[test]
fn ambiguous_descriptor_dimensions_remain_rejected() {
    let width = [0x9004_9E09, 0x8F05_820A, 0x8804_0504];
    assert!(build(&[TABLE, FROM_HEAP, &width, STORE].concat(), (8, 0)).is_err());
    let depth = [0x8805_0C05];
    assert!(build(&[TABLE, FROM_HEAP, SIZES, &depth, STORE].concat(), (8, 0)).is_err());
}

#[test]
fn descriptor_count_requires_a_structured_buffer() {
    let words = [TABLE, COUNT, STORE].concat();
    let shader = build(&words, (4, 16)).expect("record count query");
    assert!(!shader.dxil.is_empty());
    #[cfg(windows)]
    {
        use super::host::{Stride, View};
        let data = [0u32; 12];
        let texel = [[0.0; 4]; 1];
        assert_eq!(
            readback(
                &shader,
                &[
                    View::Buffer {
                        data: &data,
                        stride: Stride::Raw
                    },
                    View::Texture(1, 1, &texel),
                    View::RawUav(2),
                ]
            ),
            [3, 0]
        );
    }
    for stride in [0, 4, 6] {
        assert!(build(&words, (4, stride)).is_err());
    }
}

#[cfg(windows)]
fn readback(shader: &HostShader, views: &[super::host::View<'_>]) -> [u32; 2] {
    use super::host::{Dispatch, Host, Root};
    let host = Host::new(false).expect("SM 6.6 hardware device");
    let (root, pso) = host.compute_pipeline(shader).expect("dimension pipeline");
    let output = host
        .dispatch(
            &root,
            &pso,
            &Dispatch {
                roots: vec![(
                    0,
                    Root::Constants(vec![0; shader.user_data_dwords() as usize]),
                )],
                views,
                samplers: &[],
                groups: [1, 1, 1],
            },
        )
        .expect("dimension readback");
    output[0][..2].try_into().unwrap()
}
