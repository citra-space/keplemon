use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

const SENSOR_CARD: &str = "211  3381724 -25333969 -1521161 -5083089  3530462  U SOCORRO CAM1              S";
const NOISE_CARD: &str = "211 5   0.0003 0.0003 0.0000 0.0000  -0.0005 -0.0003  0.0000  0.0000  0.0000  BS";

fn bench_sensor_wrappers(c: &mut Criterion) {
    let mut group = c.benchmark_group("sensor");

    group.bench_function(BenchmarkId::new("get_dll_info", "string"), |b| {
        b.iter(saal::sensor::get_dll_info);
    });
    group.bench_function(BenchmarkId::new("load_file", "sensors.dat"), |b| {
        b.iter(|| {
            saal::sensor::load_file(black_box("tests/data/sensors.dat")).expect("load_file failed");
            let _count = saal::sensor::count_loaded();
            saal::sensor::clear().expect("clear failed");
        });
    });
    group.bench_function(BenchmarkId::new("load_card", "noise"), |b| {
        b.iter(|| {
            saal::sensor::load_card(black_box(SENSOR_CARD)).expect("load_card failed");
            saal::sensor::load_card(black_box(NOISE_CARD)).expect("load_card failed");
            saal::sensor::clear().expect("clear failed");
        });
    });
    group.bench_function(BenchmarkId::new("get_arrays", "loaded"), |b| {
        b.iter_batched(
            || {
                saal::sensor::load_card(SENSOR_CARD).expect("load_card failed");
                saal::sensor::load_card(NOISE_CARD).expect("load_card failed");
                let keys = saal::sensor::get_keys(saal::IDX_ORDER_READ);
                keys[keys.len() - 1]
            },
            |key| {
                let _ = saal::sensor::get_arrays(black_box(key)).expect("get_arrays failed");
                saal::sensor::clear().expect("clear failed");
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_sensor_wrappers);
criterion_main!(benches);
