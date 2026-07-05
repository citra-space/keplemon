# Error-handling regression tests created during the 2026-07 test audit.
#
# Each xfail-marked test documents a confirmed defect where invalid input or
# state escalates to a Rust panic (pyo3 PanicException) or a silent wrong
# answer instead of a typed Python exception. When a defect is fixed, the
# strict xfail flips to XPASS and the marker should be removed.
#
# Panic-triggering calls run in a subprocess where practical so an abort can
# never take down the host test run; PanicException-based ones are safe to
# call in-process (pyo3 converts the panic).

import math
import subprocess
import sys
import textwrap

import pytest

from keplemon.elements import (
    CartesianState,
    CartesianVector,
    KeplerianElements,
    KeplerianState,
    TLE,
)
from keplemon.enums import KeplerianType, ReferenceFrame, TimeSystem
from keplemon.time import Epoch

ISS_LINE_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999"
ISS_LINE_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660"


def _run_isolated(code):
    return subprocess.run(
        [sys.executable, "-c", textwrap.dedent(code)],
        capture_output=True,
        text=True,
        timeout=120,
    )


class TestPanicsInsteadOfExceptions:
    """Library policy (agents.yaml) is no-panics-in-lib; these currently panic."""

    @pytest.mark.xfail(
        reason="BUG: Epoch.from_iso with a malformed string panics (index out of "
        "bounds) instead of raising ValueError",
        raises=BaseException,
        strict=True,
    )
    def test_from_iso_invalid_string_raises_value_error(self):
        with pytest.raises(ValueError):
            Epoch.from_iso("not-a-date", TimeSystem.UTC)

    @pytest.mark.xfail(
        reason="BUG: set_thread_count panics with GlobalPoolAlreadyInitialized if "
        "called more than once per process (src/lib.rs build_global().unwrap()); "
        "second call should be a no-op or raise a typed error",
        raises=BaseException,
        strict=True,
    )
    def test_set_thread_count_twice_is_safe(self):
        proc = _run_isolated(
            """
            import keplemon
            keplemon.set_thread_count(2)
            keplemon.set_thread_count(4)
            print("OK")
            """
        )
        assert proc.returncode == 0 and "OK" in proc.stdout, proc.stderr[-500:]

    @pytest.mark.xfail(
        reason="BUG: TLECatalog.from_tle_file on an empty file panics (index out of "
        "bounds: len 0 index 1 in tle_catalog.rs chunk framing) instead of returning "
        "an empty catalog or raising ValueError",
        raises=BaseException,
        strict=True,
    )
    def test_empty_catalog_file(self, tmp_path):
        from keplemon.catalogs import TLECatalog

        empty = tmp_path / "empty.tle"
        empty.write_text("")
        catalog = TLECatalog.from_tle_file(str(empty))
        assert catalog.count == 0

    @pytest.mark.xfail(
        reason="BUG: to_fk5_greenwich_angle/gst on a UT1- or TT-system epoch panics "
        "because Epoch.to_system lacks identity/UT1/TT source conversions and the "
        "result is unwrap()ed (src/time/epoch.rs:83-85,153-160,186-214)",
        raises=BaseException,
        strict=True,
    )
    def test_gst_from_ut1_epoch(self):
        ut1 = Epoch.from_iso("1992-08-20T12:14:00.000000Z", TimeSystem.UT1)
        angle = ut1.gst
        assert 0.0 <= angle < 2.0 * math.pi


class TestSilentInvalidResults:
    """Invalid orbital inputs currently return NaN silently. NaN states poison
    downstream comparisons (e.g. a NaN close-approach distance compares false
    against every threshold, so conjunctions are silently dropped)."""

    @pytest.mark.xfail(
        reason="BUG-CANDIDATE: hyperbolic elements (e>1) silently produce NaN "
        "cartesian states instead of raising a typed error",
        strict=True,
    )
    def test_hyperbolic_elements_raise_or_return_finite(self):
        epoch = Epoch.from_iso("2020-01-01T00:00:00.000000Z", TimeSystem.UTC)
        state = KeplerianState(
            epoch,
            KeplerianElements(-26562.0, 1.2, 30.0, 10.0, 20.0, 30.0),
            ReferenceFrame.TEME,
            KeplerianType.Osculating,
        )
        cart = state.to_cartesian()
        assert math.isfinite(cart.position.x), "NaN position returned with no error"

    @pytest.mark.xfail(
        reason="BUG-CANDIDATE: a NaN cartesian state converts to NaN keplerian "
        "elements silently instead of raising a typed error",
        strict=True,
    )
    def test_nan_cartesian_input_raises(self):
        epoch = Epoch.from_iso("2020-01-01T00:00:00.000000Z", TimeSystem.UTC)
        state = CartesianState(
            epoch,
            CartesianVector(float("nan"), 0.0, 0.0),
            CartesianVector(0.0, 0.0, 0.0),
            ReferenceFrame.TEME,
        )
        kep = state.to_keplerian()
        assert math.isfinite(kep.semi_major_axis)


class TestPermissiveParsing:
    def test_bad_checksum_is_currently_accepted(self):
        """Documents current behavior: TLE line-1 checksum violations are accepted
        silently (SAAL does not validate the trailing checksum digit). If checksum
        validation is added, make it opt-in and update this test."""
        bad_line_1 = ISS_LINE_1[:68] + "1"  # correct checksum is 9
        tle = TLE.from_lines(bad_line_1, ISS_LINE_2)
        assert tle.norad_id == 25544

    def test_malformed_line_raises_value_error(self):
        with pytest.raises(ValueError):
            TLE.from_lines("1 25544U", "2 25544")
