//! Batch propagation with automatic CPU/GPU backend selection

use crate::elements::{CartesianState, TLE};
#[cfg(feature = "cuda")]
use crate::gpu::{CudaTlePropagator, Sgp4StateGpu, TleDataGpu};
use crate::time::Epoch;
use rayon::prelude::*;

/// Orbit classification for propagator selection
#[cfg(feature = "cuda")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrbitType {
    /// Near-earth orbit (period < 225 minutes) - use SGP4
    NearEarth,
    /// Deep-space orbit (period >= 225 minutes) - use SDP4 or analytical variants
    DeepSpace,
}

/// Classify orbit type based on mean motion
#[cfg(feature = "cuda")]
fn classify_orbit(mean_motion: f64) -> OrbitType {
    let period_minutes = orbital_period_minutes(mean_motion);
    if period_minutes < 225.0 {
        OrbitType::NearEarth
    } else {
        OrbitType::DeepSpace
    }
}

/// Calculate orbital period in minutes from mean motion (revolutions per day)
pub fn orbital_period_minutes(mean_motion: f64) -> f64 {
    1440.0 / mean_motion
}

/// Check if orbit is deep-space (period >= 225 minutes)
pub fn is_deep_space_orbit(mean_motion: f64) -> bool {
    orbital_period_minutes(mean_motion) >= 225.0
}

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
    #[cfg(feature = "cuda")]
    cached_propagator: Option<CudaTlePropagator>,
}

impl BatchPropagator {
    /// Create a new batch propagator with default configuration
    pub fn new() -> Self {
        Self::with_config(BatchPropagatorConfig::default())
    }

    /// Create a batch propagator with custom configuration
    pub fn with_config(config: BatchPropagatorConfig) -> Self {
        #[cfg(feature = "cuda")]
        let gpu_available = CudaTlePropagator::is_cuda_available();

        Self {
            config,
            #[cfg(feature = "cuda")]
            gpu_available,
            #[cfg(feature = "cuda")]
            cached_propagator: None,
        }
    }

    /// Create a batch propagator that shares a CUDA device with other components
    ///
    /// This avoids creating a separate CUDA context, allowing the propagator
    /// to share GPU memory and synchronization with other GPU consumers.
    #[cfg(feature = "cuda")]
    pub fn with_device(device: std::sync::Arc<cudarc::driver::CudaDevice>) -> Result<Self, String> {
        let cuda_device = crate::gpu::CudaDevice::from_arc(device);
        let propagator = CudaTlePropagator::new_with_device(cuda_device)
            .map_err(|e| format!("Failed to initialize CUDA propagator: {}", e))?;

        Ok(Self {
            config: BatchPropagatorConfig::default(),
            gpu_available: true,
            cached_propagator: Some(propagator),
        })
    }

    /// Get reference to the underlying CUDA device (if GPU is active)
    #[cfg(feature = "cuda")]
    pub fn cuda_device(&self) -> Option<&std::sync::Arc<cudarc::driver::CudaDevice>> {
        self.cached_propagator.as_ref().map(|p| p.cuda_device())
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

    /// CPU-based batch propagation using existing SGP4 implementation with rayon parallelization
    fn propagate_batch_cpu(&self, tles: &[TLE], epochs: &[Epoch]) -> Result<Vec<Vec<CartesianState>>, String> {
        // Use rayon's par_iter for parallel propagation across satellites
        // Each satellite's propagation is independent, making this embarrassingly parallel
        tles.par_iter()
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

    /// GPU-based batch propagation with automatic propagator selection
    #[cfg(feature = "cuda")]
    fn propagate_batch_gpu(&self, tles: &[TLE], epochs: &[Epoch]) -> Result<Vec<Vec<CartesianState>>, String> {
        // Classify each orbit
        let orbit_types: Vec<OrbitType> = tles.iter().map(|tle| classify_orbit(tle.get_mean_motion())).collect();

        // Check if all satellites are the same type for optimization
        let all_near_earth = orbit_types.iter().all(|t| matches!(t, OrbitType::NearEarth));
        let all_deep_space = orbit_types.iter().all(|t| matches!(t, OrbitType::DeepSpace));

        if all_near_earth {
            log::debug!("All {} satellites are near-earth, using SGP4", tles.len());
            self.propagate_with_sgp4(tles, epochs)
        } else if all_deep_space {
            log::debug!("All {} satellites are deep-space, using SDP4-Analytical", tles.len());
            self.propagate_with_sdp4_analytical(tles, epochs)
        } else {
            log::debug!(
                "Mixed orbit types: {} satellites, using partitioned approach",
                tles.len()
            );
            self.propagate_mixed(tles, epochs, &orbit_types)
        }
    }

    /// Propagate near-earth satellites with SGP4
    #[cfg(feature = "cuda")]
    fn propagate_with_sgp4(&self, tles: &[TLE], epochs: &[Epoch]) -> Result<Vec<Vec<CartesianState>>, String> {
        use crate::elements::CartesianVector;
        use crate::enums::ReferenceFrame;

        // Initialize GPU propagator
        let mut gpu_propagator = CudaTlePropagator::new().map_err(|e| format!("Failed to initialize CUDA: {}", e))?;

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

    /// Propagate deep-space satellites with SDP4-Analytical
    #[cfg(feature = "cuda")]
    fn propagate_with_sdp4_analytical(
        &self,
        tles: &[TLE],
        epochs: &[Epoch],
    ) -> Result<Vec<Vec<CartesianState>>, String> {
        use crate::elements::CartesianVector;
        use crate::enums::ReferenceFrame;
        use crate::gpu::CudaSdp4InterpolatedPropagator;

        // Initialize SDP4-Analytical propagator
        let mut propagator = CudaSdp4InterpolatedPropagator::new()
            .map_err(|e| format!("Failed to create SDP4-Analytical propagator: {}", e))?;

        // Convert TLEs to GPU format
        let tle_data: Vec<TleDataGpu> = tles.iter().map(|tle| TleDataGpu::from(tle)).collect();

        // Initialize satellites
        propagator
            .init_satellites(&tle_data)
            .map_err(|e| format!("Failed to initialize SDP4-Analytical satellites: {}", e))?;

        // Convert epochs to Julian Dates
        let jd_times: Vec<f64> = epochs
            .iter()
            .map(|epoch| TleDataGpu::jd_from_ds50(epoch.days_since_1950))
            .collect();

        // Propagate using SDP4-Analytical
        let soa = propagator
            .propagate_soa_arrays(&jd_times)
            .map_err(|e| format!("SDP4-Analytical propagation failed: {}", e))?;

        // Convert SoA to AoS
        let gpu_states: Vec<Sgp4StateGpu> = (0..soa.x.len())
            .map(|i| Sgp4StateGpu {
                x: soa.x[i],
                y: soa.y[i],
                z: soa.z[i],
                vx: soa.vx[i],
                vy: soa.vy[i],
                vz: soa.vz[i],
                error_code: soa.error_code[i],
                _padding: 0,
            })
            .collect();

        // Convert to CartesianState format (same layout as SGP4)
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
                                "Satellite {} at epoch {}: SDP4-Analytical error code {}",
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

    /// Propagate mixed orbit types by partitioning and combining results
    #[cfg(feature = "cuda")]
    fn propagate_mixed(
        &self,
        tles: &[TLE],
        epochs: &[Epoch],
        orbit_types: &[OrbitType],
    ) -> Result<Vec<Vec<CartesianState>>, String> {
        // Partition satellites by orbit type
        let mut near_earth_indices = Vec::new();
        let mut deep_space_indices = Vec::new();

        for (idx, orbit_type) in orbit_types.iter().enumerate() {
            match orbit_type {
                OrbitType::NearEarth => near_earth_indices.push(idx),
                OrbitType::DeepSpace => deep_space_indices.push(idx),
            }
        }

        log::debug!(
            "Partitioned {} satellites: {} near-earth (SGP4), {} deep-space (SDP4-Analytical)",
            tles.len(),
            near_earth_indices.len(),
            deep_space_indices.len()
        );

        // Initialize result vector with correct structure
        let mut results: Vec<Vec<CartesianState>> = vec![vec![]; tles.len()];

        // Propagate near-earth satellites if any
        if !near_earth_indices.is_empty() {
            let near_earth_tles: Vec<&TLE> = near_earth_indices.iter().map(|&idx| &tles[idx]).collect();

            let near_earth_results = self.propagate_with_sgp4(
                &near_earth_tles.iter().map(|&tle| tle.clone()).collect::<Vec<_>>(),
                epochs,
            )?;

            // Insert results back into correct positions
            for (result_idx, &original_idx) in near_earth_indices.iter().enumerate() {
                results[original_idx] = near_earth_results[result_idx].clone();
            }
        }

        // Propagate deep-space satellites if any
        if !deep_space_indices.is_empty() {
            let deep_space_tles: Vec<&TLE> = deep_space_indices.iter().map(|&idx| &tles[idx]).collect();

            let deep_space_results = self.propagate_with_sdp4_analytical(
                &deep_space_tles.iter().map(|&tle| tle.clone()).collect::<Vec<_>>(),
                epochs,
            )?;

            // Insert results back into correct positions
            for (result_idx, &original_idx) in deep_space_indices.iter().enumerate() {
                results[original_idx] = deep_space_results[result_idx].clone();
            }
        }

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

    /// GPU-resident propagation using CudaTlePropagator's two-kernel indexed scatter.
    ///
    /// Mixed LEO/GEO batches are fully supported — CudaTlePropagator partitions
    /// satellites by orbit type and launches separate SGP4/SDP4 kernels that
    /// scatter results into the correct output positions.
    ///
    /// Note: deep-space satellites use standard SDP4 (~1.6x GPU speedup) rather
    /// than SDP4-Interpolated (~20-50x) for accuracy in GPU pipelines.
    #[cfg(feature = "cuda")]
    fn propagate_batch_gpu_resident_impl(
        &mut self,
        tles: &[TLE],
        epochs: &[Epoch],
    ) -> Result<crate::gpu::Sgp4StateSoABuffers, String> {
        use crate::gpu::TleDataGpu;

        // Lazily create the propagator if not cached
        if self.cached_propagator.is_none() {
            self.cached_propagator =
                Some(CudaTlePropagator::new().map_err(|e| format!("Failed to initialize CUDA: {}", e))?);
        }

        let propagator = self.cached_propagator.as_mut().unwrap();

        // Convert TLEs to GPU format
        let tle_data: Vec<TleDataGpu> = tles.iter().map(|tle| TleDataGpu::from(tle)).collect();

        // Convert epochs to Julian Dates
        let jd_times: Vec<f64> = epochs
            .iter()
            .map(|epoch| TleDataGpu::jd_from_ds50(epoch.days_since_1950))
            .collect();

        propagator
            .init_satellites(&tle_data)
            .map_err(|e| format!("Failed to initialize satellites on GPU: {}", e))?;

        propagator
            .propagate_soa_resident(&jd_times)
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
