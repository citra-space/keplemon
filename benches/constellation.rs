use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use keplemon::bodies::Constellation;
use keplemon::catalogs::TLECatalog;
use keplemon::enums::TimeSystem;
use keplemon::time::{Epoch, TimeSpan};
use std::path::Path;

fn load_catalog(path: &str) -> TLECatalog {
    TLECatalog::from_tle_file(path).expect("failed to load TLE catalog")
}

fn bench_from_tle_catalog(c: &mut Criterion) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/2025-04-15-celestrak.3le");
    let catalog = load_catalog(path.to_str().unwrap());

    c.bench_function("constellation_from_tle_catalog", |b| {
        b.iter_batched(
            || catalog.clone(),
            |catalog| {
                let _ = Constellation::from(catalog);
            },
            BatchSize::LargeInput,
        );
    });
}

fn bench_get_ca_report_vs_many(c: &mut Criterion) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/2026-01-09-geo.3le");
    let base = Constellation::from(load_catalog(path.to_str().unwrap()));
    let start = Epoch::from_iso("2026-01-09T00:00:00.000000Z", TimeSystem::UTC);
    let end = start + TimeSpan::from_hours(24.0);

    c.bench_function("constellation_get_ca_report_vs_many", |b| {
        b.iter_batched(
            || base.clone(),
            |mut sats| {
                let _ = sats.get_ca_report_vs_many(start, end, 10.0);
            },
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(
    benches,
    bench_from_tle_catalog,
    bench_get_ca_report_vs_many,
);
criterion_main!(benches);
