from keplemon.estimation import Observation


def test_from_saal_files() -> None:
    observations = Observation.from_saal_files("tests/sensors.dat", "tests/test-b3-obs.txt")
    assert len(observations) == 5053
