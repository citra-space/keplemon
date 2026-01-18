//! Benchmark GPU vs CPU batch propagation
//! 
//! Run with: cargo bench --features cuda --bench gpu_propagation

#![cfg(feature = "cuda")]

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

#[cfg(feature = "cuda")]
use keplemon::gpu::{CudaSgp4Propagator, TleDataGpu};
use keplemon::catalogs::TLECatalog;

/// Generate synthetic test TLE data for benchmarking
#[cfg(feature = "cuda")]
fn generate_test_tles(n: usize) -> Vec<TleDataGpu> {
    // ISS-like TLE as base
    let base = TleDataGpu {
        epoch_jd: 2460500.5,
        inclination: 51.6416,
        raan: 247.4627,
        eccentricity: 0.0006703,
        arg_perigee: 130.5360,
        mean_anomaly: 325.0288,
        mean_motion: 15.72125391,
        bstar: 0.00013882,
        ndot: 0.0,
        nddot: 0.0,
    };
    
    (0..n).map(|i| {
        let mut tle = base;
        // Vary orbital elements to create distinct satellites
        tle.mean_anomaly = (i as f64 * 3.6) % 360.0;
        tle.raan = (i as f64 * 1.5) % 360.0;
        tle.inclination = 45.0 + (i as f64 * 0.01) % 45.0;
        tle
    }).collect()
}

/// Load TLEs from file and convert to GPU format
#[cfg(feature = "cuda")]
fn load_tles_from_catalog(path: &str) -> Result<Vec<TleDataGpu>, String> {
    let catalog = TLECatalog::from_tle_file(path)?;
    Ok(catalog.values().map(TleDataGpu::from).collect())
}

/// Benchmark comparing AoS vs SoA GPU kernel implementations
/// 
/// This directly benchmarks the CUDA kernels to measure the performance
/// improvement from Struct of Arrays memory layout optimization.
#[cfg(feature = "cuda")]
fn bench_aos_vs_soa_kernel(c: &mut Criterion) {
    // Load test TLEs
    let tle_path = "tests/2025-04-15-celestrak.3le";
    
    if !std::path::Path::new(tle_path).exists() {
        eprintln!("Test TLE file not found, skipping AoS vs SoA benchmark");
        return;
    }
    
    let tle_data = load_tles_from_catalog(tle_path)
        .expect("Failed to load TLE catalog");
    
    let n_sats = tle_data.len();
    
    // Create propagator and initialize
    let mut propagator = CudaSgp4Propagator::new()
        .expect("Failed to create CUDA propagator");
    
    propagator.init_satellites(&tle_data)
        .expect("Failed to initialize satellites");
    
    // Generate test times (Julian Dates)
    let base_jd = 2460500.5; // Some reference date
    
    let mut group = c.benchmark_group("aos_vs_soa_kernel");
    group.sample_size(50); // Reduce sample size for faster benchmarks
    
    // Test various time counts
    for n_times in [10, 50, 100, 500].iter() {
        let jd_times: Vec<f64> = (0..*n_times)
            .map(|i| base_jd + (i as f64) * 0.01) // ~15 min intervals
            .collect();
        
        let label = format!("{}sats_{}times", n_sats, n_times);
        
        // Benchmark AoS kernel (original)
        group.bench_with_input(
            BenchmarkId::new("AoS", &label),
            &jd_times,
            |b, times| {
                b.iter(|| {
                    propagator.propagate(black_box(times))
                        .expect("AoS propagation failed")
                });
            },
        );
        
        // Benchmark SoA kernel (optimized)
        group.bench_with_input(
            BenchmarkId::new("SoA", &label),
            &jd_times,
            |b, times| {
                b.iter(|| {
                    propagator.propagate_soa(black_box(times))
                        .expect("SoA propagation failed")
                });
            },
        );
        
        // Benchmark SoA with direct array output (no conversion)
        group.bench_with_input(
            BenchmarkId::new("SoA_arrays", &label),
            &jd_times,
            |b, times| {
                b.iter(|| {
                    propagator.propagate_soa_arrays(black_box(times))
                        .expect("SoA arrays propagation failed")
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark SoA kernel scaling with satellite count
#[cfg(feature = "cuda")]
fn bench_soa_scaling(c: &mut Criterion) {
    let tle_path = "tests/2025-04-15-celestrak.3le";
    
    if !std::path::Path::new(tle_path).exists() {
        eprintln!("Test TLE file not found, skipping scaling benchmark");
        return;
    }
    
    let all_tles = load_tles_from_catalog(tle_path)
        .expect("Failed to load TLE catalog");
    
    let base_jd = 2460500.5;
    let n_times = 100;
    let jd_times: Vec<f64> = (0..n_times)
        .map(|i| base_jd + (i as f64) * 0.01)
        .collect();
    
    let mut group = c.benchmark_group("soa_scaling");
    group.sample_size(30);
    
    // Test with different satellite counts
    for n_sats in [100, 500, 1000, 2000, 5000].iter() {
        if *n_sats > all_tles.len() {
            continue;
        }
        
        let tle_subset: Vec<TleDataGpu> = all_tles[..*n_sats].to_vec();
        
        let mut propagator = CudaSgp4Propagator::new()
            .expect("Failed to create CUDA propagator");
        propagator.init_satellites(&tle_subset)
            .expect("Failed to initialize satellites");
        
        group.bench_with_input(
            BenchmarkId::new("SoA", format!("{}sats", n_sats)),
            &jd_times,
            |b, times| {
                b.iter(|| {
                    propagator.propagate_soa_arrays(black_box(times))
                        .expect("SoA propagation failed")
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("AoS", format!("{}sats", n_sats)),
            &jd_times,
            |b, times| {
                b.iter(|| {
                    propagator.propagate(black_box(times))
                        .expect("AoS propagation failed")
                });
            },
        );
    }
    
    group.finish();
}

#[cfg(feature = "cuda")]
criterion_group!(benches, bench_aos_vs_soa_kernel, bench_soa_scaling);

#[cfg(not(feature = "cuda"))]
fn bench_placeholder(_c: &mut Criterion) {
    eprintln!("CUDA feature not enabled, no GPU benchmarks available");
}

#[cfg(not(feature = "cuda"))]
criterion_group!(benches, bench_placeholder);

criterion_main!(benches);
