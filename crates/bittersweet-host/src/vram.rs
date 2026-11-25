//! VRAM budgeting for 8–12 GB cards.
//!
//! The console target assumes a unified pool far larger than most desktop
//! cards. Rather than let the driver evict blindly and produce black
//! textures, the host layer publishes an explicit texture budget and a mip
//! bias that the runtime can honour. These are policy numbers, not driver
//! knobs: nothing here changes how much memory the card has.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pressure {
    /// 6 GB or less: aggressive mip reduction.
    Severe,
    /// 8 GB: use a conservative texture budget.
    High,
    /// 10–12 GB: the cards losing the device on world entry.
    Moderate,
    /// 16 GB and up: no forced reduction.
    Low,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VramBudget {
    pub total_mb: u64,
    /// Upper bound the texture cache may use, in MiB.
    pub texture_mb: u64,
    /// Positive values bias sampling toward smaller mips.
    pub mip_bias: i32,
    pub pressure: Pressure,
}

/// Budget a card's reported VRAM. `total_mb == 0` (unknown) is treated as
/// 8 GB, a conservative baseline, and flagged `High`.
pub fn for_vram(total_mb: u64) -> VramBudget {
    let total_mb = if total_mb == 0 { 8192 } else { total_mb };
    let (percent, mip_bias, pressure) = match total_mb {
        0..=6143 => (50, 1, Pressure::Severe),
        6144..=8191 => (55, 1, Pressure::High),
        8192..=10239 => (60, 1, Pressure::High),
        10240..=12287 => (65, 0, Pressure::Moderate),
        12288..=16383 => (70, 0, Pressure::Moderate),
        _ => (75, 0, Pressure::Low),
    };
    VramBudget {
        total_mb,
        texture_mb: total_mb * percent / 100,
        mip_bias,
        pressure,
    }
}

impl VramBudget {
    /// `DXVK_*`/`VKD3D_*` friendly form, in bytes.
    pub fn texture_bytes(&self) -> u64 {
        self.texture_mb * 1024 * 1024
    }

    /// A short line for `doctor` output.
    pub fn summary(&self) -> String {
        format!(
            "{} MiB VRAM → texture budget {} MiB ({:?}), mip bias +{}",
            self.total_mb, self.texture_mb, self.pressure, self.mip_bias
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budgets_scale_with_the_card() {
        let b = for_vram(8192);
        assert_eq!(b.texture_mb, 4915);
        assert_eq!(b.mip_bias, 1);
        assert_eq!(b.pressure, Pressure::High);
        assert_eq!(b.texture_bytes(), 4915 * 1024 * 1024);

        let b = for_vram(12288);
        assert_eq!(b.texture_mb, 8601);
        assert_eq!(b.mip_bias, 0);
        assert_eq!(b.pressure, Pressure::Moderate);

        let b = for_vram(24576);
        assert_eq!(b.texture_mb, 18432);
        assert_eq!(b.pressure, Pressure::Low);
    }

    #[test]
    fn unknown_vram_is_assumed_to_be_8_gb() {
        let b = for_vram(0);
        assert_eq!(b.total_mb, 8192);
        assert_eq!(b.pressure, Pressure::High);
    }

    #[test]
    fn small_cards_get_the_strongest_reduction() {
        let b = for_vram(4096);
        assert_eq!(b.texture_mb, 2048);
        assert_eq!(b.mip_bias, 1);
        assert_eq!(b.pressure, Pressure::Severe);
    }
}
