//! GPU Integration Tests
//!
//! This test harness includes all GPU-related integration tests from the gpu/rust/ subdirectory.
//! Each module is a separate integration test suite for GPU functionality.

#![cfg(feature = "cuda")]

// GPU test modules
#[path = "gpu/rust/test_batch_propagator.rs"]
mod test_batch_propagator;

#[path = "gpu/rust/test_gpu_cpu_parity.rs"]
mod test_gpu_cpu_parity;

#[path = "gpu/rust/pr_cuda_validation.rs"]
mod pr_cuda_validation;

#[path = "gpu/rust/benchmark_cpu_vs_gpu.rs"]
mod benchmark_cpu_vs_gpu;

#[path = "gpu/rust/benchmark_gpu.rs"]
mod benchmark_gpu;

#[path = "gpu/rust/benchmark_gpu_accuracy.rs"]
mod benchmark_gpu_accuracy;

#[path = "gpu/rust/benchmark_gpu_crossover.rs"]
mod benchmark_gpu_crossover;
