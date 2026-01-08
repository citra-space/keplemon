from pathlib import Path
import tempfile

import pytest

from pysaal import EnvironmentInterface, MainInterface


def test_get_dll_info() -> None:
    interface = EnvironmentInterface()
    assert MainInterface.DLL_VERSION in interface.info


def test_get_earth_radius() -> None:
    interface = EnvironmentInterface()
    assert interface.earth_radius == 6378.135


def test_get_fundamental_catalog() -> None:
    interface = EnvironmentInterface()
    assert interface.fundamental_catalog == 5


def test_get_j2() -> None:
    interface = EnvironmentInterface()
    assert interface.j2 == 0.001082616


def test_get_j3() -> None:
    interface = EnvironmentInterface()
    assert interface.j3 == -0.00000253881


def test_get_j4() -> None:
    interface = EnvironmentInterface()
    assert interface.j4 == -0.00000165597


def test_get_j5() -> None:
    interface = EnvironmentInterface()
    assert interface.j5 == -2.184827e-7


def test_get_earth_mu() -> None:
    interface = EnvironmentInterface()
    assert interface.earth_mu == 398600.8


def test_get_earth_flattening() -> None:
    interface = EnvironmentInterface()
    assert interface.earth_flattening == 1.0 / 298.26


def test_get_earth_rotation_rate() -> None:
    interface = EnvironmentInterface()
    assert interface.earth_rotation_rate == 0.017202791694070362


def test_get_earth_rotation_acceleration() -> None:
    interface = EnvironmentInterface()
    assert interface.earth_rotation_acceleration == 5.075514194322695e-15


def test_set_fundamental_catalog() -> None:
    interface = EnvironmentInterface()
    interface.fundamental_catalog = 4
    assert interface.fundamental_catalog == 4
    interface.fundamental_catalog = 5


def test_load_from_file_missing() -> None:
    missing_path = Path(tempfile.gettempdir()) / "saal_missing_env.txt"
    if missing_path.exists():
        missing_path.unlink()
    with pytest.raises(RuntimeError):
        EnvironmentInterface(str(missing_path))
