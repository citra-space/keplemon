//! CPU vs GPU Performance Benchmark
//!
//! Compares execution time between CPU (sequential), CPU (parallel with rayon),
//! and GPU (parallel) SGP4 propagation for varying numbers of satellites.
//!
//! Uses a realistic mix of LEO (Starlink, ISS) and GEO (TDRS, Milstar) satellites
//! to represent actual operational scenarios.

#![cfg(feature = "cuda")]

use keplemon::bodies::Satellite;
use keplemon::elements::TLE;
use keplemon::gpu::{CudaGeoNumericalPropagator, CudaTlePropagator, GeoStateGpu, cuda_tle::TleDataGpu};
use keplemon::time::{Epoch, TimeSpan};
use rayon::prelude::*;
use std::time::Instant;

// Julian Date of 1950-01-01 00:00:00 UTC
const JD_1950: f64 = 2433281.5;

/// Sample LEO satellites (Starlink, ISS, etc.)
const LEO_TLES: &[(&str, &str, &str)] = &[
    (
        "ISS (ZARYA)",
        "1 25544U 98067A   25105.52083333  .00012345  00000+0  22013-3 0  9991",
        "2 25544  51.6456 339.5765 0003456  35.8734  85.9834 15.48919755123456",
    ),
    (
        "STARLINK-1007",
        "1 44713U 19074A   25105.50000000  .00001234  00000+0  89012-4 0  9992",
        "2 44713  53.0534 123.4567 0001234  89.0123 271.0987 15.06491234567890",
    ),
    (
        "STARLINK-1008",
        "1 44238U 19029K   25105.36500741  .00000456  00000+0  42959-4 0  9992",
        "2 44238  52.9995 315.6044 0001152  77.7635 282.3485 15.06404729317252",
    ),
    (
        "STARLINK-1010",
        "1 44239U 19029L   25105.50000000  .00000567  00000+0  50123-4 0  9993",
        "2 44239  53.0012 123.7890 0001345  88.1234 271.9876 15.06423456234567",
    ),
    (
        "STARLINK-1011",
        "1 44240U 19029M   25105.42167890  .00000678  00000+0  61234-4 0  9994",
        "2 44240  52.9987 234.5678 0001456  99.2345 260.7654 15.06445678345678",
    ),
    (
        "CALSPHERE 1",
        "1 00900U 64063C   25105.39292433  .00001284  00000+0  13113-2 0  9991",
        "2 00900  90.2142  61.9643 0026290 141.5577 317.1940 13.75977801 12586",
    ),
    (
        "OSCAR 7 (AO-7)",
        "1 07530U 74089B   25105.59750213 -.00000041  00000+0  31200-4 0  9993",
        "2 07530 101.9951 109.2734 0011942 225.8401 252.0737 12.53688339306933",
    ),
];

/// Sample GEO satellites (TDRS, Milstar, etc.)
const GEO_TLES: &[(&str, &str, &str)] = &[
    (
        "TDRS 3",
        "1 19548U 88091B   26008.31539492 -.00000299  00000+0  00000+0 0  9994",
        "2 19548  12.7229 342.0612 0044050 345.9566 204.1506  1.00262839123781",
    ),
    (
        "FLTSATCOM 8 (USA 46)",
        "1 20253U 89077A   26008.91781074 -.00000365  00000+0  00000+0 0  9999",
        "2 20253  12.4650 352.1184 0008582 304.1964 268.3305  1.00275407258969",
    ),
    (
        "SKYNET 4C",
        "1 20776U 90079A   26008.60325894  .00000127  00000+0  00000+0 0  9998",
        "2 20776  13.3902 350.9041 0003547 300.8791  66.8764  1.00271932129296",
    ),
    (
        "TDRS 5",
        "1 21639U 91054B   26009.20845241  .00000086  00000+0  00000+0 0  9995",
        "2 21639  14.0903 354.9540 0013306 264.5320 116.7826  1.00278867126127",
    ),
    (
        "TDRS 6",
        "1 22314U 93003B   26009.12667159 -.00000295  00000+0  00000+0 0  9996",
        "2 22314  14.1782 358.2372 0007634 209.3870 260.5236  1.00281076120817",
    ),
    (
        "LES-5",
        "1 02866U 67066E   25105.31584826 -.00000071  00000+0  00000+0 0  9994",
        "2 02866   1.6557 113.0780 0054733 189.7818 318.7327  1.09425579126352",
    ),
    (
        "OPS 3811 (DSP 2)",
        "1 05204U 71039A   25105.62550616  .00000013  00000+0  00000+0 0  9997",
        "2 05204   0.8783 286.6814 0021578   2.4750 200.5804  0.98159868202035",
    ),
];

/// Orbit regime for benchmark configuration
#[derive(Debug, Clone, Copy)]
enum OrbitRegime {
    LeoOnly,
    GeoOnly,
    Mixed,
}

impl OrbitRegime {
    fn name(&self) -> &str {
        match self {
            OrbitRegime::LeoOnly => "LEO only",
            OrbitRegime::GeoOnly => "GEO only",
            OrbitRegime::Mixed => "Mixed (60% LEO, 40% GEO)",
        }
    }
}

/// Generate TLEs based on orbit regime
fn generate_tles(count: usize, regime: OrbitRegime) -> Vec<TLE> {
    let mut tles = Vec::with_capacity(count);

    for i in 0..count {
        let (name, line1, line2) = match regime {
            OrbitRegime::LeoOnly => LEO_TLES[i % LEO_TLES.len()],
            OrbitRegime::GeoOnly => GEO_TLES[i % GEO_TLES.len()],
            OrbitRegime::Mixed => {
                // 60% LEO, 40% GEO distribution
                let use_leo = (i % 5) < 3;
                if use_leo {
                    LEO_TLES[i % LEO_TLES.len()]
                } else {
                    GEO_TLES[i % GEO_TLES.len()]
                }
            }
        };

        // Create TLE (each satellite will be unique enough from the mix)
        let tle = TLE::from_three_lines(name, line1, line2).expect("Failed to parse TLE");

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
fn benchmark_cpu_sequential(satellites: &[Satellite], times: &[Epoch]) -> std::time::Duration {
    let start = Instant::now();

    for sat in satellites.iter() {
        for &time in times.iter() {
            let _ = sat.get_state_at_epoch(time);
        }
    }

    start.elapsed()
}

/// Benchmark CPU propagation (parallel with rayon)
fn benchmark_cpu_parallel(satellites: &[Satellite], times: &[Epoch]) -> std::time::Duration {
    let start = Instant::now();

    satellites.par_iter().for_each(|sat| {
        for &time in times.iter() {
            let _ = sat.get_state_at_epoch(time);
        }
    });

    start.elapsed()
}

/// GPU benchmark result with breakdown
struct GpuBenchmarkResult {
    /// Time including GPU→CPU transfer
    total_time: std::time::Duration,
    /// Time for kernel only (GPU-resident, no transfer)
    kernel_time: std::time::Duration,
}

/// Benchmark GPU propagation (parallel)
///
/// Uses the two-kernel optimization: partitions satellites by propagator type
/// (SGP4 for LEO, SDP4 for GEO) to eliminate warp divergence.
fn benchmark_gpu(tles: &[TLE], times_jd: &[f64]) -> Result<GpuBenchmarkResult, String> {
    // Convert to GPU format
    let tle_data_gpu: Vec<TleDataGpu> = tles.iter().map(tle_to_gpu).collect();

    // Initialize GPU propagator (partitions satellites internally)
    let mut gpu_propagator = CudaTlePropagator::new().map_err(|e| format!("Failed to create GPU propagator: {}", e))?;

    gpu_propagator
        .init_satellites(&tle_data_gpu)
        .map_err(|e| format!("Failed to initialize satellites on GPU: {}", e))?;

    // Warmup call to ensure all allocations are done
    let _ = gpu_propagator
        .propagate_soa_arrays(times_jd)
        .map_err(|e| format!("Warmup failed: {}", e))?;

    // Benchmark 1: Full pipeline including GPU→CPU transfer
    let start_total = Instant::now();
    let _states = gpu_propagator
        .propagate_soa_arrays(times_jd)
        .map_err(|e| format!("Failed to propagate on GPU: {}", e))?;
    let total_time = start_total.elapsed();

    // Benchmark 2: GPU-resident (kernel only, no transfer)
    let start_kernel = Instant::now();
    let _gpu_resident = gpu_propagator
        .propagate_soa_resident(times_jd)
        .map_err(|e| format!("Failed to propagate (GPU-resident): {}", e))?;
    let kernel_time = start_kernel.elapsed();

    Ok(GpuBenchmarkResult {
        total_time,
        kernel_time,
    })
}

/// Run benchmark for a specific satellite count and orbit regime
fn run_benchmark(n_satellites: usize, n_times: usize, regime: OrbitRegime) {
    println!("\n{}", "=".repeat(70));
    println!("Benchmarking {} satellites at {} time points", n_satellites, n_times);
    println!("Orbit regime: {}", regime.name());
    println!("Total propagations: {}", n_satellites * n_times);
    println!("{}", "=".repeat(70));

    // Generate satellites based on orbit regime
    let tles = generate_tles(n_satellites, regime);
    let satellites: Vec<Satellite> = tles.iter().map(|tle| Satellite::from(tle.clone())).collect();

    // Count LEO vs GEO for verification
    let n_leo = tles
        .iter()
        .filter(|tle| tle.get_mean_motion() > 1.5) // LEO typically > 1.5 rev/day
        .count();
    let n_geo = n_satellites - n_leo;
    println!(
        "Actual mix: {} LEO ({:.1}%), {} GEO ({:.1}%)",
        n_leo,
        (n_leo as f64 / n_satellites as f64) * 100.0,
        n_geo,
        (n_geo as f64 / n_satellites as f64) * 100.0
    );

    // Generate time points (propagate at 1-minute intervals)
    let base_epoch = tles[0].get_keplerian_state().epoch;
    let times: Vec<Epoch> = (0..n_times)
        .map(|i| base_epoch + TimeSpan::from_minutes(i as f64))
        .collect();

    // Also create JD times for GPU
    let base_jd = base_epoch.days_since_1950 + JD_1950;
    let times_jd: Vec<f64> = (0..n_times)
        .map(|i| base_jd + (i as f64) / 1440.0) // 1-minute intervals (1440 minutes/day)
        .collect();

    // CPU Sequential Benchmark
    print!("\nCPU (sequential):  ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let cpu_seq_time = benchmark_cpu_sequential(&satellites, &times);
    println!("{:.3} ms", cpu_seq_time.as_secs_f64() * 1000.0);

    // CPU Parallel Benchmark
    print!("CPU (parallel):    ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let cpu_par_time = benchmark_cpu_parallel(&satellites, &times);
    let cpu_par_ms = cpu_par_time.as_secs_f64() * 1000.0;
    let cpu_speedup = cpu_seq_time.as_secs_f64() / cpu_par_time.as_secs_f64();
    println!("{:.3} ms  ({:.2}x vs sequential)", cpu_par_ms, cpu_speedup);

    // GPU Benchmark
    match benchmark_gpu(&tles, &times_jd) {
        Ok(gpu_result) => {
            let kernel_ms = gpu_result.kernel_time.as_secs_f64() * 1000.0;
            let total_ms = gpu_result.total_time.as_secs_f64() * 1000.0;
            let transfer_ms = total_ms - kernel_ms;

            println!("GPU (kernel only): {:.3} ms", kernel_ms);
            println!(
                "GPU (+ transfer):  {:.3} ms  (transfer: {:.3} ms, {:.1}% overhead)",
                total_ms,
                transfer_ms,
                (transfer_ms / total_ms) * 100.0
            );

            // Calculate speedups vs sequential CPU
            let speedup_kernel_vs_seq = cpu_seq_time.as_secs_f64() / gpu_result.kernel_time.as_secs_f64();
            let speedup_total_vs_seq = cpu_seq_time.as_secs_f64() / gpu_result.total_time.as_secs_f64();

            // Calculate speedups vs parallel CPU
            let speedup_kernel_vs_par = cpu_par_time.as_secs_f64() / gpu_result.kernel_time.as_secs_f64();
            let speedup_total_vs_par = cpu_par_time.as_secs_f64() / gpu_result.total_time.as_secs_f64();

            println!("\nSpeedup vs CPU Sequential:");
            println!("  GPU (kernel only): {:.2}x", speedup_kernel_vs_seq);
            println!("  GPU (+ transfer):  {:.2}x", speedup_total_vs_seq);

            println!("\nSpeedup vs CPU Parallel:");
            println!("  GPU (kernel only): {:.2}x", speedup_kernel_vs_par);
            println!("  GPU (+ transfer):  {:.2}x", speedup_total_vs_par);

            // Calculate throughput
            let total_props = (n_satellites * n_times) as f64;
            let cpu_seq_throughput = total_props / cpu_seq_time.as_secs_f64();
            let cpu_par_throughput = total_props / cpu_par_time.as_secs_f64();
            let gpu_kernel_throughput = total_props / gpu_result.kernel_time.as_secs_f64();
            let gpu_total_throughput = total_props / gpu_result.total_time.as_secs_f64();

            println!("\nThroughput:");
            println!("  CPU (sequential): {:>12.0} propagations/sec", cpu_seq_throughput);
            println!("  CPU (parallel):   {:>12.0} propagations/sec", cpu_par_throughput);
            println!("  GPU (kernel):     {:>12.0} propagations/sec", gpu_kernel_throughput);
            println!("  GPU (+ transfer): {:>12.0} propagations/sec", gpu_total_throughput);
        }
        Err(e) => {
            println!("GPU ERROR: {}", e);
        }
    }
}

#[test]
#[ignore]
fn test_benchmark_cpu_vs_gpu() {
    // Skip if CUDA not available
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping CPU vs GPU benchmark");
        return;
    }

    println!("\n");
    println!("{}", "#".repeat(70));
    println!("# CPU (Sequential/Parallel) vs GPU SGP4 Performance Benchmark");
    println!("# LEO Only, GEO Only, and Mixed Constellations");
    println!("{}", "#".repeat(70));

    // Test with 10,080 time points (7 days at 1-minute intervals)
    let n_times = 10_080;

    // Benchmark different satellite counts
    let satellite_counts = vec![10, 50, 100, 1000];

    // Test all three orbit regimes
    let regimes = vec![OrbitRegime::LeoOnly, OrbitRegime::GeoOnly, OrbitRegime::Mixed];

    for &regime in regimes.iter() {
        println!("\n");
        println!("{}", "█".repeat(70));
        println!("█ ORBIT REGIME: {}", regime.name());
        println!("{}", "█".repeat(70));

        for &n_sats in satellite_counts.iter() {
            run_benchmark(n_sats, n_times, regime);
        }
    }

    // Summary
    println!("\n");
    println!("{}", "=".repeat(70));
    println!("Benchmark Complete!");
    println!("{}", "=".repeat(70));
}

/// Quick benchmark with just a few sizes for CI/testing
#[test]
#[ignore]
fn test_quick_benchmark() {
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping quick benchmark");
        return;
    }

    println!("\n=== Quick CPU vs GPU Benchmark ===\n");

    let n_times = 10;

    for &n_sats in &[10, 40, 160] {
        run_benchmark(n_sats, n_times, OrbitRegime::Mixed);
    }
}

/// Benchmark GPU GEO Analytical propagation
fn benchmark_geo_analytical(n_sats: usize, times_jd: &[f64]) -> Result<GpuBenchmarkResult, String> {
    // Generate GEO satellite states distributed around the geostationary belt
    let geo_states: Vec<GeoStateGpu> = (0..n_sats)
        .map(|i| {
            let angle = (i as f64) * 360.0 / (n_sats as f64) * std::f64::consts::PI / 180.0;
            let r = 42164.0; // GEO radius (km)
            let v = 3.0746; // GEO velocity (km/s)
            GeoStateGpu::new(
                [r * angle.cos(), r * angle.sin(), 0.0],
                [-v * angle.sin(), v * angle.cos(), 0.0],
                times_jd[0],
                None,
                None,
            )
        })
        .collect();

    let mut propagator =
        CudaGeoNumericalPropagator::new().map_err(|e| format!("Failed to create GEO propagator: {}", e))?;

    propagator
        .init_satellites(&geo_states)
        .map_err(|e| format!("Failed to initialize GEO satellites: {}", e))?;

    // Warmup
    let _ = propagator
        .propagate_soa_arrays(times_jd)
        .map_err(|e| format!("Warmup failed: {}", e))?;

    // Benchmark total time (kernel + transfer)
    // GEO Analytical doesn't have a separate GPU-resident method, but the kernel
    // dominates the time for large propagations
    let start = Instant::now();
    let _ = propagator
        .propagate_soa_arrays(times_jd)
        .map_err(|e| format!("Propagation failed: {}", e))?;
    let total_time = start.elapsed();

    // For GEO analytical, kernel time ≈ total time (transfer overhead is small)
    Ok(GpuBenchmarkResult {
        total_time,
        kernel_time: total_time,
    })
}

/// Comprehensive benchmark comparing CPU SDP4 vs GPU SDP4 vs GPU GEO Analytical
/// for GEO-regime satellites only
#[test]
#[ignore]
fn test_benchmark_geo_analytical() {
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping GEO analytical benchmark");
        return;
    }

    println!("\n");
    println!("{}", "#".repeat(70));
    println!("# CPU SDP4 vs GPU SDP4 vs GPU GEO Analytical Benchmark");
    println!("# GEO-regime satellites only");
    println!("{}", "#".repeat(70));

    // Test configurations
    let satellite_counts = vec![100, 500, 1000];
    let n_times = 10_080; // 7 days at 1-minute intervals

    for &n_sats in &satellite_counts {
        println!("\n{}", "=".repeat(70));
        println!(
            "Satellites: {} | Timesteps: {} | Total propagations: {}",
            n_sats,
            n_times,
            n_sats * n_times
        );
        println!("{}", "=".repeat(70));

        // Generate GEO TLEs for CPU and GPU SDP4 benchmarks
        let tles = generate_tles(n_sats, OrbitRegime::GeoOnly);
        let satellites: Vec<Satellite> = tles.iter().map(|tle| Satellite::from(tle.clone())).collect();

        // Generate time arrays
        let base_epoch = tles[0].get_keplerian_state().epoch;
        let times: Vec<Epoch> = (0..n_times)
            .map(|i| base_epoch + TimeSpan::from_minutes(i as f64))
            .collect();
        let base_jd = base_epoch.days_since_1950 + JD_1950;
        let times_jd: Vec<f64> = (0..n_times).map(|i| base_jd + (i as f64) / 1440.0).collect();

        // CPU Sequential SDP4
        print!("CPU SDP4 (sequential):     ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let cpu_seq_time = benchmark_cpu_sequential(&satellites, &times);
        println!("{:>10.3} ms", cpu_seq_time.as_secs_f64() * 1000.0);

        // CPU Parallel SDP4
        print!("CPU SDP4 (parallel):       ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let cpu_par_time = benchmark_cpu_parallel(&satellites, &times);
        println!("{:>10.3} ms", cpu_par_time.as_secs_f64() * 1000.0);

        // GPU SDP4
        let gpu_sdp4_result = benchmark_gpu(&tles, &times_jd);
        match &gpu_sdp4_result {
            Ok(result) => {
                println!(
                    "GPU SDP4 (kernel):         {:>10.3} ms",
                    result.kernel_time.as_secs_f64() * 1000.0
                );
            }
            Err(e) => {
                println!("GPU SDP4: ERROR - {}", e);
            }
        }

        // GPU GEO Analytical
        let geo_result = benchmark_geo_analytical(n_sats, &times_jd);
        match &geo_result {
            Ok(result) => {
                println!(
                    "GPU GEO Analytical:        {:>10.3} ms",
                    result.kernel_time.as_secs_f64() * 1000.0
                );
            }
            Err(e) => {
                println!("GPU GEO Analytical: ERROR - {}", e);
            }
        }

        // Calculate and display speedups
        if let (Ok(sdp4), Ok(geo)) = (&gpu_sdp4_result, &geo_result) {
            println!("\n--- Speedups ---");

            let geo_vs_cpu_seq = cpu_seq_time.as_secs_f64() / geo.kernel_time.as_secs_f64();
            let geo_vs_cpu_par = cpu_par_time.as_secs_f64() / geo.kernel_time.as_secs_f64();
            let geo_vs_gpu_sdp4 = sdp4.kernel_time.as_secs_f64() / geo.kernel_time.as_secs_f64();

            println!("GEO Analytical vs CPU Sequential:  {:>6.1}x", geo_vs_cpu_seq);
            println!("GEO Analytical vs CPU Parallel:    {:>6.1}x", geo_vs_cpu_par);
            println!("GEO Analytical vs GPU SDP4:        {:>6.1}x", geo_vs_gpu_sdp4);

            // Throughput
            let total_props = (n_sats * n_times) as f64;
            let geo_throughput = total_props / geo.kernel_time.as_secs_f64();
            let sdp4_throughput = total_props / sdp4.kernel_time.as_secs_f64();

            println!("\n--- Throughput ---");
            println!("GPU GEO Analytical:  {:>12.0} propagations/sec", geo_throughput);
            println!("GPU SDP4:            {:>12.0} propagations/sec", sdp4_throughput);
        }
    }

    println!("\n");
    println!("{}", "=".repeat(70));
    println!("GEO Analytical Benchmark Complete!");
    println!("{}", "=".repeat(70));
}

/// Quick GPU SDP4 benchmark: CPU parallel vs GPU (precomputed resonance)
///
/// Baseline GPU SDP4 (without precomputed resonance) was ~1.6x vs CPU parallel.
/// This test measures the new precomputed resonance GPU SDP4 speedup.
#[test]
#[ignore]
fn test_benchmark_sdp4_precomputed() {
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping SDP4 precomputed benchmark");
        return;
    }

    println!("\n");
    println!("{}", "#".repeat(70));
    println!("# GPU SDP4 (Precomputed Resonance) vs CPU Parallel SDP4");
    println!("# Baseline GPU SDP4 was ~1.6x vs CPU parallel");
    println!("{}", "#".repeat(70));

    let satellite_counts = vec![100, 500, 1000];
    let n_times = 10_080; // 7 days at 1-minute intervals

    for &n_sats in &satellite_counts {
        println!("\n{}", "=".repeat(70));
        println!(
            "Satellites: {} | Timesteps: {} | Total: {} propagations",
            n_sats,
            n_times,
            n_sats * n_times
        );
        println!("{}", "=".repeat(70));

        let tles = generate_tles(n_sats, OrbitRegime::GeoOnly);
        let satellites: Vec<Satellite> = tles.iter().map(|tle| Satellite::from(tle.clone())).collect();

        let base_epoch = tles[0].get_keplerian_state().epoch;
        let times: Vec<Epoch> = (0..n_times)
            .map(|i| base_epoch + TimeSpan::from_minutes(i as f64))
            .collect();
        let base_jd = base_epoch.days_since_1950 + JD_1950;
        let times_jd: Vec<f64> = (0..n_times).map(|i| base_jd + (i as f64) / 1440.0).collect();

        // CPU Parallel SDP4
        print!("CPU SDP4 (parallel):       ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let cpu_par_time = benchmark_cpu_parallel(&satellites, &times);
        println!("{:>10.3} ms", cpu_par_time.as_secs_f64() * 1000.0);

        // GPU SDP4 (precomputed resonance)
        let gpu_result = benchmark_gpu(&tles, &times_jd);
        match &gpu_result {
            Ok(result) => {
                let kernel_ms = result.kernel_time.as_secs_f64() * 1000.0;
                let total_ms = result.total_time.as_secs_f64() * 1000.0;
                println!("GPU SDP4 (kernel+precomp):  {:>10.3} ms", kernel_ms);
                println!("GPU SDP4 (+ transfer):     {:>10.3} ms", total_ms);

                let speedup_kernel = cpu_par_time.as_secs_f64() / result.kernel_time.as_secs_f64();
                let speedup_total = cpu_par_time.as_secs_f64() / result.total_time.as_secs_f64();
                println!("\nSpeedup vs CPU Parallel:");
                println!("  GPU (kernel+precomp):  {:>6.1}x", speedup_kernel);
                println!("  GPU (+ transfer):      {:>6.1}x", speedup_total);
                println!("  (Baseline was ~1.6x without precomputed resonance)");
            }
            Err(e) => println!("GPU SDP4: ERROR - {}", e),
        }
    }
}
