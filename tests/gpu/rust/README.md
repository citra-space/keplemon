# GPU Rust Tests

This directory contains Rust integration tests for GPU (CUDA) functionality.

## Running GPU Tests

All GPU tests are gated behind the `cuda` feature flag:

```bash
cargo test --features cuda
```

## Test Files

- `test_batch_propagator.rs` - Batch propagator GPU tests
- `test_gpu_cpu_parity.rs` - GPU vs CPU parity verification
- `pr_cuda_validation.rs` - CUDA validation tests
- `benchmark_cpu_vs_gpu.rs` - CPU vs GPU performance benchmarks
- `benchmark_gpu.rs` - GPU-only benchmarks
- `benchmark_gpu_accuracy.rs` - GPU accuracy benchmarks
- `benchmark_gpu_crossover.rs` - Crossover point analysis

## Benchmark Tests

Benchmark tests are marked with `#[ignore]` and should be run explicitly:

```bash
cargo test --release --features cuda -- --ignored --nocapture
```
