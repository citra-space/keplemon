//! Python bindings for BatchPropagator

use crate::bindings::elements::PyCartesianState;
use crate::bindings::elements::PyTLE;
use crate::bindings::time::PyEpoch;
use crate::elements::TLE;
use crate::propagation::{BatchPropagator, PropagationBackend};
use crate::time::Epoch;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "PropagationBackend")]
#[derive(Debug, Clone, Copy)]
pub enum PyPropagationBackend {
    Auto,
    Cpu,
    #[cfg(feature = "cuda")]
    Gpu,
}

impl From<PyPropagationBackend> for PropagationBackend {
    fn from(value: PyPropagationBackend) -> Self {
        match value {
            PyPropagationBackend::Auto => PropagationBackend::Auto,
            PyPropagationBackend::Cpu => PropagationBackend::Cpu,
            #[cfg(feature = "cuda")]
            PyPropagationBackend::Gpu => PropagationBackend::Gpu,
        }
    }
}

impl From<PropagationBackend> for PyPropagationBackend {
    fn from(value: PropagationBackend) -> Self {
        match value {
            PropagationBackend::Auto => PyPropagationBackend::Auto,
            PropagationBackend::Cpu => PyPropagationBackend::Cpu,
            #[cfg(feature = "cuda")]
            PropagationBackend::Gpu => PyPropagationBackend::Gpu,
        }
    }
}

#[pyclass(name = "BatchPropagator")]
pub struct PyBatchPropagator {
    inner: BatchPropagator,
}

#[pymethods]
impl PyBatchPropagator {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: BatchPropagator::new(),
        }
    }

    /// Set the backend selection strategy
    pub fn set_backend(&mut self, backend: PyPropagationBackend) -> PyResult<()> {
        self.inner = std::mem::take(&mut self.inner).set_backend(backend.into());
        Ok(())
    }

    /// Set the GPU threshold for auto-selection
    pub fn set_gpu_threshold(&mut self, threshold: usize) -> PyResult<()> {
        self.inner = std::mem::take(&mut self.inner).set_gpu_threshold(threshold);
        Ok(())
    }

    /// Check if GPU is available
    pub fn is_gpu_available(&self) -> bool {
        self.inner.is_gpu_available()
    }

    /// Propagate multiple TLEs to multiple epochs
    ///
    /// # Arguments
    /// * `tles` - List of TLEs to propagate
    /// * `epochs` - List of epochs to propagate to
    ///
    /// # Returns
    /// 2D list of states: result[sat_idx][epoch_idx]
    #[pyo3(signature = (tles, epochs))]
    pub fn propagate_batch(
        &self,
        py: Python<'_>,
        tles: Vec<PyTLE>,
        epochs: Vec<PyEpoch>,
    ) -> PyResult<Vec<Vec<PyCartesianState>>> {
        let tles: Vec<TLE> = tles.into_iter().map(|tle| tle.into()).collect();
        let epochs: Vec<Epoch> = epochs.into_iter().map(|e| e.into()).collect();

        py.detach(|| {
            self.inner
                .propagate_batch(&tles, &epochs)
                .map(|results| {
                    results
                        .into_iter()
                        .map(|sat_states| sat_states.into_iter().map(PyCartesianState::from).collect())
                        .collect()
                })
                .map_err(|e| PyValueError::new_err(e))
        })
    }
}

impl Default for PyBatchPropagator {
    fn default() -> Self {
        Self::new()
    }
}
