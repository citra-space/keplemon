//! Batch propagation with automatic CPU/GPU backend selection

#[cfg(feature = "cuda")]
use crate::gpu::CudaSgp4Propagator;

/// Backend selection for batch propagation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropagationBackend {
    /// Automatically select based on problem size and CUDA availability
    Auto,
    /// Force CPU-only propagation
    Cpu,
    /// Force GPU propagation (requires CUDA feature)
    #[cfg(feature = "cuda")]
    Gpu,
}

impl Default for PropagationBackend {
    fn default() -> Self {
        PropagationBackend::Auto
    }
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
            gpu_threshold: 1000,  // Use GPU when > 1000 total propagations
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
        let total_ops = n_satellites * n_times;
        
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
                    if self.gpu_available && total_ops >= self.config.gpu_threshold {
                        log::debug!(
                            "Auto-selected GPU backend: {} satellites × {} times = {} operations (threshold: {})",
                            n_satellites, n_times, total_ops, self.config.gpu_threshold
                        );
                        SelectedBackend::Gpu
                    } else {
                        log::debug!(
                            "Auto-selected CPU backend: {} satellites × {} times = {} operations",
                            n_satellites, n_times, total_ops
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
        let propagator = BatchPropagator::new()
            .set_backend(PropagationBackend::Cpu);
        
        let backend = propagator.select_backend(1000, 1000);
        assert_eq!(backend, SelectedBackend::Cpu);
    }
    
    #[test]
    fn test_custom_threshold() {
        let propagator = BatchPropagator::new()
            .set_gpu_threshold(10000);
        
        let backend = propagator.select_backend(50, 50); // 2500 < 10000
        assert_eq!(backend, SelectedBackend::Cpu);
    }
}
