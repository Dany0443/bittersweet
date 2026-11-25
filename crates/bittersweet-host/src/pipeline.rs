//! A capped list of "early" pipeline keys.
//!
//! Ported from `pablogonz12/Scarlight` PR #78 (`crates/victory-gpu/src/
//! pipeline/library.rs`). A title can ask for more pipelines than the early
//! list holds. The list keeps the first keys — the ones drawn with first, so
//! the measured scene — and drops the tail instead of evicting older keys.
//! Every drop is counted and reported, so a run never truncates in silence.

use std::collections::HashSet;

/// Title pipelines listed for an early start, at most. Scarlight's measured
/// lists are 2,238 keys and 1,785 buildable pipelines; the cap is a guard
/// against a title whose keys never repeat, not a tuning value.
pub const EARLY_MAX: usize = 8192;

#[derive(Debug)]
pub struct EarlyList {
    keys: HashSet<String>,
    dropped: u64,
    cap: usize,
}

impl EarlyList {
    pub fn new(cap: usize) -> Self {
        Self {
            keys: HashSet::new(),
            dropped: 0,
            cap,
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn dropped(&self) -> u64 {
        self.dropped
    }

    pub fn contains(&self, key: &str) -> bool {
        self.keys.contains(key)
    }

    /// Adds `key` unless the list is full. Returns whether it was added.
    /// A refused key never evicts one already present.
    pub fn insert(&mut self, key: &str) -> bool {
        if self.keys.contains(key) {
            return false;
        }
        if self.keys.len() >= self.cap {
            self.dropped += 1;
            return false;
        }
        self.keys.insert(key.to_owned());
        true
    }

    /// A key already present is not a drop; only genuinely new keys refused
    /// at the cap are counted.
    pub fn note_used(&mut self, key: &str) -> bool {
        self.insert(key)
    }

    pub fn was_truncated(&self) -> bool {
        self.dropped > 0 || self.len() >= self.cap
    }

    /// `(listed, cap, dropped)` for a single log record.
    pub fn report(&self) -> (usize, usize, u64) {
        (self.len(), self.cap, self.dropped)
    }
}

impl Default for EarlyList {
    fn default() -> Self {
        Self::new(EARLY_MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported test from Scarlight PR #78.
    #[test]
    fn the_list_drops_keys_at_the_cap_and_counts_them() {
        let mut list = EarlyList::new(3);
        assert!(list.insert("a"));
        assert!(list.insert("b"));
        assert!(list.insert("c"));
        assert!(!list.insert("d"), "past the cap");
        assert_eq!(list.len(), 3, "a refused key evicts nothing");
        assert!(!list.contains("d"));
        assert!(list.contains("a"), "the first keys stay");
        assert_eq!(list.dropped(), 1);
        assert!(list.was_truncated());

        // A key already listed is not a new one, cap or no cap.
        assert!(!list.note_used("a"));
        assert_eq!(list.len(), 3);
        assert_eq!(list.dropped(), 1);
    }

    #[test]
    fn a_list_below_the_cap_is_not_truncated() {
        let mut list = EarlyList::new(4);
        assert!(list.insert("a"));
        assert_eq!(list.report(), (1, 4, 0));
        assert!(!list.was_truncated());
    }

    #[test]
    fn the_real_cap_holds_one_short_and_refuses_the_last() {
        let mut list = EarlyList::new(EARLY_MAX);
        for i in 0..EARLY_MAX {
            assert!(list.insert(&format!("k{i}")), "key {i}");
        }
        assert!(!list.insert("overflow"));
        assert_eq!(list.len(), EARLY_MAX);
        assert_eq!(list.dropped(), 1);
    }
}
