//! Test that SoA (Struct of Arrays) kernel produces identical results to AoS kernel
//!
//! Run with: cargo test --features cuda --test test_soa_kernel

#![cfg(feature = "cuda")]

use keplemon::gpu::{CudaTlePropagator, TleDataGpu};

/// Create test TLE data for verification
fn get_test_tles() -> Vec<TleDataGpu> {
    // ISS TLE data (near-earth, well-tested)
    let iss = TleDataGpu {
        epoch_jd: 2460500.5,
        inclination: 51.6416,
        raan: 247.4627,
        eccentricity: 0.0006703,
        arg_perigee: 130.5360,
        mean_anomaly: 325.0288,
        mean_motion: 15.72125391,
        bstar: 0.00013882,
        ndot: 0.00000000,
        nddot: 0.0,
    };

    // STARLINK satellite (near-earth)
    let starlink = TleDataGpu {
        epoch_jd: 2460500.5,
        inclination: 53.0541,
        raan: 92.3421,
        eccentricity: 0.0001234,
        arg_perigee: 90.0,
        mean_anomaly: 270.0,
        mean_motion: 15.0,
        bstar: 0.00001,
        ndot: 0.0,
        nddot: 0.0,
    };

    // Create a batch of similar satellites
    (0..100)
        .map(|i| {
            let mut tle = if i % 2 == 0 { iss } else { starlink };
            // Vary the mean anomaly to get different positions
            tle.mean_anomaly = (tle.mean_anomaly + i as f64 * 3.6) % 360.0;
            tle.raan = (tle.raan + i as f64 * 1.0) % 360.0;
            tle
        })
        .collect()
}

#[test]
fn test_soa_matches_aos_kernel() {
    // Create propagator
    let mut propagator = CudaTlePropagator::new()
        .expect("Failed to create CUDA propagator");

    // Initialize with test TLEs
    let tles = get_test_tles();
    propagator.init_satellites(&tles)
        .expect("Failed to initialize satellites");

    // Generate test times
    let base_jd = 2460500.5;
    let jd_times: Vec<f64> = (0..50)
        .map(|i| base_jd + (i as f64) * 0.01) // ~15 min intervals
        .collect();

    // Propagate with AoS kernel (original)
    let aos_results = propagator.propagate(&jd_times)
        .expect("AoS propagation failed");

    // Propagate with SoA kernel (optimized)
    let soa_results = propagator.propagate_soa(&jd_times)
        .expect("SoA propagation failed");

    // Verify same number of results
    assert_eq!(aos_results.len(), soa_results.len(),
        "Result count mismatch: AoS={}, SoA={}", aos_results.len(), soa_results.len());

    // Compare each result
    let mut max_pos_diff = 0.0f64;
    let mut max_vel_diff = 0.0f64;
    let mut mismatch_count = 0;

    for (idx, (aos, soa)) in aos_results.iter().zip(soa_results.iter()).enumerate() {
        let pos_diff = ((aos.x - soa.x).powi(2) + 
                        (aos.y - soa.y).powi(2) + 
                        (aos.z - soa.z).powi(2)).sqrt();
        let vel_diff = ((aos.vx - soa.vx).powi(2) + 
                        (aos.vy - soa.vy).powi(2) + 
                        (aos.vz - soa.vz).powi(2)).sqrt();

        max_pos_diff = max_pos_diff.max(pos_diff);
        max_vel_diff = max_vel_diff.max(vel_diff);

        // Allow tiny floating-point differences (< 1 nanometer position, 1 nm/s velocity)
        if pos_diff > 1e-12 || vel_diff > 1e-12 {
            if mismatch_count < 5 {
                println!("Mismatch at index {}: pos_diff={:.2e} km, vel_diff={:.2e} km/s", 
                    idx, pos_diff, vel_diff);
                println!("  AoS: ({:.10}, {:.10}, {:.10}) km", aos.x, aos.y, aos.z);
                println!("  SoA: ({:.10}, {:.10}, {:.10}) km", soa.x, soa.y, soa.z);
            }
            mismatch_count += 1;
        }

        // Check error codes match
        assert_eq!(aos.error_code, soa.error_code,
            "Error code mismatch at index {}: AoS={}, SoA={}", 
            idx, aos.error_code, soa.error_code);
    }

    println!("\n=== AoS vs SoA Comparison ===");
    println!("Total comparisons: {}", aos_results.len());
    println!("Maximum position difference: {:.2e} km", max_pos_diff);
    println!("Maximum velocity difference: {:.2e} km/s", max_vel_diff);
    println!("Mismatches (>1e-12 km): {}", mismatch_count);

    // Should have zero mismatches - kernels should produce bit-identical results
    assert_eq!(mismatch_count, 0,
        "SoA kernel should produce identical results to AoS kernel");
}

#[test]
fn test_soa_arrays_format() {
    // Create propagator
    let mut propagator = CudaTlePropagator::new()
        .expect("Failed to create CUDA propagator");

    // Initialize with test TLEs
    let tles = get_test_tles();
    let n_sats = tles.len();
    propagator.init_satellites(&tles)
        .expect("Failed to initialize satellites");

    // Generate test times
    let base_jd = 2460500.5;
    let n_times = 10;
    let jd_times: Vec<f64> = (0..n_times)
        .map(|i| base_jd + (i as f64) * 0.01)
        .collect();

    // Get SoA arrays directly
    let soa_arrays = propagator.propagate_soa_arrays(&jd_times)
        .expect("SoA arrays propagation failed");

    // Verify dimensions
    assert_eq!(soa_arrays.n_sats, n_sats);
    assert_eq!(soa_arrays.n_times, n_times);
    assert_eq!(soa_arrays.x.len(), n_sats * n_times);
    assert_eq!(soa_arrays.y.len(), n_sats * n_times);
    assert_eq!(soa_arrays.z.len(), n_sats * n_times);
    assert_eq!(soa_arrays.vx.len(), n_sats * n_times);
    assert_eq!(soa_arrays.vy.len(), n_sats * n_times);
    assert_eq!(soa_arrays.vz.len(), n_sats * n_times);
    assert_eq!(soa_arrays.error_code.len(), n_sats * n_times);

    // Verify indexing: time-major order means buffer[time_idx * n_sats + sat_idx]
    // Compare with AoS results which are sat-major: results[sat_idx * n_times + time_idx]
    let aos_results = propagator.propagate(&jd_times)
        .expect("AoS propagation failed");

    for sat_idx in 0..n_sats {
        for time_idx in 0..n_times {
            let aos_idx = sat_idx * n_times + time_idx;
            let aos = &aos_results[aos_idx];

            // Use the get() helper to access SoA data
            let soa = soa_arrays.get(sat_idx, time_idx);

            assert!((aos.x - soa.x).abs() < 1e-12,
                "X mismatch at sat={}, time={}: AoS={}, SoA={}", 
                sat_idx, time_idx, aos.x, soa.x);
            assert!((aos.y - soa.y).abs() < 1e-12,
                "Y mismatch at sat={}, time={}", sat_idx, time_idx);
            assert!((aos.z - soa.z).abs() < 1e-12,
                "Z mismatch at sat={}, time={}", sat_idx, time_idx);
        }
    }

    println!("SoA arrays format test passed!");
    println!("  Satellites: {}", n_sats);
    println!("  Times: {}", n_times);
    println!("  Total elements per array: {}", n_sats * n_times);
}

#[test]
fn test_soa_into_preallocated() {
    // Create propagator
    let mut propagator = CudaTlePropagator::new()
        .expect("Failed to create CUDA propagator");

    // Initialize with test TLEs
    let tles = get_test_tles();
    let n_sats = tles.len();
    propagator.init_satellites(&tles)
        .expect("Failed to initialize satellites");

    // Generate test times
    let base_jd = 2460500.5;
    let n_times = 20;
    let jd_times: Vec<f64> = (0..n_times)
        .map(|i| base_jd + (i as f64) * 0.01)
        .collect();

    let n_results = n_sats * n_times;

    // Pre-allocate output arrays
    let mut x = vec![0.0f64; n_results];
    let mut y = vec![0.0f64; n_results];
    let mut z = vec![0.0f64; n_results];
    let mut vx = vec![0.0f64; n_results];
    let mut vy = vec![0.0f64; n_results];
    let mut vz = vec![0.0f64; n_results];
    let mut error = vec![0i32; n_results];

    // Propagate into pre-allocated arrays
    propagator.propagate_soa_into(
        &jd_times,
        &mut x, &mut y, &mut z,
        &mut vx, &mut vy, &mut vz,
        &mut error,
    ).expect("SoA into propagation failed");

    // Verify arrays were populated (not all zeros)
    assert!(x.iter().any(|&v| v != 0.0), "X array should have non-zero values");
    assert!(y.iter().any(|&v| v != 0.0), "Y array should have non-zero values");
    assert!(z.iter().any(|&v| v != 0.0), "Z array should have non-zero values");

    // Compare with regular SoA arrays method
    let soa_arrays = propagator.propagate_soa_arrays(&jd_times)
        .expect("SoA arrays propagation failed");

    for i in 0..n_results {
        assert!((x[i] - soa_arrays.x[i]).abs() < 1e-12,
            "X mismatch at {}: into={}, arrays={}", i, x[i], soa_arrays.x[i]);
        assert!((y[i] - soa_arrays.y[i]).abs() < 1e-12, "Y mismatch at {}", i);
        assert!((z[i] - soa_arrays.z[i]).abs() < 1e-12, "Z mismatch at {}", i);
    }

    println!("SoA into pre-allocated arrays test passed!");
}
