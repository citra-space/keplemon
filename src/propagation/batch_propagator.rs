//! Batch propagation with automatic CPU/GPU backend selection

use crate::elements::{CartesianState, TLE};
#[cfg(feature = "cuda")]
use crate::gpu::{CudaSgp4Propagator, TleDataGpu};
use crate::time::Epoch;

/// Backend selection for batch propagation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropagationBackend {
    /// Automatically select based on problem size and CUDA availability
    #[default]
    Auto,
    /// Force CPU-only propagation
    Cpu,
    /// Force GPU propagation (requires CUDA feature)
    #[cfg(feature = "cuda")]
    Gpu,
}

/// Configuration for batch propagation
#[derive(Debug, Clone)]
pub struct BatchPropagatorConfig {
    /// Backend selection strategy
    pub backend: PropagationBackend,

    /// Threshold for using GPU (n_satellites * n_times)
    /// Only used when backend is Auto
    pub gpu_threshold: usize,
}

impl Default for BatchPropagatorConfig {
    fn default() -> Self {
        Self {
            backend: PropagationBackend::Auto,
            gpu_threshold: 1000, // Use GPU when > 1000 total propagations
        }
    }
}

/// Batch propagator that automatically selects CPU or GPU backend
pub struct BatchPropagator {
    config: BatchPropagatorConfig,
    #[cfg(feature = "cuda")]
    gpu_available: bool,
}

impl BatchPropagator {
    /// Create a new batch propagator with default configuration
    pub fn new() -> Self {
        Self::with_config(BatchPropagatorConfig::default())
    }

    /// Create a batch propagator with custom configuration
    pub fn with_config(config: BatchPropagatorConfig) -> Self {
        #[cfg(feature = "cuda")]
        let gpu_available = CudaSgp4Propagator::is_cuda_available();

        Self {
            config,
            #[cfg(feature = "cuda")]
            gpu_available,
        }
    }

    /// Set the backend selection strategy
    pub fn set_backend(mut self, backend: PropagationBackend) -> Self {
        self.config.backend = backend;
        self
    }

    /// Set the GPU threshold for auto-selection
    pub fn set_gpu_threshold(mut self, threshold: usize) -> Self {
        self.config.gpu_threshold = threshold;
        self
    }

    /// Determine which backend should be used for this propagation
    pub fn select_backend(&self, n_satellites: usize, n_times: usize) -> SelectedBackend {
        let _total_ops = n_satellites * n_times;

        match self.config.backend {
            PropagationBackend::Cpu => SelectedBackend::Cpu,

            #[cfg(feature = "cuda")]
            PropagationBackend::Gpu => {
                if self.gpu_available {
                    SelectedBackend::Gpu
                } else {
                    log::warn!("GPU backend requested but CUDA not available, falling back to CPU");
                    SelectedBackend::Cpu
                }
            }

            PropagationBackend::Auto => {
                #[cfg(feature = "cuda")]
                {
                    if self.gpu_available && _total_ops >= self.config.gpu_threshold {
                        log::debug!(
                            "Auto-selected GPU backend: {} satellites × {} times = {} operations (threshold: {})",
                            n_satellites,
                            n_times,
                            _total_ops,
                            self.config.gpu_threshold
                        );
                        SelectedBackend::Gpu
                    } else {
                        log::debug!(
                            "Auto-selected CPU backend: {} satellites × {} times = {} operations",
                            n_satellites,
                            n_times,
                            _total_ops
                        );
                        SelectedBackend::Cpu
                    }
                }

                #[cfg(not(feature = "cuda"))]
                {
                    SelectedBackend::Cpu
                }
            }
        }
    }

    /// Check if GPU is available
    pub fn is_gpu_available(&self) -> bool {
        #[cfg(feature = "cuda")]
        return self.gpu_available;

        #[cfg(not(feature = "cuda"))]
        return false;
    }

    /// Get current configuration
    pub fn config(&self) -> &BatchPropagatorConfig {
        &self.config
    }
}

impl Default for BatchPropagator {
    fn default() -> Self {
        Self::new()
    }
}

/// The selected backend for a propagation operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBackend {
    Cpu,
    #[cfg(feature = "cuda")]
    Gpu,
}

impl std::fmt::Display for SelectedBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectedBackend::Cpu => write!(f, "CPU"),
            #[cfg(feature = "cuda")]
            SelectedBackend::Gpu => write!(f, "GPU"),
        }
    }
}

// ============================================================================
// Batch Propagation Implementation
// ============================================================================

impl BatchPropagator {
    /// Propagate multiple TLEs to multiple epochs
    ///
    /// This method automatically selects CPU or GPU backend based on problem size.
    ///
    /// # Arguments
    /// * `tles` - Array of TLEs to propagate
    /// * `epochs` - Array of epochs to propagate to
    ///
    /// # Returns
    /// 2D vector of states: `result[sat_idx][epoch_idx]`
    pub fn propagate_batch(&self, tles: &[TLE], epochs: &[Epoch]) -> Result<Vec<Vec<CartesianState>>, String> {
        if tles.is_empty() {
            return Ok(Vec::new());
        }

        if epochs.is_empty() {
            return Ok(vec![Vec::new(); tles.len()]);
        }

        let backend = self.select_backend(tles.len(), epochs.len());

        match backend {
            #[cfg(feature = "cuda")]
            SelectedBackend::Gpu => self.propagate_batch_gpu(tles, epochs),

            SelectedBackend::Cpu => self.propagate_batch_cpu(tles, epochs),
        }
    }

    /// CPU-based batch propagation using existing SGP4 implementation
    fn propagate_batch_cpu(&self, tles: &[TLE], epochs: &[Epoch]) -> Result<Vec<Vec<CartesianState>>, String> {
        tles.iter()
            .map(|tle| {
                epochs
                    .iter()
                    .map(|epoch| {
                        tle.get_cartesian_state_at_epoch(*epoch)
                            .map_err(|e| format!("CPU propagation failed: {}", e))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect()
    }

    /// GPU-based batch propagation using CUDA SGP4
    #[cfg(feature = "cuda")]
    fn propagate_batch_gpu(&self, tles: &[TLE], epochs: &[Epoch]) -> Result<Vec<Vec<CartesianState>>, String> {
        use crate::elements::CartesianVector;
        use crate::enums::ReferenceFrame;

        // Initialize GPU propagator
        let mut gpu_propagator = CudaSgp4Propagator::new().map_err(|e| format!("Failed to initialize CUDA: {}", e))?;

        // Convert TLEs to GPU format
        let tle_data: Vec<TleDataGpu> = tles.iter().map(|tle| TleDataGpu::from(tle)).collect();

        // Initialize satellites on GPU
        gpu_propagator
            .init_satellites(&tle_data)
            .map_err(|e| format!("Failed to initialize satellites on GPU: {}", e))?;

        // Convert epochs to Julian Dates
        let jd_times: Vec<f64> = epochs
            .iter()
            .map(|epoch| TleDataGpu::jd_from_ds50(epoch.days_since_1950))
            .collect();

        // Propagate on GPU
        let gpu_states = gpu_propagator
            .propagate(&jd_times)
            .map_err(|e| format!("GPU propagation failed: {}", e))?;

        // Convert GPU states back to CartesianState format
        // GPU returns flattened array: [sat0_t0, sat0_t1, ..., sat1_t0, sat1_t1, ...]
        let n_epochs = epochs.len();
        let results: Vec<Vec<CartesianState>> = tles
            .iter()
            .enumerate()
            .map(|(sat_idx, _tle)| {
                epochs
                    .iter()
                    .enumerate()
                    .map(|(time_idx, epoch)| {
                        let state_idx = sat_idx * n_epochs + time_idx;
                        let gpu_state = &gpu_states[state_idx];

                        // Check for propagation errors
                        if gpu_state.error_code != 0 {
                            return Err(format!(
                                "Satellite {} at epoch {}: error code {}",
                                sat_idx, time_idx, gpu_state.error_code
                            ));
                        }

                        Ok(CartesianState::new(
                            *epoch,
                            CartesianVector::new(gpu_state.x, gpu_state.y, gpu_state.z),
                            CartesianVector::new(gpu_state.vx, gpu_state.vy, gpu_state.vz),
                            ReferenceFrame::TEME,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(results)
    }

    /// Propagate and return GPU-resident data (CUDA feature only)
    ///
    /// This method is only available with the `cuda` feature flag and when
    /// GPU backend is selected. It returns data that remains on the GPU,
    /// avoiding the GPU→CPU transfer bottleneck.
    ///
    /// **Dual-Mode Design**: This method coexists with `propagate_batch()`.
    /// Use `propagate_batch()` when you need CPU-resident data, and this
    /// method when building GPU-accelerated pipelines.
    ///
    /// # Example
    /// ```no_run
    /// # use keplemon::propagation::BatchPropagator;
    /// # use keplemon::propagation::PropagationBackend;
    /// # use keplemon::elements::TLE;
    /// # use keplemon::time::Epoch;
    /// # let tles: Vec<TLE> = vec![];
    /// # let epochs: Vec<Epoch> = vec![];
    /// let mut propagator = BatchPropagator::new()
    ///     .set_backend(PropagationBackend::Gpu);
    ///
    /// // Option 1: CPU-resident (existing API)
    /// let cpu_results = propagator.propagate_batch(&tles, &epochs)?;
    ///
    /// // Option 2: GPU-resident (new API, large-scale pipelines)
    /// # #[cfg(feature = "cuda")]
    /// {
    ///     let gpu_results = propagator.propagate_batch_gpu_resident(&tles, &epochs)?;
    ///     // Process on GPU...
    /// }
    /// # Ok::<(), String>(())
    /// ```
    ///
    /// # Returns
    /// `Ok(Sgp4StateSoABuffers)` with GPU-resident buffers, or `Err` if:
    /// - GPU backend is not active
    /// - CUDA feature is disabled
    /// - GPU is not available
    #[cfg(feature = "cuda")]
    pub fn propagate_batch_gpu_resident(
        &mut self,
        tles: &[TLE],
        epochs: &[Epoch],
    ) -> Result<crate::gpu::Sgp4StateSoABuffers, String> {
        if tles.is_empty() {
            return Err("Empty TLE array".to_string());
        }

        if epochs.is_empty() {
            return Err("Empty epoch array".to_string());
        }

        let backend = self.select_backend(tles.len(), epochs.len());

        match backend {
            SelectedBackend::Gpu => self.propagate_batch_gpu_resident_impl(tles, epochs),
            SelectedBackend::Cpu => Err("GPU-resident data only available with GPU backend. \
                     Use propagate_batch() for CPU-resident results, or \
                     set backend to GPU/Auto."
                .to_string()),
        }
    }

    #[cfg(feature = "cuda")]
    fn propagate_batch_gpu_resident_impl(
        &mut self,
        tles: &[TLE],
        epochs: &[Epoch],
    ) -> Result<crate::gpu::Sgp4StateSoABuffers, String> {
        use crate::gpu::{CudaSgp4Propagator, TleDataGpu};

        // Initialize GPU propagator
        let mut gpu_propagator = CudaSgp4Propagator::new().map_err(|e| format!("Failed to initialize CUDA: {}", e))?;

        // Convert TLEs to GPU format
        let tle_data: Vec<TleDataGpu> = tles.iter().map(|tle| TleDataGpu::from(tle)).collect();

        // Initialize satellites on GPU
        gpu_propagator
            .init_satellites(&tle_data)
            .map_err(|e| format!("Failed to initialize satellites on GPU: {}", e))?;

        // Convert epochs to Julian Dates
        let jd_times: Vec<f64> = epochs
            .iter()
            .map(|epoch| TleDataGpu::jd_from_ds50(epoch.days_since_1950))
            .collect();

        // Propagate on GPU and return GPU-resident buffers
        gpu_propagator
            .propagate_soa_gpu_resident(&jd_times)
            .map_err(|e| format!("GPU propagation failed: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_selection() {
        let propagator = BatchPropagator::new();

        // Small problem should use CPU
        let backend = propagator.select_backend(10, 10);
        assert_eq!(backend, SelectedBackend::Cpu);

        #[cfg(feature = "cuda")]
        {
            // Large problem should use GPU if available
            let backend = propagator.select_backend(100, 100);
            if propagator.is_gpu_available() {
                assert_eq!(backend, SelectedBackend::Gpu);
            } else {
                assert_eq!(backend, SelectedBackend::Cpu);
            }
        }
    }

    #[test]
    fn test_force_cpu() {
        let propagator = BatchPropagator::new().set_backend(PropagationBackend::Cpu);

        let backend = propagator.select_backend(1000, 1000);
        assert_eq!(backend, SelectedBackend::Cpu);
    }

    #[test]
    fn test_custom_threshold() {
        let propagator = BatchPropagator::new().set_gpu_threshold(10000);

        let backend = propagator.select_backend(50, 50); // 2500 < 10000
        assert_eq!(backend, SelectedBackend::Cpu);
    }
}
