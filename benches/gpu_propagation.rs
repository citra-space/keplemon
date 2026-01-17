//! Benchmark GPU vs CPU batch propagation
//! 
//! Run with: cargo bench --features cuda --bench gpu_propagation

#![cfg(feature = "cuda")]

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use keplemon::bodies::Constellation;
use keplemon::catalogs::TLECatalog;
use keplemon::time::{Epoch, TimeSpan};
use keplemon::propagation::PropagationBackend;

fn bench_constellation_propagation(c: &mut Criterion) {
    // Load a test TLE catalog
    let tle_path = "tests/2025-04-15-celestrak.3le";
    
    if !std::path::Path::new(tle_path).exists() {
        eprintln!("Test TLE file not found, skipping benchmark");
        return;
    }
    
    let catalog = TLECatalog::from_3le_file(tle_path.to_string())
        .expect("Failed to load TLE catalog");
    
    let constellation = Constellation::from(catalog);
    let n_sats = constellation.get_satellites().len();
    
    let start = Epoch::now();
    let end = start + TimeSpan::from_hours(24.0);
    
    let mut group = c.benchmark_group("constellation_propagation");
    
    // Benchmark different time step counts
    for n_steps in [10, 50, 100].iter() {
        let step = TimeSpan::from_hours(24.0 / (*n_steps as f64));
        
        // CPU benchmark
        group.bench_with_input(
            BenchmarkId::new("CPU", format!("{}sats_{}steps", n_sats, n_steps)),
            n_steps,
            |b, _| {
                b.iter(|| {
                    constellation.get_batch_ephemeris(
                        black_box(start),
                        black_box(end),
                        black_box(step),
                        Some(PropagationBackend::Cpu),
                    )
                });
            },
        );
        
        // GPU benchmark (if available)
        if Constellation::is_gpu_available() {
            group.bench_with_input(
                BenchmarkId::new("GPU", format!("{}sats_{}steps", n_sats, n_steps)),
                n_steps,
                |b, _| {
                    b.iter(|| {
                        constellation.get_batch_ephemeris(
                            black_box(start),
                            black_box(end),
                            black_box(step),
                            Some(PropagationBackend::Gpu),
                        )
                    });
                },
            );
        }
    }
    
    group.finish();
}

criterion_group!(benches, bench_constellation_propagation);
criterion_main!(benches);
