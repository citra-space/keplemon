//! CUDA device management and kernel loading

use cudarc::driver::{CudaDevice as CudarcDevice, LaunchConfig};
use std::sync::Arc;

/// Wrapper around CUDA device with kernel management  
pub struct CudaDevice {
    device: Arc<CudarcDevice>,
}

impl CudaDevice {
    /// Create a new CUDA device
    pub fn new() -> Result<Self, CudaError> {
        Self::new_with_device_id(0)
    }

    /// Create CUDA device with specific device ID
    pub fn new_with_device_id(device_id: usize) -> Result<Self, CudaError> {
        let device = CudarcDevice::new(device_id).map_err(|e| CudaError::DeviceInitialization(e.to_string()))?;

        Ok(Self { device })
    }

    /// Get reference to the underlying device
    pub fn device(&self) -> &Arc<CudarcDevice> {
        &self.device
    }

    /// Calculate optimal launch configuration for given number of elements
    pub fn launch_config_1d(num_elements: usize) -> LaunchConfig {
        let block_size = 256;
        let grid_size = (num_elements as u32 + block_size - 1) / block_size;

        LaunchConfig {
            grid_dim: (grid_size, 1, 1),
            block_dim: (block_size, 1, 1),
            shared_mem_bytes: 0,
        }
    }

    /// Calculate optimal launch configuration for 2D grid (satellites x times)
    pub fn launch_config_2d(n_sats: usize, n_times: usize) -> LaunchConfig {
        let block_x = 16;
        let block_y = 16;
        let grid_x = (n_sats as u32 + block_x - 1) / block_x;
        let grid_y = (n_times as u32 + block_y - 1) / block_y;

        LaunchConfig {
            grid_dim: (grid_x, grid_y, 1),
            block_dim: (block_x, block_y, 1),
            shared_mem_bytes: 0,
        }
    }

    /// Check if CUDA is available on this system
    pub fn is_available() -> bool {
        CudarcDevice::count().is_ok()
    }
}

/// CUDA-specific errors
#[derive(Debug, Clone)]
pub enum CudaError {
    DeviceInitialization(String),
    KernelLoad(String),
    MemoryAllocation(String),
    AllocationFailed(String),
    KernelLaunch(String),
    Synchronization(String),
    InvalidParameter(String),
    NotInitialized,
}

impl std::fmt::Display for CudaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CudaError::DeviceInitialization(msg) => write!(f, "CUDA device initialization failed: {}", msg),
            CudaError::KernelLoad(msg) => write!(f, "Kernel load failed: {}", msg),
            CudaError::MemoryAllocation(msg) => write!(f, "GPU memory allocation failed: {}", msg),
            CudaError::AllocationFailed(msg) => write!(f, "GPU memory allocation failed: {}", msg),
            CudaError::KernelLaunch(msg) => write!(f, "Kernel launch failed: {}", msg),
            CudaError::Synchronization(msg) => write!(f, "Device synchronization failed: {}", msg),
            CudaError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            CudaError::NotInitialized => write!(f, "Propagator not initialized with satellite data"),
        }
    }
}

impl std::error::Error for CudaError {}
