use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::path::PathBuf;

fn bench_sgp4_wrappers(c: &mut Criterion) {
    let mut group = c.benchmark_group("sgp4");

    let sgp4_line_1 = "1 22222C 15058A   25363.54791667 +.00012345  10000-1  20000-1 2 0900";
    let sgp4_line_2 = "2 22222  30.0000  40.0000 0005000  60.0000  70.0000  1.2345678012345";
    let xp_line_1 = "1 33333U 15058A   25363.54791667 +.00012345  10000-1  20000-1 4  900";
    let xp_line_2 = "2 33333  30.0000  40.0000 0005000  60.0000  70.0000  8.2345678012345";
    let epoch = 27757.54791667;
    let ephem_start = epoch - 1.0;
    let ephem_stop = epoch;
    let ephem_step = 5.0;
    let ephem_frame = saal::sgp4::SGP4_EPHEM_ECI;

    let asset_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let asset_dir_str = asset_dir.to_str().expect("assets directory missing");

    let sgp4_key = saal::tle::load_lines(sgp4_line_1, sgp4_line_2);
    if sgp4_key <= 0 {
        panic!("load_lines failed");
    }
    let xp_key = saal::tle::load_lines(xp_line_1, xp_line_2);
    if xp_key <= 0 {
        panic!("load_lines failed");
    }
    saal::sgp4::load(sgp4_key).expect("sgp4 load failed");
    saal::sgp4::load(xp_key).expect("sgp4 load failed");
    let (sgp4_pos, sgp4_vel) = saal::sgp4::get_position_velocity(sgp4_key, epoch).expect("posvel failed");
    let sgp4_posvel = [
        sgp4_pos[0],
        sgp4_pos[1],
        sgp4_pos[2],
        sgp4_vel[0],
        sgp4_vel[1],
        sgp4_vel[2],
    ];
    let (xp_pos, xp_vel) = saal::sgp4::get_position_velocity(xp_key, epoch).expect("posvel failed");
    let xp_posvel = [xp_pos[0], xp_pos[1], xp_pos[2], xp_vel[0], xp_vel[1], xp_vel[2]];
    let (sgp4_xa, _) = saal::tle::lines_to_arrays(sgp4_line_1, sgp4_line_2).expect("lines_to_arrays failed");

    group.bench_function(BenchmarkId::new("get_dll_info", "string"), |b| {
        b.iter(saal::sgp4::get_dll_info);
    });
    group.bench_function(BenchmarkId::new("get_position_velocity_lla", "sgp4"), |b| {
        b.iter(|| saal::sgp4::get_position_velocity_lla(black_box(sgp4_key), black_box(epoch)));
    });
    group.bench_function(BenchmarkId::new("get_position_velocity", "sgp4"), |b| {
        b.iter(|| saal::sgp4::get_position_velocity(black_box(sgp4_key), black_box(epoch)));
    });
    group.bench_function(BenchmarkId::new("get_position", "sgp4"), |b| {
        b.iter(|| saal::sgp4::get_position(black_box(sgp4_key), black_box(epoch)));
    });
    group.bench_function(BenchmarkId::new("get_lla", "sgp4"), |b| {
        b.iter(|| saal::sgp4::get_lla(black_box(sgp4_key), black_box(epoch)));
    });
    group.bench_function(BenchmarkId::new("get_full_state", "sgp4"), |b| {
        b.iter(|| saal::sgp4::get_full_state(black_box(sgp4_key), black_box(epoch)));
    });
    group.bench_function(BenchmarkId::new("get_equinoctial", "sgp4"), |b| {
        b.iter(|| saal::sgp4::get_equinoctial(black_box(sgp4_key), black_box(epoch)));
    });
    group.bench_function(BenchmarkId::new("get_positions_velocities", "2 sats"), |b| {
        let sat_keys = [sgp4_key, xp_key];
        b.iter(|| saal::sgp4::get_positions_velocities(black_box(&sat_keys), black_box(epoch)));
    });
    group.bench_function(BenchmarkId::new("get_license_directory", "string"), |b| {
        b.iter(saal::sgp4::get_license_directory);
    });
    group.bench_function(BenchmarkId::new("set_license_directory", "assets"), |b| {
        b.iter(|| saal::sgp4::set_license_directory(black_box(asset_dir_str)));
    });
    group.bench_function(BenchmarkId::new("reepoch_tle", "sgp4"), |b| {
        b.iter(|| saal::sgp4::reepoch_tle(black_box(sgp4_key), black_box(epoch)));
    });
    group.bench_function(BenchmarkId::new("get_ephemeris", "sgp4"), |b| {
        b.iter(|| {
            saal::sgp4::get_ephemeris(
                black_box(sgp4_key),
                black_box(ephem_start),
                black_box(ephem_stop),
                black_box(ephem_step),
                black_box(ephem_frame),
            )
        });
    });
    group.bench_function(BenchmarkId::new("array_to_ephemeris", "sgp4"), |b| {
        b.iter(|| {
            saal::sgp4::array_to_ephemeris(
                black_box(&sgp4_xa),
                black_box(ephem_start),
                black_box(ephem_stop),
                black_box(ephem_step),
                black_box(ephem_frame),
            )
        });
    });
    group.bench_function(BenchmarkId::new("fit_sgp4_array", "sgp4"), |b| {
        b.iter(|| saal::sgp4::fit_sgp4_array(black_box(epoch), black_box(&sgp4_posvel), black_box(Some(0.02))));
    });
    group.bench_function(BenchmarkId::new("fit_xp_array", "xp"), |b| {
        b.iter(|| {
            saal::sgp4::fit_xp_array(
                black_box(epoch),
                black_box(&xp_posvel),
                black_box(Some(0.02)),
                black_box(Some(0.01)),
            )
        });
    });

    saal::sgp4::clear().expect("sgp4 clear failed");
    saal::tle::clear().expect("tle clear failed");

    group.bench_function(BenchmarkId::new("load", "sgp4"), |b| {
        b.iter(|| {
            let key = saal::tle::load_lines(black_box(sgp4_line_1), black_box(sgp4_line_2));
            if key <= 0 {
                panic!("load_lines failed");
            }
            saal::sgp4::load(black_box(key)).expect("sgp4 load failed");
            saal::sgp4::remove(key).expect("sgp4 remove failed");
            saal::tle::remove(key);
        });
    });
    group.bench_function(BenchmarkId::new("remove", "sgp4"), |b| {
        b.iter_batched(
            || {
                let key = saal::tle::load_lines(sgp4_line_1, sgp4_line_2);
                if key <= 0 {
                    panic!("load_lines failed");
                }
                saal::sgp4::load(key).expect("sgp4 load failed");
                key
            },
            |key| {
                saal::sgp4::remove(black_box(key)).expect("sgp4 remove failed");
                saal::tle::remove(key);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function(BenchmarkId::new("clear", "1 sat"), |b| {
        b.iter_batched(
            || {
                let key = saal::tle::load_lines(sgp4_line_1, sgp4_line_2);
                if key <= 0 {
                    panic!("load_lines failed");
                }
                saal::sgp4::load(key).expect("sgp4 load failed");
                key
            },
            |key| {
                saal::sgp4::clear().expect("sgp4 clear failed");
                saal::tle::remove(key);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function(BenchmarkId::new("get_count", "1 sat"), |b| {
        b.iter_batched(
            || {
                let key = saal::tle::load_lines(sgp4_line_1, sgp4_line_2);
                if key <= 0 {
                    panic!("load_lines failed");
                }
                saal::sgp4::load(key).expect("sgp4 load failed");
                key
            },
            |key| {
                let count = saal::sgp4::get_count();
                saal::sgp4::clear().expect("sgp4 clear failed");
                saal::tle::remove(key);
                black_box(count);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_sgp4_wrappers);
criterion_main!(benches);
