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
//! | ECI state, GEO, short-term | `CudaGeoNumericalPropagator` | Experimental, <7d only |
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
//! - **CudaGeoNumericalPropagator**: ECI-based numerical GEO propagation (experimental)
//!   - For short-term propagation only (<7 days, ~12 km error @ 7d)
//!   - Uses EGM96 constants + VSOP87/Brown ephemerides + SRP
//!   - Slower than GPU SDP4; use only when no TLE is available
//!
//! ## Recommended Propagator Selection
//!
//! ```text
//! if ephemeris_type == 4:
//!     → SGP4-XP (when implemented)
//! else if have ECI state (no TLE) AND GEO orbit AND short-term (<7d):
//!     → CudaGeoNumericalPropagator (experimental, ~12 km @ 7d)
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
    CudaTlePropagator, PropagatorOverride, PropagatorType, Sgp4ParamsGpu, Sgp4StateGpu, Sgp4StateSoABuffers, SoAArrays,
    TimestepData, TleDataGpu,
};

#[cfg(feature = "cuda")]
pub use cuda_geo_numerical::{
    CudaGeoNumericalPropagator, DEFAULT_AREA_MASS, DEFAULT_CR, GeoNumericalParamsGpu, GeoStateGpu,
};

#[cfg(feature = "cuda")]
pub use cuda_sdp4_interpolated::{CudaSdp4InterpolatedPropagator, Sdp4InterpolatedParamsGpu, Sdp4ResonanceSampleGpu};

#[cfg(feature = "cuda")]
pub use device::{CudaDevice, CudaError};
