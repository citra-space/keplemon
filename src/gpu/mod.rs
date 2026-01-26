//! GPU acceleration module for batch satellite propagation
//!
//! This module provides CUDA-accelerated propagation when the `cuda` feature is enabled.
//!
//! # GPU Propagator Selection Guide
//!
//! | Scenario | Recommended Propagator | Notes |
//! |----------|----------------------|-------|
//! | LEO batch (1000+ sats) | `CudaTlePropagator` | ~83x GPU speedup |
//! | GEO/MEO batch (speed priority) | `CudaSdp4InterpolatedPropagator` | ~20-50x, 2-10 km accuracy |
//! | GEO/MEO batch (accuracy priority) | `CudaTlePropagator` (SDP4) | ~1.6x, <0.001m accuracy |
//! | Mixed LEO/GEO | Partition by period, use both | See `BatchPropagator` |
//!
//! ## Available Propagators
//!
//! - **CudaTlePropagator**: Standard SGP4/SDP4 TLE-based propagation
//!   - SGP4 for LEO (~83x GPU speedup)
//!   - SDP4 for GEO/MEO (~1.6x GPU speedup due to iterative resonance loops)
//!   - Note: `CudaSgp4Propagator` is a deprecated alias for this type
//!
//! - **CudaSdp4InterpolatedPropagator**: SDP4-compatible GPU propagation (production)
//!   - ~20-50x GPU speedup (pre-sampled resonance interpolation)
//!   - 2-10 km position accuracy (interpolation trade-off)
//!   - Eliminates thread divergence from iterative resonance loops
//!   - Automatically selected by `BatchPropagator` for deep-space orbits (period >= 225 min)
//!
//! - **CudaGeoNumericalPropagator**: ❌ **BROKEN - DO NOT USE**
//!   - ~18x GPU speedup in theory (RK4 integrator)
//!   - Uses EGM96 constants + VSOP87/Brown ephemerides + SRP
//!   - ❌ Catastrophic accuracy failure: 32,408 km errors @ 7 days
//!
//! ## Recommended Propagator Selection
//!
//! ```text
//! if ephemeris_type == 4:
//!     → SGP4-XP (when implemented)
//! else if orbital_period >= 225 min AND want speed (km-level accuracy OK):
//!     → CudaSdp4InterpolatedPropagator (20-50x speedup, 2-10 km accuracy)
//! else if orbital_period >= 225 min AND need sub-meter accuracy:
//!     → CudaTlePropagator standard SDP4 (1.6x speedup, <0.001m accuracy)
//! else:
//!     → CudaTlePropagator (standard SGP4)
//! ```
//!
//! ## Automatic Propagator Selection
//!
//! `BatchPropagator` automatically selects the appropriate propagator based on orbital period:
//!
//! ```rust,ignore
//! use keplemon::propagation::BatchPropagator;
//!
//! let batch_prop = BatchPropagator::new();
//!
//! // Mixed batch: LEO satellites use SGP4, GEO satellites use SDP4-Interpolated
//! let results = batch_prop.propagate_batch(&tles, &epochs)?;
//!
//! // For pure LEO or GEO batches, only one propagator is instantiated
//! ```

#[cfg(feature = "cuda")]
pub mod cuda_tle;

#[cfg(feature = "cuda")]
pub mod cuda_geo_numerical;

#[cfg(feature = "cuda")]
pub mod cuda_sdp4_interpolated;

#[cfg(feature = "cuda")]
pub mod device;

// Re-export main types when CUDA is enabled
#[cfg(feature = "cuda")]
pub use cuda_tle::{
    CudaTlePropagator,
    SoAArrays,
    Sgp4StateSoABuffers,
    Sgp4StateGpu,
    Sgp4ParamsGpu,
    TleDataGpu,
    PropagatorType,
    PropagatorOverride,
};

#[cfg(feature = "cuda")]
pub use cuda_geo_numerical::{
    CudaGeoNumericalPropagator,
    GeoStateGpu,
    GeoNumericalParamsGpu,
    DEFAULT_CR,
    DEFAULT_AREA_MASS,
};

#[cfg(feature = "cuda")]
pub use cuda_sdp4_interpolated::{
    CudaSdp4InterpolatedPropagator,
    Sdp4InterpolatedParamsGpu,
    Sdp4ResonanceSampleGpu,
};

#[cfg(feature = "cuda")]
pub use device::{CudaDevice, CudaError};
