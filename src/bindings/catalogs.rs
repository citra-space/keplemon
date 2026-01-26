mod tle_catalog;
use pyo3::prelude::*;
use pyo3::py_run;
pub use tle_catalog::PyTLECatalog;

pub fn register_catalogs(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let catalogs = PyModule::new(parent_module.py(), "catalogs")?;
    catalogs.add_class::<PyTLECatalog>()?;
    py_run!(
        parent_module.py(),
        catalogs,
        "import sys; sys.modules['keplemon._keplemon.catalogs'] = catalogs"
    );
    parent_module.add_submodule(&catalogs)
}
