//! Ranges already requested for a list's host buffer copies.

use std::collections::{BTreeMap, HashMap};

/// Coverage is tracked separately for each host copy and reset whenever a
/// deferred title-memory write can change what a later refresh must see.
#[derive(Default)]
pub(crate) struct RefreshCoverage {
    copies: HashMap<usize, BTreeMap<usize, usize>>,
}

impl RefreshCoverage {
    /// Record a range and return true if some of it has not been requested.
    /// Overflow is never considered covered.
    pub(crate) fn insert(&mut self, copy: usize, start: usize, len: usize) -> bool {
        let Some(end) = start.checked_add(len) else {
            return true;
        };
        // A refresh also retains its copy. Preserve empty requests rather
        // than dropping that lifetime guarantee.
        if start == end {
            return true;
        }
        let ranges = self.copies.entry(copy).or_default();
        let previous = ranges.range(..=start).next_back().map(|(&a, &b)| (a, b));
        if previous.is_some_and(|(_, b)| b >= end) {
            return false;
        }
        let (mut from, mut to) = (start, end);
        if let Some((a, b)) = previous
            && b >= start
        {
            from = a;
            to = to.max(b);
            ranges.remove(&a);
        }
        while let Some((&a, &b)) = ranges.range(from..=to).next() {
            to = to.max(b);
            ranges.remove(&a);
        }
        ranges.insert(from, to);
        true
    }

    pub(crate) fn clear(&mut self) {
        self.copies.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::RefreshCoverage;

    #[test]
    fn nested_overlapping_and_disjoint_requests_are_tracked_per_copy() {
        let mut ranges = RefreshCoverage::default();
        assert!(ranges.insert(1, 20, 10));
        assert!(!ranges.insert(1, 22, 4));
        assert!(ranges.insert(1, 10, 12));
        assert!(!ranges.insert(1, 10, 20));
        assert!(ranges.insert(1, 40, 5));
        assert!(ranges.insert(1, 29, 12));
        assert!(!ranges.insert(1, 10, 35));
        assert!(ranges.insert(2, 22, 4));
        ranges.clear();
        assert!(ranges.insert(1, 22, 4));
    }

    #[test]
    fn empty_and_overflowing_ranges_do_not_create_false_coverage() {
        let mut ranges = RefreshCoverage::default();
        assert!(ranges.insert(1, 5, 0));
        assert!(ranges.insert(1, usize::MAX - 1, 3));
        assert!(ranges.insert(1, usize::MAX - 1, 1));
        assert!(!ranges.insert(1, usize::MAX - 1, 1));
    }

    #[test]
    fn skipped_requests_are_already_covered_even_after_many_overlaps() {
        let mut ranges = RefreshCoverage::default();
        let mut expected = [[false; 128]; 3];
        let mut seed = 0x5eeda11u32;
        for step in 0..5000 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let copy = seed as usize % expected.len();
            let start = (seed >> 8) as usize % 128;
            let len = 1 + (seed >> 16) as usize % (128 - start);
            let already = expected[copy][start..start + len]
                .iter()
                .all(|covered| *covered);
            assert_eq!(ranges.insert(copy, start, len), !already);
            expected[copy][start..start + len].fill(true);
            if step % 127 == 0 {
                ranges.clear();
                expected = [[false; 128]; 3];
            }
        }
    }
}
