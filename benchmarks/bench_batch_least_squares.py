import json
import time
from pathlib import Path

from keplemon.bodies import Observatory, Satellite, Sensor
from keplemon.elements import TLE, TopocentricElements
from keplemon.enums import KeplerianType, TimeSystem
from keplemon.estimation import BatchLeastSquares, Observation
from keplemon.time import Epoch


def load_observations(path: Path) -> list[Observation]:
    with path.open("r") as handle:
        json_obs = json.load(handle)

    observations: list[Observation] = []
    for json_ob in json_obs:
        epoch = Epoch.from_iso(json_ob["epoch"], TimeSystem.UTC)
        site = Observatory(
            json_ob["sensor_latitude"],
            json_ob["sensor_longitude"],
            json_ob["sensor_altitude"],
        )
        els = TopocentricElements(json_ob["ra"], json_ob["dec"])
        sensor = Sensor(json_ob["angular_noise"])
        ob = Observation(sensor, epoch, els, site.get_state_at_epoch(epoch).position)
        observations.append(ob)
    return observations


def build_bls(observations: list[Observation], initial_sat: Satellite) -> BatchLeastSquares:
    bls = BatchLeastSquares(observations, initial_sat)
    bls.estimate_srp = True
    bls.output_type = KeplerianType.MeanBrouwerXP
    return bls


def bench(iterations: int = 5) -> None:
    root = Path(__file__).resolve().parents[1]
    observations = load_observations(root / "local" / "test-observations.json")
    initial_tle = TLE.from_lines(
        "1 99999U          25334.80826079 -.00000092  00000 0  00000 0 0 0000",
        "2 99999   5.1462  74.9949 0001499 136.0805 318.9951  0.9987069300000",
    )
    initial_sat = Satellite.from_tle(initial_tle)

    durations: list[float] = []
    for _ in range(iterations):
        bls = build_bls(observations, initial_sat)
        start = time.perf_counter()
        bls.solve()
        durations.append(time.perf_counter() - start)

    avg = sum(durations) / len(durations)
    print(f"batch_least_squares.solve avg={avg:.6f}s min={min(durations):.6f}s")


if __name__ == "__main__":
    bench()
