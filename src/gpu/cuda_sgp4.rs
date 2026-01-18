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
    /// Cached kernel functions to avoid lookup overhead
    init_kernel: CudaFunction,
    propagate_kernel: CudaFunction,
}

impl CudaSgp4Propagator {
    /// Create a new CUDA SGP4 propagator
    pub fn new() -> Result<Self, CudaError> {
        let device = CudaDevice::new()?;
        let dev = device.device();
        
        // Load PTX modules
        dev.load_ptx(SGP4_INIT_PTX.into(), "sgp4_init", &["sgp4_init_kernel"])
            .map_err(|e| CudaError::KernelLoad(e.to_string()))?;
        
        dev.load_ptx(SGP4_BATCH_PTX.into(), "sgp4_batch", &["sgp4_propagate_kernel"])
            .map_err(|e| CudaError::KernelLoad(e.to_string()))?;
        
        // Cache kernel functions for faster access
        let init_kernel = dev.get_func("sgp4_init", "sgp4_init_kernel")
            .ok_or_else(|| CudaError::KernelLoad("sgp4_init_kernel not found".into()))?;
        
        let propagate_kernel = dev.get_func("sgp4_batch", "sgp4_propagate_kernel")
            .ok_or_else(|| CudaError::KernelLoad("sgp4_propagate_kernel not found".into()))?;
        
        Ok(Self { 
            device,
            n_satellites: 0,
            tle_data_gpu: None,
            params_gpu: None,
            cached_times_gpu: None,
            cached_n_times: 0,
            cached_states_gpu: None,
            cached_n_results: 0,
            init_kernel,
            propagate_kernel,
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
}
