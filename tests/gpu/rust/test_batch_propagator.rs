//! GPU module tests - only run when cuda feature is enabled

#![cfg(feature = "cuda")]

use keplemon::gpu::{CudaTlePropagator, device::CudaDevice};
use keplemon::propagation::{BatchPropagator, PropagationBackend};

#[test]
fn test_cuda_device_initialization() {
    // This test just checks if we can create a CUDA device
    // It will be skipped if CUDA is not available on the system
    if !CudaDevice::is_available() {
        eprintln!("CUDA not available, skipping GPU tests");
        return;
    }

    let result = CudaDevice::new();
    assert!(result.is_ok(), "Failed to create CUDA device: {:?}", result.err());
}

#[test]
fn test_cuda_propagator_creation() {
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping GPU tests");
        return;
    }

    let result = CudaTlePropagator::new();
    assert!(result.is_ok(), "Failed to create CUDA propagator: {:?}", result.err());
}

#[test]
fn test_batch_propagator_backend_selection() {
    let propagator = BatchPropagator::new();

    // Small problem should use CPU
    let backend = propagator.select_backend(10, 10);
    println!("Backend for 10×10: {:?}", backend);

    // Large problem should use GPU if available
    let backend = propagator.select_backend(1000, 100);
    println!("Backend for 1000×100: {:?}", backend);
    println!("GPU available: {}", propagator.is_gpu_available());
}

#[test]
fn test_force_gpu_backend() {
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping GPU tests");
        return;
    }

    let propagator = BatchPropagator::new().set_backend(PropagationBackend::Gpu);

    let backend = propagator.select_backend(10, 10);
    // Should be GPU even for small problem
    assert!(matches!(backend, keplemon::propagation::SelectedBackend::Gpu));
}

#[test]
fn test_force_cpu_backend() {
    let propagator = BatchPropagator::new().set_backend(PropagationBackend::Cpu);

    let backend = propagator.select_backend(10000, 1000);
    // Should be CPU even for large problem
    assert!(matches!(backend, keplemon::propagation::SelectedBackend::Cpu));
}

#[test]
fn test_custom_threshold() {
    let propagator = BatchPropagator::new().set_gpu_threshold(100000);

    // 10000 < 100000, should use CPU
    let backend = propagator.select_backend(100, 100);
    assert!(matches!(backend, keplemon::propagation::SelectedBackend::Cpu));
}
