//! Test 1: GPU Determinism Verification
//!
//! Verifies that the CUDA GPU implementation produces bit-identical results
//! across multiple runs with the same inputs. This rules out:
//! - Race conditions
//! - Uninitialized memory
//! - Non-deterministic GPU operations

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::gpu::{CudaSgp4Propagator, cuda_sgp4::TleDataGpu};

// SAAL's days_since_1950 uses Dec 31, 1949 00:00 UTC (= Jan 0.0, 1950) as reference
const JD_1950: f64 = 2433281.5;

#[test]
fn test_gpu_determinism_100_runs() {
    // Skip if CUDA not available
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping GPU determinism test");
        return;
    }

    println!("\n=== Test 1: GPU Determinism Verification ===");
    println!("Purpose: Confirm GPU produces bit-identical results across 100 runs\n");

    // GPS BIIR-2 satellite (the one with highest error in original investigation)
    let line1 = "1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993";
    let line2 = "2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789";

    let mut tle = TLE::from_lines(line1, line2, None)
        .expect("Failed to parse TLE");
    tle.name = Some("GPS BIIR-2 (PRN 13)".to_string());

    // Propagation time: epoch + 10 minutes (where divergence was observed)
    let kep = tle.get_keplerian_state();
    let jd_time = kep.epoch.days_since_1950 + JD_1950 + 10.0 / 1440.0; // +10 minutes

    println!("Satellite: GPS BIIR-2 (PRN 13)");
    println!("Propagation: epoch + 10 minutes");
    println!("Runs: 100");
    println!();

    // Convert TLE to GPU format
    let tle_data_gpu = TleDataGpu {
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
    };

    // Run 100 times and collect results
    let mut all_results = Vec::new();

    for run in 0..100 {
        // Initialize fresh GPU propagator for each run
        let mut propagator = CudaSgp4Propagator::new()
            .expect("Failed to create GPU propagator");

        propagator.init_satellites(&[tle_data_gpu.clone()])
            .expect("Failed to initialize satellite on GPU");

        let result = propagator.propagate(&[jd_time])
            .expect("Propagation failed");

        if result.is_empty() {
            panic!("No results returned from propagation");
        }

        all_results.push(result[0].clone());

        if (run + 1) % 10 == 0 {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    }
    println!("\n");

    // Compare all results against the first one
    let reference = &all_results[0];

    let mut max_pos_diff: f64 = 0.0;
    let mut max_vel_diff: f64 = 0.0;
    let mut any_error_code_diff = false;

    for (run, result) in all_results.iter().enumerate().skip(1) {
        // Position difference
        let dx = result.x - reference.x;
        let dy = result.y - reference.y;
        let dz = result.z - reference.z;
        let pos_diff = (dx * dx + dy * dy + dz * dz).sqrt();

        // Velocity difference
        let dvx = result.vx - reference.vx;
        let dvy = result.vy - reference.vy;
        let dvz = result.vz - reference.vz;
        let vel_diff = (dvx * dvx + dvy * dvy + dvz * dvz).sqrt();

        max_pos_diff = max_pos_diff.max(pos_diff);
        max_vel_diff = max_vel_diff.max(vel_diff);

        // Check error codes
        if result.error_code != reference.error_code {
            any_error_code_diff = true;
            println!("Run {}: error_code differs! ref={}, got={}",
                run + 1, reference.error_code, result.error_code);
        }

        // Report any non-zero differences
        if pos_diff > 0.0 || vel_diff > 0.0 {
            println!("Run {}: pos_diff = {:.3e} km, vel_diff = {:.3e} km/s",
                run + 1, pos_diff, vel_diff);
            println!("  Reference: x={:.10}, y={:.10}, z={:.10}",
                reference.x, reference.y, reference.z);
            println!("  Run {}:     x={:.10}, y={:.10}, z={:.10}",
                run + 1, result.x, result.y, result.z);

            // Print hex representation for exact comparison
            println!("  Reference x bits: {:016x}", reference.x.to_bits());
            println!("  Run {} x bits:     {:016x}", run + 1, result.x.to_bits());
        }
    }

    // Print summary
    println!("\n=== Results ===");
    println!("Total runs: 100");
    println!("Max position difference: {:.3e} km ({:.3e} mm)", max_pos_diff, max_pos_diff * 1e6);
    println!("Max velocity difference: {:.3e} km/s ({:.3e} mm/s)", max_vel_diff, max_vel_diff * 1e6);

    if max_pos_diff == 0.0 && max_vel_diff == 0.0 {
        println!("\n✅ PASS: All 100 runs produced BIT-IDENTICAL results");
        println!("Conclusion: GPU implementation is deterministic");
        println!("            No race conditions or uninitialized memory");
    } else {
        println!("\n❌ FAIL: Results vary across runs");
        println!("This indicates:");
        println!("  - Race condition in CUDA kernel");
        println!("  - Uninitialized memory");
        println!("  - Non-deterministic GPU operation");
        panic!("GPU is not deterministic!");
    }

    // Assertion for test framework
    assert_eq!(max_pos_diff, 0.0, "GPU should produce identical position results");
    assert_eq!(max_vel_diff, 0.0, "GPU should produce identical velocity results");
    assert!(!any_error_code_diff, "GPU should produce identical error codes");
}
