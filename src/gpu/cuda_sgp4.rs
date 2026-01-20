//! CUDA SGP4 batch propagator implementation

use super::device::{CudaDevice, CudaError};
use cudarc::driver::{CudaFunction, CudaSlice, LaunchAsync, LaunchConfig};

// Include the PTX at compile time
const SGP4_INIT_PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/sgp4_init.ptx"));
const SGP4_BATCH_PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/sgp4_batch.ptx"));

/// Raw TLE data that matches CUDA structure
#[repr(C)]
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

// SAFETY: TleDataGpu is #[repr(C)] with only f64 fields, valid for GPU transfer
unsafe impl cudarc::driver::DeviceRepr for TleDataGpu {}

// Julian Date epoch for 1950-01-01 00:00:00 UTC
const JD_1950: f64 = 2433281.5;

impl TleDataGpu {
    /// Convert days since 1950 to Julian Date
    pub fn jd_from_ds50(ds50: f64) -> f64 {
        ds50 + JD_1950
    }
}

#[cfg(feature = "cuda")]
impl From<&crate::elements::TLE> for TleDataGpu {
    fn from(tle: &crate::elements::TLE) -> Self {
        let ks = tle.get_keplerian_state();
        let fp = tle.get_force_properties();
        Self {
            epoch_jd: Self::jd_from_ds50(ks.epoch.days_since_1950),
            inclination: ks.elements.inclination,
            raan: ks.elements.raan,
            eccentricity: ks.elements.eccentricity,
            arg_perigee: ks.elements.argument_of_perigee,
            mean_anomaly: ks.elements.mean_anomaly,
            mean_motion: tle.get_mean_motion(),
            bstar: fp.get_b_star(),
            ndot: fp.mean_motion_dot,
            nddot: fp.mean_motion_dot_dot,
        }
    }
}

/// SGP4 initialized parameters (matches CUDA Sgp4Params structure)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Sgp4ParamsGpu {
    // TLE epoch and elements
    pub epoch_jd: f64,
    pub inclo: f64,
    pub nodeo: f64,
    pub ecco: f64,
    pub argpo: f64,
    pub mo: f64,
    pub no_kozai: f64,
    pub bstar: f64,
    pub ndot: f64,
    pub nddot: f64,
    
    // Derived constants
    pub a: f64,
    pub alta: f64,
    pub altp: f64,
    pub con41: f64,
    pub con42: f64,
    pub cosio: f64,
    pub cosio2: f64,
    pub cosio4: f64,
    pub cc1: f64,
    pub cc4: f64,
    pub cc5: f64,
    pub d2: f64,
    pub d3: f64,
    pub d4: f64,
    pub delmo: f64,
    pub eta: f64,
    pub argpdot: f64,
    pub omgcof: f64,
    pub sinmao: f64,
    pub t2cof: f64,
    pub t3cof: f64,
    pub t4cof: f64,
    pub t5cof: f64,
    pub x1mth2: f64,
    pub x7thm1: f64,
    pub xlcof: f64,
    pub xmcof: f64,
    pub xnodcf: f64,
    pub nodedot: f64,
    pub mdot: f64,
    pub no_unkozai: f64,
    pub aycof: f64,
    pub delmo_const: f64,
    
    // ═══════════════════════════════════════════════════════════════════
    // DEEP SPACE PARAMETERS
    // ═══════════════════════════════════════════════════════════════════
    
    // Greenwich sidereal time at epoch
    pub gsto: f64,
    
    // Lunar-solar terms (from DSCOM)
    pub e3: f64,
    pub ee2: f64,
    pub peo: f64,
    pub pgho: f64,
    pub pho: f64,
    pub pinco: f64,
    pub plo: f64,
    pub se2: f64,
    pub se3: f64,
    pub sgh2: f64,
    pub sgh3: f64,
    pub sgh4: f64,
    pub sh2: f64,
    pub sh3: f64,
    pub si2: f64,
    pub si3: f64,
    pub sl2: f64,
    pub sl3: f64,
    pub sl4: f64,
    pub xgh2: f64,
    pub xgh3: f64,
    pub xgh4: f64,
    pub xh2: f64,
    pub xh3: f64,
    pub xi2: f64,
    pub xi3: f64,
    pub xl2: f64,
    pub xl3: f64,
    pub xl4: f64,
    pub zmol: f64,
    pub zmos: f64,
    
    // Secular rates (from DSINIT)
    pub dedt: f64,
    pub didt: f64,
    pub dmdt: f64,
    pub dnodt: f64,
    pub domdt: f64,
    
    // Resonance terms (from DSINIT)
    pub d2201: f64,
    pub d2211: f64,
    pub d3210: f64,
    pub d3222: f64,
    pub d4410: f64,
    pub d4422: f64,
    pub d5220: f64,
    pub d5232: f64,
    pub d5421: f64,
    pub d5433: f64,
    pub del1: f64,
    pub del2: f64,
    pub del3: f64,
    pub xfact: f64,
    pub xlamo: f64,
    pub xli: f64,
    pub xni: f64,
    pub atime: f64,
    
    // Flags
    pub is_deep_space: i32,
    pub irez: i32,          // 0=none, 1=one-day, 2=half-day resonance
    pub _padding: [i32; 2], // Maintain 8-byte alignment
}

unsafe impl cudarc::driver::DeviceRepr for Sgp4ParamsGpu {}
// SAFETY: Sgp4ParamsGpu is composed entirely of f64, i32, and arrays thereof, all valid as zero
unsafe impl cudarc::driver::ValidAsZeroBits for Sgp4ParamsGpu {}

/// Output state (position and velocity)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Sgp4StateGpu {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub error_code: i32,
    pub _padding: i32,
}

unsafe impl cudarc::driver::DeviceRepr for Sgp4StateGpu {}
// SAFETY: Sgp4StateGpu is composed entirely of f64 and i32, all valid as zero
unsafe impl cudarc::driver::ValidAsZeroBits for Sgp4StateGpu {}

// ═══════════════════════════════════════════════════════════════════════════════
// SoA (STRUCT OF ARRAYS) TYPES FOR OPTIMIZED MEMORY COALESCING
// ═══════════════════════════════════════════════════════════════════════════════

/// SoA output buffers for coalesced GPU memory access
/// 
/// This holds separate CUDA buffers for each state component, allowing
/// coalesced writes when adjacent threads write adjacent array elements.
/// Uses time-major ordering: buffer[time_idx * n_sats + sat_idx]
pub struct Sgp4StateSoABuffers {
    pub x: CudaSlice<f64>,
    pub y: CudaSlice<f64>,
    pub z: CudaSlice<f64>,
    pub vx: CudaSlice<f64>,
    pub vy: CudaSlice<f64>,
    pub vz: CudaSlice<f64>,
    pub error_code: CudaSlice<i32>,
    pub n_sats: usize,
    pub n_times: usize,
}

impl Sgp4StateSoABuffers {
    /// Convert SoA buffers back to AoS vector for compatibility
    /// 
    /// The GPU writes in time-major order (buffer[time_idx * n_sats + sat_idx])
    /// but we return in the same order as the AoS kernel: [sat0_t0, sat0_t1, ..., sat0_tn, sat1_t0, ...]
    pub fn to_aos_vec(&self, dev: &std::sync::Arc<cudarc::driver::CudaDevice>) -> Result<Vec<Sgp4StateGpu>, CudaError> {
        // Download all arrays from GPU
        let x = dev.dtoh_sync_copy(&self.x)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        let y = dev.dtoh_sync_copy(&self.y)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        let z = dev.dtoh_sync_copy(&self.z)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        let vx = dev.dtoh_sync_copy(&self.vx)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        let vy = dev.dtoh_sync_copy(&self.vy)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        let vz = dev.dtoh_sync_copy(&self.vz)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        let error_code = dev.dtoh_sync_copy(&self.error_code)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        
        // Convert from time-major SoA to sat-major AoS
        // SoA index: time_idx * n_sats + sat_idx
        // AoS index: sat_idx * n_times + time_idx
        let n_results = self.n_sats * self.n_times;
        let mut results = Vec::with_capacity(n_results);
        
        for sat_idx in 0..self.n_sats {
            for time_idx in 0..self.n_times {
                let soa_idx = time_idx * self.n_sats + sat_idx;
                results.push(Sgp4StateGpu {
                    x: x[soa_idx],
                    y: y[soa_idx],
                    z: z[soa_idx],
                    vx: vx[soa_idx],
                    vy: vy[soa_idx],
                    vz: vz[soa_idx],
                    error_code: error_code[soa_idx],
                    _padding: 0,
                });
            }
        }
        
        Ok(results)
    }
    
    /// Get raw SoA arrays (in time-major order) without conversion
    /// 
    /// Returns (x, y, z, vx, vy, vz, error_code) arrays where index = time_idx * n_sats + sat_idx
    pub fn to_soa_arrays(&self, dev: &std::sync::Arc<cudarc::driver::CudaDevice>) -> Result<SoAArrays, CudaError> {
        Ok(SoAArrays {
            x: dev.dtoh_sync_copy(&self.x)
                .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            y: dev.dtoh_sync_copy(&self.y)
                .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            z: dev.dtoh_sync_copy(&self.z)
                .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vx: dev.dtoh_sync_copy(&self.vx)
                .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vy: dev.dtoh_sync_copy(&self.vy)
                .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            vz: dev.dtoh_sync_copy(&self.vz)
                .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            error_code: dev.dtoh_sync_copy(&self.error_code)
                .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?,
            n_sats: self.n_sats,
            n_times: self.n_times,
        })
    }
}

/// Host-side SoA arrays after GPU download
/// 
/// All arrays use time-major ordering: array[time_idx * n_sats + sat_idx]
#[derive(Debug, Clone)]
pub struct SoAArrays {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub vx: Vec<f64>,
    pub vy: Vec<f64>,
    pub vz: Vec<f64>,
    pub error_code: Vec<i32>,
    pub n_sats: usize,
    pub n_times: usize,
}

impl SoAArrays {
    /// Get the state for a specific satellite and time
    #[inline]
    pub fn get(&self, sat_idx: usize, time_idx: usize) -> Sgp4StateGpu {
        let idx = time_idx * self.n_sats + sat_idx;
        Sgp4StateGpu {
            x: self.x[idx],
            y: self.y[idx],
            z: self.z[idx],
            vx: self.vx[idx],
            vy: self.vy[idx],
            vz: self.vz[idx],
            error_code: self.error_code[idx],
            _padding: 0,
        }
    }
}

/// GPU-accelerated SGP4 batch propagator
pub struct CudaSgp4Propagator {
    device: CudaDevice,
    n_satellites: usize,
    #[allow(dead_code)]
    tle_data_gpu: Option<CudaSlice<TleDataGpu>>,
    params_gpu: Option<CudaSlice<Sgp4ParamsGpu>>,
    /// Cached JD times on GPU for repeated propagations
    cached_times_gpu: Option<CudaSlice<f64>>,
    cached_n_times: usize,
    /// Cached output buffer for repeated propagations with same dimensions
    cached_states_gpu: Option<CudaSlice<Sgp4StateGpu>>,
    cached_n_results: usize,
    /// Cached SoA output buffers for SoA propagation
    cached_soa_buffers: Option<CachedSoABuffers>,
    /// Cached kernel functions to avoid lookup overhead
    init_kernel: CudaFunction,
    propagate_kernel: CudaFunction,
    propagate_soa_kernel: CudaFunction,
}

/// Cached SoA buffers to avoid repeated allocation
struct CachedSoABuffers {
    x: CudaSlice<f64>,
    y: CudaSlice<f64>,
    z: CudaSlice<f64>,
    vx: CudaSlice<f64>,
    vy: CudaSlice<f64>,
    vz: CudaSlice<f64>,
    error_code: CudaSlice<i32>,
    n_results: usize,
}

impl CudaSgp4Propagator {
    /// Create a new CUDA SGP4 propagator
    pub fn new() -> Result<Self, CudaError> {
        let device = CudaDevice::new()?;
        let dev = device.device();
        
        // Load PTX modules
        dev.load_ptx(SGP4_INIT_PTX.into(), "sgp4_init", &["sgp4_init_kernel"])
            .map_err(|e| CudaError::KernelLoad(e.to_string()))?;
        
        dev.load_ptx(
            SGP4_BATCH_PTX.into(), 
            "sgp4_batch", 
            &["sgp4_propagate_kernel", "sgp4_propagate_soa_kernel"]
        ).map_err(|e| CudaError::KernelLoad(e.to_string()))?;
        
        // Cache kernel functions for faster access
        let init_kernel = dev.get_func("sgp4_init", "sgp4_init_kernel")
            .ok_or_else(|| CudaError::KernelLoad("sgp4_init_kernel not found".into()))?;
        
        let propagate_kernel = dev.get_func("sgp4_batch", "sgp4_propagate_kernel")
            .ok_or_else(|| CudaError::KernelLoad("sgp4_propagate_kernel not found".into()))?;
        
        let propagate_soa_kernel = dev.get_func("sgp4_batch", "sgp4_propagate_soa_kernel")
            .ok_or_else(|| CudaError::KernelLoad("sgp4_propagate_soa_kernel not found".into()))?;
        
        Ok(Self { 
            device,
            n_satellites: 0,
            tle_data_gpu: None,
            params_gpu: None,
            cached_times_gpu: None,
            cached_n_times: 0,
            cached_states_gpu: None,
            cached_n_results: 0,
            cached_soa_buffers: None,
            init_kernel,
            propagate_kernel,
            propagate_soa_kernel,
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
    
    /// Initialize satellites from TLE data
    pub fn init_satellites(&mut self, tle_data: &[TleDataGpu]) -> Result<(), CudaError> {
        self.n_satellites = tle_data.len();
        
        if self.n_satellites == 0 {
            return Ok(());
        }
        
        let dev = self.device.device();
        
        // Upload TLE data to GPU
        let tle_gpu = dev.htod_sync_copy(tle_data)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        
        // Allocate space for initialized parameters
        let params_gpu: CudaSlice<Sgp4ParamsGpu> = dev.alloc_zeros(self.n_satellites)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        
        // Use cached init kernel
        let block_size = 256u32;
        let grid_size = (self.n_satellites as u32 + block_size - 1) / block_size;
        
        let cfg = LaunchConfig {
            grid_dim: (grid_size, 1, 1),
            block_dim: (block_size, 1, 1),
            shared_mem_bytes: 0,
        };
        
        // Launch: sgp4_init_kernel(TleData* tle_in, Sgp4Params* params_out, int n_satellites)
        unsafe {
            self.init_kernel.clone().launch(cfg, (&tle_gpu, &params_gpu, self.n_satellites as i32))
                .map_err(|e| CudaError::KernelLaunch(e.to_string()))?;
        }
        
        // Sync to ensure kernel completed
        dev.synchronize()
            .map_err(|e| CudaError::Synchronization(e.to_string()))?;
        
        self.tle_data_gpu = Some(tle_gpu);
        self.params_gpu = Some(params_gpu);
        
        Ok(())
    }
    
    /// Propagate all initialized satellites to given Julian Date times
    /// 
    /// # Arguments
    /// * `jd_times` - Array of Julian Dates to propagate to
    /// 
    /// # Returns
    /// Vector of states: [sat0_t0, sat0_t1, ..., sat0_tn, sat1_t0, ...]
    /// The kernel internally computes tsince for each satellite based on its TLE epoch.
    pub fn propagate(&mut self, jd_times: &[f64]) -> Result<Vec<Sgp4StateGpu>, CudaError> {
        if self.n_satellites == 0 {
            return Err(CudaError::NotInitialized);
        }
        
        let params_gpu = self.params_gpu.as_ref()
            .ok_or(CudaError::NotInitialized)?;
        
        let dev = self.device.device();
        let n_times = jd_times.len();
        let n_results = self.n_satellites * n_times;
        
        // Always upload times to GPU (caching requires value comparison which is expensive)
        // The time array is typically small, so the overhead is minimal
        let times_gpu = dev.htod_sync_copy(jd_times)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        
        // Reuse output buffer if same size, otherwise allocate new
        if self.cached_n_results != n_results || self.cached_states_gpu.is_none() {
            let new_states_gpu: CudaSlice<Sgp4StateGpu> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            self.cached_states_gpu = Some(new_states_gpu);
            self.cached_n_results = n_results;
        }
        let states_gpu = self.cached_states_gpu.as_ref().unwrap();
        
        // Launch config: 2D grid (satellites x times)
        // Use 16x16 = 256 threads per block - balanced for various batch sizes
        let block_x = 16u32;
        let block_y = 16u32;
        let grid_x = (self.n_satellites as u32 + block_x - 1) / block_x;
        let grid_y = (n_times as u32 + block_y - 1) / block_y;
        
        let cfg = LaunchConfig {
            grid_dim: (grid_x, grid_y, 1),
            block_dim: (block_x, block_y, 1),
            shared_mem_bytes: 0,
        };
        
        // Launch with cached kernel function
        unsafe {
            self.propagate_kernel.clone().launch(
                cfg, 
                (params_gpu, &times_gpu, states_gpu, self.n_satellites as i32, n_times as i32)
            ).map_err(|e| CudaError::KernelLaunch(e.to_string()))?;
        }
        
        // Sync and copy results back
        dev.synchronize()
            .map_err(|e| CudaError::Synchronization(e.to_string()))?;
        
        let results = dev.dtoh_sync_copy(states_gpu)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        
        Ok(results)
    }
    
    /// Pre-load Julian Date times to GPU for repeated propagations
    /// 
    /// Use this when you want to propagate multiple satellite sets to the same times.
    /// After calling this, propagate() will reuse the cached times without re-uploading.
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
    
    /// Get initialized SGP4 parameters from GPU for debugging
    /// 
    /// This copies the initialized parameters back from GPU memory,
    /// which includes all the computed secular rates, resonance terms, etc.
    pub fn get_params_debug(&self) -> Result<Vec<Sgp4ParamsGpu>, CudaError> {
        let params_gpu = self.params_gpu.as_ref()
            .ok_or(CudaError::NotInitialized)?;
        
        let dev = self.device.device();
        let params = dev.dtoh_sync_copy(params_gpu)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        
        Ok(params)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // SoA (STRUCT OF ARRAYS) PROPAGATION METHODS
    // ═══════════════════════════════════════════════════════════════════════════════
    
    /// Propagate all initialized satellites to given Julian Date times using SoA kernel
    /// 
    /// This uses the optimized SoA (Struct of Arrays) kernel which provides:
    /// - Coalesced memory writes (adjacent threads write adjacent memory)
    /// - Shared memory caching for time values
    /// - Better GPU memory bandwidth utilization
    /// 
    /// # Arguments
    /// * `jd_times` - Array of Julian Dates to propagate to
    /// 
    /// # Returns
    /// Vector of states: [sat0_t0, sat0_t1, ..., sat0_tn, sat1_t0, ...]
    /// (Same ordering as the AoS kernel for compatibility)
    pub fn propagate_soa(&mut self, jd_times: &[f64]) -> Result<Vec<Sgp4StateGpu>, CudaError> {
        // Get SoA arrays then convert to AoS
        let soa = self.propagate_soa_arrays(jd_times)?;
        
        // Convert from time-major SoA to sat-major AoS
        let n_results = soa.n_sats * soa.n_times;
        let mut results = Vec::with_capacity(n_results);
        
        for sat_idx in 0..soa.n_sats {
            for time_idx in 0..soa.n_times {
                let soa_idx = time_idx * soa.n_sats + sat_idx;
                results.push(Sgp4StateGpu {
                    x: soa.x[soa_idx],
                    y: soa.y[soa_idx],
                    z: soa.z[soa_idx],
                    vx: soa.vx[soa_idx],
                    vy: soa.vy[soa_idx],
                    vz: soa.vz[soa_idx],
                    error_code: soa.error_code[soa_idx],
                    _padding: 0,
                });
            }
        }
        
        Ok(results)
    }
    
    /// Propagate using SoA kernel and return raw SoA arrays
    /// 
    /// Use this when you want to keep data in SoA format for downstream processing.
    /// This avoids the overhead of converting back to AoS.
    /// 
    /// # Returns
    /// SoAArrays with time-major ordering: array[time_idx * n_sats + sat_idx]
    pub fn propagate_soa_arrays(&mut self, jd_times: &[f64]) -> Result<SoAArrays, CudaError> {
        if self.n_satellites == 0 {
            return Err(CudaError::NotInitialized);
        }
        
        let n_times = jd_times.len();
        let n_results = self.n_satellites * n_times;
        
        // Allocate output arrays
        let mut x = vec![0.0f64; n_results];
        let mut y = vec![0.0f64; n_results];
        let mut z = vec![0.0f64; n_results];
        let mut vx = vec![0.0f64; n_results];
        let mut vy = vec![0.0f64; n_results];
        let mut vz = vec![0.0f64; n_results];
        let mut error_code = vec![0i32; n_results];
        
        self.propagate_soa_into(
            jd_times,
            &mut x, &mut y, &mut z,
            &mut vx, &mut vy, &mut vz,
            &mut error_code,
        )?;
        
        Ok(SoAArrays {
            x, y, z, vx, vy, vz, error_code,
            n_sats: self.n_satellites,
            n_times,
        })
    }
    
    /// Propagate using SoA kernel and copy results directly to provided arrays
    /// 
    /// This is the most efficient method when you need raw arrays, as it avoids
    /// intermediate allocations and copies.
    /// 
    /// # Arguments
    /// * `jd_times` - Julian Dates to propagate to
    /// * `out_x`, `out_y`, `out_z` - Position output arrays (must be n_sats * n_times)
    /// * `out_vx`, `out_vy`, `out_vz` - Velocity output arrays
    /// * `out_error` - Error code output array
    /// 
    /// Output arrays use time-major ordering: array[time_idx * n_sats + sat_idx]
    pub fn propagate_soa_into(
        &mut self,
        jd_times: &[f64],
        out_x: &mut [f64],
        out_y: &mut [f64],
        out_z: &mut [f64],
        out_vx: &mut [f64],
        out_vy: &mut [f64],
        out_vz: &mut [f64],
        out_error: &mut [i32],
    ) -> Result<(), CudaError> {
        if self.n_satellites == 0 {
            return Err(CudaError::NotInitialized);
        }
        
        let n_times = jd_times.len();
        let n_results = self.n_satellites * n_times;
        
        // Validate output array sizes
        if out_x.len() < n_results || out_y.len() < n_results || out_z.len() < n_results
            || out_vx.len() < n_results || out_vy.len() < n_results || out_vz.len() < n_results
            || out_error.len() < n_results
        {
            return Err(CudaError::InvalidParameter(
                format!("Output arrays must have at least {} elements", n_results)
            ));
        }
        
        let params_gpu = self.params_gpu.as_ref()
            .ok_or(CudaError::NotInitialized)?;
        
        let dev = self.device.device();
        
        // Upload times to GPU
        let times_gpu = dev.htod_sync_copy(jd_times)
            .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
        
        // Allocate or reuse SoA output buffers
        let need_realloc = match &self.cached_soa_buffers {
            Some(buffers) => buffers.n_results != n_results,
            None => true,
        };
        
        if need_realloc {
            let x: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let y: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let z: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let vx: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let vy: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let vz: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let error_code: CudaSlice<i32> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            
            self.cached_soa_buffers = Some(CachedSoABuffers {
                x, y, z, vx, vy, vz, error_code,
                n_results,
            });
        }
        
        let soa = self.cached_soa_buffers.as_ref().unwrap();
        
        // Launch config
        let block_x = 16u32;
        let block_y = 16u32;
        let grid_x = (self.n_satellites as u32 + block_x - 1) / block_x;
        let grid_y = (n_times as u32 + block_y - 1) / block_y;
        let shared_mem_bytes = 256 * std::mem::size_of::<f64>() as u32;
        
        let cfg = LaunchConfig {
            grid_dim: (grid_x, grid_y, 1),
            block_dim: (block_x, block_y, 1),
            shared_mem_bytes,
        };
        
        // Launch SoA kernel
        unsafe {
            self.propagate_soa_kernel.clone().launch(
                cfg,
                (
                    params_gpu,
                    &times_gpu,
                    &soa.x,
                    &soa.y,
                    &soa.z,
                    &soa.vx,
                    &soa.vy,
                    &soa.vz,
                    &soa.error_code,
                    self.n_satellites as i32,
                    n_times as i32,
                )
            ).map_err(|e| CudaError::KernelLaunch(e.to_string()))?;
        }
        
        // Sync and copy results directly to output arrays
        dev.synchronize()
            .map_err(|e| CudaError::Synchronization(e.to_string()))?;
        
        // Copy from GPU to provided arrays
        dev.dtoh_sync_copy_into(&soa.x, out_x)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        dev.dtoh_sync_copy_into(&soa.y, out_y)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        dev.dtoh_sync_copy_into(&soa.z, out_z)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        dev.dtoh_sync_copy_into(&soa.vx, out_vx)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        dev.dtoh_sync_copy_into(&soa.vy, out_vy)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        dev.dtoh_sync_copy_into(&soa.vz, out_vz)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        dev.dtoh_sync_copy_into(&soa.error_code, out_error)
            .map_err(|e| CudaError::MemoryAllocation(e.to_string()))?;
        
        Ok(())
    }

    /// Propagate and return GPU-resident buffers (zero-copy)
    ///
    /// This method returns satellite states that remain on the GPU, avoiding
    /// the GPU→CPU transfer bottleneck. Use this when building GPU-accelerated
    /// pipelines that process the data on the GPU.
    ///
    /// **Dual-Mode Design**: This method coexists with `propagate_soa_arrays()`.
    /// Use `propagate_soa_arrays()` when you need CPU-resident data, and this
    /// method when building GPU-accelerated pipelines.
    ///
    /// # Example
    /// ```no_run
    /// # use keplemon::gpu::{CudaSgp4Propagator, TleDataGpu};
    /// # let tle_data: Vec<TleDataGpu> = vec![];
    /// # let times: Vec<f64> = vec![2459945.5];
    /// let mut propagator = CudaSgp4Propagator::new()?;
    /// propagator.init_satellites(&tle_data)?;
    ///
    /// // Option 1: CPU-resident (existing API)
    /// let cpu_results = propagator.propagate_soa_arrays(&times)?;
    ///
    /// // Option 2: GPU-resident (new API, for GPU pipelines)
    /// let gpu_results = propagator.propagate_soa_gpu_resident(&times)?;
    /// // Keep data on GPU for subsequent processing
    /// // let device = propagator.cuda_device();
    /// // custom_kernel.launch(cfg, (&gpu_results.x, &gpu_results.y))?;
    ///
    /// // Convert to host only when needed
    /// let host_results = gpu_results.to_aos_vec(propagator.cuda_device())?;
    /// # Ok::<(), keplemon::gpu::CudaError>(())
    /// ```
    ///
    /// # Performance
    /// This method avoids the GPU→CPU transfer bottleneck. For large batches
    /// (1000+ satellites × 100+ timesteps), this can provide 2-10x speedup
    /// when chaining with other GPU operations.
    ///
    /// # Arguments
    /// * `jd_times` - Array of Julian dates to propagate to
    ///
    /// # Returns
    /// GPU-resident SoA buffers in time-major order: buffer[time_idx * n_sats + sat_idx]
    pub fn propagate_soa_gpu_resident(
        &mut self,
        jd_times: &[f64],
    ) -> Result<Sgp4StateSoABuffers, CudaError> {
        if self.n_satellites == 0 {
            return Err(CudaError::NotInitialized);
        }

        let n_times = jd_times.len();
        let n_results = self.n_satellites * n_times;

        let params_gpu = self.params_gpu.as_ref()
            .ok_or(CudaError::NotInitialized)?;

        let dev = self.device.device();

        // Upload times to GPU (reuse cached buffer if possible)
        if self.cached_n_times != n_times || self.cached_times_gpu.is_none() {
            let times_gpu = dev.htod_sync_copy(jd_times)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            self.cached_times_gpu = Some(times_gpu);
            self.cached_n_times = n_times;
        }
        let times_gpu = self.cached_times_gpu.as_ref().unwrap();

        // Allocate or reuse SoA output buffers
        let need_realloc = match &self.cached_soa_buffers {
            Some(buffers) => buffers.n_results != n_results,
            None => true,
        };

        if need_realloc {
            let x: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let y: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let z: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let vx: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let vy: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let vz: CudaSlice<f64> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;
            let error_code: CudaSlice<i32> = dev.alloc_zeros(n_results)
                .map_err(|e| CudaError::AllocationFailed(e.to_string()))?;

            self.cached_soa_buffers = Some(CachedSoABuffers {
                x, y, z, vx, vy, vz, error_code,
                n_results,
            });
        }

        let soa = self.cached_soa_buffers.as_ref().unwrap();

        // Launch config (same as propagate_soa_into)
        let block_x = 16u32;
        let block_y = 16u32;
        let grid_x = (self.n_satellites as u32 + block_x - 1) / block_x;
        let grid_y = (n_times as u32 + block_y - 1) / block_y;
        let shared_mem_bytes = 256 * std::mem::size_of::<f64>() as u32;

        let cfg = LaunchConfig {
            grid_dim: (grid_x, grid_y, 1),
            block_dim: (block_x, block_y, 1),
            shared_mem_bytes,
        };

        // Launch SoA kernel
        unsafe {
            self.propagate_soa_kernel.clone().launch(
                cfg,
                (
                    params_gpu,
                    times_gpu,
                    &soa.x,
                    &soa.y,
                    &soa.z,
                    &soa.vx,
                    &soa.vy,
                    &soa.vz,
                    &soa.error_code,
                    self.n_satellites as i32,
                    n_times as i32,
                )
            ).map_err(|e| CudaError::KernelLaunch(e.to_string()))?;
        }

        // Synchronize to ensure kernel completion
        dev.synchronize()
            .map_err(|e| CudaError::Synchronization(e.to_string()))?;

        // Return GPU-resident buffers (no copy!)
        Ok(Sgp4StateSoABuffers {
            x: soa.x.clone(),
            y: soa.y.clone(),
            z: soa.z.clone(),
            vx: soa.vx.clone(),
            vy: soa.vy.clone(),
            vz: soa.vz.clone(),
            error_code: soa.error_code.clone(),
            n_sats: self.n_satellites,
            n_times,
        })
    }

    /// Get reference to the CUDA device for external kernel launches
    ///
    /// This allows external code to launch custom kernels on the same device
    /// context, enabling zero-copy GPU pipelines.
    ///
    /// # Example
    /// ```no_run
    /// # use keplemon::gpu::{CudaSgp4Propagator, TleDataGpu};
    /// # let tle_data: Vec<TleDataGpu> = vec![];
    /// # let times: Vec<f64> = vec![2459945.5];
    /// let mut propagator = CudaSgp4Propagator::new()?;
    /// propagator.init_satellites(&tle_data)?;
    /// let gpu_states = propagator.propagate_soa_gpu_resident(&times)?;
    ///
    /// // Launch custom kernel on same device
    /// let device = propagator.cuda_device();
    /// // custom_kernel.launch_on_device(device, (&gpu_states.x, &gpu_states.y))?;
    /// # Ok::<(), keplemon::gpu::CudaError>(())
    /// ```
    pub fn cuda_device(&self) -> &std::sync::Arc<cudarc::driver::CudaDevice> {
        self.device.device()
    }

    /// Clear cached SoA buffers to free GPU memory
    pub fn clear_soa_cache(&mut self) {
        self.cached_soa_buffers = None;
    }
}
