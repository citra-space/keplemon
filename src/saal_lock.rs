use std::sync::Mutex;

static SAAL_SGP4_KEY_LOCK: Mutex<()> = Mutex::new(());

pub fn with_sgp4_key_lock<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = SAAL_SGP4_KEY_LOCK.lock().expect("sgp4 key lock poisoned");
    f()
}
