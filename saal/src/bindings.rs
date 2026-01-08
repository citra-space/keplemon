#![cfg(feature = "python")]

use pyo3::prelude::*;

mod astro_interface;
mod environment_interface;
mod main_interface;
mod obs_interface;
mod sensor_interface;
mod sgp4_interface;
mod time_interface;
mod tle_interface;

pub fn register_bindings(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    main_interface::register_main_interface(parent_module)?;
    astro_interface::register_astro_interface(parent_module)?;
    environment_interface::register_environment_interface(parent_module)?;
    obs_interface::register_obs_interface(parent_module)?;
    sensor_interface::register_sensor_interface(parent_module)?;
    sgp4_interface::register_sgp4_interface(parent_module)?;
    time_interface::register_time_func_interface(parent_module)?;
    tle_interface::register_tle_interface(parent_module)?;
    Ok(())
}
