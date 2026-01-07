#![cfg(feature = "python")]

use crate::{get_thread_count, set_thread_count};
use pyo3::prelude::*;

mod bodies;
mod catalogs;
mod elements;
mod enums;
mod estimation;
mod events;
mod propagation;
mod time;

#[pyfunction(name = "get_thread_count")]
pub fn get_thread_count_py() -> PyResult<usize> {
    Ok(get_thread_count())
}

#[pyfunction(name = "set_thread_count")]
pub fn set_thread_count_py(count: usize) -> PyResult<()> {
    set_thread_count(count);
    Ok(())
}

#[pymodule]
pub fn register_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    enums::register_enums(m)?;
    time::register_time(m)?;
    elements::register_elements(m)?;
    bodies::register_bodies(m)?;
    catalogs::register_catalogs(m)?;
    events::register_events(m)?;
    propagation::register_propagation(m)?;
    estimation::register_estimation(m)?;
    m.add_function(wrap_pyfunction!(get_thread_count_py, m)?)?;
    m.add_function(wrap_pyfunction!(set_thread_count_py, m)?)?;
    Ok(())
}
