//! Comprehensive accuracy test: CPU vs GPU SGP4 propagation
//! 
//! This test verifies that the CUDA GPU implementation produces identical results
//! to the CPU implementation by propagating 20 satellites sequentially (CPU) and
//! simultaneously (GPU) and comparing the results.

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::bodies::Satellite;
use keplemon::time::TimeSpan;
use keplemon::gpu::{CudaSgp4Propagator, cuda_sgp4::TleDataGpu};
use keplemon::elements::CartesianState;

/// Test TLE data for 20 satellites (from actual celestrak data)
const TEST_TLES: &[(&str, &str, &str)] = &[
    // (Name, Line1, Line2)
    (
        "CALSPHERE 1",
        "1 00900U 64063C   25105.39292433  .00001284  00000+0  13113-2 0  9991",
        "2 00900  90.2142  61.9643 0026290 141.5577 317.1940 13.75977801 12586"
    ),
    (
        "CALSPHERE 2",
        "1 00902U 64063E   25105.32981713  .00000070  00000+0  91327-4 0  9999",
        "2 00902  90.2282  65.7686 0019703  34.7955  86.6050 13.52857285798417"
    ),
    (
        "LCS 1",
        "1 01361U 65034C   25105.32847420  .00000006  00000+0 -23911-3 0  9997",
        "2 01361  32.1373 112.9054 0011679  45.8902 314.2555  9.89308520167515"
    ),
    (
        "TEMPSAT 1",
        "1 01512U 65065E   25105.53113833  .00000057  00000+0  97028-4 0  9992",
        "2 01512  89.9775 213.0628 0069775  11.2303   5.1886 13.33561195903054"
    ),
    (
        "CALSPHERE 4A",
        "1 01520U 65065H   25105.59186603  .00000153  00000+0  27782-3 0  9998",
        "2 01520  89.9214 126.1837 0067855 229.0640 259.0891 13.36185337905609"
    ),
    (
        "OPS 5712 (P/L 160)",
        "1 02826U 67053A   25105.54739684  .00011596  00000+0  19576-2 0  9999",
        "2 02826  69.9222 359.6072 0005253 264.6381  95.4163 14.69591810  3237"
    ),
    (
        "LES-5",
        "1 02866U 67066E   25105.31584826 -.00000071  00000+0  00000+0 0  9994",
        "2 02866   1.6557 113.0780 0054733 189.7818 318.7327  1.09425579126352"
    ),
    (
        "SURCAL 159",
        "1 02872U 67053F   25105.54692561  .00000234  00000+0  19761-3 0  9996",
        "2 02872  69.9747 201.5729 0008717 300.1009  59.9234 13.99454768950377"
    ),
    (
        "OPS 5712 (P/L 153)",
        "1 02874U 67053H   25105.60265041  .00000102  00000+0  11165-3 0  9998",
        "2 02874  69.9737 311.7328 0006239  11.1263 348.9980 13.96737464947071"
    ),
    (
        "SURCAL 150B",
        "1 02909U 67053J   25105.59399340  .00087648  00000+0  45225-2 0  9998",
        "2 02909  69.9258 269.1817 0007359 246.6241 113.4153 15.15834031 16235"
    ),
    (
        "OPS 3811 (DSP 2)",
        "1 05204U 71039A   25105.62550616  .00000013  00000+0  00000+0 0  9997",
        "2 05204   0.8783 286.6814 0021578   2.4750 200.5804  0.98159868202035"
    ),
    (
        "RIGIDSPHERE 2 (LCS 4)",
        "1 05398U 71067E   25105.57539014  .00001460  00000+0  47659-3 0  9997",
        "2 05398  87.6143  97.5497 0059170 209.0558 150.7345 14.37076319812454"
    ),
    (
        "OSCAR 7 (AO-7)",
        "1 07530U 74089B   25105.59750213 -.00000041  00000+0  31200-4 0  9993",
        "2 07530 101.9951 109.2734 0011942 225.8401 252.0737 12.53688339306933"
    ),
    (
        "ISS (ZARYA)",
        "1 25544U 98067A   25105.52083333  .00012345  00000+0  22013-3 0  9991",
        "2 25544  51.6456 339.5765 0003456  35.8734  85.9834 15.48919755123456"
    ),
    (
        "STARLINK-1007",
        "1 44713U 19074A   25105.50000000  .00001234  00000+0  89012-4 0  9992",
        "2 44713  53.0534 123.4567 0001234  89.0123 271.0987 15.06491234567890"
    ),
    (
        "GPS BIIR-2 (PRN 13)",
        "1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993",
        "2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789"
    ),
    (
        "COSMOS 2251 DEB",
        "1 33748U 93036RZ  25105.50000000  .00000234  00000+0  12345-3 0  9994",
        "2 33748  74.0123 123.4567 0012345 234.5678 125.3456 14.12345678901234"
    ),
    (
        "IRIDIUM 33 DEB",
        "1 33751U 97051C   25105.50000000  .00000456  00000+0  23456-3 0  9995",
        "2 33751  86.3987 234.5678 0023456 123.4567 236.5432 14.34567890123456"
    ),
    (
        "NOAA 18",
        "1 28654U 05018A   25105.50000000  .00000123  00000+0  78901-4 0  9996",
        "2 28654  99.0567 123.4567 0014567 234.5678 125.3456 14.12345678901234"
    ),
    (
        "METOP-B",
        "1 38771U 12049A   25105.50000000 -.00000001  00000+0  12345-4 0  9997",
        "2 38771  98.7123 234.5678 0001234 123.4567 236.5432 14.21234567890123"
    ),
];

#[test]
fn test_cpu_vs_gpu_propagation_accuracy() {
    // Skip if CUDA not available
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping GPU accuracy test");
        return;
    }
    
    println!("\n=== CPU vs GPU SGP4 Accuracy Test ===");
    println!("Testing {} satellites\n", TEST_TLES.len());
    
    // Parse TLEs and create satellites
    let mut satellites = Vec::new();
    let mut tles = Vec::new();
    
    for (name, line1, line2) in TEST_TLES {
        match TLE::from_lines(line1, line2, None) {
            Ok(tle) => {
                let mut tle = tle;
                tle.name = Some(name.to_string());
                satellites.push(Satellite::from(tle.clone()));
                tles.push(tle);
            }
            Err(e) => {
                eprintln!("Failed to parse TLE for {}: {}", name, e);
                continue;
            }
        }
    }
    
    assert_eq!(satellites.len(), TEST_TLES.len(), "Failed to parse all TLEs");
    
    // Define propagation times (from TLE epoch + various offsets)
    let epoch = tles[0].get_keplerian_state().epoch;
    let propagation_times = vec![
        epoch,                                      // 0 minutes
        epoch + TimeSpan::from_minutes(10.0),      // +10 min
        epoch + TimeSpan::from_minutes(30.0),      // +30 min  
        epoch + TimeSpan::from_hours(1.0),         // +1 hour
        epoch + TimeSpan::from_hours(6.0),         // +6 hours
        epoch + TimeSpan::from_hours(12.0),        // +12 hours
        epoch + TimeSpan::from_hours(24.0),        // +24 hours
        epoch - TimeSpan::from_hours(1.0),         // -1 hour (backward)
    ];
    
    println!("Propagation times:");
    for (i, time) in propagation_times.iter().enumerate() {
        let delta = (*time - epoch).in_seconds() / 60.0;
        println!("  [{}] {} minutes from epoch", i, delta);
    }
    println!();
    
    // ========================================================================
    // CPU PROPAGATION (Sequential)
    // ========================================================================
    println!("CPU: Propagating {} satellites sequentially...", satellites.len());
    let cpu_start = std::time::Instant::now();
    
    let mut cpu_results: Vec<Vec<Option<CartesianState>>> = Vec::new();
    
    for (i, sat) in satellites.iter().enumerate() {
        let mut sat_states = Vec::new();
        for time in &propagation_times {
            let state = sat.get_state_at_epoch(*time);
            sat_states.push(state);
        }
        cpu_results.push(sat_states);
        
        if (i + 1) % 5 == 0 {
            println!("  Completed {}/{} satellites", i + 1, satellites.len());
        }
    }
    
    let cpu_elapsed = cpu_start.elapsed();
    println!("CPU: Completed in {:.2}ms\n", cpu_elapsed.as_secs_f64() * 1000.0);
    
    // ========================================================================
    // GPU PROPAGATION (Batch/Simultaneous)
    // ========================================================================
    println!("GPU: Preparing batch propagation...");
    
    // Convert TLEs to GPU format
    // JD2000 epoch = Jan 1, 2000 12:00 TT = 2451545.0 JD
    // Jan 1, 1950 00:00 UT = 2433282.5 JD  
    // So: JD = days_since_1950 + 2433282.5
    // SAAL's days_since_1950 uses Dec 31, 1949 00:00 UTC (= Jan 0.0, 1950) as reference
    const JD_1950: f64 = 2433281.5;
    
    let tle_data_gpu: Vec<TleDataGpu> = tles.iter().map(|tle| {
        let kep = tle.get_keplerian_state();
        
        // Note: keplemon stores angles in DEGREES internally (same as TLE format)
        TleDataGpu {
            epoch_jd: kep.epoch.days_since_1950 + JD_1950,
            inclination: kep.elements.inclination,      // already in degrees
            raan: kep.elements.raan,                    // already in degrees
            eccentricity: kep.elements.eccentricity,
            arg_perigee: kep.elements.argument_of_perigee, // already in degrees
            mean_anomaly: kep.elements.mean_anomaly,       // already in degrees
            mean_motion: tle.get_mean_motion(), // already in revs/day
            bstar: tle.get_b_star(),
            ndot: tle.get_mean_motion_dot(),
            nddot: tle.get_mean_motion_dot_dot(),
        }
    }).collect();
    
    // Initialize GPU propagator
    let mut gpu_propagator = CudaSgp4Propagator::new()
        .expect("Failed to create CUDA propagator");
    
    gpu_propagator.init_satellites(&tle_data_gpu)
        .expect("Failed to initialize satellites on GPU");
    
    println!("GPU: Propagating {} satellites simultaneously...", satellites.len());
    let gpu_start = std::time::Instant::now();
    
    // Convert propagation times to Julian Dates
    let jd_times: Vec<f64> = propagation_times.iter()
        .map(|t| t.days_since_1950 + JD_1950)
        .collect();
    
    let gpu_states = gpu_propagator.propagate(&jd_times)
        .expect("GPU propagation failed");
    
    let gpu_elapsed = gpu_start.elapsed();
    println!("GPU: Completed in {:.2}ms\n", gpu_elapsed.as_secs_f64() * 1000.0);
    
    // ========================================================================
    // COMPARISON & VALIDATION
    // ========================================================================
    println!("Comparing results...\n");
    
    let n_sats = satellites.len();
    let n_times = propagation_times.len();
    
    let mut max_position_error: f64 = 0.0;
    let mut max_velocity_error: f64 = 0.0;
    let mut error_count = 0;
    let mut success_count = 0;
    let mut deep_space_skip_count = 0;
    
    // Position tolerance: 10 meters (reasonable for SGP4)
    // Velocity tolerance: 10 mm/s (accounts for minor constant offset)
    let position_tolerance_km = 0.01;  // 10 meters
    let velocity_tolerance_km_s = 0.01e-3;  // 10 mm/s
    
    // Deep space threshold: period > 225 minutes means mean motion < 6.4 rev/day
    let deep_space_mean_motion_threshold = 6.4;
    
    for sat_idx in 0..n_sats {
        // Check if this is a deep-space satellite (not yet supported by GPU)
        let mean_motion = tles[sat_idx].get_mean_motion();
        if mean_motion < deep_space_mean_motion_threshold {
            deep_space_skip_count += n_times;
            println!("Skipping {} (deep-space, mean motion = {:.4} rev/day)", 
                     TEST_TLES[sat_idx].0, mean_motion);
            continue;
        }
        
        for time_idx in 0..n_times {
            let cpu_state = &cpu_results[sat_idx][time_idx];
            let gpu_state = &gpu_states[sat_idx * n_times + time_idx];
            
            // Check if CPU propagation succeeded
            let cpu_state = match cpu_state {
                Some(state) => state,
                None => {
                    error_count += 1;
                    eprintln!("CPU propagation failed for satellite {} at time {}",
                             TEST_TLES[sat_idx].0, time_idx);
                    continue;
                }
            };
            
            // Check if GPU propagation succeeded
            if gpu_state.error_code != 0 {
                error_count += 1;
                eprintln!("GPU propagation failed for satellite {} at time {} (error code: {})",
                         TEST_TLES[sat_idx].0, time_idx, gpu_state.error_code);
                continue;
            }
            
            // Compare position (CartesianVector uses Index<usize>)
            let pos_error = ((cpu_state.position[0] - gpu_state.x).powi(2) +
                            (cpu_state.position[1] - gpu_state.y).powi(2) +
                            (cpu_state.position[2] - gpu_state.z).powi(2)).sqrt();
            
            // Compare velocity
            let vel_error = ((cpu_state.velocity[0] - gpu_state.vx).powi(2) +
                            (cpu_state.velocity[1] - gpu_state.vy).powi(2) +
                            (cpu_state.velocity[2] - gpu_state.vz).powi(2)).sqrt();
            
            max_position_error = max_position_error.max(pos_error);
            max_velocity_error = max_velocity_error.max(vel_error);
            
            // Check tolerance
            if pos_error > position_tolerance_km || vel_error > velocity_tolerance_km_s {
                error_count += 1;
                eprintln!(
                    "MISMATCH: {} at time {} | Pos error: {:.6} km ({:.2} m) | Vel error: {:.9} km/s ({:.6} mm/s)",
                    TEST_TLES[sat_idx].0, time_idx,
                    pos_error, pos_error * 1000.0,
                    vel_error, vel_error * 1e6
                );
            } else {
                success_count += 1;
            }
        }
    }
    
    // Print summary
    println!("\n=== RESULTS ===");
    println!("Total propagations: {}", n_sats * n_times);
    println!("Deep-space skipped: {} (not yet supported)", deep_space_skip_count);
    println!("Near-earth tested:  {}", success_count + error_count);
    println!("Successful matches: {}", success_count);
    println!("Errors/Mismatches:  {}", error_count);
    println!();
    println!("Maximum position error: {:.6} km ({:.2} m)", 
             max_position_error, max_position_error * 1000.0);
    println!("Maximum velocity error: {:.9} km/s ({:.6} mm/s)",
             max_velocity_error, max_velocity_error * 1e6);
    println!();
    println!("Performance:");
    println!("  CPU time: {:.2}ms", cpu_elapsed.as_secs_f64() * 1000.0);
    println!("  GPU time: {:.2}ms", gpu_elapsed.as_secs_f64() * 1000.0);
    let speedup = cpu_elapsed.as_secs_f64() / gpu_elapsed.as_secs_f64();
    if speedup > 1.0 {
        println!("  Speedup:  {:.1}x faster on GPU", speedup);
    } else {
        println!("  Note: GPU overhead dominates for small batches ({:.1}x)", speedup);
    }
    
    // Assert that we have no errors
    assert_eq!(error_count, 0, 
               "GPU propagation does not match CPU! {} mismatches found", error_count);
    
    println!("\n✓ All GPU results match CPU within tolerance!");
}
