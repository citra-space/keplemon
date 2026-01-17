# CUDA Support for Keplemon

This directory contains CUDA kernels for GPU-accelerated SGP4 batch propagation.

## Files

- `sgp4_constants.cuh` - Physical and mathematical constants for SGP4
- `sgp4_types.cuh` - Shared data structures between CUDA and Rust
- `sgp4_init.cu` - (TBD) TLE initialization kernel
- `sgp4_batch.cu` - (TBD) Main batch propagation kernel

## Building

CUDA kernels are compiled to PTX at build time when the `cuda` feature is enabled:

```bash
cargo build --features cuda
```

Requires NVIDIA CUDA Toolkit (nvcc) to be installed.

## Architecture

The GPU implementation follows Vallado's SGP4 algorithm with optimizations for batch processing:

1. **Initialization Phase** - Parse TLEs and compute derived constants (once per satellite)
2. **Propagation Phase** - Propagate all satellites to all requested times in parallel

Each CUDA thread handles one (satellite, time) pair for optimal parallelization.
