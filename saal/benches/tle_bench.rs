use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::path::PathBuf;

fn bench_tle_wrappers(c: &mut Criterion) {
    let mut group = c.benchmark_group("tle");

    let line_1 = "1 11111U 98067A   25363.54791667 +.00012345  10000-1  20000-1 0 0900";
    let line_2 = "2 11111  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345";
    let null_line_1 = "1 11111U          25363.54791667 +.00012345  00000 0  00000 0 0 0900";

    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("2025-12-30-celestrak.tle");
    let file_path_str = file_path.to_str().expect("celestrak.tle missing");

    let (xa_tle, xs_tle) = saal::tle::lines_to_arrays(line_1, line_2).expect("lines_to_arrays failed");

    group.bench_function(BenchmarkId::new("get_dll_info", "string"), |b| {
        b.iter(saal::tle::get_dll_info);
    });
    group.bench_function(BenchmarkId::new("fix_blank_exponent_sign", "mut"), |b| {
        b.iter_batched(
            || null_line_1.to_string(),
            |mut line| {
                saal::tle::fix_blank_exponent_sign(black_box(&mut line));
                black_box(line);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function(BenchmarkId::new("add_check_sums", "mut"), |b| {
        b.iter_batched(
            || (line_1.to_string(), line_2.to_string()),
            |(mut line_1_owned, mut line_2_owned)| {
                saal::tle::add_check_sums(&mut line_1_owned, &mut line_2_owned).expect("add_check_sums failed");
                black_box((line_1_owned, line_2_owned));
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function(BenchmarkId::new("lines_to_arrays", "sgp"), |b| {
        b.iter(|| saal::tle::lines_to_arrays(black_box(line_1), black_box(line_2)));
    });
    group.bench_function(BenchmarkId::new("arrays_to_lines", "sgp"), |b| {
        b.iter(|| saal::tle::arrays_to_lines(black_box(xa_tle), black_box(&xs_tle)));
    });
    group.bench_function(BenchmarkId::new("get_check_sums", "line1/2"), |b| {
        b.iter(|| saal::tle::get_check_sums(black_box(line_1), black_box(line_2)));
    });
    group.bench_function(BenchmarkId::new("load_lines", "sgp"), |b| {
        b.iter(|| {
            let key = saal::tle::load_lines(black_box(line_1), black_box(line_2));
            if key <= 0 {
                panic!("load_lines failed");
            }
            saal::tle::remove(key);
        });
    });
    group.bench_function(BenchmarkId::new("get_lines", "sgp"), |b| {
        b.iter_batched(
            || saal::tle::load_lines(line_1, line_2),
            |key| {
                if key <= 0 {
                    panic!("load_lines failed");
                }
                let _ = saal::tle::get_lines(black_box(key)).expect("get_lines failed");
                saal::tle::remove(key);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function(BenchmarkId::new("load_arrays", "sgp"), |b| {
        b.iter(|| {
            let key = saal::tle::load_arrays(black_box(xa_tle), black_box(&xs_tle)).expect("load_arrays failed");
            saal::tle::remove(key);
        });
    });
    group.bench_function(BenchmarkId::new("get_arrays", "sgp"), |b| {
        b.iter_batched(
            || saal::tle::load_lines(line_1, line_2),
            |key| {
                if key <= 0 {
                    panic!("load_lines failed");
                }
                let _ = saal::tle::get_arrays(black_box(key)).expect("get_arrays failed");
                saal::tle::remove(key);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function(BenchmarkId::new("remove_key", "sgp"), |b| {
        b.iter(|| {
            let key = saal::tle::load_lines(black_box(line_1), black_box(line_2));
            if key <= 0 {
                panic!("load_lines failed");
            }
            saal::tle::remove(black_box(key));
        });
    });
    group.bench_function(BenchmarkId::new("remove_all", "1 sat"), |b| {
        b.iter(|| {
            let key = saal::tle::load_lines(black_box(line_1), black_box(line_2));
            if key <= 0 {
                panic!("load_lines failed");
            }
            saal::tle::clear().expect("remove_all failed");
        });
    });
    group.bench_function(BenchmarkId::new("get_count", "1 sat"), |b| {
        b.iter_batched(
            || saal::tle::load_lines(line_1, line_2),
            |key| {
                if key <= 0 {
                    panic!("load_lines failed");
                }
                let count = saal::tle::get_count();
                saal::tle::remove(key);
                black_box(count);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function(BenchmarkId::new("get_keys", "1 sat"), |b| {
        b.iter_batched(
            || saal::tle::load_lines(line_1, line_2),
            |key| {
                if key <= 0 {
                    panic!("load_lines failed");
                }
                let keys = saal::tle::get_keys(saal::IDX_ORDER_ASC);
                saal::tle::remove(key);
                black_box(keys);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function(BenchmarkId::new("load_file", "celestrak"), |b| {
        b.iter(|| {
            let count = saal::tle::load_file(black_box(file_path_str)).expect("load_file failed");
            black_box(count);
            saal::tle::clear().expect("remove_all failed");
        });
    });
    group.bench_function(BenchmarkId::new("remove_nulls", "parsed"), |b| {
        b.iter(|| {
            let parsed = saal::tle::lines_to_arrays(black_box(null_line_1), black_box(line_2))
                .map(|(xa_tle, xs_tle)| saal::tle::ParsedTLE::from((xa_tle, xs_tle)))
                .expect("lines_to_arrays failed");
            let _ = parsed.get_lines(true).expect("get_lines failed");
        });
    });

    group.finish();
}

criterion_group!(benches, bench_tle_wrappers);
criterion_main!(benches);
