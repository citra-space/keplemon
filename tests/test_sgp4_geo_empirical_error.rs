//! Empirical measurement of SGP4 error at GEO altitudes
//!
//! This test demonstrates the accuracy penalty of using SGP4 (near-earth propagator)
//! for GEO satellites that should use SDP4 (deep-space propagator).
//!
//! The `force_near_earth` override flag allows forcing all satellites to use SGP4
//! propagation logic, even when they have orbital periods >= 225 minutes that would
//! normally trigger SDP4.
//!
//! Expected errors when forcing SGP4 for GEO:
//! - Missing lunar-solar perturbations (DPPER)
//! - Missing resonance terms (DSINIT)
//! - Missing deep-space secular rates (DSPACE)
//! - Typical errors: 5-10 km/day, 100-200 km at 15 days, 1000+ km at 1 year

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::gpu::{CudaSgp4Propagator, cuda_sgp4::TleDataGpu};
use keplemon::time::{Epoch, TimeSpan};

const JD_1950: f64 = 2433281.5;

/// GEO satellites for testing
const GEO_TLES: &[(&str, &str, &str)] = &[
    (
        "TDRS 3",
        "1 19548U 88091B   26008.31539492 -.00000299  00000+0  00000+0 0  9994",
        "2 19548  12.7229 342.0612 0044050 345.9566 204.1506  1.00262839123781",
    ),
    (
        "LES-5",
        "1 02866U 67066E   25105.31584826 -.00000071  00000+0  00000+0 0  9994",
        "2 02866   1.6557 113.0780 0054733 189.7818 318.7327  1.09425579126352",
    ),
    (
        "SKYNET 4C",
        "1 20776U 90079A   26008.60325894  .00000127  00000+0  00000+0 0  9998",
        "2 20776  13.3902 350.9041 0003547 300.8791  66.8764  1.00271932129296",
    ),
];

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

#[test]
fn test_sgp4_geo_empirical_error() {
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping SGP4 at GEO error test");
        return;
    }

    println!("\n{}", "=".repeat(80));
    println!("EMPIRICAL MEASUREMENT: SGP4 Error at GEO Altitudes");
    println!("{}", "=".repeat(80));

    // Parse GEO TLEs
    let tles: Vec<TLE> = GEO_TLES
        .iter()
        .filter_map(|(name, line1, line2)| TLE::from_three_lines(name, line1, line2).ok())
        .collect();

    let tle_data_gpu: Vec<TleDataGpu> = tles.iter().map(tle_to_gpu).collect();

    println!("\nTest satellites:");
    for (i, tle) in tles.iter().enumerate() {
        let period = 1440.0 / tle.get_mean_motion();
        println!(
            "  {} - {}: {:.2} rev/day ({:.0} min period)",
            i + 1,
            GEO_TLES[i].0,
            tle.get_mean_motion(),
            period
        );
    }

    // Define test times
    let base_epoch = tles[0].get_epoch();
    let test_intervals = vec![
        ("1 hour", TimeSpan::from_hours(1.0)),
        ("6 hours", TimeSpan::from_hours(6.0)),
        ("1 day", TimeSpan::from_days(1.0)),
        ("3 days", TimeSpan::from_days(3.0)),
        ("7 days", TimeSpan::from_days(7.0)),
        ("14 days", TimeSpan::from_days(14.0)),
        ("30 days", TimeSpan::from_days(30.0)),
    ];

    let times: Vec<f64> = std::iter::once(base_epoch)
        .chain(test_intervals.iter().map(|(_, dt)| base_epoch + *dt))
        .map(|e| e.days_since_1950 + JD_1950)
        .collect();

    // ========================================================================
    // CORRECT: SDP4 (deep-space propagator) - REFERENCE
    // ========================================================================
    println!("\nPropagating with SDP4 (correct for GEO)...");
    let mut propagator_sdp4 = CudaSgp4Propagator::new().expect("Failed to create GPU propagator");
    propagator_sdp4
        .init_satellites(&tle_data_gpu)
        .expect("Failed to initialize with SDP4");

    let sdp4_results = propagator_sdp4
        .propagate_soa_arrays(&times)
        .expect("SDP4 propagation failed");

    // ========================================================================
    // WRONG: SGP4 (near-earth propagator forced on GEO satellites)
    // ========================================================================
    println!("Propagating with forced SGP4 (incorrect for GEO)...");
    let mut propagator_sgp4 = CudaSgp4Propagator::new().expect("Failed to create GPU propagator");
    propagator_sgp4
        .init_satellites(&tle_data_gpu)
        .expect("Failed to initialize");

    // *** NEW: Use force_near_earth override to actually force SGP4 behavior ***
    propagator_sgp4
        .set_force_near_earth_override(true)
        .expect("Failed to set force_near_earth override");

    let sgp4_results = propagator_sgp4
        .propagate_soa_arrays(&times)
        .expect("Forced SGP4 propagation failed");

    // ========================================================================
    // COMPUTE ERRORS
    // ========================================================================
    println!("\n{}", "=".repeat(80));
    println!("Position Errors: SGP4 (wrong) vs SDP4 (correct) at GEO");
    println!("{}", "=".repeat(80));

    println!(
        "\n{:>12} | {:>20} | {:>20} | {:>20}",
        "Time", "TDRS 3 Error (km)", "LES-5 Error (km)", "SKYNET 4C Error (km)"
    );
    println!("{}", "-".repeat(80));

    let mut max_errors: Vec<f64> = vec![0.0; tles.len()];

    for (time_idx, (label, _)) in std::iter::once(("Epoch", TimeSpan::from_hours(0.0)))
        .chain(test_intervals.iter().copied())
        .enumerate()
    {
        print!("{:>12} |", label);

        for sat_idx in 0..tles.len() {
            let idx = time_idx * tles.len() + sat_idx;

            // SDP4 position (correct)
            let sdp4_x = sdp4_results.x[idx];
            let sdp4_y = sdp4_results.y[idx];
            let sdp4_z = sdp4_results.z[idx];

            // SGP4 position (wrong - forced)
            let sgp4_x = sgp4_results.x[idx];
            let sgp4_y = sgp4_results.y[idx];
            let sgp4_z = sgp4_results.z[idx];

            // Position error
            let dx = sgp4_x - sdp4_x;
            let dy = sgp4_y - sdp4_y;
            let dz = sgp4_z - sdp4_z;
            let error_km = (dx * dx + dy * dy + dz * dz).sqrt();

            max_errors[sat_idx] = max_errors[sat_idx].max(error_km);

            print!(" {:>19.3} |", error_km);
        }
        println!();
    }

    println!("{}", "=".repeat(80));
    println!("Maximum Errors Over 30-Day Period:");
    for (i, &max_err) in max_errors.iter().enumerate() {
        println!("  {}: {:.3} km", GEO_TLES[i].0, max_err);
    }

    println!("\n{}", "=".repeat(80));
    println!("Analysis:");
    println!("{}", "=".repeat(80));

    // Check if we're seeing real errors now
    let has_significant_errors = max_errors.iter().any(|&e| e > 1.0);

    if has_significant_errors {
        println!("\n✓ SUCCESS: force_near_earth override is working!");
        println!("  Errors show real difference between SGP4 and SDP4 at GEO.");
        println!("\nWhy SGP4 fails at GEO:");
        println!("  - Missing lunar-solar perturbations (DPPER)");
        println!("  - Missing resonance terms (12-hour, 24-hour)");
        println!("  - Missing deep-space secular rates (DSPACE)");
        println!("  - Near-earth atmospheric drag model inappropriate at GEO");

        // Verify errors are in expected range (5-10 km/day typical)
        let one_day_errors: Vec<f64> = (0..tles.len())
            .map(|sat_idx| {
                let idx = 3 * tles.len() + sat_idx; // 1-day is index 3
                let dx = sgp4_results.x[idx] - sdp4_results.x[idx];
                let dy = sgp4_results.y[idx] - sdp4_results.y[idx];
                let dz = sgp4_results.z[idx] - sdp4_results.z[idx];
                (dx * dx + dy * dy + dz * dz).sqrt()
            })
            .collect();

        println!("\nError rates:");
        for (i, &err) in one_day_errors.iter().enumerate() {
            println!("  {} at 1 day: {:.2} km ({:.2} km/day)", GEO_TLES[i].0, err, err);
        }

        let avg_one_day_error = one_day_errors.iter().sum::<f64>() / one_day_errors.len() as f64;
        println!("\nAverage error at 1 day: {:.2} km/day", avg_one_day_error);

        // Check for typical vs. resonance-affected errors
        let typical_errors: Vec<f64> = one_day_errors.iter().filter(|&&e| e < 100.0).copied().collect();
        let resonance_errors: Vec<f64> = one_day_errors.iter().filter(|&&e| e >= 100.0).copied().collect();

        if !typical_errors.is_empty() {
            let avg_typical = typical_errors.iter().sum::<f64>() / typical_errors.len() as f64;
            println!("✓ Typical errors (non-resonance): {:.2} km/day", avg_typical);
            if avg_typical > 2.0 && avg_typical < 20.0 {
                println!("  (matches expected range: 2-20 km/day)");
            }
        }

        if !resonance_errors.is_empty() {
            let avg_resonance = resonance_errors.iter().sum::<f64>() / resonance_errors.len() as f64;
            println!("⚠ Resonance-affected satellites: {:.0} km/day", avg_resonance);
            println!("  (LES-5 at 1316 min period is near 24-hour resonance)");
            println!("  (Missing resonance terms cause extremely large errors)");
        }
    } else {
        println!("\n⚠ WARNING: No significant errors detected!");
        println!("  force_near_earth override may not be working correctly.");
        println!("  Expected errors > 1 km at multi-day intervals.");
    }

    println!("\n{}", "=".repeat(80));
    println!("Conclusion:");
    println!("{}", "=".repeat(80));

    if has_significant_errors {
        println!("The force_near_earth override successfully demonstrates that SGP4");
        println!("is inappropriate for GEO satellites. Errors accumulate rapidly due to");
        println!("missing deep-space perturbations. Always use SDP4 for period >= 225 min.");
    } else {
        println!("The test did not show expected errors. This may indicate:");
        println!("  1. The override is not working correctly");
        println!("  2. The satellites are not being detected as deep-space");
        println!("  3. A compilation or caching issue occurred");
    }
    println!("{}", "=".repeat(80));

    // Assert that we see meaningful errors (override is working)
    assert!(
        has_significant_errors,
        "Expected significant errors (>1 km) when forcing SGP4 for GEO satellites"
    );

    // Assert errors are in reasonable range for typical GEO (non-resonance)
    // Resonance-affected satellites (like LES-5 near 24-hour) can have much larger errors
    let max_error = max_errors.iter().copied().fold(0.0, f64::max);
    assert!(
        max_error < 2000.0,
        "Errors should not exceed 2000 km for 30-day propagation (got max: {:.1} km)",
        max_error
    );

    println!("\n✓ Test passed: force_near_earth override is working correctly!");
    println!("  Demonstrated real SGP4 vs SDP4 errors at GEO altitudes.");
}
