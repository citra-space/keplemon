mod epoch;
mod time_components;
mod time_span;

pub use epoch::PyEpoch;
use pyo3::prelude::*;
use pyo3::py_run;
pub use time_components::PyTimeComponents;
pub use time_span::PyTimeSpan;

pub fn register_time(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let time = PyModule::new(parent_module.py(), "time")?;
    time.add_class::<PyTimeSpan>()?;
    time.add_class::<PyEpoch>()?;
    time.add_class::<PyTimeComponents>()?;
    py_run!(
        parent_module.py(),
        time,
        "import sys; sys.modules['keplemon._keplemon.time'] = time"
    );
    parent_module.add_submodule(&time)
}
