use std::collections::HashMap;
use std::sync::Arc;

type Span = (u64, u64);

struct Buffer<T> {
    copy: Arc<T>,
    views: HashMap<u32, Span>,
    span: Option<Span>,
}

pub(crate) struct BufferSweep<T> {
    pub(crate) passes: u64,
    slots: HashMap<u32, u64>,
    buffers: HashMap<u64, Buffer<T>>,
}

impl<T> Default for BufferSweep<T> {
    fn default() -> Self {
        Self {
            passes: 0,
            slots: HashMap::new(),
            buffers: HashMap::new(),
        }
    }
}

fn union(a: Span, b: Span) -> Span {
    (a.0.min(b.0), a.1.max(b.1))
}

impl<T> BufferSweep<T> {
    pub(crate) fn clear(&mut self) {
        self.slots.clear();
        self.buffers.clear();
    }

    pub(crate) fn remove(&mut self, slot: u32) {
        if let Some(id) = self.slots.remove(&slot) {
            self.unlink(slot, id);
        }
    }

    fn unlink(&mut self, slot: u32, id: u64) {
        let buffer = self.buffers.get_mut(&id).unwrap();
        let (from, to) = buffer.views.remove(&slot).unwrap();
        if buffer.views.is_empty() {
            self.buffers.remove(&id);
        } else if buffer
            .span
            .is_some_and(|span| span.0 == from || span.1 == to)
        {
            buffer.span = None;
        }
    }

    pub(crate) fn set(&mut self, slot: u32, id: u64, copy: Arc<T>, range: Span) {
        if let Some(old) = self.slots.insert(slot, id)
            && old != id
        {
            self.unlink(slot, old);
        }
        let buffer = self.buffers.entry(id).or_insert_with(|| Buffer {
            copy,
            views: HashMap::new(),
            span: Some(range),
        });
        let previous = buffer.views.insert(slot, range);
        if let Some(span) = buffer.span {
            // Expanding a bound is cheap. A shrinking edge may uncover another view.
            buffer.span = if previous.is_some_and(|(from, to)| {
                (from == span.0 && range.0 > from) || (to == span.1 && range.1 < to)
            }) {
                None
            } else {
                Some(union(span, range))
            };
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.slots.len()
    }

    pub(crate) fn spans(&mut self) -> Vec<(Arc<T>, u64, u64)> {
        self.buffers
            .values_mut()
            .map(|buffer| {
                let (from, to) = *buffer
                    .span
                    .get_or_insert_with(|| buffer.views.values().copied().reduce(union).unwrap());
                (buffer.copy.clone(), from, to)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewritten_and_removed_slots_update_bounds_and_release_copies() {
        let first = Arc::new(());
        let second = Arc::new(());
        let weak = Arc::downgrade(&first);
        let mut sweep = BufferSweep::default();
        sweep.set(1, 10, first.clone(), (0, 12));
        sweep.set(2, 10, first.clone(), (4, 8));
        sweep.set(3, 10, first.clone(), (6, 16));
        assert_eq!(sweep.spans(), [(Arc::new(()), 0, 16)]);

        sweep.set(1, 10, first, (5, 9));
        assert_eq!(sweep.spans(), [(Arc::new(()), 4, 16)]);
        sweep.remove(3);
        assert_eq!(sweep.spans(), [(Arc::new(()), 4, 9)]);
        sweep.set(1, 11, second.clone(), (20, 30));
        sweep.remove(2);
        assert!(weak.upgrade().is_none());
        assert_eq!(sweep.len(), 1);
        let spans = sweep.spans();
        assert_eq!((spans[0].1, spans[0].2), (20, 30));
        assert!(Arc::ptr_eq(&spans[0].0, &second));
        sweep.clear();
        assert!(sweep.spans().is_empty());
        assert_eq!(sweep.len(), 0);
    }
}
