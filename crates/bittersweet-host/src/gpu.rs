//! GPU identity and the capabilities the host layer must respect.
//!
//! Everything here is derived from PCI vendor/device ids and sysfs, never
//! from a running game. The one table that matters for correctness is the
//! AMD mesh-shader gate, ported from `pablogonz12/Scarlight` PR #80
//! (commit range on `main`, 2026-11-25). On families below RDNA2, RADV does
//! not expose `VK_EXT_mesh_shader`; on Vega it aborts the process rather
//! than refusing the pipeline, so a mesh pipeline must be refused before it
//! reaches the driver.

/// PCI vendor ids we care about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vendor {
    Amd,
    Nvidia,
    Intel,
    Microsoft,
    Other(u16),
}

impl Vendor {
    pub fn from_pci(id: u16) -> Self {
        match id {
            0x1002 => Vendor::Amd,
            0x10de => Vendor::Nvidia,
            0x8086 => Vendor::Intel,
            0x1414 => Vendor::Microsoft,
            other => Vendor::Other(other),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Vendor::Amd => "AMD",
            Vendor::Nvidia => "NVIDIA",
            Vendor::Intel => "Intel",
            Vendor::Microsoft => "Microsoft",
            Vendor::Other(_) => "unknown",
        }
    }
}

/// Coarse AMD graphics family, for reporting and policy only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmdFamily {
    /// gfx9: Vega and the Vega-derived APUs.
    Gcn,
    /// gfx10.1: Navi 10/12/14.
    Rdna1,
    /// gfx10.3 and newer: RDNA2/3/4, where RADV exposes mesh shaders.
    Rdna2Plus,
    /// An AMD id we do not classify; treated as RDNA2+ for mesh purposes,
    /// matching the driver's feature query rather than a guess.
    Unknown,
}

/// The AMD family, where RADV does **not** expose mesh shaders; `None`
/// means it does, or the adapter is not AMD.
///
/// Ported from Scarlight PR #80 (`crates/victory-gpu/src/host.rs`,
/// `amd_mesh_family`). Device ids are PCI ids as found in
/// `/sys/class/drm/card*/device/device`.
pub fn amd_mesh_refusal_family(device: u16) -> Option<&'static str> {
    match device {
        // Vega 10 (gfx900), Vega 12 (gfx904), Vega 20 (gfx906).
        0x6860..=0x687f | 0x69a0..=0x69af | 0x66a0..=0x66af => Some("gfx9 (Vega)"),
        // Vega APUs: Raven Ridge, Picasso, Renoir, Lucienne, Cezanne,
        // Van Gogh, Mendocino (gfx902/gfx909/gfx90c/gfx1033).
        0x15d8 | 0x15dd | 0x1636 | 0x1638 | 0x163f | 0x164c | 0x1506 => Some("gfx9 APU"),
        // Navi 10/12/14 (gfx1010-1012): pipelines create, but the driver
        // does not advertise VK_EXT_mesh_shader.
        0x7310..=0x731f | 0x7340..=0x734f | 0x7360..=0x736f => Some("gfx10.1 (RDNA1)"),
        _ => None,
    }
}

pub fn amd_family(device: u16) -> AmdFamily {
    match amd_mesh_refusal_family(device) {
        Some("gfx9 (Vega)") | Some("gfx9 APU") => AmdFamily::Gcn,
        Some("gfx10.1 (RDNA1)") => AmdFamily::Rdna1,
        Some(_) => AmdFamily::Unknown,
        None => match device {
            // A few well-known ids, for a readable report. Policy does not
            // depend on the distinction between RDNA2, 3 and 4.
            0x73bf => AmdFamily::Rdna2Plus, // Navi 21
            0x744c => AmdFamily::Rdna2Plus, // Navi 31
            0x7550 => AmdFamily::Rdna2Plus, // Navi 48 (RX 9070 XT)
            _ => AmdFamily::Unknown,
        },
    }
}

/// Largest native wave width of the vendor's shader cores.
///
/// The console target uses 64-wide waves. AMD GCN and RDNA can run wave64
/// natively; NVIDIA and Intel cannot, so a 64-lane compute program has to be
/// lowered. This is the number the shader layer checks before choosing a
/// translation, not a claim about a particular card's clocks.
pub fn max_wave_lanes(vendor: Vendor, amd: AmdFamily) -> u32 {
    match vendor {
        Vendor::Amd => match amd {
            AmdFamily::Gcn => 64,
            AmdFamily::Rdna1 | AmdFamily::Rdna2Plus | AmdFamily::Unknown => 64,
        },
        // Ampere/Ada/Blackwell execute 32-wide; two warps can be fused by the
        // compiler, but a wave64 intrinsic is not native.
        Vendor::Nvidia => 32,
        // Xe executes 8/16/32-wide depending on the kernel.
        Vendor::Intel => 32,
        _ => 32,
    }
}

pub fn needs_wave64_emulation(vendor: Vendor, amd: AmdFamily) -> bool {
    max_wave_lanes(vendor, amd) < 64
}

/// Whether a mesh pipeline may be handed to this adapter at all.
///
/// Mirrors Scarlight's `mesh_pipelines_safe`: the D3D12 tier decides on
/// non-AMD hardware, and AMD families below RDNA2 are refused by id.
pub fn mesh_pipelines_safe(vendor: Vendor, device: u16) -> bool {
    vendor != Vendor::Amd || amd_mesh_refusal_family(device).is_none()
}

/// The deny reason to put in a log line, if any.
pub fn mesh_refusal(vendor: Vendor, device: u16) -> Option<&'static str> {
    if vendor == Vendor::Amd {
        amd_mesh_refusal_family(device)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Scarlight PR #80: only the families without mesh shaders are named.
    #[test]
    fn only_families_without_mesh_shaders_are_named() {
        assert_eq!(amd_mesh_refusal_family(0x687f), Some("gfx9 (Vega)"));
        assert_eq!(amd_mesh_refusal_family(0x66af), Some("gfx9 (Vega)"));
        assert_eq!(amd_mesh_refusal_family(0x15d8), Some("gfx9 APU"));
        assert_eq!(amd_mesh_refusal_family(0x731f), Some("gfx10.1 (RDNA1)"));
        assert_eq!(amd_mesh_refusal_family(0x7360), Some("gfx10.1 (RDNA1)"));
        // Mesh shaders exist from RDNA2 up: no refusal.
        assert_eq!(amd_mesh_refusal_family(0x73bf), None);
        assert_eq!(amd_mesh_refusal_family(0x744c), None);
        assert_eq!(amd_mesh_refusal_family(0x7550), None);
        // Other vendors are not ours to guess at.
        assert_eq!(amd_mesh_refusal_family(0xe20b), None);
        assert_eq!(amd_mesh_refusal_family(0x2684), None);
    }

    #[test]
    fn a_mesh_pipeline_is_only_refused_on_old_amd() {
        assert!(!mesh_pipelines_safe(Vendor::Amd, 0x687f));
        assert!(mesh_pipelines_safe(Vendor::Amd, 0x7550));
        assert!(mesh_pipelines_safe(Vendor::Nvidia, 0x2684));
        assert!(mesh_pipelines_safe(Vendor::Intel, 0xe20b));
        assert_eq!(mesh_refusal(Vendor::Amd, 0x731f), Some("gfx10.1 (RDNA1)"));
        assert_eq!(mesh_refusal(Vendor::Nvidia, 0x731f), None);
    }

    #[test]
    fn wave64_is_native_on_amd_and_emulated_elsewhere() {
        assert_eq!(max_wave_lanes(Vendor::Amd, AmdFamily::Gcn), 64);
        assert_eq!(max_wave_lanes(Vendor::Amd, AmdFamily::Rdna2Plus), 64);
        assert!(needs_wave64_emulation(Vendor::Nvidia, AmdFamily::Unknown));
        assert!(needs_wave64_emulation(Vendor::Intel, AmdFamily::Unknown));
        assert!(!needs_wave64_emulation(Vendor::Amd, AmdFamily::Rdna1));
    }

    #[test]
    fn family_classification() {
        assert_eq!(amd_family(0x687f), AmdFamily::Gcn);
        assert_eq!(amd_family(0x731f), AmdFamily::Rdna1);
        assert_eq!(amd_family(0x7550), AmdFamily::Rdna2Plus);
        assert_eq!(Vendor::from_pci(0x1002), Vendor::Amd);
        assert_eq!(Vendor::from_pci(0x10de), Vendor::Nvidia);
        assert_eq!(Vendor::from_pci(0x1234), Vendor::Other(0x1234));
    }
}
