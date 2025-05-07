pub mod astro_func_interface;
pub mod env_const_interface;
pub mod ext_ephem_interface;
pub mod main_interface;
pub mod sat_state_interface;
pub mod sgp4_prop_interface;
pub mod time_func_interface;
pub mod tle_interface;

mod get_set_string;
pub use get_set_string::GetSetString;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_saal(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let saal = PyModule::new(parent_module.py(), "saal")?;
    saal.add_function(wrap_pyfunction!(main_interface::get_key_mode, &saal)?)?;
    saal.add_function(wrap_pyfunction!(main_interface::set_key_mode, &saal)?)?;
    py_run!(
        parent_module.py(),
        saal,
        "import sys; sys.modules['keplemon._keplemon.saal'] = saal"
    );

    parent_module.add_submodule(&saal)
}
