//! Kernel parameter checks relevant to console memory-page aliasing.
//!
//! The compatibility layer maps many small views of title memory. The
//! default `vm.max_map_count` on most distributions (65,530) is far below
//! what a long world-entry run needs; the failure mode is an `mmap` error
//! deep in the runtime, not a clear message. This module only advises — it
//! never writes to `/proc` or runs `sysctl`.

/// Below this, the host is very likely to hit `ENOMEM` from `mmap` during
/// world entry. 1,048,576 is 16× the common default and still modest.
pub const MIN_MAX_MAP_COUNT: u64 = 1_048_576;

/// The value SteamOS/Steam Deck guidance uses for D3D12 translation layers.
pub const RECOMMENDED_MAX_MAP_COUNT: u64 = 2_147_483_642;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelAdvice {
    /// At or above the recommended value.
    Good(u64),
    /// Above the minimum but below the recommendation.
    Acceptable(u64),
    /// Too low for a long session.
    TooLow(u64),
    /// The file could not be read (containers, unusual kernels).
    Unknown,
}

impl KernelAdvice {
    pub fn is_problem(self) -> bool {
        matches!(self, KernelAdvice::TooLow(_))
    }

    pub fn summary(self) -> String {
        match self {
            KernelAdvice::Good(v) => format!("vm.max_map_count = {v} (recommended)"),
            KernelAdvice::Acceptable(v) => format!(
                "vm.max_map_count = {v} (usable; {RECOMMENDED_MAX_MAP_COUNT} recommended for long sessions)"
            ),
            KernelAdvice::TooLow(v) => format!(
                "vm.max_map_count = {v} is too low (need at least {MIN_MAX_MAP_COUNT}); world entry may fail with ENOMEM"
            ),
            KernelAdvice::Unknown => "vm.max_map_count could not be read".to_owned(),
        }
    }

    /// The `sysctl.d` line a user can apply themselves. Printed, never
    /// applied by the tool.
    pub fn suggested_sysctl(self) -> Option<String> {
        match self {
            KernelAdvice::TooLow(_) | KernelAdvice::Acceptable(_) => {
                Some(format!("vm.max_map_count={RECOMMENDED_MAX_MAP_COUNT}"))
            }
            _ => None,
        }
    }
}

pub fn assess(current: Option<u64>) -> KernelAdvice {
    match current {
        None => KernelAdvice::Unknown,
        Some(v) if v >= RECOMMENDED_MAX_MAP_COUNT => KernelAdvice::Good(v),
        Some(v) if v >= MIN_MAX_MAP_COUNT => KernelAdvice::Acceptable(v),
        Some(v) => KernelAdvice::TooLow(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_is_flagged() {
        let advice = assess(Some(65_530));
        assert!(advice.is_problem());
        assert_eq!(
            advice.suggested_sysctl().as_deref(),
            Some("vm.max_map_count=2147483642")
        );
    }

    #[test]
    fn the_minimum_is_acceptable_and_the_recommendation_is_good() {
        assert_eq!(
            assess(Some(MIN_MAX_MAP_COUNT)),
            KernelAdvice::Acceptable(MIN_MAX_MAP_COUNT)
        );
        assert_eq!(
            assess(Some(RECOMMENDED_MAX_MAP_COUNT)),
            KernelAdvice::Good(RECOMMENDED_MAX_MAP_COUNT)
        );
        assert_eq!(assess(None), KernelAdvice::Unknown);
        assert!(!assess(None).is_problem());
    }
}
