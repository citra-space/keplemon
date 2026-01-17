//! CUDA SGP4 batch propagator implementation

use super::device::{CudaDevice, CudaError};

// Note: For now, this module provides the structure for CUDA SGP4.
// Full integration with keplemon TLE types will be added in subsequent iterations.

/// Raw TLE data that matches CUDA structure
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct TleDataGpu {
    pub epoch_jd: f64,
    pub inclination: f64,      // degrees
    pub raan: f64,             // degrees
    pub eccentricity: f64,
    pub arg_perigee: f64,      // degrees
    pub mean_anomaly: f64,     // degrees
    pub mean_motion: f64,      // revs/day
    pub bstar: f64,
    pub ndot: f64,
    pub nddot: f64,
}

/// Output state (position and velocity)
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct Sgp4StateGpu {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub error_code: i32,
    pub _padding: f64,
}

/// GPU-accelerated SGP4 batch propagator
pub struct CudaSgp4Propagator {
    device: CudaDevice,
}

impl CudaSgp4Propagator {
    /// Create a new CUDA SGP4 propagator
    pub fn new() -> Result<Self, CudaError> {
        let device = CudaDevice::new()?;
        Ok(Self { device })
    }
    
    /// Check if CUDA is available
    pub fn is_cuda_available() -> bool {
        CudaDevice::is_available()
    }
    
    /// Get reference to CUDA device
    pub fn device(&self) -> &CudaDevice {
        &self.device
    }
}

impl Default for CudaSgp4Propagator {
    fn default() -> Self {
        Self::new().expect("Failed to initialize CUDA device")
    }
}
