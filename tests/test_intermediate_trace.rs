//! Test: Intermediate Value Trace
//!
//! Compares GPU and CPU intermediate values step-by-step
//! to identify the exact point of divergence.

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::gpu::{CudaSgp4Propagator, cuda_sgp4::TleDataGpu};

const JD_1950: f64 = 2433281.5;

#[test]
fn test_intermediate_trace() {
    // Skip if CUDA not available
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping intermediate trace test");
        return;
    }

    println!("\n=== Intermediate Value Trace: GPU vs CPU ===");
    println!("Purpose: Find exact point of divergence\n");

    // GPS BIIR-2 satellite - matching TLE used in CPU comparison
    let line1 = "1 24876U 97035A   06236.40952540 -.00000105  00000-0  10000-3 0  3985";
    let line2 = "2 24876  55.4467 245.5669 0123080 320.3768  38.6245  2.00569566 67521";

    let mut tle = TLE::from_lines(line1, line2, None)
        .expect("Failed to parse TLE");
    tle.name = Some("GPS BIIR-2 (PRN 13)".to_string());

    // Propagation time: epoch + 10 minutes
    let kep = tle.get_keplerian_state();
    let delta_min = 10.0;
    let jd_time = kep.epoch.days_since_1950 + JD_1950 + delta_min / 1440.0;

    println!("Satellite: GPS BIIR-2 (PRN 13)");
    println!("Propagation: epoch + 10 minutes");
    println!("TLE epoch JD: {:.10}", kep.epoch.days_since_1950 + JD_1950);
    println!("Target JD: {:.10}", jd_time);
    println!("tsince: {:.10} minutes\n", delta_min);

    // Initialize GPU
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

    let mut gpu_propagator = CudaSgp4Propagator::new()
        .expect("Failed to create GPU propagator");

    gpu_propagator.init_satellites(&[tle_data_gpu.clone()])
        .expect("Failed to initialize satellite on GPU");

    println!("==========================================");
    println!("GPU DEBUG OUTPUT:");
    println!("==========================================\n");

    // Propagate on GPU - this will print debug output
    let gpu_result = gpu_propagator.propagate(&[jd_time])
        .expect("GPU propagation failed");

    if gpu_result.is_empty() || gpu_result[0].error_code != 0 {
        panic!("GPU propagation error");
    }

    let gpu_state = &gpu_result[0];

    println!("\n==========================================");
    println!("GPU FINAL RESULT:");
    println!("==========================================");
    println!("Position (km): x={:.10}, y={:.10}, z={:.10}",
        gpu_state.x, gpu_state.y, gpu_state.z);
    println!("Velocity (km/s): vx={:.10}, vy={:.10}, vz={:.10}",
        gpu_state.vx, gpu_state.vy, gpu_state.vz);

    println!("\n==========================================");
    println!("CPU COMPARISON:");
    println!("==========================================");

    // Now get CPU result for comparison
    use keplemon::bodies::Satellite;
    use keplemon::time::TimeSpan;

    let satellite = Satellite::from(tle.clone());
    let target_epoch = kep.epoch + TimeSpan::from_minutes(delta_min);

    let cpu_state = satellite.get_state_at_epoch(target_epoch)
        .expect("CPU propagation failed");

    println!("Position (km): x={:.10}, y={:.10}, z={:.10}",
        cpu_state.position[0], cpu_state.position[1], cpu_state.position[2]);
    println!("Velocity (km/s): vx={:.10}, vy={:.10}, vz={:.10}",
        cpu_state.velocity[0], cpu_state.velocity[1], cpu_state.velocity[2]);

    println!("\n==========================================");
    println!("POSITION DIFFERENCE:");
    println!("==========================================");
    let dx = (gpu_state.x - cpu_state.position[0]) * 1000.0; // Convert to meters
    let dy = (gpu_state.y - cpu_state.position[1]) * 1000.0;
    let dz = (gpu_state.z - cpu_state.position[2]) * 1000.0;
    let pos_error = (dx * dx + dy * dy + dz * dz).sqrt();

    println!("dx: {:.3} m", dx);
    println!("dy: {:.3} m", dy);
    println!("dz: {:.3} m", dz);
    println!("Total: {:.3} m", pos_error);

    println!("\n==========================================");
    println!("INSTRUCTIONS:");
    println!("==========================================");
    println!("1. Compare GPU debug output values above");
    println!("2. Look for differences in:");
    println!("   - Deep Space Secular (nm, em, inclm, mm, argpm, nodem)");
    println!("   - Deep Space Periodics (after dpper)");
    println!("   - Long Period Periodics (axnl, aynl, xl)");
    println!("   - Argument of Latitude (sinu, cosu, su)");
    println!("3. First divergence point indicates root cause");
    println!("\nNOTE: GPU values are printed above. Now manually compute");
    println!("CPU intermediate values using python-sgp4 or SAAL for comparison.");
}
