use std::sync::{
    Arc, Mutex, OnceLock, Weak,
    atomic::{AtomicUsize, Ordering},
};

use crate::descriptor::{
    Heap, View,
    host::{HostHeap, buffer_copy},
};
use crate::host::HostList;
use windows::Win32::Graphics::Direct3D12::D3D12_RANGE;

const LIMIT: usize = 8;
static OUTSTANDING: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
pub(crate) struct Mailbox(Arc<Mutex<Vec<Observation>>>);
struct Observation {
    frame: u64,
    record: u32,
    shader: String,
    source_slot: u32,
    source_view: View,
    source: Weak<crate::shadow::Shadow>,
    slots: Vec<(u32, u32)>,
}
struct Ticket;
impl Drop for Ticket {
    fn drop(&mut self) {
        OUTSTANDING.fetch_sub(1, Ordering::Relaxed);
    }
}
pub(crate) fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("VICTORY_MATERIAL_TEXTURE_TRACE").is_ok_and(|v| v == "1"))
}
fn fields(hash: &str) -> Option<&'static [u32]> {
    match hash {
        "a402e650d0acbc92" => Some(&[0, 4, 8, 12, 16, 20, 24]),
        "b15ba4896d650511" => Some(&[0, 4, 8, 12, 16, 20, 24, 28, 32]),
        _ => None,
    }
}
fn byte_range(record: u32, fields: &[u32], len: u64) -> Option<(u64, usize)> {
    let bytes = fields.last()?.checked_add(4)?;
    let end = record.checked_add(bytes)?;
    (record.is_multiple_of(4) && u64::from(end) <= len)
        .then_some((u64::from(record), bytes as usize))
}
fn indices(words: &[u32], fields: &[u32], count: u32) -> Vec<(u32, u32)> {
    fields
        .iter()
        .filter_map(|&field| {
            let slot = words.get((field / 4) as usize)?.wrapping_shl(5) >> 5;
            (slot < count).then_some((field, slot))
        })
        .collect()
}
fn take_sample(entries: &mut Vec<(usize, u32, String)>, key: (usize, u32, String)) -> bool {
    if entries.iter().filter(|entry| entry.2 == key.2).count() >= LIMIT / 2
        || entries.contains(&key)
    {
        return false;
    }
    entries.push(key);
    true
}
pub(crate) fn record(list: &HostList, heap: &Heap, table: u64, record: u32, shader: &str) {
    if !enabled() {
        return;
    }
    let Some(fields) = fields(shader) else { return };
    let frame = crate::queue::presented() + 1;
    if !crate::capture::listed(frame) {
        return;
    }
    static BUDGET: Mutex<(u64, Vec<(usize, u32, String)>)> = Mutex::new((0, Vec::new()));
    {
        let mut budget = BUDGET.lock().unwrap_or_else(|e| e.into_inner());
        if budget.0 != frame {
            *budget = (frame, Vec::new());
        }
        let key = (std::ptr::from_ref(heap) as usize, record, shader.to_owned());
        if !take_sample(&mut budget.1, key) {
            return;
        }
    }
    let Some(handle) = table.checked_add(688 * 4) else {
        return;
    };
    let Some(slot) = heap.index(handle) else {
        return;
    };
    if heap.handle(slot) != Some(handle) {
        return;
    }
    let Some(host) = heap.host() else { return };
    let Some(view @ View::Placed(false, _, _)) = crate::descriptor::view(handle) else {
        return;
    };
    let Some((source, address, len)) = buffer_copy(&view) else {
        return;
    };
    let Some((at, bytes)) = byte_range(record, fields, len) else {
        return;
    };
    let Some(at) = address.checked_add(at) else {
        return;
    };
    if source.offset_in(at).is_none()
        || source.left(at) < bytes as u64
        || host.shows(slot, &source.buffer) != Some(true)
    {
        return;
    }
    let Some(gpu) = crate::host::host() else {
        return;
    };
    if OUTSTANDING
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
            (n < LIMIT).then_some(n + 1)
        })
        .is_err()
    {
        return;
    }
    let ticket = Ticket;
    let Some(back) = gpu.read_back_buffer(bytes as u64) else {
        return;
    };
    list.buffer_state(&source.buffer, crate::host::BUFFER_READ);
    // The bounded source is the live GPU buffer bound at this draw.
    unsafe {
        list.list
            .CopyBufferRegion(&back, 0, &source.buffer, source.offset(at), bytes as u64);
    }
    list.keep(&source.buffer);
    list.keep(&back);
    let weak_source = Arc::downgrade(&source);
    let mailbox = Arc::downgrade(&host.material_trace.0);
    let count = host.count();
    let shader = shader.to_owned();
    list.defer_after(move || {
        let _ticket = ticket;
        let Some(mailbox) = mailbox.upgrade() else {
            return;
        };
        let mut ready = mailbox.lock().unwrap_or_else(|e| e.into_inner());
        if ready.len() >= LIMIT {
            return;
        }
        let mut data = std::ptr::null_mut();
        let range = D3D12_RANGE {
            Begin: 0,
            End: bytes,
        };
        // Completion follows the fence; the readback allocation holds every word.
        unsafe {
            if back.Map(0, Some(&range), Some(&mut data)).is_err() {
                return;
            }
            if !data.is_null() {
                let words = std::slice::from_raw_parts(data.cast::<u32>(), bytes / 4);
                ready.push(Observation {
                    frame,
                    record,
                    shader,
                    source_slot: slot,
                    source_view: view,
                    source: weak_source,
                    slots: indices(words, fields, count),
                });
            }
            back.Unmap(0, Some(&D3D12_RANGE { Begin: 0, End: 0 }));
        }
    });
}
pub(crate) fn report(heap: &Heap, host: &HostHeap, frame: u64) {
    if !enabled() {
        return;
    }
    let ready = std::mem::take(
        &mut *host
            .material_trace
            .0
            .lock()
            .unwrap_or_else(|e| e.into_inner()),
    );
    for obs in ready {
        let current = frame.abs_diff(obs.frame) <= 4
            && heap
                .handle(obs.source_slot)
                .and_then(crate::descriptor::view)
                .as_ref()
                == Some(&obs.source_view)
            && obs
                .source
                .upgrade()
                .is_some_and(|s| host.shows(obs.source_slot, &s.buffer) == Some(true));
        for (offset, slot) in obs.slots {
            tracing::info!(shader = obs.shader, draw_frame = obs.frame, record = obs.record, offset, slot, source_current = current, resident = host.slot_resident(slot), view = ?host.describe_slot(slot), "character material texture");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn each_character_shader_gets_samples_when_draw_order_is_fixed() {
        let mut entries = Vec::new();
        for shader in ["a402e650d0acbc92", "b15ba4896d650511"] {
            for record in 0..16 {
                take_sample(&mut entries, (1, record, shader.to_owned()));
            }
        }
        assert_eq!(entries.len(), LIMIT);
        for shader in ["a402e650d0acbc92", "b15ba4896d650511"] {
            assert_eq!(
                entries.iter().filter(|entry| entry.2 == shader).count(),
                LIMIT / 2
            );
        }
    }

    #[test]
    fn character_records_are_bounded_and_indices_follow_wrapping_shift() {
        let regular = fields("a402e650d0acbc92").unwrap();
        let packed = fields("b15ba4896d650511").unwrap();
        assert_eq!(byte_range(4, regular, 32), Some((4, 28)));
        assert_eq!(byte_range(4, packed, 32), None);
        assert_eq!(byte_range(4, packed, 40), Some((4, 36)));
        assert_eq!(byte_range(3, packed, 40), None);
        assert_eq!(byte_range(u32::MAX - 3, packed, u64::MAX), None);
        assert_eq!(
            indices(&[0xf800_0003, 7, 90], &[0, 4, 8, 12], 8),
            [(0, 3), (4, 7)]
        );
        assert!(fields("unknown").is_none());
    }
}
