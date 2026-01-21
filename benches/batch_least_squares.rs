use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use keplemon::bodies::{Observatory, Satellite, Sensor};
use keplemon::elements::{TLE, TopocentricElements};
use keplemon::enums::{KeplerianType, TimeSystem};
use keplemon::estimation::{BatchLeastSquares, Observation};
use keplemon::time::Epoch;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JsonObservation {
    sensor_latitude: f64,
    sensor_longitude: f64,
    sensor_altitude: f64,
    ra: f64,
    dec: f64,
    epoch: String,
    angular_noise: f64,
}

fn load_observations() -> Vec<Observation> {
    let raw =
        std::fs::read_to_string("tests/test-observations.json").expect("failed to read tests/test-observations.json");
    let json_obs: Vec<JsonObservation> = serde_json::from_str(&raw).expect("failed to parse observations json");

    let mut observations = Vec::with_capacity(json_obs.len());
    for json_ob in json_obs {
        let epoch = Epoch::from_iso(&json_ob.epoch, TimeSystem::UTC);
        let site = Observatory::new(
            json_ob.sensor_latitude,
            json_ob.sensor_longitude,
            json_ob.sensor_altitude,
        );
        let els = TopocentricElements::new(json_ob.ra, json_ob.dec);
        let sensor = Sensor::new(json_ob.angular_noise);
        let ob = Observation::new(sensor, epoch, els, site.get_state_at_epoch(epoch).position);
        observations.push(ob);
    }
    observations
}

fn build_bls(observations: &[Observation], initial_sat: &Satellite) -> BatchLeastSquares {
    let mut bls = BatchLeastSquares::new(observations.to_vec(), initial_sat);
    bls.set_estimate_srp(true);
    bls.set_output_type(KeplerianType::MeanBrouwerXP);
    bls
}

fn bench_batch_least_squares(c: &mut Criterion) {
    let observations = load_observations();
    let initial_tle = TLE::from_lines(
        "1 99999U          25334.80826079 -.00000092  00000 0  00000 0 0 0000",
        "2 99999   5.1462  74.9949 0001499 136.0805 318.9951  0.9987069300000",
        None,
    )
    .expect("failed to parse initial TLE");
    let initial_sat = Satellite::from(initial_tle);

    c.bench_function("batch_least_squares_solve", |b| {
        b.iter_batched(
            || build_bls(&observations, &initial_sat),
            |mut bls| {
                bls.solve().expect("solve failed");
            },
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(benches, bench_batch_least_squares);
criterion_main!(benches);
