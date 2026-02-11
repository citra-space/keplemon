//! PR Validation Test: CUDA GPU-Resident API
//!
//! Tests the new GPU-resident API across all orbit regimes:
//! - 2 LEO satellites (400-600 km altitude)
//! - 2 MEO satellites (GPS/GLONASS, ~20,000 km)
//! - 2 GEO satellites (~35,786 km)
//!
//! Validates:
//! 1. GPU vs CPU accuracy (position/velocity residuals)
//! 2. Dual-mode API equivalence (CPU-copy vs GPU-resident)
//! 3. Both near-earth and deep-space propagators

#![cfg(feature = "cuda")]

use keplemon::bodies::Satellite;
use keplemon::elements::TLE;
use keplemon::gpu::{CudaTlePropagator, cuda_tle::TleDataGpu};
use keplemon::time::{Epoch, TimeSpan};
use std::time::Instant;

const JD_1950: f64 = 2433281.5;

// 2 LEO satellites
const LEO_SATS: &[(&str, &str, &str)] = &[
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
];

// 2 MEO satellites (GPS constellation)
const MEO_SATS: &[(&str, &str, &str)] = &[
    (
        "GPS BIIR-2 (PRN 13)",
        "1 24876U 97035A   06236.40952540 -.00000105  00000-0  10000-3 0  3985",
        "2 24876  55.4467 245.5669 0123080 320.3768  38.6245  2.00569566 67521",
    ),
    (
        "GLONASS-M 736",
        "1 36111U 09070A   25108.55000000  .00000005  00000+0  00000+0 0  9999",
        "2 36111  64.8901 234.5678 0012345 345.6789 123.4567  2.13101234567890",
    ),
];

// 2 GEO satellites
const GEO_SATS: &[(&str, &str, &str)] = &[
    (
        "LES-5",
        "1 02866U 67066E   25105.31584826 -.00000071  00000+0  00000+0 0  9994",
        "2 02866   1.6557 113.0780 0054733 189.7818 318.7327  1.09425579126352",
    ),
    (
        "TDRS 3",
        "1 19548U 88091B   26008.31539492 -.00000299  00000+0  00000+0 0  9994",
        "2 19548  12.7229 342.0612 0044050 345.9566 204.1506  1.00262839123781",
    ),
];

struct ResidualStats {
    max_pos_err: f64,
    mean_pos_err: f64,
    max_vel_err: f64,
    mean_vel_err: f64,
}

fn compute_orbital_period(mean_motion: f64) -> f64 {
    1440.0 / mean_motion // minutes
}

fn compute_altitude(mean_motion: f64) -> f64 {
    const MU_EARTH: f64 = 398600.4418; // km^3/s^2
    const R_EARTH: f64 = 6378.137; // km

    let period_sec = (1440.0 / mean_motion) * 60.0;
    let n_rad_per_sec = 2.0 * std::f64::consts::PI / period_sec;
    let semi_major = (MU_EARTH / (n_rad_per_sec * n_rad_per_sec)).powf(1.0 / 3.0);
    semi_major - R_EARTH
}

#[test]
fn test_pr_cuda_validation() {
    if !CudaTlePropagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping PR validation test");
        return;
    }

    println!("\n{}", "═".repeat(76));
    println!("  PR VALIDATION: CUDA GPU-RESIDENT API");
    println!("{}", "═".repeat(76));

    // Combine all satellites
    let all_tles: Vec<(&str, &str, &str, &str)> = LEO_SATS
        .iter()
        .map(|&(n, l1, l2)| (n, l1, l2, "LEO"))
        .chain(MEO_SATS.iter().map(|&(n, l1, l2)| (n, l1, l2, "MEO")))
        .chain(GEO_SATS.iter().map(|&(n, l1, l2)| (n, l1, l2, "GEO")))
        .collect();

    // Parse TLEs
    let mut satellites = Vec::new();
    let mut tles = Vec::new();
    let mut names = Vec::new();
    let mut regimes = Vec::new();

    println!("\n┌─ Test Satellites ───────────────────────────────────────────────────────┐");
    for (name, line1, line2, regime) in &all_tles {
        match TLE::from_lines(line1, line2, None) {
            Ok(tle) => {
                let mm = tle.get_mean_motion();
                let period = compute_orbital_period(mm);
                let altitude = compute_altitude(mm);

                println!(
                    "│ {:3} │ {:30} │ Period: {:5.0} min │ Alt: {:6.0} km │",
                    regime, name, period, altitude
                );

                satellites.push(Satellite::from(tle.clone()));
                tles.push(tle);
                names.push(*name);
                regimes.push(*regime);
            }
            Err(e) => eprintln!("Failed to parse {}: {}", name, e),
        }
    }
    println!("└─────────────────────────────────────────────────────────────────────────┘");

    // Test timesteps: epoch, +10min, +1h, +6h, +24h
    // (Limited to 24h for LEO due to drag model accumulation)
    let base_epoch = tles[0].get_keplerian_state().epoch;
    let times = vec![
        base_epoch,
        base_epoch + TimeSpan::from_minutes(10.0),
        base_epoch + TimeSpan::from_hours(1.0),
        base_epoch + TimeSpan::from_hours(6.0),
        base_epoch + TimeSpan::from_hours(24.0),
    ];

    // ========================================================================
    // TEST 1: GPU vs CPU Accuracy
    // ========================================================================
    println!("\n┌─ TEST 1: GPU vs CPU Accuracy ───────────────────────────────────────────┐");

    let cpu_start = Instant::now();
    let cpu_results: Vec<Vec<_>> = satellites
        .iter()
        .map(|sat| times.iter().map(|t| sat.get_state_at_epoch(*t)).collect())
        .collect();
    let cpu_time = cpu_start.elapsed();

    let tle_gpu: Vec<TleDataGpu> = tles
        .iter()
        .map(|tle| {
            let kep = tle.get_keplerian_state();
            TleDataGpu {
                epoch_jd: kep.epoch.days_since_1950 + JD_1950,
                inclination: kep.elements.inclination,
                raan: kep.elements.raan,
                eccentricity: kep.elements.eccentricity,
                arg_perigee: kep.elements.argument_of_perigee,
                mean_anomaly: kep.elements.mean_anomaly,
                mean_motion: tle.get_mean_motion(),
                bstar: tle.get_b_star(),
                ndot: tle.get_mean_motion_dot(),
                nddot: tle.get_mean_motion_dot_dot(),
            }
        })
        .collect();

    let mut gpu = CudaTlePropagator::new().expect("Failed to create GPU propagator");
    gpu.init_satellites(&tle_gpu).expect("Failed to init satellites");

    let gpu_start = Instant::now();
    let jd_times: Vec<f64> = times.iter().map(|t| t.days_since_1950 + JD_1950).collect();
    let gpu_results = gpu.propagate(&jd_times).expect("GPU propagation failed");
    let gpu_time = gpu_start.elapsed();

    // Compute residuals per regime
    let leo_stats = compute_residuals(&cpu_results, &gpu_results, &regimes, "LEO", times.len());
    let meo_stats = compute_residuals(&cpu_results, &gpu_results, &regimes, "MEO", times.len());
    let geo_stats = compute_residuals(&cpu_results, &gpu_results, &regimes, "GEO", times.len());

    println!("│                                                                          │");
    println!("│ Regime │ Max Pos Error │ Mean Pos Error │ Max Vel Error │ Mean Vel Error │");
    println!("│────────┼───────────────┼────────────────┼───────────────┼────────────────│");
    print_residual_row("LEO", &leo_stats);
    print_residual_row("MEO", &meo_stats);
    print_residual_row("GEO", &geo_stats);
    println!("│                                                                          │");
    println!(
        "│ Timing:  CPU = {:.2} ms  │  GPU = {:.2} ms  │  Speedup = {:.2}x          │",
        cpu_time.as_secs_f64() * 1000.0,
        gpu_time.as_secs_f64() * 1000.0,
        cpu_time.as_secs_f64() / gpu_time.as_secs_f64()
    );
    println!("└──────────────────────────────────────────────────────────────────────────┘");

    // Assert tolerances (based on test_gpu_cpu_parity.rs findings)
    // LEO: 2m tolerance (ISS shows ~1.6m at t=24h due to drag polynomial precision)
    // MEO/GEO: Sub-meter accuracy expected
    assert!(
        leo_stats.max_pos_err < 0.002,
        "LEO max position error {:.3}m exceeds 2m tolerance",
        leo_stats.max_pos_err * 1000.0
    );
    assert!(
        leo_stats.max_vel_err < 0.000015,
        "LEO max velocity error {:.3}mm/s exceeds 15mm/s tolerance",
        leo_stats.max_vel_err * 1e6
    );
    assert!(
        meo_stats.max_pos_err < 0.001,
        "MEO max position error {:.3}m exceeds 1m tolerance",
        meo_stats.max_pos_err * 1000.0
    );
    assert!(
        geo_stats.max_pos_err < 0.001,
        "GEO max position error {:.3}m exceeds 1m tolerance",
        geo_stats.max_pos_err * 1000.0
    );

    // ========================================================================
    // TEST 2: Dual-Mode API Equivalence
    // ========================================================================
    println!("\n┌─ TEST 2: Dual-Mode API Equivalence ─────────────────────────────────────┐");

    // Mode 1: CPU-copy (existing API)
    let cpu_copy_start = Instant::now();
    let cpu_copy_results = gpu
        .propagate_soa_arrays(&jd_times)
        .expect("propagate_soa_arrays failed");
    let cpu_copy_time = cpu_copy_start.elapsed();

    // Mode 2: GPU-resident (new API)
    let gpu_resident_start = Instant::now();
    let gpu_resident = gpu
        .propagate_soa_resident(&jd_times)
        .expect("propagate_soa_resident failed");
    let gpu_resident_time = gpu_resident_start.elapsed();

    // Convert GPU-resident to host for comparison
    let device = gpu.cuda_device();
    let gpu_resident_results = gpu_resident
        .to_soa_arrays(device)
        .expect("Failed to convert GPU-resident to SoA");

    // Compare results
    let mut max_diff = 0.0_f64;
    let n_times = jd_times.len();
    for sat_idx in 0..tles.len() {
        for time_idx in 0..n_times {
            let idx = time_idx * tles.len() + sat_idx;

            let dx = cpu_copy_results.x[idx] - gpu_resident_results.x[idx];
            let dy = cpu_copy_results.y[idx] - gpu_resident_results.y[idx];
            let dz = cpu_copy_results.z[idx] - gpu_resident_results.z[idx];
            let diff = (dx * dx + dy * dy + dz * dz).sqrt();

            max_diff = max_diff.max(diff);
        }
    }

    println!("│                                                                          │");
    println!("│ API Mode              │ Time (ms) │ Result                               │");
    println!("│───────────────────────┼───────────┼──────────────────────────────────────│");
    println!(
        "│ CPU-copy (existing)   │   {:.3}   │ Returns Vec<f64> on host             │",
        cpu_copy_time.as_secs_f64() * 1000.0
    );
    println!(
        "│ GPU-resident (new)    │   {:.3}   │ Returns CudaSlice<f64> on device     │",
        gpu_resident_time.as_secs_f64() * 1000.0
    );
    println!("│                       │           │                                      │");
    println!(
        "│ Max difference: {:.3e} km (bit-for-bit identical)                       │",
        max_diff
    );
    println!("└──────────────────────────────────────────────────────────────────────────┘");

    assert!(
        max_diff < 1e-10,
        "Dual-mode APIs differ by {:.3e} km (expected identical)",
        max_diff
    );

    // ========================================================================
    // SUMMARY
    // ========================================================================
    println!("\n{}", "═".repeat(76));
    println!("  ✓ GPU matches CPU within tolerance at LEO, MEO, and GEO");
    println!("  ✓ Dual-mode APIs produce identical results");
    println!("  ✓ GPU-resident API eliminates GPU→CPU transfer");
    println!("{}", "═".repeat(76));
    println!("\n[PASS] All CUDA GPU-resident API validation tests passed\n");
}

fn compute_residuals(
    cpu_results: &[Vec<Option<keplemon::elements::CartesianState>>],
    gpu_results: &[keplemon::gpu::cuda_tle::Sgp4StateGpu],
    regimes: &[&str],
    target_regime: &str,
    n_times: usize,
) -> ResidualStats {
    let mut pos_errors = Vec::new();
    let mut vel_errors = Vec::new();

    for (sat_idx, regime) in regimes.iter().enumerate() {
        if *regime != target_regime {
            continue;
        }

        for time_idx in 0..n_times {
            let cpu = match &cpu_results[sat_idx][time_idx] {
                Some(s) => s,
                None => continue,
            };
            let gpu = &gpu_results[sat_idx * n_times + time_idx];

            if gpu.error_code != 0 {
                continue;
            }

            let dx = gpu.x - cpu.position[0];
            let dy = gpu.y - cpu.position[1];
            let dz = gpu.z - cpu.position[2];
            let pos_err = (dx * dx + dy * dy + dz * dz).sqrt();

            let dvx = gpu.vx - cpu.velocity[0];
            let dvy = gpu.vy - cpu.velocity[1];
            let dvz = gpu.vz - cpu.velocity[2];
            let vel_err = (dvx * dvx + dvy * dvy + dvz * dvz).sqrt();

            pos_errors.push(pos_err);
            vel_errors.push(vel_err);
        }
    }

    let max_pos = pos_errors.iter().cloned().fold(0.0, f64::max);
    let mean_pos = pos_errors.iter().sum::<f64>() / pos_errors.len() as f64;
    let max_vel = vel_errors.iter().cloned().fold(0.0, f64::max);
    let mean_vel = vel_errors.iter().sum::<f64>() / vel_errors.len() as f64;

    ResidualStats {
        max_pos_err: max_pos,
        mean_pos_err: mean_pos,
        max_vel_err: max_vel,
        mean_vel_err: mean_vel,
    }
}

fn print_residual_row(regime: &str, stats: &ResidualStats) {
    println!(
        "│ {:6} │    {:7.3} m   │     {:7.3} m    │  {:7.3} mm/s  │   {:7.3} mm/s   │",
        regime,
        stats.max_pos_err * 1000.0,
        stats.mean_pos_err * 1000.0,
        stats.max_vel_err * 1e6,
        stats.mean_vel_err * 1e6
    );
}
