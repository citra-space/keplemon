use std::sync::{Mutex, MutexGuard, PoisonError};

pub static GLOBAL_TEST_LOCK: Mutex<()> = Mutex::new(());

/// Helper to acquire the test lock, handling poison errors gracefully.
/// Since the lock protects no actual data (just serializes SAAL calls),
/// poison state is not meaningful and can be safely ignored.
pub fn lock_for_test() -> MutexGuard<'static, ()> {
    GLOBAL_TEST_LOCK.lock().unwrap_or_else(PoisonError::into_inner)
}
