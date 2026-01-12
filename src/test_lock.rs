use std::sync::Mutex;

pub static GLOBAL_TEST_LOCK: Mutex<()> = Mutex::new(());
