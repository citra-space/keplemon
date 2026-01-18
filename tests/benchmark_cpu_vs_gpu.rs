//! CPU vs GPU Performance Benchmark
//! 
//! Compares execution time between CPU (sequential) and GPU (parallel) SGP4 propagation
//! for varying numbers of satellites: 10, 20, 40, 80, 160, and 1000.
//! 
//! Uses a realistic mix of LEO (Starlink, ISS) and GEO (TDRS, Milstar) satellites
//! to represent actual operational scenarios.

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::bodies::Satellite;
use keplemon::gpu::{CudaSgp4Propagator, cuda_sgp4::TleDataGpu};
use keplemon::time::{Epoch, TimeSpan};
use std::time::Instant;

// Julian Date of 1950-01-01 00:00:00 UTC
const JD_1950: f64 = 2433281.5;

/// Sample LEO satellites (Starlink, ISS, etc.)
const LEO_TLES: &[(&str, &str, &str)] = &[
    (
        "ISS (ZARYA)",
        "1 25544U 98067A   25105.52083333  .00012345  00000+0  22013-3 0  9991",
        "2 25544  51.6456 339.5765 0003456  35.8734  85.9834 15.48919755123456"
    ),
    (
        "STARLINK-1007",
        "1 44713U 19074A   25105.50000000  .00001234  00000+0  89012-4 0  9992",
        "2 44713  53.0534 123.4567 0001234  89.0123 271.0987 15.06491234567890"
    ),
    (
        "STARLINK-1008",
        "1 44238U 19029K   25105.36500741  .00000456  00000+0  42959-4 0  9992",
        "2 44238  52.9995 315.6044 0001152  77.7635 282.3485 15.06404729317252"
    ),
    (
        "STARLINK-1010",
        "1 44239U 19029L   25105.50000000  .00000567  00000+0  50123-4 0  9993",
        "2 44239  53.0012 123.7890 0001345  88.1234 271.9876 15.06423456234567"
    ),
    (
        "STARLINK-1011",
        "1 44240U 19029M   25105.42167890  .00000678  00000+0  61234-4 0  9994",
        "2 44240  52.9987 234.5678 0001456  99.2345 260.7654 15.06445678345678"
    ),
    (
        "CALSPHERE 1",
        "1 00900U 64063C   25105.39292433  .00001284  00000+0  13113-2 0  9991",
        "2 00900  90.2142  61.9643 0026290 141.5577 317.1940 13.75977801 12586"
    ),
    (
        "OSCAR 7 (AO-7)",
        "1 07530U 74089B   25105.59750213 -.00000041  00000+0  31200-4 0  9993",
        "2 07530 101.9951 109.2734 0011942 225.8401 252.0737 12.53688339306933"
    ),
];

/// Sample GEO satellites (TDRS, Milstar, etc.)
const GEO_TLES: &[(&str, &str, &str)] = &[
    (
        "TDRS 3",
        "1 19548U 88091B   26008.31539492 -.00000299  00000+0  00000+0 0  9994",
        "2 19548  12.7229 342.0612 0044050 345.9566 204.1506  1.00262839123781"
    ),
    (
        "FLTSATCOM 8 (USA 46)",
        "1 20253U 89077A   26008.91781074 -.00000365  00000+0  00000+0 0  9999",
        "2 20253  12.4650 352.1184 0008582 304.1964 268.3305  1.00275407258969"
    ),
    (
        "SKYNET 4C",
        "1 20776U 90079A   26008.60325894  .00000127  00000+0  00000+0 0  9998",
        "2 20776  13.3902 350.9041 0003547 300.8791  66.8764  1.00271932129296"
    ),
    (
        "TDRS 5",
        "1 21639U 91054B   26009.20845241  .00000086  00000+0  00000+0 0  9995",
        "2 21639  14.0903 354.9540 0013306 264.5320 116.7826  1.00278867126127"
    ),
    (
        "TDRS 6",
        "1 22314U 93003B   26009.12667159 -.00000295  00000+0  00000+0 0  9996",
        "2 22314  14.1782 358.2372 0007634 209.3870 260.5236  1.00281076120817"
    ),
    (
        "LES-5",
        "1 02866U 67066E   25105.31584826 -.00000071  00000+0  00000+0 0  9994",
        "2 02866   1.6557 113.0780 0054733 189.7818 318.7327  1.09425579126352"
    ),
    (
        "OPS 3811 (DSP 2)",
        "1 05204U 71039A   25105.62550616  .00000013  00000+0  00000+0 0  9997",
        "2 05204   0.8783 286.6814 0021578   2.4750 200.5804  0.98159868202035"
    ),
];

/// Generate a mix of LEO and GEO TLEs
/// Mix ratio: ~60% LEO, ~40% GEO (typical operational distribution)
fn generate_mixed_tles(count: usize) -> Vec<TLE> {
    let mut tles = Vec::with_capacity(count);
    
    for i in 0..count {
        // 60% LEO, 40% GEO distribution
        let use_leo = (i % 5) < 3;
        
        let (name, line1, line2) = if use_leo {
            LEO_TLES[i % LEO_TLES.len()]
        } else {
            GEO_TLES[i % GEO_TLES.len()]
        };
        
        // Create TLE (each satellite will be unique enough from the mix)
        let tle = TLE::from_three_lines(name, line1, line2)
            .expect("Failed to parse TLE");
        
        tles.push(tle);
    }
    
    tles
}

/// Convert TLE to GPU format
fn tle_to_gpu(tle: &TLE) -> TleDataGpu {
    let epoch = tle.get_epoch();
    TleDataGpu {
        epoch_jd: epoch.days_since_1950 + JD_1950,
        inclination: tle.get_inclination(),
        raan: tle.get_raan(),
        eccentricity: tle.get_eccentricity(),
        arg_perigee: tle.get_argument_of_perigee(),
        mean_anomaly: tle.get_mean_anomaly(),
        mean_motion: tle.get_mean_motion(),
        bstar: tle.get_b_star(),
        ndot: tle.get_mean_motion_dot(),
        nddot: tle.get_mean_motion_dot_dot(),
    }
}

/// Benchmark CPU propagation (sequential)
fn benchmark_cpu(satellites: &[Satellite], times: &[Epoch]) -> std::time::Duration {
    let start = Instant::now();
    
    for sat in satellites.iter() {
        for &time in times.iter() {
            let _ = sat.get_state_at_epoch(time);
        }
    }
    
    start.elapsed()
}

/// Benchmark GPU propagation (parallel)
fn benchmark_gpu(tles: &[TLE], times_jd: &[f64]) -> Result<std::time::Duration, String> {
    // Convert to GPU format
    let tle_data_gpu: Vec<TleDataGpu> = tles.iter().map(tle_to_gpu).collect();
    
    // Initialize GPU propagator
    let mut gpu_propagator = CudaSgp4Propagator::new()
        .map_err(|e| format!("Failed to create GPU propagator: {}", e))?;
    
    gpu_propagator.init_satellites(&tle_data_gpu)
        .map_err(|e| format!("Failed to initialize satellites on GPU: {}", e))?;
    
    // Benchmark propagation
    let start = Instant::now();
    
    let _states = gpu_propagator.propagate(times_jd)
        .map_err(|e| format!("Failed to propagate on GPU: {}", e))?;
    
    Ok(start.elapsed())
}

/// Run benchmark for a specific satellite count
fn run_benchmark(n_satellites: usize, n_times: usize) {
    println!("\n{}", "=".repeat(70));
    println!("Benchmarking {} satellites at {} time points", n_satellites, n_times);
    println!("Total propagations: {}", n_satellites * n_times);
    println!("{}", "=".repeat(70));
    
    // Generate mixed LEO/GEO satellites
    let tles = generate_mixed_tles(n_satellites);
    let satellites: Vec<Satellite> = tles.iter()
        .map(|tle| Satellite::from(tle.clone()))
        .collect();
    
    // Count LEO vs GEO
    let n_leo = tles.iter()
        .filter(|tle| tle.get_mean_motion() > 1.5)  // LEO typically > 1.5 rev/day
        .count();
    let n_geo = n_satellites - n_leo;
    println!("Mix: {} LEO ({:.1}%), {} GEO ({:.1}%)", 
             n_leo, (n_leo as f64 / n_satellites as f64) * 100.0,
             n_geo, (n_geo as f64 / n_satellites as f64) * 100.0);
    
    // Generate time points (propagate over 24 hours at 1-hour intervals)
    let base_epoch = tles[0].get_keplerian_state().epoch;
    let times: Vec<Epoch> = (0..n_times)
        .map(|i| base_epoch + TimeSpan::from_hours(i as f64))
        .collect();
    
    // Also create JD times for GPU
    let base_jd = base_epoch.days_since_1950 + JD_1950;
    let times_jd: Vec<f64> = (0..n_times)
        .map(|i| base_jd + (i as f64) / 24.0)  // 1-hour intervals
        .collect();
    
    // CPU Benchmark
    print!("\nCPU (sequential): ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let cpu_time = benchmark_cpu(&satellites, &times);
    println!("{:.3} ms", cpu_time.as_secs_f64() * 1000.0);
    
    // GPU Benchmark
    print!("GPU (parallel):   ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    match benchmark_gpu(&tles, &times_jd) {
        Ok(gpu_time) => {
            println!("{:.3} ms", gpu_time.as_secs_f64() * 1000.0);
            
            // Calculate speedup
            let speedup = cpu_time.as_secs_f64() / gpu_time.as_secs_f64();
            println!("\nSpeedup: {:.2}x", speedup);
            
            // Calculate throughput
            let total_props = (n_satellites * n_times) as f64;
            let cpu_throughput = total_props / cpu_time.as_secs_f64();
            let gpu_throughput = total_props / gpu_time.as_secs_f64();
            
            println!("Throughput:");
            println!("  CPU: {:.0} propagations/sec", cpu_throughput);
            println!("  GPU: {:.0} propagations/sec", gpu_throughput);
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }
}

#[test]
fn test_benchmark_cpu_vs_gpu() {
    // Skip if CUDA not available
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping CPU vs GPU benchmark");
        return;
    }
    
    println!("\n");
    println!("{}", "#".repeat(70));
    println!("# CPU vs GPU SGP4 Performance Benchmark");
    println!("# Mixed LEO/GEO Satellite Propagation");
    println!("{}", "#".repeat(70));
    
    // Test with 24 time points (1 day at hourly intervals)
    let n_times = 24;
    
    // Benchmark different satellite counts
    let satellite_counts = vec![10, 20, 40, 80, 160, 1000];
    
    for &n_sats in satellite_counts.iter() {
        run_benchmark(n_sats, n_times);
    }
    
    // Summary
    println!("\n");
    println!("{}", "=".repeat(70));
    println!("Benchmark Complete!");
    println!("{}", "=".repeat(70));
}

/// Quick benchmark with just a few sizes for CI/testing
#[test]
fn test_quick_benchmark() {
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping quick benchmark");
        return;
    }
    
    println!("\n=== Quick CPU vs GPU Benchmark ===\n");
    
    let n_times = 10;
    
    for &n_sats in &[10, 40, 160] {
        run_benchmark(n_sats, n_times);
    }
}
