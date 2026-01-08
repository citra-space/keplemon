use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_main_wrappers(c: &mut Criterion) {
    let mut group = c.benchmark_group("main");

    group.bench_function(BenchmarkId::new("get_dll_info", "string"), |b| {
        b.iter(saal::get_dll_info);
    });
    group.bench_function(BenchmarkId::new("get_key_mode", "enum"), |b| {
        b.iter(|| saal::get_key_mode().expect("get_key_mode failed"));
    });
    group.bench_function(BenchmarkId::new("set_duplicate_key_mode", "toggle"), |b| {
        b.iter(|| {
            saal::set_duplicate_key_mode(black_box(saal::DUPKEY_ACTUAL))
                .expect("set_duplicate_key_mode failed");
            saal::set_duplicate_key_mode(black_box(saal::DUPKEY_ZERO))
                .expect("set_duplicate_key_mode failed");
            saal::reset_key_mode();
        });
    });
    group.bench_function(BenchmarkId::new("set_key_mode", "toggle"), |b| {
        b.iter(|| {
            saal::set_key_mode(black_box(saal::ALL_KEYMODE_NODUP)).expect("set_key_mode failed");
            saal::set_key_mode(black_box(saal::ALL_KEYMODE_DMA)).expect("set_key_mode failed");
            saal::reset_key_mode();
        });
    });

    group.finish();
}

criterion_group!(benches, bench_main_wrappers);
criterion_main!(benches);
