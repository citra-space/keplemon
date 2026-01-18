//! Single satellite debug test: CPU vs GPU SGP4 propagation
//! 
//! This test propagates a single well-known satellite using both CPU and GPU
//! implementations to help debug and validate the CUDA SGP4 algorithm.

#![cfg(feature = "cuda")]

use keplemon::elements::TLE;
use keplemon::bodies::Satellite;
use keplemon::time::TimeSpan;
use keplemon::gpu::{CudaSgp4Propagator, cuda_sgp4::TleDataGpu};

/// ISS TLE - well-known LEO satellite
const ISS_LINE1: &str = "1 25544U 98067A   25105.52083333  .00012345  00000+0  22013-3 0  9991";
const ISS_LINE2: &str = "2 25544  51.6456 339.5765 0003456  35.8734  85.9834 15.48919755123456";

/// Julian Date offset for 1950
const JD_1950: f64 = 2433282.5;

#[test]
fn test_single_satellite_debug() {
    // Skip if CUDA not available
    if !CudaSgp4Propagator::is_cuda_available() {
        eprintln!("CUDA not available, skipping test");
        return;
    }
    
    println!("\n=== Single Satellite Debug Test ===\n");
    
    // Parse ISS TLE
    let tle = TLE::from_lines(ISS_LINE1, ISS_LINE2, None)
        .expect("Failed to parse ISS TLE");
    
    let satellite = Satellite::from(tle.clone());
    let kep = tle.get_keplerian_state();
    let epoch = kep.epoch;
    
    println!("Satellite: ISS (ZARYA)");
    println!("TLE Epoch: days_since_1950 = {}", epoch.days_since_1950);
    println!("TLE Epoch JD: {}", epoch.days_since_1950 + JD_1950);
    println!();
    
    // Print TLE parameters
    // NOTE: keplemon stores angles in DEGREES (as per TLE format)
    println!("TLE Parameters:");
    println!("  Inclination:     {:.4}°", kep.elements.inclination);
    println!("  RAAN:            {:.4}°", kep.elements.raan);
    println!("  Eccentricity:    {:.7}", kep.elements.eccentricity);
    println!("  Arg of Perigee:  {:.4}°", kep.elements.argument_of_perigee);
    println!("  Mean Anomaly:    {:.4}°", kep.elements.mean_anomaly);
    println!("  Mean Motion:     {:.8} rev/day", tle.get_mean_motion());
    println!("  B*:              {:.10}", tle.get_b_star());
    println!("  ndot:            {:.10}", tle.get_mean_motion_dot());
    println!("  nddot:           {:.10}", tle.get_mean_motion_dot_dot());
    println!();
    
    // ========================================================================
    // CPU PROPAGATION at epoch (t=0)
    // ========================================================================
    println!("--- Propagation at Epoch (t=0) ---\n");
    
    // First, let's propagate at epoch using the TLE directly
    let cpu_state_0 = tle.get_cartesian_state_at_epoch(epoch)
        .expect("CPU propagation at epoch failed");
    
    println!("CPU State at epoch:");
    println!("  Position: [{:.6}, {:.6}, {:.6}] km", 
             cpu_state_0.position[0], cpu_state_0.position[1], cpu_state_0.position[2]);
    println!("  Velocity: [{:.6}, {:.6}, {:.6}] km/s",
             cpu_state_0.velocity[0], cpu_state_0.velocity[1], cpu_state_0.velocity[2]);
    
    let cpu_pos_mag = (cpu_state_0.position[0].powi(2) + 
                       cpu_state_0.position[1].powi(2) + 
                       cpu_state_0.position[2].powi(2)).sqrt();
    let cpu_vel_mag = (cpu_state_0.velocity[0].powi(2) + 
                       cpu_state_0.velocity[1].powi(2) + 
                       cpu_state_0.velocity[2].powi(2)).sqrt();
    println!("  |r| = {:.3} km (altitude ~{:.1} km)", cpu_pos_mag, cpu_pos_mag - 6378.137);
    println!("  |v| = {:.6} km/s", cpu_vel_mag);
    println!();
    
    // ========================================================================
    // GPU PROPAGATION at epoch (t=0)  
    // ========================================================================
    
    // Convert TLE to GPU format
    // NOTE: keplemon stores angles in DEGREES (same as TLE format), CUDA kernel also expects DEGREES
    // So no conversion is needed!
    
    let tle_data_gpu = TleDataGpu {
        epoch_jd: epoch.days_since_1950 + JD_1950,
        inclination: kep.elements.inclination,      // already in degrees
        raan: kep.elements.raan,                    // already in degrees
        eccentricity: kep.elements.eccentricity,
        arg_perigee: kep.elements.argument_of_perigee, // already in degrees
        mean_anomaly: kep.elements.mean_anomaly,       // already in degrees
        mean_motion: tle.get_mean_motion(),
        bstar: tle.get_b_star(),
        ndot: tle.get_mean_motion_dot(),
        nddot: tle.get_mean_motion_dot_dot(),
    };
    
    println!("GPU TLE Data:");
    println!("  epoch_jd:     {:.6}", tle_data_gpu.epoch_jd);
    println!("  inclination:  {:.4}°", tle_data_gpu.inclination);
    println!("  raan:         {:.4}°", tle_data_gpu.raan);
    println!("  eccentricity: {:.7}", tle_data_gpu.eccentricity);
    println!("  arg_perigee:  {:.4}°", tle_data_gpu.arg_perigee);
    println!("  mean_anomaly: {:.4}°", tle_data_gpu.mean_anomaly);
    println!("  mean_motion:  {:.8} rev/day", tle_data_gpu.mean_motion);
    println!("  bstar:        {:.10}", tle_data_gpu.bstar);
    println!();
    
    // Initialize GPU propagator
    let mut gpu_propagator = CudaSgp4Propagator::new()
        .expect("Failed to create CUDA propagator");
    
    gpu_propagator.init_satellites(&[tle_data_gpu])
        .expect("Failed to initialize satellite on GPU");
    
    // Propagate at epoch (JD = epoch_jd, so tsince should be 0)
    let jd_epoch = epoch.days_since_1950 + JD_1950;
    let gpu_states = gpu_propagator.propagate(&[jd_epoch])
        .expect("GPU propagation failed");
    
    let gpu_state_0 = &gpu_states[0];
    
    println!("GPU State at epoch:");
    println!("  Position: [{:.6}, {:.6}, {:.6}] km", 
             gpu_state_0.x, gpu_state_0.y, gpu_state_0.z);
    println!("  Velocity: [{:.6}, {:.6}, {:.6}] km/s",
             gpu_state_0.vx, gpu_state_0.vy, gpu_state_0.vz);
    println!("  Error code: {}", gpu_state_0.error_code);
    
    let gpu_pos_mag = (gpu_state_0.x.powi(2) + gpu_state_0.y.powi(2) + gpu_state_0.z.powi(2)).sqrt();
    let gpu_vel_mag = (gpu_state_0.vx.powi(2) + gpu_state_0.vy.powi(2) + gpu_state_0.vz.powi(2)).sqrt();
    println!("  |r| = {:.3} km", gpu_pos_mag);
    println!("  |v| = {:.6} km/s", gpu_vel_mag);
    println!();
    
    // Compare
    let pos_error = ((cpu_state_0.position[0] - gpu_state_0.x).powi(2) +
                     (cpu_state_0.position[1] - gpu_state_0.y).powi(2) +
                     (cpu_state_0.position[2] - gpu_state_0.z).powi(2)).sqrt();
    let vel_error = ((cpu_state_0.velocity[0] - gpu_state_0.vx).powi(2) +
                     (cpu_state_0.velocity[1] - gpu_state_0.vy).powi(2) +
                     (cpu_state_0.velocity[2] - gpu_state_0.vz).powi(2)).sqrt();
    
    println!("=== COMPARISON at t=0 ===");
    println!("Position error: {:.6} km ({:.2} m)", pos_error, pos_error * 1000.0);
    println!("Velocity error: {:.9} km/s ({:.6} mm/s)", vel_error, vel_error * 1e6);
    println!();
    
    // ========================================================================
    // Propagation at t = +10 minutes
    // ========================================================================
    println!("--- Propagation at t = +10 minutes ---\n");
    
    let epoch_plus_10 = epoch + TimeSpan::from_minutes(10.0);
    
    let cpu_state_10 = tle.get_cartesian_state_at_epoch(epoch_plus_10)
        .expect("CPU propagation at +10 min failed");
    
    println!("CPU State at +10 min:");
    println!("  Position: [{:.6}, {:.6}, {:.6}] km", 
             cpu_state_10.position[0], cpu_state_10.position[1], cpu_state_10.position[2]);
    println!("  Velocity: [{:.6}, {:.6}, {:.6}] km/s",
             cpu_state_10.velocity[0], cpu_state_10.velocity[1], cpu_state_10.velocity[2]);
    
    let jd_plus_10 = epoch_plus_10.days_since_1950 + JD_1950;
    let gpu_states_10 = gpu_propagator.propagate(&[jd_plus_10])
        .expect("GPU propagation at +10 min failed");
    
    let gpu_state_10 = &gpu_states_10[0];
    
    println!("GPU State at +10 min:");
    println!("  Position: [{:.6}, {:.6}, {:.6}] km", 
             gpu_state_10.x, gpu_state_10.y, gpu_state_10.z);
    println!("  Velocity: [{:.6}, {:.6}, {:.6}] km/s",
             gpu_state_10.vx, gpu_state_10.vy, gpu_state_10.vz);
    println!("  Error code: {}", gpu_state_10.error_code);
    println!();
    
    let pos_error_10 = ((cpu_state_10.position[0] - gpu_state_10.x).powi(2) +
                        (cpu_state_10.position[1] - gpu_state_10.y).powi(2) +
                        (cpu_state_10.position[2] - gpu_state_10.z).powi(2)).sqrt();
    let vel_error_10 = ((cpu_state_10.velocity[0] - gpu_state_10.vx).powi(2) +
                        (cpu_state_10.velocity[1] - gpu_state_10.vy).powi(2) +
                        (cpu_state_10.velocity[2] - gpu_state_10.vz).powi(2)).sqrt();
    
    println!("=== COMPARISON at t=+10min ===");
    println!("Position error: {:.6} km ({:.2} m)", pos_error_10, pos_error_10 * 1000.0);
    println!("Velocity error: {:.9} km/s ({:.6} mm/s)", vel_error_10, vel_error_10 * 1e6);
    println!();
    
    // Assert tolerance (start with loose tolerance, tighten as we fix bugs)
    let pos_tol = 1.0; // 1 km tolerance to start
    assert!(gpu_state_0.error_code == 0, "GPU propagation failed with error code {}", gpu_state_0.error_code);
    assert!(pos_error < pos_tol, "Position error at epoch too large: {} km", pos_error);
    
    println!("✓ Test passed within tolerance!");
}
