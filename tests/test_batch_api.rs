//! Integration tests for high-level batch propagation API
//!
//! Tests the BatchPropagator, TLE batch methods, and Constellation GPU integration

#[cfg(feature = "cuda")]
use keplemon::bodies::Constellation;
#[cfg(feature = "cuda")]
use keplemon::elements::TLE;
#[cfg(feature = "cuda")]
use keplemon::enums::TimeSystem;
#[cfg(feature = "cuda")]
use keplemon::propagation::{BatchPropagator, PropagationBackend};
#[cfg(feature = "cuda")]
use keplemon::time::{Epoch, TimeSpan};

#[cfg(feature = "cuda")]
const ISS_LINE1: &str = "1 25544U 98067A   24018.42784701  .00012593  00000+0  22161-3 0  9999";
#[cfg(feature = "cuda")]
const ISS_LINE2: &str = "2 25544  51.6415 290.2344 0005730  47.3470  92.9368 15.50238117436068";

#[cfg(feature = "cuda")]
const STARLINK_LINE1: &str = "1 44713U 19074A   24018.50000000  .00001749  00000+0  13559-3 0  9998";
#[cfg(feature = "cuda")]
const STARLINK_LINE2: &str = "2 44713  53.0544 123.4567 0001234  45.6789  12.3456 15.06400000262500";

#[test]
#[cfg(feature = "cuda")]
fn test_batch_propagator_basic() {
    // Create a few test TLEs
    let tle1 = TLE::from_lines(ISS_LINE1, ISS_LINE2, None).unwrap();
    let tle2 = TLE::from_lines(STARLINK_LINE1, STARLINK_LINE2, None).unwrap();
    let tles = vec![tle1, tle2];

    // Create test epochs
    let start = Epoch::from_iso("2024-01-18T12:00:00.000000Z", TimeSystem::UTC);
    let epochs: Vec<Epoch> = (0..10)
        .map(|i| start + TimeSpan::from_minutes(i as f64 * 6.0))
        .collect();

    // Test CPU backend
    let propagator = BatchPropagator::new().set_backend(PropagationBackend::Cpu);

    let cpu_results = propagator
        .propagate_batch(&tles, &epochs)
        .expect("CPU propagation failed");

    assert_eq!(cpu_results.len(), 2, "Should have 2 satellites");
    assert_eq!(cpu_results[0].len(), 10, "Each satellite should have 10 states");

    // Test GPU backend (if available)
    if BatchPropagator::new().is_gpu_available() {
        let propagator = BatchPropagator::new().set_backend(PropagationBackend::Gpu);

        let gpu_results = propagator
            .propagate_batch(&tles, &epochs)
            .expect("GPU propagation failed");

        assert_eq!(gpu_results.len(), 2, "Should have 2 satellites");
        assert_eq!(gpu_results[0].len(), 10, "Each satellite should have 10 states");

        // Compare CPU vs GPU results (should be very close)
        for (sat_idx, (cpu_sat, gpu_sat)) in cpu_results.iter().zip(&gpu_results).enumerate() {
            for (time_idx, (cpu_state, gpu_state)) in cpu_sat.iter().zip(gpu_sat).enumerate() {
                let pos_diff = (cpu_state.position - gpu_state.position).get_magnitude();
                assert!(
                    pos_diff < 0.1, // Within 100m
                    "Satellite {} at time {}: position diff = {:.6} km",
                    sat_idx,
                    time_idx,
                    pos_diff
                );
            }
        }
    }
}

#[test]
#[cfg(feature = "cuda")]
fn test_tle_propagate_batch() {
    let tle1 = TLE::from_lines(ISS_LINE1, ISS_LINE2, None).unwrap();
    let tle2 = TLE::from_lines(STARLINK_LINE1, STARLINK_LINE2, None).unwrap();
    let tles = vec![tle1, tle2];

    let start = Epoch::from_iso("2024-01-18T12:00:00.000000Z", TimeSystem::UTC);
    let epochs: Vec<Epoch> = (0..5).map(|i| start + TimeSpan::from_hours(i as f64)).collect();

    // Test the static batch method
    let results = TLE::propagate_batch(&tles, &epochs).expect("Batch propagation failed");

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].len(), 5);
    assert_eq!(results[1].len(), 5);

    // Verify states are reasonable
    for sat_states in &results {
        for state in sat_states {
            let pos_mag = state.position.get_magnitude();
            // LEO satellite should be between 6500-8000 km from Earth center
            assert!(
                pos_mag > 6500.0 && pos_mag < 8000.0,
                "Position magnitude {} km is outside expected range",
                pos_mag
            );
        }
    }
}

#[test]
#[cfg(feature = "cuda")]
fn test_tle_propagate_to_epochs() {
    let tle = TLE::from_lines(ISS_LINE1, ISS_LINE2, None).unwrap();

    let start = Epoch::from_iso("2024-01-18T12:00:00.000000Z", TimeSystem::UTC);

    // Test with few epochs (should use CPU)
    let few_epochs: Vec<Epoch> = (0..10).map(|i| start + TimeSpan::from_minutes(i as f64)).collect();

    let few_results = tle
        .propagate_to_epochs(&few_epochs)
        .expect("Few epochs propagation failed");

    assert_eq!(few_results.len(), 10);

    // Test with many epochs (should use GPU if available)
    let many_epochs: Vec<Epoch> = (0..150).map(|i| start + TimeSpan::from_minutes(i as f64)).collect();

    let many_results = tle
        .propagate_to_epochs(&many_epochs)
        .expect("Many epochs propagation failed");

    assert_eq!(many_results.len(), 150);

    // Verify continuity (adjacent states should be close)
    for i in 0..many_results.len() - 1 {
        let pos_diff = (many_results[i].position - many_results[i + 1].position).get_magnitude();
        // With 1-minute spacing, ISS moves ~450 km
        assert!(pos_diff < 500.0, "Position jump too large: {} km", pos_diff);
    }
}

#[test]
#[cfg(feature = "cuda")]
fn test_auto_backend_selection() {
    let tle = TLE::from_lines(ISS_LINE1, ISS_LINE2, None).unwrap();
    let tles = vec![tle; 10]; // 10 identical satellites

    let start = Epoch::from_iso("2024-01-18T12:00:00.000000Z", TimeSystem::UTC);

    let propagator = BatchPropagator::new(); // Auto backend

    // Small problem: 10 sats × 5 times = 50 operations < 1000 threshold
    let _small_epochs: Vec<Epoch> = (0..5).map(|i| start + TimeSpan::from_hours(i as f64)).collect();

    let backend = propagator.select_backend(10, 5);
    assert_eq!(backend, keplemon::propagation::SelectedBackend::Cpu);

    // Large problem: 10 sats × 150 times = 1500 operations > 1000 threshold
    let large_epochs: Vec<Epoch> = (0..150).map(|i| start + TimeSpan::from_minutes(i as f64)).collect();

    let backend = propagator.select_backend(10, 150);
    if propagator.is_gpu_available() {
        assert_eq!(backend, keplemon::propagation::SelectedBackend::Gpu);
    } else {
        assert_eq!(backend, keplemon::propagation::SelectedBackend::Cpu);
    }

    // Actually run the large problem
    let results = propagator
        .propagate_batch(&tles, &large_epochs)
        .expect("Large batch propagation failed");

    assert_eq!(results.len(), 10);
    assert_eq!(results[0].len(), 150);
}

#[test]
#[cfg(feature = "cuda")]
fn test_constellation_batch_methods() {
    let mut constellation = Constellation::new();

    // Add a few satellites
    let tle1 = TLE::from_lines(ISS_LINE1, ISS_LINE2, None).unwrap();
    let tle2 = TLE::from_lines(STARLINK_LINE1, STARLINK_LINE2, None).unwrap();

    use keplemon::bodies::Satellite;
    let sat1 = Satellite::from(tle1);
    let sat2 = Satellite::from(tle2);

    constellation.add("ISS".to_string(), sat1);
    constellation.add("Starlink".to_string(), sat2);

    let start = Epoch::from_iso("2024-01-18T12:00:00.000000Z", TimeSystem::UTC);
    let end = start + TimeSpan::from_hours(2.0);
    let step = TimeSpan::from_minutes(10.0);

    // Test batch ephemeris with CPU backend
    let results = constellation.get_batch_ephemeris(start, end, step, Some(PropagationBackend::Cpu));

    assert_eq!(results.len(), 2, "Should have 2 satellites");

    for (sat_id, states) in &results {
        assert!(states.len() > 0, "Satellite {} should have states", sat_id);

        // Verify all states are Some (successful propagation)
        for (i, state_opt) in states.iter().enumerate() {
            assert!(state_opt.is_some(), "Satellite {} state {} should be Some", sat_id, i);
        }
    }

    // Test GPU backend if available
    if Constellation::is_gpu_available() {
        let gpu_results = constellation.get_batch_ephemeris(start, end, step, Some(PropagationBackend::Gpu));

        assert_eq!(gpu_results.len(), 2, "GPU should also have 2 satellites");
    }
}

#[test]
#[cfg(feature = "cuda")]
fn test_empty_inputs() {
    // Test empty TLE list
    let empty_tles: Vec<TLE> = vec![];
    let start = Epoch::from_iso("2024-01-18T12:00:00.000000Z", TimeSystem::UTC);
    let epochs = vec![start];

    let propagator = BatchPropagator::new();
    let results = propagator.propagate_batch(&empty_tles, &epochs).unwrap();
    assert_eq!(results.len(), 0);

    // Test empty epochs list
    let tle = TLE::from_lines(ISS_LINE1, ISS_LINE2, None).unwrap();
    let empty_epochs: Vec<Epoch> = vec![];

    let results = propagator.propagate_batch(&[tle.clone()], &empty_epochs).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].len(), 0);

    // Test single TLE with empty epochs
    let results = tle.propagate_to_epochs(&empty_epochs).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
#[cfg(feature = "cuda")]
fn test_gpu_availability_reporting() {
    let is_available = BatchPropagator::new().is_gpu_available();
    println!("GPU available: {}", is_available);

    let is_constellation_available = Constellation::is_gpu_available();
    assert_eq!(
        is_available, is_constellation_available,
        "BatchPropagator and Constellation should report same GPU availability"
    );
}
