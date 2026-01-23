use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use keplemon::bodies::Observatory;
use keplemon::bodies::Satellite;
use keplemon::elements::TLE;
use keplemon::enums::TimeSystem;
use keplemon::time::{Epoch, TimeSpan};

fn make_satellite(line_1: &str, line_2: &str) -> Satellite {
    let tle = TLE::from_lines(line_1, line_2, None).expect("failed to parse TLE");
    Satellite::from(tle)
}

fn bench_from_tle(c: &mut Criterion) {
    let line_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999";
    let line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660";
    c.bench_function("satellite_from_tle", |b| {
        b.iter(|| {
            let tle = TLE::from_lines(line_1, line_2, None).expect("failed to parse TLE");
            let _sat = Satellite::from(tle);
        });
    });
}

fn bench_get_close_approach(c: &mut Criterion) {
    let sat_2 = make_satellite(
        "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
        "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950",
    );
    let sat_3 = make_satellite(
        "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
        "2 37605   2.1234  87.2060 0006091 220.8721 161.7206  1.00271635 50950",
    );
    let start = Epoch::from_iso("2025-04-15T12:00:00.000000Z", TimeSystem::UTC);
    let end = Epoch::from_iso("2025-04-16T12:00:00.000000Z", TimeSystem::UTC);

    c.bench_function("satellite_get_close_approach", |b| {
        b.iter_batched(
            || (sat_2.clone(), sat_3.clone()),
            |(mut left, mut right)| {
                let _ = left.get_close_approach(&mut right, start, end, 25.0);
            },
            BatchSize::LargeInput,
        );
    });
}

fn bench_get_relative_state(c: &mut Criterion) {
    let sat_2 = make_satellite(
        "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
        "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950",
    );
    let sat_3 = make_satellite(
        "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
        "2 37605   2.1234  87.2060 0006091 220.8721 161.7206  1.00271635 50950",
    );
    let epoch = Epoch::from_iso("2025-04-15T12:32:28.532000Z", TimeSystem::UTC);

    c.bench_function("satellite_get_relative_state_at_epoch", |b| {
        b.iter(|| {
            let _ = sat_2.get_relative_state_at_epoch(&sat_3, epoch);
        });
    });
}

fn bench_get_body_angles(c: &mut Criterion) {
    let sat_2 = make_satellite(
        "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
        "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950",
    );
    let sat_3 = make_satellite(
        "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990",
        "2 37605   2.1234  87.2060 0006091 220.8721 161.7206  1.00271635 50950",
    );
    let epoch = Epoch::from_iso("2025-04-15T12:32:28.532000Z", TimeSystem::UTC);

    c.bench_function("satellite_get_body_angles_at_epoch", |b| {
        b.iter(|| {
            let _ = sat_2.get_body_angles_at_epoch(&sat_3, epoch);
        });
    });
}

fn bench_state_at_epoch_comparison(c: &mut Criterion) {
    let line_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999";
    let line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660";
    let tle = TLE::from_lines("ISS", line_1, Some(line_2)).expect("failed to parse TLE");
    let mut sat_with_cache = Satellite::from(tle.clone());
    let sat_no_cache = Satellite::from(tle);

    // Cache ephemeris covering a 1-hour window
    let start = Epoch::from_iso("2020-07-18T12:00:00.000000Z", TimeSystem::UTC);
    let end = Epoch::from_iso("2020-07-18T13:00:00.000000Z", TimeSystem::UTC);
    let step = TimeSpan::from_seconds(10.0);
    sat_with_cache
        .get_ephemeris(start, end, step)
        .expect("failed to cache ephemeris");

    // Query epoch in the middle of the cached range
    let query_epoch = Epoch::from_iso("2020-07-18T12:30:00.000000Z", TimeSystem::UTC);

    let mut group = c.benchmark_group("state_at_epoch");

    group.bench_function("get_state_at_epoch (propagator)", |b| {
        b.iter(|| {
            let _ = sat_no_cache.get_state_at_epoch(query_epoch);
        });
    });

    group.bench_function("interpolate_state_at_epoch (cached)", |b| {
        b.iter(|| {
            let _ = sat_with_cache.interpolate_state_at_epoch(query_epoch);
        });
    });

    group.finish();
}

fn bench_get_observatory_access_report(c: &mut Criterion) {
    let line_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999";
    let line_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660";
    let tle = TLE::from_lines("ISS", line_1, Some(line_2)).expect("failed to parse TLE");
    let base_sat = Satellite::from(tle);

    let obs1 = Observatory::new(34.0, -118.0, 100.0);
    let obs2 = Observatory::new(51.5, -0.1, 50.0);
    let obs3 = Observatory::new(-33.9, 18.4, 20.0);
    let observatories = vec![obs1, obs2, obs3];
    let start = Epoch::from_iso("2025-04-18T04:00:00.000000Z", TimeSystem::UTC);
    let end = Epoch::from_iso("2025-04-18T08:00:00.000000Z", TimeSystem::UTC);
    let min_elevation = 10.0;
    let min_duration = TimeSpan::from_minutes(1.0);

    c.bench_function("satellite_get_observatory_access_report", |b| {
        b.iter_batched(
            || base_sat.clone(),
            |mut sat| {
                let _ = sat.get_observatory_access_report(
                    observatories.clone(),
                    start,
                    end,
                    min_elevation,
                    min_duration,
                );
            },
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(
    benches,
    bench_from_tle,
    bench_get_close_approach,
    bench_get_relative_state,
    bench_get_body_angles,
    bench_state_at_epoch_comparison,
    bench_get_observatory_access_report
);
criterion_main!(benches);
