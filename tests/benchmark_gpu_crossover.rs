//! GPU Crossover Point Analysis
//!
//! Systematically explores the parameter space to find where GPU becomes
//! beneficial compared to CPU for different propagation scenarios:
//! - Various time steps (dt): 1 min, 10 min, 1 hour
//! - Various orbital period counts: 1, 5, 10, 20 periods
//! - Different satellite counts: 10, 40, 100, 500
//!
//! This helps answer: "At what point does GPU acceleration become worthwhile?"

#![cfg(feature = "cuda")]

use keplemon::bodies::Satellite;
use keplemon::elements::TLE;
use keplemon::gpu::{CudaTlePropagator, cuda_tle::TleDataGpu};
use keplemon::time::{Epoch, TimeSpan};
use std::time::Instant;

const JD_1950: f64 = 2433281.5;

// Representative LEO satellite (ISS - ~90 minute period)
const LEO_TLE: (&str, &str, &str) = (
    "ISS (ZARYA)",
    "1 25544U 98067A   25105.52083333  .00012345  00000+0  22013-3 0  9991",
    "2 25544  51.6456 339.5765 0003456  35.8734  85.9834 15.48919755123456",
);

// Representative GEO satellite (TDRS - ~24 hour period)
const GEO_TLE: (&str, &str, &str) = (
    "TDRS 3",
    "1 19548U 88091B   26008.31539492 -.00000299  00000+0  00000+0 0  9994",
    "2 19548  12.7229 342.0612 0044050 345.9566 204.1506  1.00262839123781",
);

struct PropagationScenario {
    dt_minutes: f64,
    n_periods: usize,
    description: &'static str,
}

impl PropagationScenario {
    fn calculate_time_points(&self, orbital_period_minutes: f64) -> usize {
        let total_duration = orbital_period_minutes * self.n_periods as f64;
        (total_duration / self.dt_minutes).ceil() as usize
    }
}

const SCENARIOS: &[PropagationScenario] = &[
    PropagationScenario {
        dt_minutes: 1.0,
        n_periods: 1,
        description: "1-min dt, 1 period",
    },
    PropagationScenario {
        dt_minutes: 1.0,
        n_periods: 5,
        description: "1-min dt, 5 periods",
    },
    PropagationScenario {
        dt_minutes: 1.0,
        n_periods: 10,
        description: "1-min dt, 10 periods",
    },
    PropagationScenario {
        dt_minutes: 1.0,
        n_periods: 20,
        description: "1-min dt, 20 periods",
    },
    PropagationScenario {
        dt_minutes: 10.0,
        n_periods: 1,
        description: "10-min dt, 1 period",
    },
    PropagationScenario {
        dt_minutes: 10.0,
        n_periods: 5,
        description: "10-min dt, 5 periods",
    },
    PropagationScenario {
        dt_minutes: 10.0,
        n_periods: 10,
        description: "10-min dt, 10 periods",
    },
    PropagationScenario {
        dt_minutes: 10.0,
        n_periods: 20,
        description: "10-min dt, 20 periods",
    },
    PropagationScenario {
        dt_minutes: 60.0,
        n_periods: 1,
        description: "1-hour dt, 1 period",
    },
    PropagationScenario {
        dt_minutes: 60.0,
        n_periods: 5,
        description: "1-hour dt, 5 periods",
    },
    PropagationScenario {
        dt_minutes: 60.0,
        n_periods: 10,
        description: "1-hour dt, 10 periods",
    },
    PropagationScenario {
        dt_minutes: 60.0,
        n_periods: 20,
        description: "1-hour dt, 20 periods",
    },
];

fn generate_tles(count: usize, use_leo: bool) -> Vec<TLE> {
    let (name, line1, line2) = if use_leo { LEO_TLE } else { GEO_TLE };

    (0..count)
        .map(|_| TLE::from_three_lines(name, line1, line2).expect("Failed to parse TLE"))
        .collect()
}

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

fn benchmark_cpu(satellites: &[Satellite], times: &[Epoch]) -> std::time::Duration {
    let start = Instant::now();
    for sat in satellites.iter() {
        for &time in times.iter() {
            let _ = sat.get_state_at_epoch(time);
        }
    }
    start.elapsed()
}

fn benchmark_gpu(tles: &[TLE], times_jd: &[f64]) -> Result<std::time::Duration, String> {
    let tle_data_gpu: Vec<TleDataGpu> = tles.iter().map(tle_to_gpu).collect();

    let mut gpu_propagator = CudaTlePropagator::new()
        .map_err(|e| format!("Failed to create GPU propagator: {}", e))?;

    gpu_propagator.init_satellites(&tle_data_gpu)
        .map_err(|e| format!("Failed to initialize satellites on GPU: {}", e))?;

    let start = Instant::now();
    let _states = gpu_propagator
        .propagate(times_jd)
        .map_err(|e| format!("Failed to propagate on GPU: {}", e))?;

    Ok(start.elapsed())
}

fn run_crossover_analysis(n_satellites: usize, scenario: &PropagationScenario, use_leo: bool) {
    let orbit_type = if use_leo { "LEO" } else { "GEO" };
    let orbital_period_minutes = if use_leo { 90.0 } else { 1436.0 }; // ISS: ~90min, TDRS: ~24h

    let n_times = scenario.calculate_time_points(orbital_period_minutes);
    let _total_props = n_satellites * n_times;

    // Generate satellites and time points
    let tles = generate_tles(n_satellites, use_leo);
    let satellites: Vec<Satellite> = tles.iter().map(|tle| Satellite::from(tle.clone())).collect();

    let base_epoch = tles[0].get_keplerian_state().epoch;
    let times: Vec<Epoch> = (0..n_times)
        .map(|i| base_epoch + TimeSpan::from_minutes(i as f64 * scenario.dt_minutes))
        .collect();

    let base_jd = base_epoch.days_since_1950 + JD_1950;
    let times_jd: Vec<f64> = (0..n_times)
        .map(|i| base_jd + (i as f64 * scenario.dt_minutes) / 1440.0)
        .collect();

    // Benchmark CPU
    let cpu_time = benchmark_cpu(&satellites, &times);

    // Benchmark GPU
    let gpu_result = benchmark_gpu(&tles, &times_jd);

    match gpu_result {
        Ok(gpu_time) => {
            let speedup = cpu_time.as_secs_f64() / gpu_time.as_secs_f64();
            let cpu_ms = cpu_time.as_secs_f64() * 1000.0;
            let gpu_ms = gpu_time.as_secs_f64() * 1000.0;

            let winner = if speedup >= 1.0 { "GPU" } else { "CPU" };

            println!(
                "{:4} {:>3} sats | {:25} | {:7} pts | CPU: {:8.2} ms | GPU: {:8.2} ms | Speedup: {:5.2}x | {} WINS",
                orbit_type, n_satellites, scenario.description, n_times, cpu_ms, gpu_ms, speedup, winner
            );
        }
        Err(e) => {
            println!(
                "{:4} {:>3} sats | {:25} | ERROR: {}",
                orbit_type, n_satellites, scenario.description, e
            );
        }
    }
}

#[test]
#[ignore]
fn test_gpu_crossover_analysis() {
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping crossover analysis");
        return;
    }

    println!("\n{}", "=".repeat(130));
    println!("GPU CROSSOVER POINT ANALYSIS");
    println!("Finding where GPU becomes beneficial vs CPU for different propagation scenarios");
    println!("{}", "=".repeat(130));
    println!();

    let satellite_counts = vec![10, 40, 100, 500];

    // LEO Analysis
    println!("{}", "=".repeat(130));
    println!("LEO SATELLITES (ISS-like, ~90 minute period)");
    println!("{}", "=".repeat(130));

    for &n_sats in satellite_counts.iter() {
        for scenario in SCENARIOS {
            run_crossover_analysis(n_sats, scenario, true);
        }
        println!();
    }

    // GEO Analysis
    println!("{}", "=".repeat(130));
    println!("GEO SATELLITES (TDRS-like, ~24 hour period)");
    println!("{}", "=".repeat(130));

    for &n_sats in satellite_counts.iter() {
        for scenario in SCENARIOS {
            run_crossover_analysis(n_sats, scenario, false);
        }
        println!();
    }

    println!("{}", "=".repeat(130));
    println!("ANALYSIS COMPLETE");
    println!("{}", "=".repeat(130));
    println!("\nKey Findings:");
    println!("- Look for speedup >= 1.0 to identify GPU crossover points");
    println!("- GPU overhead is amortized better with more time points");
    println!("- Larger satellite counts favor GPU even with fewer time points");
}

/// Quick crossover test for CI
#[test]
#[ignore]
fn test_quick_crossover() {
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping quick crossover test");
        return;
    }

    println!("\n=== Quick GPU Crossover Test ===\n");

    let scenarios = &[
        PropagationScenario {
            dt_minutes: 10.0,
            n_periods: 5,
            description: "10-min dt, 5 periods",
        },
        PropagationScenario {
            dt_minutes: 10.0,
            n_periods: 10,
            description: "10-min dt, 10 periods",
        },
        PropagationScenario {
            dt_minutes: 10.0,
            n_periods: 20,
            description: "10-min dt, 20 periods",
        },
    ];

    for &n_sats in &[40, 100] {
        for scenario in scenarios {
            run_crossover_analysis(n_sats, scenario, true);
        }
        println!();
    }
}
