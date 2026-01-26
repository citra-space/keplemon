//! Integration test for BatchPropagator automatic propagator selection
//!
//! This test verifies that BatchPropagator correctly routes satellites to
//! the appropriate propagator based on orbital period:
//! - Near-earth (period < 225 min) → SGP4
//! - Deep-space (period >= 225 min) → SDP4-Analytical

use keplemon::elements::TLE;
use keplemon::enums::TimeSystem;
use keplemon::propagation::{BatchPropagator, PropagationBackend, orbital_period_minutes};
use keplemon::time::Epoch;

#[test]
#[cfg(feature = "cuda")]
fn test_propagator_selection_near_earth() {
    // ISS - near-earth orbit (period ~90 minutes)
    let tle_iss = TLE::from_lines(
        "ISS (ZARYA)",
        "1 25544U 98067A   23001.00000000  .00016717  00000-0  10270-3 0  9005",
        Some("2 25544  51.6400 208.9163 0006317  69.9862 290.1961 15.54225995000006"),
    )
    .unwrap();

    let period = orbital_period_minutes(tle_iss.get_mean_motion());
    println!("ISS orbital period: {:.2} minutes", period);
    assert!(period < 225.0, "ISS should be near-earth");

    let propagator = BatchPropagator::new()
        .set_backend(PropagationBackend::Gpu);

    let epochs = vec![
        Epoch::from_days_since_1950(23001.0, TimeSystem::UTC),
        Epoch::from_days_since_1950(23001.5, TimeSystem::UTC),
    ];

    // Should use SGP4 internally
    let results = propagator.propagate_batch(&[tle_iss], &epochs);
    assert!(results.is_ok(), "Near-earth propagation should succeed");

    let states = results.unwrap();
    assert_eq!(states.len(), 1);
    assert_eq!(states[0].len(), 2);
}

#[test]
#[cfg(feature = "cuda")]
fn test_propagator_selection_deep_space() {
    // INTELSAT 902 - GEO orbit (period ~1436 minutes)
    let tle_geo = TLE::from_lines(
        "INTELSAT 902",
        "1 26900U 01039A   23001.50000000 -.00000275  00000-0  00000-0 0  9992",
        Some("2 26900   0.0167  74.2938 0001928 140.1234 291.8230  1.00274002000017"),
    )
    .unwrap();

    let period = orbital_period_minutes(tle_geo.get_mean_motion());
    println!("INTELSAT 902 orbital period: {:.2} minutes", period);
    assert!(period >= 225.0, "GEO should be deep-space");

    let propagator = BatchPropagator::new()
        .set_backend(PropagationBackend::Gpu);

    let epochs = vec![
        Epoch::from_days_since_1950(23001.5, TimeSystem::UTC),
        Epoch::from_days_since_1950(23002.0, TimeSystem::UTC),
    ];

    // Should use SDP4-Analytical internally
    let results = propagator.propagate_batch(&[tle_geo], &epochs);
    assert!(results.is_ok(), "Deep-space propagation should succeed");

    let states = results.unwrap();
    assert_eq!(states.len(), 1);
    assert_eq!(states[0].len(), 2);
}

#[test]
#[cfg(feature = "cuda")]
fn test_propagator_selection_mixed() {
    // Mixed batch: ISS (LEO) + INTELSAT 902 (GEO)
    let tle_iss = TLE::from_lines(
        "ISS (ZARYA)",
        "1 25544U 98067A   23001.00000000  .00016717  00000-0  10270-3 0  9005",
        Some("2 25544  51.6400 208.9163 0006317  69.9862 290.1961 15.54225995000006"),
    )
    .unwrap();

    let tle_geo = TLE::from_lines(
        "INTELSAT 902",
        "1 26900U 01039A   23001.50000000 -.00000275  00000-0  00000-0 0  9992",
        Some("2 26900   0.0167  74.2938 0001928 140.1234 291.8230  1.00274002000017"),
    )
    .unwrap();

    let period_iss = orbital_period_minutes(tle_iss.get_mean_motion());
    let period_geo = orbital_period_minutes(tle_geo.get_mean_motion());

    println!("ISS period: {:.2} min (near-earth)", period_iss);
    println!("INTELSAT 902 period: {:.2} min (deep-space)", period_geo);

    let propagator = BatchPropagator::new()
        .set_backend(PropagationBackend::Gpu);

    let epochs = vec![
        Epoch::from_days_since_1950(23001.0, TimeSystem::UTC),
        Epoch::from_days_since_1950(23001.5, TimeSystem::UTC),
    ];

    // Should partition: ISS → SGP4, INTELSAT → SDP4-Analytical
    let results = propagator.propagate_batch(&[tle_iss.clone(), tle_geo.clone()], &epochs);
    assert!(results.is_ok(), "Mixed propagation should succeed");

    let states = results.unwrap();
    assert_eq!(states.len(), 2, "Should have results for both satellites");
    assert_eq!(states[0].len(), 2, "ISS should have 2 states");
    assert_eq!(states[1].len(), 2, "INTELSAT should have 2 states");

    // Verify results are reasonable
    let iss_altitude_0 = (states[0][0].position.get_magnitude() - 6371.0).abs();
    let geo_altitude_0 = (states[1][0].position.get_magnitude() - 6371.0).abs();

    println!("ISS altitude: {:.1} km", iss_altitude_0);
    println!("GEO altitude: {:.1} km", geo_altitude_0);

    assert!(iss_altitude_0 < 1000.0, "ISS should be in LEO");
    assert!(geo_altitude_0 > 35000.0, "INTELSAT should be at GEO altitude");
}

#[test]
fn test_orbital_period_helper() {
    // Test the helper function
    let mean_motion_iss = 15.54225995; // rev/day for ISS
    let period_iss = orbital_period_minutes(mean_motion_iss);

    let mean_motion_geo = 1.00274002; // rev/day for GEO
    let period_geo = orbital_period_minutes(mean_motion_geo);

    println!("ISS: {:.2} rev/day = {:.2} min period", mean_motion_iss, period_iss);
    println!("GEO: {:.5} rev/day = {:.2} min period", mean_motion_geo, period_geo);

    assert!(period_iss > 85.0 && period_iss < 95.0, "ISS period should be ~90 min");
    assert!(period_geo > 1430.0 && period_geo < 1450.0, "GEO period should be ~1436 min (1 day)");

    assert!(period_iss < 225.0, "ISS is near-earth");
    assert!(period_geo >= 225.0, "GEO is deep-space");
}
