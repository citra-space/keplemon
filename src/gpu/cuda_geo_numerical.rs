//! CUDA GEO Analytical Propagator Implementation
//!
//! This module provides GPU-accelerated propagation for GEO satellites using an
//! analytical perturbation model. Unlike SDP4, this propagator:
//! - Has no iteration loops (purely formulaic)
//! - Achieves ~80x GPU speedup (comparable to SGP4)
//! - Includes J2-J4 geopotential, lunar/solar gravity, and SRP
//!
//! The approach converts TLE -> ECI at epoch, then propagates using the analytical model.

use super::device::{CudaDevice, CudaError};
use super::cuda_tle::{Sgp4StateGpu, SoAArrays};
use cudarc::driver::{CudaFunction, CudaSlice, LaunchAsync, LaunchConfig};

// Include the PTX at compile time
const GEO_NUMERICAL_PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/geo_numerical.ptx"));

/// Default reflectivity coefficient for SRP (partial reflector with absorption)
pub const DEFAULT_CR: f64 = 1.3;

/// Default area-to-mass ratio for typical GEO satellites (m²/kg)
pub const DEFAULT_AREA_MASS: f64 = 0.02;

/// GEO satellite state in ECI frame
///
/// Used to initialize the analytical propagator from state vectors.
/// Can be created from TLEs using `from_tle()` which propagates to epoch using SDP4.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GeoStateGpu {
    /// Position X in TEME/ECI frame (km)
    pub x: f64,
    /// Position Y in TEME/ECI frame (km)
    pub y: f64,
    /// Position Z in TEME/ECI frame (km)
    pub z: f64,
    /// Velocity X in TEME/ECI frame (km/s)
    pub vx: f64,
    /// Velocity Y in TEME/ECI frame (km/s)
    pub vy: f64,
    /// Velocity Z in TEME/ECI frame (km/s)
    pub vz: f64,
    /// Reference epoch (Julian Date)
    pub epoch_jd: f64,
    /// Reflectivity coefficient for SRP (default 1.3)
    pub cr: f64,
    /// Area-to-mass ratio (m²/kg, default ~0.02)
    pub area_mass: f64,
    /// Semi-major axis (km) - set during initialization
    pub sma: f64,
    /// Original satellite index for output mapping
    pub original_index: i32,
    /// Padding for alignment
    pub _padding: i32,
}

// SAFETY: GeoStateGpu is #[repr(C)] with only primitive fields, valid for GPU transfer
unsafe impl cudarc::driver::DeviceRepr for GeoStateGpu {}

impl Default for GeoStateGpu {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            epoch_jd: 0.0,
            cr: DEFAULT_CR,
            area_mass: DEFAULT_AREA_MASS,
            sma: 0.0,
            original_index: 0,
            _padding: 0,
        }
    }
}

impl GeoStateGpu {
    /// Create a new GEO state with custom SRP parameters
    pub fn new(
        pos: [f64; 3],
        vel: [f64; 3],
        epoch_jd: f64,
        cr: Option<f64>,
        area_mass: Option<f64>,
    ) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
            z: pos[2],
            vx: vel[0],
            vy: vel[1],
            vz: vel[2],
            epoch_jd,
            cr: cr.unwrap_or(DEFAULT_CR),
            area_mass: area_mass.unwrap_or(DEFAULT_AREA_MASS),
            sma: 0.0,
            original_index: 0,
            _padding: 0,
        }
    }

    /// Create GEO state from Sgp4StateGpu (e.g., from SDP4 propagation at epoch)
    pub fn from_sgp4_state(state: &Sgp4StateGpu, epoch_jd: f64, cr: Option<f64>, area_mass: Option<f64>) -> Self {
        Self {
            x: state.x,
            y: state.y,
            z: state.z,
            vx: state.vx,
            vy: state.vy,
            vz: state.vz,
            epoch_jd,
            cr: cr.unwrap_or(DEFAULT_CR),
            area_mass: area_mass.unwrap_or(DEFAULT_AREA_MASS),
            sma: 0.0,
            original_index: 0,
            _padding: 0,
        }
    }

    /// Get position as array
    pub fn position(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// Get velocity as array
    pub fn velocity(&self) -> [f64; 3] {
        [self.vx, self.vy, self.vz]
    }
}

/// Precomputed GEO analytical propagator parameters
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GeoNumericalParamsGpu {
    // Initial mean orbital elements (at epoch)
    pub sma: f64,
    pub ecc: f64,
    pub inc: f64,
    pub raan: f64,
    pub argp: f64,
    pub mean_anom: f64,

    // Reference epoch
    pub epoch_jd: f64,

    // Mean motion (rad/s)
    pub n: f64,

    // SRP parameters
    pub cr: f64,
    pub area_mass: f64,

    // J2 secular rate coefficients (precomputed)
    pub n_dot_j2: f64,
    pub raan_dot_j2: f64,
    pub argp_dot_j2: f64,

    // J2 parameters (precomputed for efficiency)
    pub j2_factor: f64,
    pub cos_inc: f64,
    pub sin_inc: f64,
    pub cos_inc_sq: f64,
    pub sin_inc_sq: f64,

    // Original satellite index for output mapping
    pub original_index: i32,

    // Padding for alignment
    pub _padding: i32,
}

unsafe impl cudarc::driver::DeviceRepr for GeoNumericalParamsGpu {}
// SAFETY: GeoNumericalParamsGpu is composed entirely of f64 and i32, all valid as zero
unsafe impl cudarc::driver::ValidAsZeroBits for GeoNumericalParamsGpu {}

/// Cached SoA buffers for GEO propagator
struct CachedGeoSoABuffers {
    x: CudaSlice<f64>,
    y: CudaSlice<f64>,
    z: CudaSlice<f64>,
    vx: CudaSlice<f64>,
    vy: CudaSlice<f64>,
    vz: CudaSlice<f64>,
    error_code: CudaSlice<i32>,
    n_results: usize,
}

/// GPU-accelerated GEO Analytical Propagator
///
/// This propagator is designed for GEO satellites where SDP4's iterative resonance
/// loops cause poor GPU performance. It uses analytical perturbation models that
/// are purely formulaic, achieving ~80x GPU speedup (comparable to SGP4).
///
/// # Perturbation Models
/// - J2-J4 geopotential (secular and long-period)
/// - Simplified lunar/solar third-body gravity
/// - Solar radiation pressure (SRP)
///
/// # Accuracy
/// Expected accuracy is ~1 km over several days for GEO satellites, comparable to SDP4.
///
/// # Usage
/// ```no_run
/// use keplemon::gpu::{CudaGeoNumericalPropagator, GeoStateGpu};
///
/// // Create propagator
/// let mut propagator = CudaGeoNumericalPropagator::new()?;
///
/// // Initialize with ECI states at epoch
/// let states = vec![
///     GeoStateGpu::new([42164.0, 0.0, 0.0], [0.0, 3.07, 0.0], 2459945.5, None, None),
/// ];
/// propagator.init_satellites(&states)?;
///
/// // Propagate to multiple times
/// let times = vec![2459945.5, 2459946.5, 2459947.5];
/// let results = propagator.propagate_soa_arrays(&times)?;
/// # Ok::<(), keplemon::gpu::CudaError>(())
/// ```
pub struct CudaGeoNumericalPropagator {
    device: CudaDevice,
    n_satellites: usize,
    states_gpu: Option<CudaSlice<GeoStateGpu>>,
    params_gpu: Option<CudaSlice<GeoNumericalParamsGpu>>,
    cached_times_gpu: Option<CudaSlice<f64>>,
    cached_n_times: usize,
    cached_soa_buffers: Option<CachedGeoSoABuffers>,
    // Cached kernel functions
    init_kernel: CudaFunction,
    propagate_kernel: CudaFunction,
    propagate_soa_kernel: CudaFunction,
    propagate_soa_indexed_kernel: CudaFunction,
    propagate_eci_kernel: CudaFunction,
    // Index mappings for partitioned propagation
    original_indices: Vec<usize>,
    indices_gpu: Option<CudaSlice<i32>>,
}

impl CudaGeoNumericalPropagator {
    /// Create a new CUDA GEO Analytical propagator
    pub fn new() -> Result<Self, CudaError> {
        let device = CudaDevice::new()?;
        let dev = device.device();

        // Load PTX module
        dev.load_ptx(
            GEO_NUMERICAL_PTX.into(),
            "geo_numerical",
            &[
                "geo_init_kernel",
                "geo_propagate_kernel",
                "geo_propagate_soa_kernel",
                "geo_propagate_soa_indexed_kernel",
                "geo_propagate_eci_kernel",
            ],
        ).map_err(|e| CudaError::KernelLoad(e.to_string()))?;

        // Cache kernel functions
        let init_kernel = dev.get_func("geo_numerical", "geo_init_kernel")
            .ok_or_else(|| CudaError::KernelLoad("geo_init_kernel not found".into()))?;

        let propagate_kernel = dev.get_func("geo_numerical", "geo_propagate_kernel")
            .ok_or_else(|| CudaError::KernelLoad("geo_propagate_kernel not found".into()))?;

        let propagate_soa_kernel = dev.get_func("geo_numerical", "geo_propagate_soa_kernel")
            .ok_or_else(|| CudaError::KernelLoad("geo_propagate_soa_kernel not found".into()))?;

        let propagate_soa_indexed_kernel = dev.get_func("geo_numerical", "geo_propagate_soa_indexed_kernel")
            .ok_or_else(|| CudaError::KernelLoad("geo_propagate_soa_indexed_kernel not found".into()))?;

        let propagate_eci_kernel = dev.get_func("geo_numerical", "geo_propagate_eci_kernel")
            .ok_or_else(|| CudaError::KernelLoad("geo_propagate_eci_kernel not found".into()))?;

        Ok(Self {
            device,
            n_satellites: 0,
            states_gpu: None,
            params_gpu: None,
            cached_times_gpu: None,
            cached_n_times: 0,
            cached_soa_buffers: None,
            init_kernel,
            propagate_kernel,
            propagate_soa_kernel,
            propagate_soa_indexed_kernel,
            propagate_eci_kernel,
            original_indices: Vec::new(),
            indices_gpu: None,
        })
    }

    /// Check if CUDA is available
    pub fn is_cuda_available() -> bool {
        CudaDevice::is_available()
    }

    /// Get reference to CUDA device
    pub fn device(&self) -> &CudaDevice {
        &self.device
    }

    /// Initialize satellites from ECI states
    ///
    /// This uploads the states to GPU and runs the initialization kernel to
    /// precompute orbital parameters for efficient propagation.
    pub fn init_satellites(&mut self, states: &[GeoStateGpu]) -> Result<(), CudaError> {
        self.n_satellites = states.len();

        if self.n_satellites == 0 {
            return Ok(());
        }

        let dev = self.device.device();

        // Upload ECI states to GPU
        let states_gpu = dev.htod_sync_copy(states)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

        // Allocate params buffer
        let params_gpu: CudaSlice<GeoNumericalParamsGpu> = dev.alloc_zeros(self.n_satellites)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

        // Launch init kernel
        let block_size = 256u32;
        let grid_size = (self.n_satellites as u32 + block_size - 1) / block_size;

        let cfg = LaunchConfig {
            grid_dim: (grid_size, 1, 1),
            block_dim: (block_size, 1, 1),
            shared_mem_bytes: 0,
        };

        unsafe {
            self.init_kernel.clone().launch(cfg, (&states_gpu, &params_gpu, self.n_satellites as i32))
                .map_err(|e| CudaError::KernelLaunch(e.to_string()))?;
        }

        dev.synchronize()
            .map_err(|e| CudaError::Synchronization(e.to_string()))?;

        self.states_gpu = Some(states_gpu);
        self.params_gpu = Some(params_gpu);

        // Store original indices for potential partitioned propagation
        self.original_indices = (0..self.n_satellites).collect();
        let indices_i32: Vec<i32> = self.original_indices.iter().map(|&i| i as i32).collect();
        self.indices_gpu = Some(dev.htod_sync_copy(&indices_i32)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?);

        // Clear cached buffers
        self.cached_soa_buffers = None;

        Ok(())
    }

    /// Initialize satellites from ECI states with custom SRP parameters
    ///
    /// Allows setting reflectivity coefficient and area-to-mass ratio per satellite.
    pub fn init_satellites_with_srp(
        &mut self,
        positions: &[[f64; 3]],
        velocities: &[[f64; 3]],
        epochs: &[f64],
        cr_values: &[f64],
        area_mass_values: &[f64],
    ) -> Result<(), CudaError> {
        if positions.len() != velocities.len()
            || positions.len() != epochs.len()
            || positions.len() != cr_values.len()
            || positions.len() != area_mass_values.len()
        {
            return Err(CudaError::InvalidParameter(
                "All input arrays must have the same length".into()
            ));
        }

        let states: Vec<GeoStateGpu> = positions.iter()
            .zip(velocities.iter())
            .zip(epochs.iter())
            .zip(cr_values.iter())
            .zip(area_mass_values.iter())
            .enumerate()
            .map(|(i, ((((pos, vel), &epoch), &cr), &area_mass))| {
                let mut state = GeoStateGpu::new(*pos, *vel, epoch, Some(cr), Some(area_mass));
                state.original_index = i as i32;
                state
            })
            .collect();

        self.init_satellites(&states)
    }

    /// Propagate all initialized satellites to given Julian Date times
    ///
    /// Returns positions and velocities at each time for each satellite.
    ///
    /// # Returns
    /// Vector of states: [sat0_t0, sat0_t1, ..., sat0_tn, sat1_t0, ...]
    pub fn propagate(&mut self, jd_times: &[f64]) -> Result<Vec<Sgp4StateGpu>, CudaError> {
        if self.n_satellites == 0 {
            return Err(CudaError::NotInitialized);
        }

        let params_gpu = self.params_gpu.as_ref()
            .ok_or(CudaError::NotInitialized)?;

        let dev = self.device.device();
        let n_times = jd_times.len();
        let n_results = self.n_satellites * n_times;

        // Upload times
        let times_gpu = dev.htod_sync_copy(jd_times)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

        // Allocate output
        let states_gpu: CudaSlice<Sgp4StateGpu> = dev.alloc_zeros(n_results)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

        // Launch propagation kernel
        let block_x = 16u32;
        let block_y = 16u32;
        let grid_x = (self.n_satellites as u32 + block_x - 1) / block_x;
        let grid_y = (n_times as u32 + block_y - 1) / block_y;

        let cfg = LaunchConfig {
            grid_dim: (grid_x, grid_y, 1),
            block_dim: (block_x, block_y, 1),
            shared_mem_bytes: 0,
        };

        unsafe {
            self.propagate_kernel.clone().launch(
                cfg,
                (params_gpu, &times_gpu, &states_gpu, self.n_satellites as i32, n_times as i32)
            ).map_err(|e| CudaError::KernelLaunch(e.to_string()))?;
        }

        dev.synchronize()
            .map_err(|e| CudaError::Synchronization(e.to_string()))?;

        let results = dev.dtoh_sync_copy(&states_gpu)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;

        Ok(results)
    }

    /// Propagate using SoA kernel and return raw SoA arrays
    ///
    /// This is the most efficient method for batch propagation.
    ///
    /// # Returns
    /// SoAArrays with time-major ordering: array[time_idx * n_sats + sat_idx]
    pub fn propagate_soa_arrays(&mut self, jd_times: &[f64]) -> Result<SoAArrays, CudaError> {
        if self.n_satellites == 0 {
            return Err(CudaError::NotInitialized);
        }

        let params_gpu = self.params_gpu.as_ref()
            .ok_or(CudaError::NotInitialized)?;

        let dev = self.device.device();
        let n_times = jd_times.len();
        let n_results = self.n_satellites * n_times;

        // Upload times
        let times_gpu = dev.htod_sync_copy(jd_times)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

        // Allocate or reuse SoA buffers
        let need_realloc = match &self.cached_soa_buffers {
            Some(buffers) => buffers.n_results != n_results,
            None => true,
        };

        if need_realloc {
            self.cached_soa_buffers = Some(CachedGeoSoABuffers {
                x: dev.alloc_zeros(n_results).map_err(|e| CudaError::AllocationFailed(e.to_string()))?,
                y: dev.alloc_zeros(n_results).map_err(|e| CudaError::AllocationFailed(e.to_string()))?,
                z: dev.alloc_zeros(n_results).map_err(|e| CudaError::AllocationFailed(e.to_string()))?,
                vx: dev.alloc_zeros(n_results).map_err(|e| CudaError::AllocationFailed(e.to_string()))?,
                vy: dev.alloc_zeros(n_results).map_err(|e| CudaError::AllocationFailed(e.to_string()))?,
                vz: dev.alloc_zeros(n_results).map_err(|e| CudaError::AllocationFailed(e.to_string()))?,
                error_code: dev.alloc_zeros(n_results).map_err(|e| CudaError::AllocationFailed(e.to_string()))?,
                n_results,
            });
        }

        let soa = self.cached_soa_buffers.as_ref().unwrap();
        let shared_mem_bytes = 256 * std::mem::size_of::<f64>() as u32;

        // Launch SoA kernel
        let block_x = 16u32;
        let block_y = 16u32;
        let grid_x = (self.n_satellites as u32 + block_x - 1) / block_x;
        let grid_y = (n_times as u32 + block_y - 1) / block_y;

        let cfg = LaunchConfig {
            grid_dim: (grid_x, grid_y, 1),
            block_dim: (block_x, block_y, 1),
            shared_mem_bytes,
        };

        unsafe {
            self.propagate_soa_kernel.clone().launch(
                cfg,
                (
                    params_gpu,
                    &times_gpu,
                    &soa.x, &soa.y, &soa.z,
                    &soa.vx, &soa.vy, &soa.vz,
                    &soa.error_code,
                    self.n_satellites as i32,
                    n_times as i32,
                )
            ).map_err(|e| CudaError::KernelLaunch(e.to_string()))?;
        }

        dev.synchronize()
            .map_err(|e| CudaError::Synchronization(e.to_string()))?;

        // Download results
        Ok(SoAArrays {
            x: dev.dtoh_sync_copy(&soa.x).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            y: dev.dtoh_sync_copy(&soa.y).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            z: dev.dtoh_sync_copy(&soa.z).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vx: dev.dtoh_sync_copy(&soa.vx).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vy: dev.dtoh_sync_copy(&soa.vy).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vz: dev.dtoh_sync_copy(&soa.vz).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            error_code: dev.dtoh_sync_copy(&soa.error_code).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            n_sats: self.n_satellites,
            n_times,
        })
    }

    /// Propagate directly from ECI states without pre-initialization
    ///
    /// This is less efficient for repeated propagations of the same satellites
    /// but simpler when you just need a one-off propagation.
    pub fn propagate_eci_direct(
        &mut self,
        eci_states: &[GeoStateGpu],
        jd_times: &[f64],
    ) -> Result<SoAArrays, CudaError> {
        let dev = self.device.device();
        let n_sats = eci_states.len();
        let n_times = jd_times.len();
        let n_results = n_sats * n_times;

        // Upload ECI states
        let states_gpu = dev.htod_sync_copy(eci_states)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

        // Upload times
        let times_gpu = dev.htod_sync_copy(jd_times)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

        // Allocate output
        let out_x: CudaSlice<f64> = dev.alloc_zeros(n_results)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        let out_y: CudaSlice<f64> = dev.alloc_zeros(n_results)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        let out_z: CudaSlice<f64> = dev.alloc_zeros(n_results)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        let out_vx: CudaSlice<f64> = dev.alloc_zeros(n_results)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        let out_vy: CudaSlice<f64> = dev.alloc_zeros(n_results)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        let out_vz: CudaSlice<f64> = dev.alloc_zeros(n_results)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        let out_error: CudaSlice<i32> = dev.alloc_zeros(n_results)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

        // Launch kernel
        let block_x = 16u32;
        let block_y = 16u32;
        let grid_x = (n_sats as u32 + block_x - 1) / block_x;
        let grid_y = (n_times as u32 + block_y - 1) / block_y;
        let shared_mem_bytes = 256 * std::mem::size_of::<f64>() as u32;

        let cfg = LaunchConfig {
            grid_dim: (grid_x, grid_y, 1),
            block_dim: (block_x, block_y, 1),
            shared_mem_bytes,
        };

        unsafe {
            self.propagate_eci_kernel.clone().launch(
                cfg,
                (
                    &states_gpu,
                    &times_gpu,
                    &out_x, &out_y, &out_z,
                    &out_vx, &out_vy, &out_vz,
                    &out_error,
                    n_sats as i32,
                    n_times as i32,
                )
            ).map_err(|e| CudaError::KernelLaunch(e.to_string()))?;
        }

        dev.synchronize()
            .map_err(|e| CudaError::Synchronization(e.to_string()))?;

        Ok(SoAArrays {
            x: dev.dtoh_sync_copy(&out_x).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            y: dev.dtoh_sync_copy(&out_y).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            z: dev.dtoh_sync_copy(&out_z).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vx: dev.dtoh_sync_copy(&out_vx).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vy: dev.dtoh_sync_copy(&out_vy).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vz: dev.dtoh_sync_copy(&out_vz).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            error_code: dev.dtoh_sync_copy(&out_error).map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            n_sats,
            n_times,
        })
    }

    /// Pre-load Julian Date times to GPU for repeated propagations
    pub fn cache_times(&mut self, jd_times: &[f64]) -> Result<(), CudaError> {
        let dev = self.device.device();
        let times_gpu = dev.htod_sync_copy(jd_times)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        self.cached_times_gpu = Some(times_gpu);
        self.cached_n_times = jd_times.len();
        Ok(())
    }

    /// Clear cached times to free GPU memory
    pub fn clear_time_cache(&mut self) {
        self.cached_times_gpu = None;
        self.cached_n_times = 0;
    }

    /// Clear cached SoA buffers to free GPU memory
    pub fn clear_soa_cache(&mut self) {
        self.cached_soa_buffers = None;
    }

    /// Get reference to the CUDA device for external kernel launches
    pub fn cuda_device(&self) -> &std::sync::Arc<cudarc::driver::CudaDevice> {
        self.device.device()
    }

    /// Get the number of initialized satellites
    pub fn num_satellites(&self) -> usize {
        self.n_satellites
    }

    /// Get initialized parameters from GPU for debugging
    pub fn get_params_debug(&self) -> Result<Vec<GeoNumericalParamsGpu>, CudaError> {
        let params_gpu = self.params_gpu.as_ref()
            .ok_or(CudaError::NotInitialized)?;

        let dev = self.device.device();
        let params = dev.dtoh_sync_copy(params_gpu)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;

        Ok(params)
    }
}
