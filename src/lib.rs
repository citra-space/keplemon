pub mod bodies;
pub mod catalogs;
pub mod configs;
pub mod elements;
pub mod enums;
pub mod estimation;
pub mod events;
pub mod exceptions;
pub mod propagation;
pub mod saal;
pub mod time;

use ctor::ctor;
use pyo3::prelude::*;
use rayon::current_num_threads;

#[ctor]
fn init() {
    saal::MainInterface::set_key_mode(enums::SAALKeyMode::DirectMemoryAccess);
}

#[pyfunction]
fn get_thread_count() -> usize {
    current_num_threads()
}

#[pyfunction]
fn set_thread_count(count: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(count)
        .build_global()
        .unwrap();
}

// The top-level module that includes functions and nested submodules.
#[pymodule]
fn _keplemon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(get_thread_count, m)?)?;
    m.add_function(wrap_pyfunction!(set_thread_count, m)?)?;
    m.add_function(wrap_pyfunction!(saal::sgp4_prop_interface::set_license_file_path, m)?)?;
    m.add_function(wrap_pyfunction!(saal::sgp4_prop_interface::get_license_file_path, m)?)?;
    m.add_function(wrap_pyfunction!(
        saal::astro_func_interface::py_set_jpl_ephemeris_file_path,
        m
    )?)?;
    saal::register_saal(m)?;
    saal::astro_func_interface::register_astro_func_interface(m)?;
    saal::time_func_interface::register_time_func_interface(m)?;
    enums::register_enums(m)?;
    time::register_time(m)?;
    elements::register_elements(m)?;
    propagation::register_propagation(m)?;
    catalogs::register_catalogs(m)?;
    bodies::register_bodies(m)?;
    events::register_events(m)?;
    estimation::register_estimation(m)?;
    exceptions::register_exceptions(m)?;
    Ok(())
}
