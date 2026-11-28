use std::mem::ManuallyDrop;
use std::sync::OnceLock;

use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_UINT;

use crate::host::HostList;

const RESOLVE: &[u8] = b"CS_ScreenProbes_TemporalResolve";
pub(crate) const SLOT: u8 = 1;
pub(crate) const OFFSET: u32 = 176;
// RGB9E5 shared exponent 15 scales each 307 mantissa to about 0.6.
const LIGHT: u32 = (15 << 27) | (307 << 18) | (307 << 9) | 307;

pub(crate) fn selected(container: &[u8]) -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("VICTORY_AMBIENT").is_some_and(|v| v == "1"))
        && container.windows(RESOLVE.len()).any(|w| w == RESOLVE)
}

pub(crate) fn compatible_view(words: [u32; 10]) -> bool {
    words[0] == DXGI_FORMAT_R32_UINT.0 as u32
        && words[1] == D3D12_UAV_DIMENSION_TEXTURE2D.0 as u32
        && words[2..4] == [0, 0]
}

pub(crate) fn record(list: &HostList, texture: &ID3D12Resource) -> windows::core::Result<()> {
    let device = &crate::host::host()
        .ok_or_else(windows::core::Error::empty)?
        .device;
    // SAFETY: live 2D UAV in its unordered-access state and an open list.
    unsafe {
        let make_heap = |flags| -> windows::core::Result<ID3D12DescriptorHeap> {
            device.CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
                NumDescriptors: 1,
                Flags: flags,
                ..Default::default()
            })
        };
        let gpu = make_heap(D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE)?;
        let cpu = make_heap(D3D12_DESCRIPTOR_HEAP_FLAG_NONE)?;
        let cpu_handle = cpu.GetCPUDescriptorHandleForHeapStart();
        device.CreateUnorderedAccessView(texture, None, None, cpu_handle);
        device.CreateUnorderedAccessView(
            texture,
            None,
            None,
            gpu.GetCPUDescriptorHandleForHeapStart(),
        );
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_UAV,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                UAV: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER {
                    pResource: std::mem::transmute_copy(texture),
                }),
            },
        };
        list.keep(&gpu);
        list.keep(&cpu);
        list.keep(texture);
        list.list.SetDescriptorHeaps(&[Some(gpu.clone())]);
        list.list.ResourceBarrier(std::slice::from_ref(&barrier));
        list.list.ClearUnorderedAccessViewUint(
            gpu.GetGPUDescriptorHandleForHeapStart(),
            cpu_handle,
            texture,
            &[LIGHT, 0, 0, 0],
            &[],
        );
        list.list.ResourceBarrier(&[barrier]);
    }
    Ok(())
}

pub(crate) fn compatible_texture(d: &D3D12_RESOURCE_DESC) -> bool {
    d.Format == DXGI_FORMAT_R32_UINT
        && d.Dimension == D3D12_RESOURCE_DIMENSION_TEXTURE2D
        && d.DepthOrArraySize == 1
        && d.MipLevels == 1
        && d.SampleDesc.Count == 1
        && d.Flags.contains(D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS)
}

#[cfg(all(test, feature = "diagnostics"))]
mod tests {
    use super::*;

    #[test]
    fn writes_packed_light_to_every_texel() {
        let host = crate::host::host().unwrap();
        let list = host.new_list().unwrap();
        let mut texture: Option<ID3D12Resource> = None;
        // SAFETY: fresh synthetic texture and a readback after its UAV work.
        unsafe {
            host.device
                .CreateCommittedResource(
                    &D3D12_HEAP_PROPERTIES {
                        Type: D3D12_HEAP_TYPE_DEFAULT,
                        ..Default::default()
                    },
                    D3D12_HEAP_FLAG_NONE,
                    &D3D12_RESOURCE_DESC {
                        Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                        Width: 3,
                        Height: 2,
                        DepthOrArraySize: 1,
                        MipLevels: 1,
                        Format: DXGI_FORMAT_R32_UINT,
                        SampleDesc: windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC {
                            Count: 1,
                            Quality: 0,
                        },
                        Flags: D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS,
                        ..Default::default()
                    },
                    D3D12_RESOURCE_STATE_UNORDERED_ACCESS,
                    None,
                    &mut texture,
                )
                .unwrap();
            let texture = texture.unwrap();
            assert!(compatible_texture(&texture.GetDesc()));
            record(&list, &texture).unwrap();
            list.list.ResourceBarrier(&[crate::host::transition(
                &texture,
                D3D12_RESOURCE_STATE_UNORDERED_ACCESS,
                D3D12_RESOURCE_STATE_COPY_SOURCE,
            )]);
            let (back, layout) =
                crate::capture::read_back(&host.device, &list.list, &texture).unwrap();
            list.close();
            host.execute(&[&list]);
            let mut data = std::ptr::null_mut();
            back.Map(0, None, Some(&mut data)).unwrap();
            for y in 0..2 {
                let offset = layout.Offset as usize + y * layout.Footprint.RowPitch as usize;
                let row = data
                    .cast::<u8>()
                    .add(offset)
                    .cast::<[u32; 3]>()
                    .read_unaligned();
                assert_eq!(row, [0x7cce_6733; 3]);
            }
            back.Unmap(0, None);
        }
    }
}
