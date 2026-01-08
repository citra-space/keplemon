use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use saal::obs::ParsedB3;

const B3_CARD: &str = "U0001151013352142520112J85202 2220398         -01207880+03706326+05814970 9 4  10001100011";

fn base_parsed_b3(equinox: i32) -> ParsedB3 {
    ParsedB3 {
        classification: "U".to_string(),
        norad_id: 11111,
        sensor_number: 500,
        epoch: 25934.75,
        declination: Some(-20.6075648583427),
        right_ascension: Some(57.6850704027472),
        range: Some(28002.6701345644),
        range_rate: None,
        azimuth: None,
        elevation: None,
        elevation_rate: None,
        azimuth_rate: None,
        year_of_equinox: Some(equinox),
        range_acceleration: None,
        observation_type: 9,
        track_position: 3,
        association_status: 1,
        site_tag: 11111,
        spadoc_tag: 11111,
        position: Some([0.0, 0.0, 0.0]),
    }
}

fn bench_obs_wrappers(c: &mut Criterion) {
    let mut group = c.benchmark_group("obs");

    group.bench_function(BenchmarkId::new("get_dll_info", "string"), |b| {
        b.iter(saal::obs::get_dll_info);
    });
    group.bench_function(BenchmarkId::new("parse_b3", "card"), |b| {
        b.iter(|| ParsedB3::from_line(black_box(B3_CARD)));
    });
    group.bench_function(BenchmarkId::new("get_line", "b3"), |b| {
        let parsed = base_parsed_b3(saal::obs::EQUINOX_OBSYEAR);
        b.iter(|| parsed.get_line());
    });

    group.finish();
}

criterion_group!(benches, bench_obs_wrappers);
criterion_main!(benches);
