#[cfg(feature = "python")]
mod bindings;
#[cfg(feature = "python")]
use pyo3::prelude::*;
pub mod bodies;
pub mod catalogs;
pub mod configs;
pub mod elements;
pub mod enums;
pub mod estimation;
pub mod events;
pub mod propagation;
pub mod time;

#[cfg(feature = "cuda")]
pub mod gpu;

#[cfg(test)]
mod test_lock;
use rayon::current_num_threads;

pub fn get_thread_count() -> usize {
    current_num_threads()
}

pub fn set_thread_count(count: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(count)
        .build_global()
        .unwrap();
}

#[cfg(feature = "python")]
#[pymodule]
fn _keplemon(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    bindings::register_bindings(parent_module)?;
    Ok(())
}
