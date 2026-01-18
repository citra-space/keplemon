//! Test 6: Component-Wise Error Analysis
//!
//! Identifies which orbital element causes the position error by analyzing:
//! - X, Y, Z errors in TEME frame
//! - Radial, In-track, Cross-track (RIC) errors
//! - Mapping dominant component to orbital element

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::gpu::{CudaSgp4Propagator, cuda_sgp4::TleDataGpu};
use keplemon::bodies::Satellite;
use keplemon::time::TimeSpan;

const JD_1950: f64 = 2433281.5;

#[test]
fn test_component_wise_error() {
    // Skip if CUDA not available
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping component-wise error test");
        return;
    }

    println!("\n=== Test 6: Component-Wise Error Analysis ===");
    println!("Purpose: Identify which orbital element causes the position error\n");

    // GPS BIIR-2 satellite
    let line1 = "1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993";
    let line2 = "2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789";

    let mut tle = TLE::from_lines(line1, line2, None)
        .expect("Failed to parse TLE");
    tle.name = Some("GPS BIIR-2 (PRN 13)".to_string());

    // Propagation time: epoch + 10 minutes
    let kep = tle.get_keplerian_state();
    let delta_min = 10.0;
    let jd_time = kep.epoch.days_since_1950 + JD_1950 + delta_min / 1440.0;
    let target_epoch = kep.epoch + TimeSpan::from_minutes(delta_min);

    println!("Satellite: GPS BIIR-2 (PRN 13)");
    println!("Propagation: epoch + 10 minutes\n");

    // Initialize GPU propagator
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

    // CPU propagator
    let satellite = Satellite::from(tle.clone());

    // GPU result
    let gpu_result = gpu_propagator.propagate(&[jd_time])
        .expect("GPU propagation failed");

    if gpu_result.is_empty() || gpu_result[0].error_code != 0 {
        panic!("GPU propagation error");
    }

    let gpu_state = &gpu_result[0];

    // CPU result
    let cpu_state = satellite.get_state_at_epoch(target_epoch)
        .expect("CPU propagation failed");

    // Calculate Cartesian errors (km)
    let dx_km = gpu_state.x - cpu_state.position[0];
    let dy_km = gpu_state.y - cpu_state.position[1];
    let dz_km = gpu_state.z - cpu_state.position[2];
    let pos_error_km = (dx_km * dx_km + dy_km * dy_km + dz_km * dz_km).sqrt();

    // Convert to meters
    let dx_m = dx_km * 1000.0;
    let dy_m = dy_km * 1000.0;
    let dz_m = dz_km * 1000.0;
    let pos_error_m = pos_error_km * 1000.0;

    println!("=== Cartesian Errors (TEME Frame) ===");
    println!("X error: {:.3} m ({:.1}% of total)", dx_m, (dx_m.abs() / pos_error_m) * 100.0);
    println!("Y error: {:.3} m ({:.1}% of total)", dy_m, (dy_m.abs() / pos_error_m) * 100.0);
    println!("Z error: {:.3} m ({:.1}% of total)", dz_m, (dz_m.abs() / pos_error_m) * 100.0);
    println!("Total position error: {:.3} m\n", pos_error_m);

    // Calculate RIC (Radial, In-track, Cross-track) frame errors
    // Position vector (use CPU as reference)
    let r = cpu_state.position;
    let v = cpu_state.velocity;

    let r_mag = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
    let v_mag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();

    // Radial unit vector (R)
    let r_hat = [r[0] / r_mag, r[1] / r_mag, r[2] / r_mag];

    // Cross-track unit vector (C) = r × v / |r × v|
    let h = [
        r[1] * v[2] - r[2] * v[1],
        r[2] * v[0] - r[0] * v[2],
        r[0] * v[1] - r[1] * v[0],
    ];
    let h_mag = (h[0] * h[0] + h[1] * h[1] + h[2] * h[2]).sqrt();
    let c_hat = [h[0] / h_mag, h[1] / h_mag, h[2] / h_mag];

    // In-track unit vector (I) = C × R
    let i_hat = [
        c_hat[1] * r_hat[2] - c_hat[2] * r_hat[1],
        c_hat[2] * r_hat[0] - c_hat[0] * r_hat[2],
        c_hat[0] * r_hat[1] - c_hat[1] * r_hat[0],
    ];

    // Project error vector onto RIC frame
    let radial_error = dx_km * r_hat[0] + dy_km * r_hat[1] + dz_km * r_hat[2];
    let in_track_error = dx_km * i_hat[0] + dy_km * i_hat[1] + dz_km * i_hat[2];
    let cross_track_error = dx_km * c_hat[0] + dy_km * c_hat[1] + dz_km * c_hat[2];

    let radial_error_m = radial_error * 1000.0;
    let in_track_error_m = in_track_error * 1000.0;
    let cross_track_error_m = cross_track_error * 1000.0;

    println!("=== RIC Frame Errors ===");
    println!("Radial (R) error:     {:.3} m ({:.1}% of total)", radial_error_m, (radial_error_m.abs() / pos_error_m) * 100.0);
    println!("In-track (I) error:   {:.3} m ({:.1}% of total)", in_track_error_m, (in_track_error_m.abs() / pos_error_m) * 100.0);
    println!("Cross-track (C) error: {:.3} m ({:.1}% of total)", cross_track_error_m, (cross_track_error_m.abs() / pos_error_m) * 100.0);
    println!("Total (sanity check): {:.3} m\n",
        (radial_error_m.powi(2) + in_track_error_m.powi(2) + cross_track_error_m.powi(2)).sqrt());

    // Determine dominant component
    let max_abs = radial_error_m.abs().max(in_track_error_m.abs()).max(cross_track_error_m.abs());

    println!("=== Component-to-Element Mapping ===");

    if cross_track_error_m.abs() == max_abs && (cross_track_error_m.abs() / pos_error_m) > 0.5 {
        println!("✅ Cross-track (C) error is DOMINANT ({:.1}%)", (cross_track_error_m.abs() / pos_error_m) * 100.0);
        println!("   → This indicates an INCLINATION (i) difference");
        println!("   → Likely cause:");
        println!("     - dpper periodic inclination correction difference");
        println!("     - Deep space secular rate (didt) difference");
        println!("     - Inclination-dependent term calculation");
    } else if radial_error_m.abs() == max_abs && (radial_error_m.abs() / pos_error_m) > 0.5 {
        println!("✅ Radial (R) error is DOMINANT ({:.1}%)", (radial_error_m.abs() / pos_error_m) * 100.0);
        println!("   → This indicates a semi-major axis (a) or eccentricity (e) difference");
        println!("   → Likely cause:");
        println!("     - Mean motion (n) calculation difference");
        println!("     - Eccentricity update difference");
        println!("     - Kepler equation solution difference");
    } else if in_track_error_m.abs() == max_abs && (in_track_error_m.abs() / pos_error_m) > 0.5 {
        println!("✅ In-track (I) error is DOMINANT ({:.1}%)", (in_track_error_m.abs() / pos_error_m) * 100.0);
        println!("   → This indicates RAAN (Ω) or argument of perigee (ω) difference");
        println!("   → Likely cause:");
        println!("     - RAAN precession difference");
        println!("     - Argument of perigee rotation difference");
        println!("     - Mean anomaly (M) difference");
    } else {
        println!("ℹ️  Error is MIXED across multiple components");
        println!("   → Multiple orbital elements contribute");
        println!("   → Further investigation needed (Test 4: Intermediate Value Trace)");
    }

    println!("\n=== Orbital Element Context ===");
    println!("Inclination: {:.4}° (GPS-like)", kep.elements.inclination.to_degrees());
    println!("Eccentricity: {:.6}", kep.elements.eccentricity);
    println!("RAAN: {:.4}°", kep.elements.raan.to_degrees());
    println!("Argument of perigee: {:.4}°", kep.elements.argument_of_perigee.to_degrees());
    println!("Mean anomaly: {:.4}°", kep.elements.mean_anomaly.to_degrees());
    println!("Mean motion: {:.8} rev/day", tle.get_mean_motion());
}
