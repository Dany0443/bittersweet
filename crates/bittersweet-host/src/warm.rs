//! Shader warm-up modes and their time budget.
//!
//! Ported from `PastiiJay/klbf-lab` PRs #8 ("Reject simultaneous shader
//! warm-up and full pipeline pre-build") and #7 ("Stop scanning shader
//! inputs when the warm-up time budget expires"). Both are host policy: a
//! full pre-build and an interactive warm-up compete for the same workers
//! and the same cache, and a warm-up that overruns its budget must stop
//! between entries, not only inside the hashing loop.

use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarmMode {
    /// Scan and translate what is needed for this session.
    Warm,
    /// Fill the host pipeline library for a later run.
    Cache,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeConflict {
    WarmAndCache,
}

impl std::fmt::Display for ModeConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("shader warm-up and full pipeline pre-build are separate modes")
    }
}

impl std::error::Error for ModeConflict {}

/// Reject `--warm` together with `--cache --pipelines`, as klbf-lab PR #8
/// does in the CLI. Kept as a pure function so the rule is testable without
/// a process invocation.
pub fn check_modes(warm: bool, cache: bool) -> Result<Option<WarmMode>, ModeConflict> {
    match (warm, cache) {
        (true, true) => Err(ModeConflict::WarmAndCache),
        (true, false) => Ok(Some(WarmMode::Warm)),
        (false, true) => Ok(Some(WarmMode::Cache)),
        (false, false) => Ok(None),
    }
}

/// A wall-clock budget for a directory walk.
///
/// `limit == None` means "no budget". The expiry check is cheap and is
/// meant to be called both before descending into a directory and once per
/// entry, so a large tree cannot run past the budget by more than one entry.
#[derive(Debug)]
pub struct ScanBudget {
    started: Instant,
    limit: Option<Duration>,
    expired: bool,
    skipped: u64,
}

impl ScanBudget {
    pub fn new(limit: Option<Duration>) -> Self {
        Self {
            started: Instant::now(),
            limit,
            expired: false,
            skipped: 0,
        }
    }

    pub fn unlimited() -> Self {
        Self::new(None)
    }

    pub fn limit(&self) -> Option<Duration> {
        self.limit
    }

    /// Whether the budget is spent. Latches, so a caller that checks twice
    /// sees the same answer and the skip counter is only advanced by
    /// [`Self::note_skipped`].
    pub fn expired(&mut self) -> bool {
        if !self.expired
            && self
                .limit
                .is_some_and(|limit| self.started.elapsed() >= limit)
        {
            self.expired = true;
        }
        self.expired
    }

    /// Records entries not visited because the budget was spent.
    pub fn note_skipped(&mut self, count: u64) {
        self.skipped = self.skipped.saturating_add(count);
    }

    pub fn skipped(&self) -> u64 {
        self.skipped
    }

    /// True when the walk was cut short, for the caller's report.
    pub fn was_cut_short(&self) -> bool {
        self.skipped > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported rule from klbf-lab PR #8.
    #[test]
    fn warm_and_cache_are_separate_modes() {
        assert_eq!(check_modes(true, false), Ok(Some(WarmMode::Warm)));
        assert_eq!(check_modes(false, true), Ok(Some(WarmMode::Cache)));
        assert_eq!(check_modes(false, false), Ok(None));
        assert_eq!(check_modes(true, true), Err(ModeConflict::WarmAndCache));
    }

    #[test]
    fn a_zero_budget_expires_immediately_and_latches() {
        let mut budget = ScanBudget::new(Some(Duration::ZERO));
        assert!(budget.expired());
        assert!(budget.expired(), "the answer latches");
        budget.note_skipped(3);
        assert_eq!(budget.skipped(), 3);
        assert!(budget.was_cut_short());
    }

    #[test]
    fn unlimited_never_expires() {
        let mut budget = ScanBudget::unlimited();
        assert!(!budget.expired());
        assert!(!budget.was_cut_short());
    }
}
