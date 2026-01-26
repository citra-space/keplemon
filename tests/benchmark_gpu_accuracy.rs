//! Comprehensive GPU Propagator Accuracy Benchmark
//!
//! This benchmark compares all GPU propagators against the SAAL CPU reference
//! across different orbital regimes and time spans to measure real-world accuracy.

#[cfg(feature = "cuda")]
use keplemon::bodies::Satellite;
#[cfg(feature = "cuda")]
use keplemon::elements::TLE;
#[cfg(feature = "cuda")]
use keplemon::gpu::{CudaTlePropagator, CudaSdp4InterpolatedPropagator, CudaGeoNumericalPropagator, TleDataGpu, PropagatorOverride};
#[cfg(feature = "cuda")]
use keplemon::time::TimeSpan;

#[test]
#[cfg(feature = "cuda")]
fn benchmark_sgp4_leo_accuracy() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║            SGP4 LEO Accuracy Benchmark                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // LEO satellites with different characteristics
    let test_cases = vec![
        ("ISS (ZARYA)",
         "1 25544U 98067A   24001.50000000  .00016717  00000-0  10270-3 0  9005",
         "2 25544  51.6400 208.9163 0006317  69.9862 290.1961 15.54225995000006"),
        ("STARLINK-1007",
         "1 44713U 19074A   24001.50000000  .00001174  00000-0  97034-4 0  9996",
         "2 44713  53.0541 123.4562 0001293  84.7891 275.3262 15.06387291000015"),
        ("NOAA 18",
         "1 28654U 05018A   24001.50000000  .00000085  00000-0  72762-4 0  9999",
         "2 28654  99.0518 339.5750 0014388  26.9533 333.2555 14.12501716000007"),
    ];

    let mut max_pos_error: f64 = 0.0;
    let mut max_vel_error: f64 = 0.0;
    let mut errors = Vec::new();

    for (name, line1, line2) in test_cases {
        let tle = TLE::from_lines(line1, line2, None).unwrap();
        let sat = Satellite::from(tle.clone());
        let epoch = tle.get_keplerian_state().epoch;

        // Test at multiple time spans
        let time_spans = vec![
            TimeSpan::from_hours(0.0),
            TimeSpan::from_hours(24.0),
            TimeSpan::from_days(7.0),
            TimeSpan::from_days(30.0),
        ];

        for time_span in &time_spans {
            let test_time = epoch + *time_span;

            // CPU reference (SAAL)
            let cpu_state = sat.get_state_at_epoch(test_time).unwrap();

            // GPU (CudaTlePropagator - SGP4)
            let mut gpu_prop = CudaTlePropagator::new().unwrap();
            let tle_data = vec![TleDataGpu::from(&tle)];
            gpu_prop.init_satellites(&tle_data).unwrap();

            let jd_time = TleDataGpu::jd_from_ds50(test_time.days_since_1950);
            let gpu_states = gpu_prop.propagate(&vec![jd_time]).unwrap();
            let gpu_state = &gpu_states[0];

            // Calculate errors
            let dx = gpu_state.x - cpu_state.position[0];
            let dy = gpu_state.y - cpu_state.position[1];
            let dz = gpu_state.z - cpu_state.position[2];
            let pos_err = (dx * dx + dy * dy + dz * dz).sqrt();

            let dvx = gpu_state.vx - cpu_state.velocity[0];
            let dvy = gpu_state.vy - cpu_state.velocity[1];
            let dvz = gpu_state.vz - cpu_state.velocity[2];
            let vel_err = (dvx * dvx + dvy * dvy + dvz * dvz).sqrt();

            max_pos_error = max_pos_error.max(pos_err);
            max_vel_error = max_vel_error.max(vel_err);

            let days = time_span.in_seconds() / 86400.0;
            errors.push((name, days, pos_err, vel_err));

            if pos_err > 0.010 { // > 10 meters
                println!("  {} +{:.1}d: pos_err={:.6}km ({:.3}m) vel_err={:.9}km/s ({:.3}mm/s)",
                         name, days, pos_err, pos_err * 1000.0, vel_err, vel_err * 1e6);
            }
        }
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ SGP4 LEO Summary                                              ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Max position error: {:.6} km ({:.3} m)", max_pos_error, max_pos_error * 1000.0);
    println!("║  Max velocity error: {:.9} km/s ({:.3} mm/s)", max_vel_error, max_vel_error * 1e6);
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Show worst cases
    errors.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    println!("\nWorst 5 position errors:");
    for (name, days, pos_err, vel_err) in errors.iter().take(5) {
        println!("  {} +{:.1}d: {:.6}km ({:.3}m), vel={:.9}km/s ({:.3}mm/s)",
                 name, days, pos_err, pos_err * 1000.0, vel_err, vel_err * 1e6);
    }
}

#[test]
#[cfg(feature = "cuda")]
fn benchmark_sdp4_analytical_geo_accuracy() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║       SDP4-Analytical GEO/MEO Accuracy Benchmark              ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let test_cases = vec![
        ("INTELSAT 902 (GEO)",
         "1 26900U 01039A   24001.50000000 -.00000275  00000-0  00000+0 0  9992",
         "2 26900   0.0167  74.2938 0001928 140.1234 291.8230  1.00274002000017"),
        ("GPS BIIR-2 (PRN 13)",
         "1 24876U 97035A   24001.50000000 -.00000031  00000-0  00000+0 0  9999",
         "2 24876  55.4547  44.2474 0048409 301.8144  57.7806  2.00565440000011"),
        ("GLONASS-M 736",
         "1 32393U 07052A   24001.50000000  .00000012  00000-0  00000+0 0  9999",
         "2 32393  64.3421 111.5644 0002156 311.2345  48.7645  2.13102512000018"),
    ];

    let mut max_pos_error: f64 = 0.0;
    let mut max_vel_error: f64 = 0.0;
    let mut errors = Vec::new();

    for (name, line1, line2) in test_cases {
        let tle = TLE::from_lines(line1, line2, None).unwrap();
        let sat = Satellite::from(tle.clone());
        let epoch = tle.get_keplerian_state().epoch;

        // Test at multiple time spans
        let time_spans = vec![
            TimeSpan::from_hours(0.0),
            TimeSpan::from_hours(24.0),
            TimeSpan::from_days(7.0),
            TimeSpan::from_days(30.0),
        ];

        for time_span in &time_spans {
            let test_time = epoch + *time_span;

            // CPU reference (SAAL)
            let cpu_state = sat.get_state_at_epoch(test_time).unwrap();

            // GPU (CudaSdp4InterpolatedPropagator)
            let mut gpu_prop = CudaSdp4InterpolatedPropagator::new().unwrap();
            let tle_data = vec![TleDataGpu::from(&tle)];
            gpu_prop.init_satellites(&tle_data).unwrap();

            let jd_time = TleDataGpu::jd_from_ds50(test_time.days_since_1950);
            let gpu_states = gpu_prop.propagate(&vec![jd_time]).unwrap();
            let gpu_state = &gpu_states[0];

            // Calculate errors
            let dx = gpu_state.x - cpu_state.position[0];
            let dy = gpu_state.y - cpu_state.position[1];
            let dz = gpu_state.z - cpu_state.position[2];
            let pos_err = (dx * dx + dy * dy + dz * dz).sqrt();

            let dvx = gpu_state.vx - cpu_state.velocity[0];
            let dvy = gpu_state.vy - cpu_state.velocity[1];
            let dvz = gpu_state.vz - cpu_state.velocity[2];
            let vel_err = (dvx * dvx + dvy * dvy + dvz * dvz).sqrt();

            max_pos_error = max_pos_error.max(pos_err);
            max_vel_error = max_vel_error.max(vel_err);

            let days = time_span.in_seconds() / 86400.0;
            errors.push((name, days, pos_err, vel_err));

            if pos_err > 0.001 { // > 1 meter
                println!("  {} +{:.1}d: pos_err={:.6}km ({:.3}m) vel_err={:.9}km/s ({:.3}mm/s)",
                         name, days, pos_err, pos_err * 1000.0, vel_err, vel_err * 1e6);
            }
        }
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ SDP4-Analytical GEO/MEO Summary                               ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Max position error: {:.9} km ({:.6} m)", max_pos_error, max_pos_error * 1000.0);
    println!("║  Max velocity error: {:.12} km/s ({:.6} mm/s)", max_vel_error, max_vel_error * 1e6);
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Show worst cases
    errors.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    println!("\nWorst 5 position errors:");
    for (name, days, pos_err, vel_err) in errors.iter().take(5) {
        println!("  {} +{:.1}d: {:.9}km ({:.6}m), vel={:.9}km/s ({:.6}mm/s)",
                 name, days, pos_err, pos_err * 1000.0, vel_err, vel_err * 1e6);
    }
}

#[test]
#[cfg(feature = "cuda")]
fn benchmark_sdp4_standard_geo_accuracy() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║       SDP4-Standard GEO/MEO Accuracy Benchmark                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let test_cases = vec![
        ("INTELSAT 902 (GEO)",
         "1 26900U 01039A   24001.50000000 -.00000275  00000-0  00000+0 0  9992",
         "2 26900   0.0167  74.2938 0001928 140.1234 291.8230  1.00274002000017"),
        ("GPS BIIR-2 (PRN 13)",
         "1 24876U 97035A   24001.50000000 -.00000031  00000-0  00000+0 0  9999",
         "2 24876  55.4547  44.2474 0048409 301.8144  57.7806  2.00565440000011"),
        ("GLONASS-M 736",
         "1 32393U 07052A   24001.50000000  .00000012  00000-0  00000+0 0  9999",
         "2 32393  64.3421 111.5644 0002156 311.2345  48.7645  2.13102512000018"),
    ];

    let mut max_pos_error: f64 = 0.0;
    let mut max_vel_error: f64 = 0.0;
    let mut errors = Vec::new();

    for (name, line1, line2) in test_cases {
        let tle = TLE::from_lines(line1, line2, None).unwrap();
        let sat = Satellite::from(tle.clone());
        let epoch = tle.get_keplerian_state().epoch;

        // Test at multiple time spans
        let time_spans = vec![
            TimeSpan::from_hours(0.0),
            TimeSpan::from_hours(24.0),
            TimeSpan::from_days(7.0),
            TimeSpan::from_days(30.0),
        ];

        for time_span in &time_spans {
            let test_time = epoch + *time_span;

            // CPU reference (SAAL)
            let cpu_state = sat.get_state_at_epoch(test_time).unwrap();

            // GPU (CudaTlePropagator - Standard SDP4, not analytical)
            let mut gpu_prop = CudaTlePropagator::new().unwrap();
            let tle_data = vec![TleDataGpu::from(&tle)];

            // Force standard SDP4 (not analytical)
            gpu_prop.init_satellites_with_override(&tle_data, PropagatorOverride::ForceSdp4).unwrap();

            let jd_time = TleDataGpu::jd_from_ds50(test_time.days_since_1950);
            let gpu_states = gpu_prop.propagate(&vec![jd_time]).unwrap();
            let gpu_state = &gpu_states[0];

            // Calculate errors
            let dx = gpu_state.x - cpu_state.position[0];
            let dy = gpu_state.y - cpu_state.position[1];
            let dz = gpu_state.z - cpu_state.position[2];
            let pos_err = (dx * dx + dy * dy + dz * dz).sqrt();

            let dvx = gpu_state.vx - cpu_state.velocity[0];
            let dvy = gpu_state.vy - cpu_state.velocity[1];
            let dvz = gpu_state.vz - cpu_state.velocity[2];
            let vel_err = (dvx * dvx + dvy * dvy + dvz * dvz).sqrt();

            max_pos_error = max_pos_error.max(pos_err);
            max_vel_error = max_vel_error.max(vel_err);

            let days = time_span.in_seconds() / 86400.0;
            errors.push((name, days, pos_err, vel_err));

            if pos_err > 0.0001 { // > 0.1 meter
                println!("  {} +{:.1}d: pos_err={:.9}km ({:.6}m) vel_err={:.12}km/s ({:.6}mm/s)",
                         name, days, pos_err, pos_err * 1000.0, vel_err, vel_err * 1e6);
            }
        }
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ SDP4-Standard GEO/MEO Summary                                 ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Max position error: {:.9} km ({:.6} m)", max_pos_error, max_pos_error * 1000.0);
    println!("║  Max velocity error: {:.12} km/s ({:.6} mm/s)", max_vel_error, max_vel_error * 1e6);
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Show worst cases
    errors.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    println!("\nWorst 5 position errors:");
    for (name, days, pos_err, vel_err) in errors.iter().take(5) {
        println!("  {} +{:.1}d: {:.9}km ({:.6}m), vel={:.12}km/s ({:.6}mm/s)",
                 name, days, pos_err, pos_err * 1000.0, vel_err, vel_err * 1e6);
    }
}

#[test]
#[cfg(feature = "cuda")]
fn benchmark_geo_analytical_accuracy() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║       GEO-Analytical Accuracy Benchmark                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let test_cases = vec![
        ("INTELSAT 902 (GEO)",
         "1 26900U 01039A   24001.50000000 -.00000275  00000-0  00000+0 0  9992",
         "2 26900   0.0167  74.2938 0001928 140.1234 291.8230  1.00274002000017"),
        ("GPS BIIR-2 (PRN 13)",
         "1 24876U 97035A   24001.50000000 -.00000031  00000-0  00000+0 0  9999",
         "2 24876  55.4547  44.2474 0048409 301.8144  57.7806  2.00565440000011"),
    ];

    let mut max_pos_error: f64 = 0.0;
    let mut max_vel_error: f64 = 0.0;
    let mut errors = Vec::new();

    for (name, line1, line2) in test_cases {
        let tle = TLE::from_lines(line1, line2, None).unwrap();
        let sat = Satellite::from(tle.clone());
        let epoch = tle.get_keplerian_state().epoch;

        // Get ECI state at epoch
        let eci_state = sat.get_state_at_epoch(epoch).unwrap();

        // Convert to GeoStateGpu
        let epoch_jd = TleDataGpu::jd_from_ds50(epoch.days_since_1950);
        let geo_state = keplemon::gpu::GeoStateGpu::new(
            [eci_state.position[0], eci_state.position[1], eci_state.position[2]],
            [eci_state.velocity[0], eci_state.velocity[1], eci_state.velocity[2]],
            epoch_jd,
            None,
            None,
        );

        // Test at multiple time spans
        let time_spans = vec![
            TimeSpan::from_hours(0.0),
            TimeSpan::from_hours(24.0),
            TimeSpan::from_days(7.0),
            TimeSpan::from_days(30.0),
        ];

        for time_span in &time_spans {
            let test_time = epoch + *time_span;

            // CPU reference (SAAL)
            let cpu_state = sat.get_state_at_epoch(test_time).unwrap();

            // GPU (CudaGeoNumericalPropagator)
            let mut gpu_prop = CudaGeoNumericalPropagator::new().unwrap();

            // Initialize with GeoStateGpu
            gpu_prop.init_satellites(&vec![geo_state]).unwrap();

            // Propagate (jd_times, not elapsed seconds)
            let jd_time = TleDataGpu::jd_from_ds50(test_time.days_since_1950);
            let gpu_states = gpu_prop.propagate(&vec![jd_time]).unwrap();
            let gpu_state = &gpu_states[0];

            // Calculate errors
            let dx = gpu_state.x - cpu_state.position[0];
            let dy = gpu_state.y - cpu_state.position[1];
            let dz = gpu_state.z - cpu_state.position[2];
            let pos_err = (dx * dx + dy * dy + dz * dz).sqrt();

            let dvx = gpu_state.vx - cpu_state.velocity[0];
            let dvy = gpu_state.vy - cpu_state.velocity[1];
            let dvz = gpu_state.vz - cpu_state.velocity[2];
            let vel_err = (dvx * dvx + dvy * dvy + dvz * dvz).sqrt();

            max_pos_error = max_pos_error.max(pos_err);
            max_vel_error = max_vel_error.max(vel_err);

            let days = time_span.in_seconds() / 86400.0;
            errors.push((name, days, pos_err, vel_err));

            if pos_err > 0.001 { // > 1 meter
                println!("  {} +{:.1}d: pos_err={:.6}km ({:.3}m) vel_err={:.9}km/s ({:.3}mm/s)",
                         name, days, pos_err, pos_err * 1000.0, vel_err, vel_err * 1e6);
            }
        }
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ GEO-Analytical Summary                                        ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Max position error: {:.9} km ({:.6} m)", max_pos_error, max_pos_error * 1000.0);
    println!("║  Max velocity error: {:.12} km/s ({:.6} mm/s)", max_vel_error, max_vel_error * 1e6);
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Show worst cases
    errors.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    println!("\nWorst 5 position errors:");
    for (name, days, pos_err, vel_err) in errors.iter().take(5) {
        println!("  {} +{:.1}d: {:.9}km ({:.6}m), vel={:.9}km/s ({:.6}mm/s)",
                 name, days, pos_err, pos_err * 1000.0, vel_err, vel_err * 1e6);
    }
}

#[test]
#[cfg(feature = "cuda")]
fn compare_all_propagators_summary() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║          GPU PROPAGATOR ACCURACY BENCHMARK SUMMARY            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("Running comprehensive accuracy tests...\n");

    // Run all benchmarks
    benchmark_sgp4_leo_accuracy();
    println!("\n{}", "═".repeat(70));

    benchmark_sdp4_standard_geo_accuracy();
    println!("\n{}", "═".repeat(70));

    benchmark_sdp4_analytical_geo_accuracy();
    println!("\n{}", "═".repeat(70));

    benchmark_geo_analytical_accuracy();

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                     BENCHMARK COMPLETE                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}
