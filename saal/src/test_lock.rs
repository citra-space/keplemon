use std::sync::Mutex;

pub static TEST_LOCK: Mutex<()> = Mutex::new(());
