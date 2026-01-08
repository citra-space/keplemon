use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::path::PathBuf;

fn bench_time_wrappers(c: &mut Criterion) {
    let mut group = c.benchmark_group("time");

    let ds50 = 8431.0;
    let dtg20 = "1973/001 0000 00.000";
    let constants_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("time_constants.dat");
    let constants_path_str = constants_path.to_str().expect("time_constants.dat missing");

    group.bench_function(BenchmarkId::new("get_dll_info", "string"), |b| {
        b.iter(saal::time::get_dll_info);
    });
    group.bench_function(BenchmarkId::new("ymd_components_to_ds50", "1973-01-01"), |b| {
        b.iter(|| saal::time::ymd_components_to_ds50(black_box(1973), 1, 1, 0, 0, 0.0));
    });
    group.bench_function(BenchmarkId::new("ds50_to_ymd_components", "8431.0"), |b| {
        b.iter(|| saal::time::ds50_to_ymd_components(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("dtg_to_ds50", "dtg20"), |b| {
        b.iter(|| saal::time::dtg_to_ds50(black_box(dtg20)));
    });
    group.bench_function(BenchmarkId::new("ds50_to_dtg20", "8431.0"), |b| {
        b.iter(|| saal::time::ds50_to_dtg20(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("ds50_to_dtg19", "8431.0"), |b| {
        b.iter(|| saal::time::ds50_to_dtg19(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("ds50_to_dtg17", "8431.0"), |b| {
        b.iter(|| saal::time::ds50_to_dtg17(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("ds50_to_dtg15", "8431.0"), |b| {
        b.iter(|| saal::time::ds50_to_dtg15(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("year_doy_to_ds50", "1973/1.0"), |b| {
        b.iter(|| saal::time::year_doy_to_ds50(black_box(1973), black_box(1.0)));
    });
    group.bench_function(BenchmarkId::new("ds50_to_year_doy", "8431.0"), |b| {
        b.iter(|| saal::time::ds50_to_year_doy(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("tai_to_utc", "8431.0"), |b| {
        b.iter(|| saal::time::tai_to_utc(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("utc_to_tai", "8431.0"), |b| {
        b.iter(|| saal::time::utc_to_tai(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("utc_to_ut1", "8431.0"), |b| {
        b.iter(|| saal::time::utc_to_ut1(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("utc_to_tt", "8431.0"), |b| {
        b.iter(|| saal::time::utc_to_tt(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("tai_to_ut1", "8431.0"), |b| {
        b.iter(|| saal::time::tai_to_ut1(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("load_constants", "time_constants.dat"), |b| {
        b.iter(|| saal::time::load_constants(black_box(constants_path_str)).expect("load_constants failed"));
    });
    group.bench_function(BenchmarkId::new("get_fk4_greenwich_angle", "8431.0"), |b| {
        b.iter(|| saal::time::get_fk4_greenwich_angle(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("get_fk5_greenwich_angle", "8431.0"), |b| {
        b.iter(|| saal::time::get_fk5_greenwich_angle(black_box(ds50)));
    });
    group.bench_function(BenchmarkId::new("constants_loaded", "bool"), |b| {
        b.iter(saal::time::constants_loaded);
    });

    group.finish();
}

criterion_group!(benches, bench_time_wrappers);
criterion_main!(benches);
