//! Quick timing comparison between AoS and SoA kernels
//!
//! Run with: cargo test --features cuda --release quick_timing -- --nocapture

#![cfg(feature = "cuda")]

use keplemon::catalogs::TLECatalog;
use keplemon::gpu::{CudaTlePropagator, TleDataGpu};
use std::time::Instant;

fn load_tles(n: usize) -> Vec<TleDataGpu> {
    let catalog = TLECatalog::from_tle_file("tests/2025-04-15-celestrak.3le").expect("Failed to load TLE file");
    catalog.values().take(n).map(TleDataGpu::from).collect()
}

#[test]
#[ignore]
fn quick_timing_comparison() {
    let n_sats = 1000;
    let n_times = 100;
    let n_warmup = 3;
    let n_iters = 10;

    let tles = load_tles(n_sats);
    println!("\n=== Quick SoA vs AoS Timing ===");
    println!("Satellites: {}", tles.len());
    println!("Time steps: {}", n_times);
    println!("Iterations: {}", n_iters);

    let mut propagator = CudaTlePropagator::new().expect("Failed to create propagator");
    propagator.init_satellites(&tles).expect("Failed to init satellites");

    let base_jd = 2460500.5;
    let jd_times: Vec<f64> = (0..n_times).map(|i| base_jd + (i as f64) * 0.01).collect();

    // Warmup
    for _ in 0..n_warmup {
        let _ = propagator.propagate(&jd_times);
        let _ = propagator.propagate_soa_arrays(&jd_times);
    }

    // Time AoS
    let start = Instant::now();
    for _ in 0..n_iters {
        let _ = propagator.propagate(&jd_times).unwrap();
    }
    let aos_time = start.elapsed();
    let aos_per_iter = aos_time.as_secs_f64() / n_iters as f64 * 1000.0;

    // Time SoA
    let start = Instant::now();
    for _ in 0..n_iters {
        let _ = propagator.propagate_soa_arrays(&jd_times).unwrap();
    }
    let soa_time = start.elapsed();
    let soa_per_iter = soa_time.as_secs_f64() / n_iters as f64 * 1000.0;

    let speedup = aos_per_iter / soa_per_iter;

    println!("\nResults:");
    println!("  AoS kernel: {:.3} ms/call", aos_per_iter);
    println!("  SoA kernel: {:.3} ms/call", soa_per_iter);
    println!("  Speedup:    {:.2}x", speedup);

    if speedup > 1.0 {
        println!("  => SoA is {:.1}% faster", (speedup - 1.0) * 100.0);
    } else {
        println!("  => AoS is {:.1}% faster", (1.0 / speedup - 1.0) * 100.0);
    }
}
