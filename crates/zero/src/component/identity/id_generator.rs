use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

/* ───────────────── global, thread-safe singleton ─────────────── */
static GLOBAL_ID_GEN: OnceLock<IdGenerator> = OnceLock::new();
/// Get the global counter, initialising it on first use.
///
#[inline]
pub fn global_id_gen() -> &'static IdGenerator {
    GLOBAL_ID_GEN.get_or_init(IdGenerator::default)
}

/// Thread-safe generator that hands out monotonically-increasing `u32`s.
///
/// *When the counter reaches `u32::MAX` it wraps to `1` and emits a single
/// `warn!` log message.  After the wrap duplicated IDs **will** appear again;
/// decide at a higher level if that is acceptable for your application.*
#[derive(Debug)]
pub struct IdGenerator {
    counter: AtomicU32,
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self {
            counter: AtomicU32::new(0),     // first call → returns 1
        }
    }
}

impl IdGenerator {
    /// Get the next ID.  Always succeeds; may wrap after ~4 billion calls.
    pub fn next_id(&self) -> u32 {
        let prev = self.counter.fetch_add(1, Ordering::Relaxed);
        let id   = prev.wrapping_add(1);    // 0 → 1, u32::MAX → 0

        if id == 0 {
            // We just produced 0, which means `prev` was u32::MAX.
            // Reset to 1 for the *next* caller, and warn exactly once.
            println!("IdGenerator wrapped around after u32::MAX; restarting at 1");
            // Try to change the zero back to 1, ignore race if another thread did it.
            let _ = self
                .counter
                .compare_exchange(0, 1, Ordering::Relaxed, Ordering::Relaxed);
            1                                   // return 1 this call
        } else {
            id
        }
    }
}

/* -----------------------------------------------------------
	Testing wrap around.
	This test is here because it uses private member of the struct.
	(tests live in same module, so we may touch the private field)
	wrap-around: counter == u32::MAX  → id == 1, then 2
----------------------------------------------------------- */
#[cfg(test)]
mod tests
{
use std::sync::atomic::Ordering;
use super::IdGenerator;

#[test]
fn wraps_back_to_one() {
	let g = IdGenerator::default();
	g.counter.store(u32::MAX, Ordering::Relaxed);   // Force the internal counter to its last value:
	assert_eq!(g.next_id(), 1);                     // First call after wrap: returns 1 (and logs a warn!)
	assert_eq!(g.next_id(), 2);                     // Second call: 2
	}
}
