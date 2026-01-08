use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_environment_wrappers(c: &mut Criterion) {
    let mut group = c.benchmark_group("environment");

    group.bench_function(BenchmarkId::new("get_dll_info", "string"), |b| {
        b.iter(saal::environment::get_dll_info);
    });
    group.bench_function(BenchmarkId::new("get_earth_radius", "km"), |b| {
        b.iter(saal::environment::get_earth_radius);
    });
    group.bench_function(BenchmarkId::new("get_fundamental_catalog", "enum"), |b| {
        b.iter(|| saal::environment::get_fundamental_catalog().expect("get_fundamental_catalog failed"));
    });
    group.bench_function(BenchmarkId::new("set_fundamental_catalog", "toggle"), |b| {
        b.iter(|| {
            saal::environment::set_fundamental_catalog(black_box(saal::environment::XF_FKMOD_4));
            saal::environment::set_fundamental_catalog(black_box(saal::environment::XF_FKMOD_5));
        });
    });
    group.bench_function(BenchmarkId::new("get_j2", "value"), |b| {
        b.iter(saal::environment::get_j2);
    });
    group.bench_function(BenchmarkId::new("get_j3", "value"), |b| {
        b.iter(saal::environment::get_j3);
    });
    group.bench_function(BenchmarkId::new("get_j4", "value"), |b| {
        b.iter(saal::environment::get_j4);
    });
    group.bench_function(BenchmarkId::new("get_j5", "value"), |b| {
        b.iter(saal::environment::get_j5);
    });
    group.bench_function(BenchmarkId::new("get_earth_mu", "value"), |b| {
        b.iter(saal::environment::get_earth_mu);
    });
    group.bench_function(BenchmarkId::new("get_earth_flattening", "value"), |b| {
        b.iter(saal::environment::get_earth_flattening);
    });
    group.bench_function(BenchmarkId::new("get_earth_rotation_rate", "value"), |b| {
        b.iter(saal::environment::get_earth_rotation_rate);
    });
    group.bench_function(BenchmarkId::new("get_earth_rotation_acceleration", "value"), |b| {
        b.iter(saal::environment::get_earth_rotation_acceleration);
    });

    group.finish();
}

criterion_group!(benches, bench_environment_wrappers);
criterion_main!(benches);
