//! GPU acceleration module for batch satellite propagation
//!
//! This module provides CUDA-accelerated SGP4 propagation when the `cuda` feature is enabled.
//! It transparently falls back to CPU when CUDA is not available.

#[cfg(feature = "cuda")]
pub mod cuda_sgp4;

#[cfg(feature = "cuda")]
pub mod device;

// Re-export main types when CUDA is enabled
#[cfg(feature = "cuda")]
pub use cuda_sgp4::{CudaSgp4Propagator, Sgp4ParamsGpu, Sgp4StateGpu, Sgp4StateSoABuffers, SoAArrays, TleDataGpu};

#[cfg(feature = "cuda")]
pub use device::{CudaDevice, CudaError};
