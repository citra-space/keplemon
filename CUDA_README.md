# CUDA GPU Acceleration for Keplemon

This branch adds optional CUDA GPU acceleration for batch satellite propagation in keplemon.

## Features

- **GPU-Accelerated SGP4**: Batch propagate hundreds or thousands of satellites simultaneously
- **Automatic Backend Selection**: Intelligently chooses GPU or CPU based on problem size
- **Transparent Integration**: Same API works with or without CUDA
- **Feature Flags**: Enable with `--features cuda`, zero overhead when disabled
- **Graceful Fallback**: Automatically uses CPU when CUDA unavailable

## Quick Start

### Build with CUDA Support

```bash
# Regular build (CPU only)
cargo build

# Build with CUDA support
cargo build --features cuda

# Run tests
cargo test --features cuda

# Run benchmarks
cargo bench --features cuda --bench gpu_propagation
```

### Requirements

- **CUDA Toolkit** 12.6+ (nvcc compiler)
- **NVIDIA GPU** with Compute Capability 5.0+ (Maxwell architecture or newer)
- Set `CUDA_PATH` environment variable if CUDA is not in `/usr/local/cuda`

### Example Usage

```rust
use keplemon::bodies::Constellation;
use keplemon::catalogs::TLECatalog;
use keplemon::time::{Epoch, TimeSpan};
use keplemon::propagation::PropagationBackend;

// Load satellite catalog
let catalog = TLECatalog::from_3le_file("satellites.tle")?;
let constellation = Constellation::from(catalog);

// Define time range
let start = Epoch::now();
let end = start + TimeSpan::from_hours(24.0);
let step = TimeSpan::from_minutes(10.0);

// Auto-select backend (GPU for large problems)
let states = constellation.get_batch_ephemeris(start, end, step, None);

// Force GPU backend
let states_gpu = constellation.get_batch_ephemeris(
    start, end, step,
    Some(PropagationBackend::Gpu)
);

// Force CPU backend
let states_cpu = constellation.get_batch_ephemeris(
    start, end, step,
    Some(PropagationBackend::Cpu)
);

// Check if GPU is available
if Constellation::is_gpu_available() {
    println!("CUDA GPU acceleration is available!");
}
```

## Architecture

### CUDA Kernels (`kernels/`)

- **sgp4_init.cu**: Initialize satellite parameters from TLEs
- **sgp4_batch.cu**: Batch SGP4 propagation kernel (Vallado's algorithm)
- **sgp4_types.cuh**: Shared data structures
- **sgp4_constants.cuh**: Physical and mathematical constants

### Rust GPU Module (`src/gpu/`)

- **cuda_sgp4.rs**: High-level GPU propagator interface
- **device.rs**: CUDA device management and kernel loading
- **memory.rs**: GPU memory utilities

### Batch Propagation (`src/propagation/`)

- **batch_propagator.rs**: Backend selection logic
  - `Auto`: Choose GPU when `n_sats × n_times > threshold` (default 1000)
  - `Cpu`: Force CPU propagation
  - `Gpu`: Force GPU propagation

## Performance

### GPU Crossover Points

Based on comprehensive benchmarking, GPU acceleration becomes beneficial when:

- **40+ satellites** with 45+ time points (e.g., 5 LEO periods at 10-min intervals)
- **100+ satellites** with just 9+ time points (e.g., 1 LEO period at 10-min intervals)
- **10 satellites** with 90+ time points (e.g., 1 LEO period at 1-min intervals)

**Critical Threshold**: ~900-1000 total propagations (satellites × time points)

**GPU Overhead**: ~0.15-0.20 ms (memory transfer + kernel launch)

### Performance by Scenario

**LEO Satellites (~90 minute period):**

| Satellites | Time Points             | Speedup | Recommendation |
| ---------- | ----------------------- | ------- | -------------- |
| 10         | 90 (1-min dt, 1 period) | 1.05x   | GPU wins       |
| 40         | 90 (1-min dt, 1 period) | 4.72x   | GPU wins       |
| 100        | 90 (1-min dt, 1 period) | 10.36x  | GPU wins       |
| 500        | 90 (1-min dt, 1 period) | 19.59x  | GPU wins       |

**GEO Satellites (~24 hour period):**

| Satellites | Time Points               | Speedup | Recommendation           |
| ---------- | ------------------------- | ------- | ------------------------ |
| 10         | 1436 (1-min dt, 1 period) | 15.34x  | GPU wins                 |
| 40         | 1436 (1-min dt, 1 period) | 31.79x  | GPU wins                 |
| 100        | 1436 (1-min dt, 1 period) | 41.40x  | **Best GPU performance** |
| 500        | 144 (10-min dt, 1 period) | 28.43x  | GPU wins                 |

**Week-Long Propagation (168 hours):**

| Satellites | CPU Time  | GPU Time | Speedup | Throughput      |
| ---------- | --------- | -------- | ------- | --------------- |
| 10         | 1.99 ms   | 5.06 ms  | 0.40x   | CPU better      |
| 40         | 7.68 ms   | 5.37 ms  | 1.43x   | GPU better      |
| 100        | 19.32 ms  | 6.89 ms  | 2.80x   | GPU better      |
| 1,000      | 191.05 ms | 57.40 ms | 3.33x   | 4.54M props/sec |

For detailed crossover analysis across different time steps, orbital periods, and satellite counts,
see [tests/GPU_CROSSOVER_ANALYSIS.md](tests/GPU_CROSSOVER_ANALYSIS.md).

### Running Benchmarks

```bash
# CPU vs GPU comparison (7-day stress test)
cargo test --features cuda --release benchmark_cpu_vs_gpu -- --nocapture

# Crossover point analysis
cargo test --features cuda --release test_gpu_crossover_analysis -- --nocapture
```

## Implementation Status

- [x] **Phase 0**: Branch setup and CUDA scaffolding
- [x] **Phase 1**: CUDA SGP4 kernels (init + propagation)
- [x] **Phase 2**: Rust GPU bindings (cudarc integration)
- [x] **Phase 3**: High-level API (batch propagator + constellation)
- [x] **Phase 4**: Testing and benchmarking
- [ ] **Phase 5**: Documentation and merge to main

### Completed Components

✅ SGP4 initialization kernel with derived constants  
✅ SGP4 batch propagation kernel (near-earth and deep-space satellites)  
✅ Deep space propagation bug fix (dpper baseline periodics)  
✅ CUDA device management and kernel loading  
✅ Automatic backend selection based on problem size  
✅ Constellation batch propagation methods  
✅ Unit tests for backend selection  
✅ Performance benchmarks (CPU vs GPU)  
✅ Comprehensive crossover point analysis  
✅ Build system integration (PTX compilation)

### TODO

- [ ] Full TLE → GPU data structure conversion
- [ ] Integration tests with real satellite data
- [ ] Python bindings for GPU features
- [ ] Multi-GPU support for very large catalogs
- [ ] Memory pooling for repeated propagations
- [ ] Async GPU operations for pipeline overlap

## Building without CUDA

The code compiles and runs normally without CUDA:

```bash
# Regular build - no CUDA dependencies
cargo build

# All tests still work (GPU tests skipped)
cargo test
```

Feature flags ensure zero overhead when CUDA is not needed.

## Troubleshooting

### "nvcc not found"

Install CUDA Toolkit or set `CUDA_PATH`:

```bash
export CUDA_PATH=/usr/local/cuda
```

### "CUDA device initialization failed"

- Check NVIDIA drivers: `nvidia-smi`
- Verify GPU compute capability: must be 5.0+
- Check CUDA version matches cudarc feature flag in Cargo.toml

### "GPU not available" at runtime

The code will automatically fall back to CPU. Check:

```rust
if Constellation::is_gpu_available() {
    println!("GPU available");
} else {
    println!("Using CPU fallback");
}
```

## Technical Details

### Memory Layout

GPU data structures are carefully aligned for optimal memory access:

```rust
#[repr(C, align(16))]
struct Sgp4StateGpu {
    x, y, z: f64,      // Position (km, TEME frame)
    vx, vy, vz: f64,   // Velocity (km/s)
    error_code: i32,   // Propagation status
}
```

### Kernel Launch Configuration

- **1D grid**: Satellite initialization (256 threads/block)
- **2D grid**: Batch propagation (16×16 threads/block)
- Satellites mapped to X dimension, time steps to Y dimension
- Each thread computes one (satellite, time) pair

### Compilation Options

CUDA kernels are compiled with aggressive optimizations:

- `-O3`: Maximum optimization
- `--use_fast_math`: Fast math operations
- `-arch=sm_50`: Support Maxwell architecture and newer

## Contributing

When adding features to the CUDA implementation:

1. Maintain feature flag compatibility (`#[cfg(feature = "cuda")]`)
2. Ensure graceful CPU fallback
3. Add tests to `tests/gpu/`
4. Update benchmarks in `benches/gpu_propagation.rs`
5. Keep CUDA kernels in `kernels/` directory

## References

- [Vallado, D. A. - "Fundamentals of Astrodynamics and Applications"](https://celestrak.org/software/vallado-sw.php)
- [SGP4 Algorithm Documentation](https://celestrak.org/publications/AIAA/2006-6753/)
- [cudarc Rust CUDA Bindings](https://github.com/coreylowman/cudarc)
- [NVIDIA CUDA Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/)

## License

Same as keplemon - MIT License
