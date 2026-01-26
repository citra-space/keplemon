//! GPU SGP4 Performance Benchmark
//!
//! Tests GPU propagation with realistic batch sizes to measure optimization impact.

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::gpu::{CudaTlePropagator, cuda_tle::TleDataGpu};

// Julian Date of 1950-01-01 00:00:00 UTC
const JD_1950: f64 = 2433281.5;

/// Generate synthetic TLE data for benchmarking
/// Creates variations of ISS-like orbits
fn generate_test_tles(count: usize) -> Vec<TleDataGpu> {
    // Base ISS TLE
    let base_tle = TLE::from_three_lines(
        "ISS (ZARYA)",
        "1 25544U 98067A   24001.50000000  .00016717  00000-0  10270-3 0  9025",
        "2 25544  51.6400 208.9163 0006703 300.0000  60.0000 15.49560000    15",
    )
    .expect("Failed to parse base TLE");

    let epoch = base_tle.get_epoch();

    (0..count)
        .map(|i| {
            // Vary orbital elements slightly for each satellite
            let incl_offset = (i as f64 * 0.1) % 10.0;
            let raan_offset = (i as f64 * 3.7) % 360.0;
            let ma_offset = (i as f64 * 17.3) % 360.0;

            TleDataGpu {
                epoch_jd: epoch.days_since_1950 + JD_1950,
                inclination: base_tle.get_inclination() + incl_offset,
                raan: (base_tle.get_raan() + raan_offset) % 360.0,
                eccentricity: base_tle.get_eccentricity(),
                arg_perigee: base_tle.get_argument_of_perigee(),
                mean_anomaly: (base_tle.get_mean_anomaly() + ma_offset) % 360.0,
                mean_motion: base_tle.get_mean_motion(),
                bstar: base_tle.get_b_star(),
                ndot: base_tle.get_mean_motion_dot(),
                nddot: base_tle.get_mean_motion_dot_dot(),
            }
        })
        .collect()
}

/// Generate time points for propagation
fn generate_time_points(base_jd: f64, count: usize, interval_minutes: f64) -> Vec<f64> {
    (0..count)
        .map(|i| {
            base_jd + (i as f64 * interval_minutes) / 1440.0 // Convert minutes to days
        })
        .collect()
}

#[test]
#[ignore]
fn benchmark_gpu_propagation() {
    println!("\n=== GPU SGP4 Performance Benchmark ===\n");

    // Test configurations: (num_satellites, num_times)
    let configs = [
        (100, 10),
        (100, 100),
        (1000, 10),
        (1000, 100),
        (5000, 10),
        (5000, 100),
        (10000, 10),
        (10000, 100),
    ];

    let mut gpu_propagator = CudaTlePropagator::new()
        .expect("Failed to create CUDA propagator");

    println!("{:>10} {:>10} {:>12} {:>15} {:>15}",
             "Sats", "Times", "Total Props", "Time (ms)", "Props/sec");
    println!("{}", "-".repeat(65));

    for (n_sats, n_times) in configs {
        // Generate test data
        let tle_data = generate_test_tles(n_sats);
        let base_jd = tle_data[0].epoch_jd;
        let jd_times = generate_time_points(base_jd, n_times, 10.0);

        // Initialize satellites
        gpu_propagator
            .init_satellites(&tle_data)
            .expect("Failed to initialize satellites");

        // Warm-up run
        let _ = gpu_propagator.propagate(&jd_times);

        // Timed runs (average of 5)
        let n_runs = 5;
        let mut total_time = std::time::Duration::ZERO;

        for _ in 0..n_runs {
            let start = std::time::Instant::now();
            let _results = gpu_propagator.propagate(&jd_times).expect("GPU propagation failed");
            total_time += start.elapsed();
        }

        let avg_time_ms = total_time.as_secs_f64() * 1000.0 / n_runs as f64;
        let total_props = n_sats * n_times;
        let props_per_sec = total_props as f64 / (avg_time_ms / 1000.0);

        println!(
            "{:>10} {:>10} {:>12} {:>15.3} {:>15.0}",
            n_sats, n_times, total_props, avg_time_ms, props_per_sec
        );
    }

    println!("\nBenchmark complete.");
}

#[test]
#[ignore]
fn benchmark_init_vs_propagate() {
    println!("\n=== Init vs Propagate Timing ===\n");

    let n_sats = 5000;
    let n_times = 100;

    let tle_data = generate_test_tles(n_sats);
    let base_jd = tle_data[0].epoch_jd;
    let jd_times = generate_time_points(base_jd, n_times, 10.0);

    let mut gpu_propagator = CudaTlePropagator::new()
        .expect("Failed to create CUDA propagator");

    // Time init
    let init_start = std::time::Instant::now();
    gpu_propagator
        .init_satellites(&tle_data)
        .expect("Failed to initialize satellites");
    let init_time = init_start.elapsed();

    // Time first propagate (includes JIT, cache miss)
    let first_prop_start = std::time::Instant::now();
    let _ = gpu_propagator.propagate(&jd_times);
    let first_prop_time = first_prop_start.elapsed();

    // Time subsequent propagates (warm cache)
    let n_runs = 10;
    let mut total_prop_time = std::time::Duration::ZERO;
    for _ in 0..n_runs {
        let start = std::time::Instant::now();
        let _ = gpu_propagator.propagate(&jd_times);
        total_prop_time += start.elapsed();
    }
    let avg_prop_time = total_prop_time / n_runs;

    println!(
        "Configuration: {} satellites × {} times = {} propagations\n",
        n_sats,
        n_times,
        n_sats * n_times
    );
    println!("Init time:              {:>10.3} ms", init_time.as_secs_f64() * 1000.0);
    println!(
        "First propagate:        {:>10.3} ms (cold)",
        first_prop_time.as_secs_f64() * 1000.0
    );
    println!(
        "Subsequent propagate:   {:>10.3} ms (warm, avg of {})",
        avg_prop_time.as_secs_f64() * 1000.0,
        n_runs
    );

    let total_props = n_sats * n_times;
    let props_per_sec = total_props as f64 / avg_prop_time.as_secs_f64();
    println!("\nThroughput: {:.0} propagations/second", props_per_sec);
}
