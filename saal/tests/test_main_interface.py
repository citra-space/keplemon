from pathlib import Path
import tempfile

import pytest

from pysaal import MainInterface


def test_get_dll_info() -> None:
    interface = MainInterface()
    assert MainInterface.DLL_VERSION in interface.info


def test_get_key_mode() -> None:
    interface = MainInterface()
    assert interface.key_mode == 1


def test_set_duplicate_key_mode_return_key() -> None:
    interface = MainInterface()
    interface.duplicate_key_mode = 1
    assert interface.duplicate_key_mode == 1
    interface.reset_key_mode()


def test_get_duplicate_key_mode_return_zero() -> None:
    interface = MainInterface()
    interface.duplicate_key_mode = 0
    assert interface.duplicate_key_mode == 0
    interface.reset_key_mode()


def test_load_from_file_missing() -> None:
    missing_path = Path(tempfile.gettempdir()) / "saal_missing_input.txt"
    if missing_path.exists():
        missing_path.unlink()
    with pytest.raises(RuntimeError):
        MainInterface(str(missing_path))
