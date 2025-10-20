import pytest
import json
from keplemon.bodies import Observatory, Sensor, Satellite
from keplemon.estimation import Observation, BatchLeastSquares
from keplemon.time import Epoch
from keplemon.enums import TimeSystem, KeplerianType
from keplemon.elements import TopocentricElements, TLE


def test_raise_od_exception():

    with open("tests/gh-issue-000006.json", "r") as f:
        issue_data = json.load(f)

    # Update your a priori state here
    tle = TLE.from_lines(issue_data["line_1"], issue_data["line_2"])
    sat = Satellite.from_tle(tle)

    keplemon_obs = []

    # This can be updated per observation to reflect actual sensor capabilities
    dummy_sensor = Sensor(angular_noise=0.001)

    for json_ob in issue_data["obs"]:

        # Keys here are only available because the input example data is UDL-formatted
        epoch = Epoch.from_iso(json_ob["epoch"], TimeSystem.UTC)

        site = Observatory(issue_data["sensor"]["lat"], issue_data["sensor"]["lon"], issue_data["sensor"]["alt"])
        observer_teme = site.get_state_at_epoch(epoch)

        # KepLemon expects TEME.  The `from_j2000` constructor will be needed for most observations
        teme_topo = TopocentricElements.from_j2000(epoch, json_ob["ra"], json_ob["dec"])

        ob = Observation(dummy_sensor, epoch, teme_topo, observer_teme.position)
        keplemon_obs.append(ob)

    # Create the BLS object
    bls = BatchLeastSquares(keplemon_obs, sat)

    # You can override the output type if you'd like to use a more accurate model than basic SGP4
    bls.output_type = KeplerianType.MeanBrouwerXP

    # This will iterate until the weighted RMS tolerance is achieved OR max iterations is achieved
    with pytest.raises(RuntimeError):
        bls.solve()
