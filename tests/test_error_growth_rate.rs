//! Test 5: Error Growth Rate Analysis
//!
//! Determines if GPU vs CPU position error is:
//! - Constant over time (initialization-related)
//! - Linear growth (secular rate difference)
//! - Quadratic growth (integration/numerical drift)

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::gpu::{CudaSgp4Propagator, cuda_sgp4::TleDataGpu};
use keplemon::bodies::Satellite;
use keplemon::time::TimeSpan;

const JD_1950: f64 = 2433281.5;

#[test]
fn test_error_growth_rate() {
    // Skip if CUDA not available
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping error growth rate test");
        return;
    }

    println!("\n=== Test 5: Error Growth Rate Analysis ===");
    println!("Purpose: Distinguish initialization error from propagation drift\n");

    // GPS BIIR-2 satellite
    let line1 = "1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993";
    let line2 = "2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789";

    let mut tle = TLE::from_lines(line1, line2, None)
        .expect("Failed to parse TLE");
    tle.name = Some("GPS BIIR-2 (PRN 13)".to_string());

    // Propagation times: t=0, 10min, 1h, 6h, 12h, 24h, 48h, 168h (7 days)
    let time_deltas_minutes = vec![0.0, 10.0, 60.0, 360.0, 720.0, 1440.0, 2880.0, 10080.0];

    println!("Satellite: GPS BIIR-2 (PRN 13)");
    println!("Propagation times: t=0, 10min, 1h, 6h, 12h, 24h, 48h, 168h\n");

    // Initialize GPU propagator
    let kep = tle.get_keplerian_state();
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

    println!("Time (min) | Time (hrs) | Position Error (m) | X Error (m) | Y Error (m) | Z Error (m)");
    println!("-----------|------------|-------------------|-------------|-------------|-------------");

    let mut errors_vs_time = Vec::new();

    for &delta_min in &time_deltas_minutes {
        let jd_time = kep.epoch.days_since_1950 + JD_1950 + delta_min / 1440.0;
        let target_epoch = kep.epoch + TimeSpan::from_minutes(delta_min);

        // GPU result
        let gpu_result = gpu_propagator.propagate(&[jd_time])
            .expect("GPU propagation failed");

        if gpu_result.is_empty() || gpu_result[0].error_code != 0 {
            eprintln!("GPU propagation error at t={} min", delta_min);
            continue;
        }

        let gpu_state = &gpu_result[0];

        // CPU result
        let cpu_state = match satellite.get_state_at_epoch(target_epoch) {
            Some(state) => state,
            None => {
                eprintln!("CPU propagation error at t={} min", delta_min);
                continue;
            }
        };

        // Calculate position error
        let dx = (gpu_state.x - cpu_state.position[0]) * 1000.0; // Convert km to m
        let dy = (gpu_state.y - cpu_state.position[1]) * 1000.0;
        let dz = (gpu_state.z - cpu_state.position[2]) * 1000.0;
        let pos_error = (dx * dx + dy * dy + dz * dz).sqrt();

        errors_vs_time.push((delta_min, pos_error));

        let hours = delta_min / 60.0;
        println!("{:10.1} | {:10.2} | {:17.3} | {:11.3} | {:11.3} | {:11.3}",
            delta_min, hours, pos_error, dx, dy, dz);
    }

    println!("\n=== Error Growth Analysis ===");

    if errors_vs_time.len() < 3 {
        eprintln!("Not enough data points for analysis");
        return;
    }

    // Calculate statistics
    let mean_error: f64 = errors_vs_time.iter().map(|(_, e)| e).sum::<f64>() / errors_vs_time.len() as f64;
    let min_error = errors_vs_time.iter().map(|(_, e)| e).fold(f64::INFINITY, |a, &b| a.min(b));
    let max_error = errors_vs_time.iter().map(|(_, e)| e).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let error_range = max_error - min_error;
    let relative_variation = error_range / mean_error;

    // Calculate linear growth rate (simple slope)
    let first_error = errors_vs_time[0].1;
    let last_error = errors_vs_time[errors_vs_time.len() - 1].1;
    let total_time_hours = time_deltas_minutes[time_deltas_minutes.len() - 1] / 60.0;
    let growth_rate_m_per_hour = (last_error - first_error) / total_time_hours;

    println!("Mean error: {:.3} m", mean_error);
    println!("Min error: {:.3} m", min_error);
    println!("Max error: {:.3} m", max_error);
    println!("Error range: {:.3} m", error_range);
    println!("Relative variation: {:.2}%", relative_variation * 100.0);
    println!("Linear growth rate: {:.3} m/hour ({:.3} m/day)", growth_rate_m_per_hour, growth_rate_m_per_hour * 24.0);

    println!("\n=== Interpretation ===");

    if relative_variation < 0.15 {
        println!("✅ Error is CONSTANT over time (variation < 15%)");
        println!("   → This indicates an INITIALIZATION-related difference");
        println!("   → Not propagation drift or integration error");
        println!("   → Likely due to:");
        println!("     - Coordinate transformation difference");
        println!("     - Initial orbital element computation");
        println!("     - Floating-point precision in initialization");
    } else if growth_rate_m_per_hour.abs() > 1.0 {
        println!("⚠️  Error GROWS over time ({:.3} m/day)", growth_rate_m_per_hour * 24.0);
        println!("   → This indicates propagation-related difference");
        println!("   → Investigate:");
        println!("     - Integration step differences");
        println!("     - Secular rate calculations");
        println!("     - Periodic term accumulation");
    } else {
        println!("ℹ️  Error shows MINOR growth (< 1 m/hour)");
        println!("   → Primarily initialization-related with small drift");
        println!("   → Further analysis needed (Tests 4, 6)");
    }

    println!("\n=== Model Fit: error(t) = a + b*t ===");
    println!("Constant term (a): {:.3} m (initialization offset)", first_error);
    println!("Linear term (b): {:.3} m/hour (drift rate)", growth_rate_m_per_hour);
}
