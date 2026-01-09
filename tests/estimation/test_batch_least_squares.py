import json
from keplemon.estimation import Observation, BatchLeastSquares
from keplemon.elements import TopocentricElements, TLE
from keplemon.bodies import Satellite, Observatory, Sensor
from keplemon.time import Epoch, TimeSpan
from keplemon.enums import TimeSystem, KeplerianType


def test_solve() -> None:

    initial_tle = TLE.from_lines(
        "1 99999U          25334.80826079 -.00000092  00000 0  00000 0 0 0000",
        "2 99999   5.1462  74.9949 0001499 136.0805 318.9951  0.9987069300000",
    )
    initial_sat = Satellite.from_tle(initial_tle)
    truth_tle = TLE.from_lines(
        "1 99999U          25335.68208465 +.00000000  16437-1  00000+0 4 00000",
        "2 99999   5.2391  74.7607 0001808 154.7302 254.7343  0.99871681000004",
    )
    truth_sat = Satellite.from_tle(truth_tle)

    with open("./local/test-observations.json", "r") as f:
        json_obs = json.load(f)

    observations: list[Observation] = []

    for json_ob in json_obs:
        lat = json_ob["sensor_latitude"]
        lon = json_ob["sensor_longitude"]
        alt = json_ob["sensor_altitude"]
        ra = json_ob["ra"]
        dec = json_ob["dec"]
        ob_time = json_ob["epoch"]

        epoch = Epoch.from_iso(ob_time, TimeSystem.UTC)
        site = Observatory(lat, lon, alt)
        els = TopocentricElements(ra, dec)
        sensor = Sensor(json_ob["angular_noise"])
        ob = Observation(sensor, epoch, els, site.get_state_at_epoch(epoch).position)
        observations.append(ob)

    bls = BatchLeastSquares(observations, initial_sat)
    bls.output_type = KeplerianType.MeanBrouwerXP
    bls.estimate_srp = True
    bls.solve()

    assert truth_sat.keplerian_state
    start = truth_sat.keplerian_state.epoch

    end = start + TimeSpan.from_days(1.0)
    step = TimeSpan.from_minutes(5.0)
    next_epoch = start
    bls_sat = bls.current_estimate
    ranges = []
    while next_epoch <= end:
        state = bls_sat.get_relative_state_at_epoch(truth_sat, next_epoch)
        assert state is not None
        ranges.append(state.position.magnitude)
        next_epoch += step

    assert bls.rms
    assert bls.rms < 0.2
    assert max(ranges) < 0.5
