use std::collections::VecDeque;

type Range = (u64, u64);

pub(super) struct Remaps {
    entries: VecDeque<(u64, Range)>,
    discarded: u64,
    seen: u64,
}

impl Remaps {
    pub(super) const fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            discarded: 0,
            seen: 0,
        }
    }

    pub(super) fn position(&self) -> u64 {
        self.entries.back().map_or(self.discarded, |entry| entry.0)
    }

    pub(super) fn push(&mut self, start: u64, end: u64, capacity: usize) -> bool {
        if let Some((sequence, last)) = self.entries.back_mut()
            && *sequence > self.seen
            && last.1 == start
        {
            last.1 = end;
            return false;
        }
        let sequence = self.position() + 1;
        let dropped = self.entries.len() == capacity;
        if dropped {
            self.discarded = self.entries.pop_front().unwrap().0;
        }
        self.entries.push_back((sequence, (start, end)));
        dropped
    }

    /// A lagging reader must check all of its slots after losing history.
    pub(super) fn read(&mut self, cursor: &mut u64) -> Option<Vec<Range>> {
        let previous = *cursor;
        *cursor = self.position();
        self.seen = *cursor;
        if previous < self.discarded {
            return None;
        }
        let start = self.entries.partition_point(|entry| entry.0 <= previous);
        Some(self.entries.range(start..).map(|entry| entry.1).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::Remaps;

    #[test]
    fn texture_heaps_read_independently_and_only_lagging_heaps_rescan() {
        let mut journal = Remaps::new();
        let (mut fast, mut slow) = (journal.position(), journal.position());
        assert!(!journal.push(0, 8, 2));
        assert!(!journal.push(8, 16, 2));
        assert_eq!(journal.read(&mut fast), Some(vec![(0, 16)]));
        assert!(!journal.push(16, 24, 2));
        assert_eq!(journal.read(&mut slow), Some(vec![(0, 16), (16, 24)]));
        assert_eq!(journal.read(&mut fast), Some(vec![(16, 24)]));
        assert!(journal.push(32, 40, 2));
        assert_eq!(journal.read(&mut fast), Some(vec![(32, 40)]));
        assert!(journal.push(48, 56, 2));
        assert!(journal.push(64, 72, 2));
        assert_eq!(journal.read(&mut slow), None);
        assert_eq!(journal.read(&mut slow), Some(vec![]));
        assert_eq!(journal.read(&mut fast), Some(vec![(48, 56), (64, 72)]));
    }
}
