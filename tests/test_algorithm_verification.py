# Algorithmic verification tests created during the 2026-07 test audit.
#
# Oracles used:
#   1. python-sgp4 (pip install sgp4) — Brandon Rhodes' wrapper of the official
#      Vallado SGP4 reference code, validated against the published AIAA 2006-6753
#      verification suite. Used to cross-check SAAL-backed propagation.
#   2. Vallado, "Fundamentals of Astrodynamics and Applications" 4th ed. —
#      published worked examples (rv2coe Example 2-6, GMST Example 3-5).
#   3. Public leap-second table (IERS) for UTC->TAI offsets.
#   4. Exact analytic geometry for vector math and simulated close approaches.
#
# TLE data comes from the public Celestrak/space-track snapshots already
# committed under tests/.

import math
import os

import pytest
from sgp4.api import Satrec, jday

from keplemon.elements import (
    CartesianState,
    CartesianVector,
    Ephemeris,
    EquinoctialElements,
    KeplerianElements,
    KeplerianState,
    SphericalVector,
    TLE,
)
from keplemon.enums import KeplerianType, ReferenceFrame, TimeSystem
from keplemon.time import Epoch, TimeSpan

HERE = os.path.dirname(os.path.abspath(__file__))

# Public ISS TLE already used by the existing Rust test-suite
ISS_LINE_1 = "1 25544U 98067A   20200.51605324 +.00000884  00000 0  22898-4 0 0999"
ISS_LINE_2 = "2 25544  51.6443  93.0000 0001400  84.0000 276.0000 15.4930007023660"


# ---------------------------------------------------------------------------
# helpers
# ---------------------------------------------------------------------------

def _epoch_from_ymdhms(year, mon, day, hr, minute, sec):
    return Epoch.from_iso(
        f"{year:04d}-{mon:02d}-{day:02d}T{hr:02d}:{minute:02d}:{sec:09.6f}Z",
        TimeSystem.UTC,
    )


def _propagate_with_python_sgp4(line_1, line_2, year, mon, day, hr, minute, sec):
    sat = Satrec.twoline2rv(line_1, line_2)
    jd, fr = jday(year, mon, day, hr, minute, sec)
    error, r, v = sat.sgp4(jd, fr)
    assert error == 0, f"python-sgp4 error code {error}"
    return r, v


def _tle_checksum(line):
    total = 0
    for ch in line[:68]:
        if ch.isdigit():
            total += int(ch)
        elif ch == "-":
            total += 1
    return total % 10


# ---------------------------------------------------------------------------
# 1. SGP4 propagation cross-validation (near-earth) vs python-sgp4
# ---------------------------------------------------------------------------

class TestSgp4CrossValidation:
    # Near-earth (period < 225 min): SAAL and the public reference code follow
    # the same AIAA 2006-6753 algorithm, so agreement should be sub-meter.
    POS_TOL_KM = 1e-3
    VEL_TOL_KMS = 1e-6

    @pytest.mark.parametrize(
        "minutes_after",
        [0.0, 45.0, 90.0, 360.0, 1440.0],
        ids=lambda m: f"t+{m:g}min",
    )
    def test_iss_vs_reference_implementation(self, minutes_after):
        tle = TLE.from_lines(ISS_LINE_1, ISS_LINE_2)
        base = (2020, 7, 18, 12, 0, 0.0)
        jd0, fr0 = jday(*base)

        epoch = _epoch_from_ymdhms(*base) + TimeSpan.from_minutes(minutes_after)
        state = tle.get_state_at_epoch(epoch)
        assert state.frame == ReferenceFrame.TEME

        sat = Satrec.twoline2rv(ISS_LINE_1, ISS_LINE_2)
        error, r_ref, v_ref = sat.sgp4(jd0, fr0 + minutes_after / 1440.0)
        assert error == 0

        for i, axis in enumerate("xyz"):
            assert getattr(state.position, axis) == pytest.approx(r_ref[i], abs=self.POS_TOL_KM), (
                f"TEME position {axis} mismatch vs reference SGP4"
            )
            assert getattr(state.velocity, axis) == pytest.approx(v_ref[i], abs=self.VEL_TOL_KMS)

    def test_celestrak_catalog_sample_vs_reference(self):
        """Cross-check a sample of real near-earth catalog TLEs at epoch +90 min."""
        path = os.path.join(HERE, "2025-04-15-celestrak.tle")
        with open(path) as f:
            lines = [ln.rstrip("\n") for ln in f if ln.strip()]

        checked = 0
        i = 0
        while i + 1 < len(lines) and checked < 20:
            l1, l2 = lines[i], lines[i + 1]
            i += 2
            if not (l1.startswith("1 ") and l2.startswith("2 ")):
                i -= 1
                continue
            mean_motion = float(l2[52:63])
            if mean_motion < 6.4:  # deep-space handled separately
                continue

            sat = Satrec.twoline2rv(l1, l2)
            # 90 minutes past TLE epoch
            jd, fr = sat.jdsatepoch, sat.jdsatepochF + 90.0 / 1440.0
            error, r_ref, v_ref = sat.sgp4(jd, fr)
            if error != 0:
                continue

            tle = TLE.from_lines(l1, l2)
            epoch = tle.epoch + TimeSpan.from_minutes(90.0)
            state = tle.get_state_at_epoch(epoch)

            dx = state.position.x - r_ref[0]
            dy = state.position.y - r_ref[1]
            dz = state.position.z - r_ref[2]
            miss = math.sqrt(dx * dx + dy * dy + dz * dz)
            assert miss < self.POS_TOL_KM, (
                f"NORAD {sat.satnum}: {miss * 1e3:.3f} m from reference SGP4"
            )
            checked += 1

        assert checked >= 10, f"only {checked} near-earth TLEs were cross-checked"

    def test_geo_vs_reference_implementation(self):
        """Deep-space (SDP4) cross-check with a real GEO TLE from the repo data."""
        line_1 = "1 37605U 11022A   25105.58543138  .00000096  00000+0  00000+0 0  9990"
        line_2 = "2 37605   1.0234  87.2060 0005091 220.8721 161.7206  1.00271635 50950"
        tle = TLE.from_lines(line_1, line_2)
        sat = Satrec.twoline2rv(line_1, line_2)

        for minutes_after in (0.0, 720.0):
            error, r_ref, _ = sat.sgp4(sat.jdsatepoch, sat.jdsatepochF + minutes_after / 1440.0)
            assert error == 0
            epoch = tle.epoch + TimeSpan.from_minutes(minutes_after)
            state = tle.get_state_at_epoch(epoch)
            dx = state.position.x - r_ref[0]
            dy = state.position.y - r_ref[1]
            dz = state.position.z - r_ref[2]
            miss = math.sqrt(dx * dx + dy * dy + dz * dz)
            # Deep-space implementations may diverge slightly; anything beyond
            # 100 m indicates an algorithmic difference worth investigating.
            assert miss < 0.1, f"GEO t+{minutes_after} min: {miss * 1e3:.1f} m from reference"


# ---------------------------------------------------------------------------
# 2. Cartesian -> Keplerian vs published textbook example (Vallado Ex. 2-6)
# ---------------------------------------------------------------------------

class TestCartesianToKeplerian:
    def test_vallado_rv2coe_example(self):
        epoch = Epoch.from_iso("2020-01-01T00:00:00.000000Z", TimeSystem.UTC)
        state = CartesianState(
            epoch,
            CartesianVector(6524.834, 6862.875, 6448.296),
            CartesianVector(4.901327, 5.533756, -1.976341),
            ReferenceFrame.TEME,
        )
        kep = state.to_keplerian()

        # Published solution (Vallado 4th ed., Example 2-6):
        #   a = 36127.343 km, e = 0.832853, i = 87.870 deg,
        #   raan = 227.898 deg, argp = 53.38 deg, true anomaly = 92.335 deg
        # Allow for the WGS-72 vs WGS-84 gravitational-parameter difference.
        assert kep.semi_major_axis == pytest.approx(36127.343, abs=0.5)
        assert kep.eccentricity == pytest.approx(0.832853, abs=1e-4)
        assert kep.inclination == pytest.approx(87.870, abs=0.01)
        assert kep.raan == pytest.approx(227.898, abs=0.01)
        assert kep.argument_of_perigee == pytest.approx(53.38, abs=0.02)

        # Convert the returned mean anomaly to true anomaly and compare
        e = kep.eccentricity
        m = math.radians(kep.mean_anomaly)
        ea = m
        for _ in range(60):
            ea = ea - (ea - e * math.sin(ea) - m) / (1.0 - e * math.cos(ea))
        nu = 2.0 * math.atan2(
            math.sqrt(1 + e) * math.sin(ea / 2.0),
            math.sqrt(1 - e) * math.cos(ea / 2.0),
        )
        assert math.degrees(nu) % 360.0 == pytest.approx(92.335, abs=0.05)

    @pytest.mark.parametrize(
        "a, e, i, raan, argp, ma",
        [
            (6798.0, 0.0002, 51.64, 93.0, 84.0, 276.0),   # LEO, near-circular
            (26562.0, 0.7411, 63.4, 120.0, 270.0, 10.0),  # Molniya, high e
            (42164.2, 0.0004, 0.05, 75.0, 200.0, 100.0),  # GEO, near-equatorial
            (7178.0, 0.001, 98.6, 250.0, 90.0, 200.0),    # SSO, retrograde
            (24400.0, 0.73, 5.0, 10.0, 178.0, 350.0),     # GTO
        ],
        ids=["leo", "molniya", "geo", "sso", "gto"],
    )
    def test_keplerian_cartesian_round_trip(self, a, e, i, raan, argp, ma):
        epoch = Epoch.from_iso("2020-01-01T00:00:00.000000Z", TimeSystem.UTC)
        kep = KeplerianState(
            epoch,
            KeplerianElements(a, e, i, raan, argp, ma),
            ReferenceFrame.TEME,
            KeplerianType.Osculating,
        )
        cart = kep.to_cartesian()
        back = cart.to_keplerian()

        assert back.semi_major_axis == pytest.approx(a, abs=1e-6 * a)
        assert back.eccentricity == pytest.approx(e, abs=1e-8)
        assert back.inclination == pytest.approx(i, abs=1e-6)
        assert back.raan == pytest.approx(raan, abs=1e-5)
        # argp/ma are individually ill-conditioned at low eccentricity;
        # their sum is always well-defined.
        assert (back.argument_of_perigee + back.mean_anomaly) % 360.0 == pytest.approx(
            (argp + ma) % 360.0, abs=1e-4
        )
        if e > 0.01:
            assert back.argument_of_perigee == pytest.approx(argp, abs=1e-4)
            assert back.mean_anomaly == pytest.approx(ma, abs=1e-4)

        # and cartesian round trip
        cart_back = back.to_cartesian()
        for axis in "xyz":
            assert getattr(cart_back.position, axis) == pytest.approx(
                getattr(cart.position, axis), abs=1e-6
            )


# ---------------------------------------------------------------------------
# 3. Equinoctial elements (previously zero coverage)
# ---------------------------------------------------------------------------

class TestEquinoctialElements:
    def test_to_keplerian_against_hand_formulas(self):
        # Standard equinoctial definitions:
        #   af = e*cos(argp + raan), ag = e*sin(argp + raan)
        #   chi = tan(i/2)*sin(raan), psi = tan(i/2)*cos(raan)
        #   mean_longitude = ma + argp + raan
        e, i, raan, argp, ma = 0.1, 45.0, 30.0, 60.0, 45.0
        n = 2.0  # revs/day

        af = e * math.cos(math.radians(argp + raan))
        ag = e * math.sin(math.radians(argp + raan))
        chi = math.tan(math.radians(i / 2)) * math.sin(math.radians(raan))
        psi = math.tan(math.radians(i / 2)) * math.cos(math.radians(raan))
        mean_longitude = (ma + argp + raan) % 360.0

        eq = EquinoctialElements(af, ag, chi, psi, mean_longitude, n)
        kep = eq.to_keplerian()

        assert kep.eccentricity == pytest.approx(e, abs=1e-9)
        assert kep.inclination == pytest.approx(i, abs=1e-7)
        assert kep.raan == pytest.approx(raan, abs=1e-7)
        assert kep.argument_of_perigee == pytest.approx(argp, abs=1e-6)
        assert kep.mean_anomaly == pytest.approx(ma, abs=1e-6)


# ---------------------------------------------------------------------------
# 4. Time systems against public references
# ---------------------------------------------------------------------------

class TestTimeSystems:
    @pytest.mark.parametrize(
        "iso, tai_minus_utc",
        [
            ("2014-06-01T00:00:00.000000Z", 35.0),
            ("2015-07-01T00:00:00.000000Z", 36.0),
            ("2017-01-01T00:00:00.000000Z", 37.0),
            ("2024-01-01T00:00:00.000000Z", 37.0),
        ],
    )
    def test_utc_to_tai_leap_seconds(self, iso, tai_minus_utc):
        """UTC->TAI offsets from the public IERS leap-second table."""
        utc = Epoch.from_iso(iso, TimeSystem.UTC)
        tai = utc.to_system(TimeSystem.TAI)
        offset_seconds = (tai.days_since_1950 - utc.days_since_1950) * 86400.0
        assert offset_seconds == pytest.approx(tai_minus_utc, abs=1e-6)

    def test_gmst_vallado_example_3_5(self):
        """Vallado 4th ed. Example 3-5: GMST at 1992-08-20 12:14:00 UT1.

        Input is given as UTC because a UT1-system epoch currently panics
        (see test_greenwich_angle_from_ut1_epoch_should_not_panic). The
        tolerance allows for |UT1-UTC| < 0.9 s (~0.004 deg of rotation).
        """
        utc = Epoch.from_iso("1992-08-20T12:14:00.000000Z", TimeSystem.UTC)
        gmst_deg = math.degrees(utc.to_fk5_greenwich_angle()) % 360.0
        assert gmst_deg == pytest.approx(152.578788, abs=5e-3)

    @pytest.mark.xfail(
        reason="BUG: Epoch.to_system(UT1->UT1) identity conversion is unimplemented and "
        "get_gst/to_fk4_greenwich_angle/to_fk5_greenwich_angle unwrap() the error, so any "
        "UT1- or TT-system epoch raises PanicException instead of returning a value "
        "(src/time/epoch.rs:83-85,153-160,186-214)",
        raises=BaseException,
        strict=True,
    )
    def test_greenwich_angle_from_ut1_epoch_should_not_panic(self):
        ut1 = Epoch.from_iso("1992-08-20T12:14:00.000000Z", TimeSystem.UT1)
        gmst_deg = math.degrees(ut1.to_fk5_greenwich_angle()) % 360.0
        assert gmst_deg == pytest.approx(152.578788, abs=5e-3)

    def test_dtg_round_trip(self):
        epoch = Epoch.from_iso("2025-04-15T12:32:28.532000Z", TimeSystem.UTC)
        tolerances_s = {"to_dtg_20": 1e-6, "to_dtg_19": 1e-6, "to_dtg_17": 1e-3, "to_dtg_15": 1e-6}
        for fmt, tol_s in tolerances_s.items():
            dtg = getattr(epoch, fmt)()
            back = Epoch.from_dtg(dtg, TimeSystem.UTC)
            err_s = abs(back.days_since_1950 - epoch.days_since_1950) * 86400.0
            assert err_s <= tol_s, f"{fmt} round-trip error {err_s} s"

    def test_epoch_arithmetic_consistency(self):
        epoch = Epoch.from_iso("2020-02-29T12:00:00.000000Z", TimeSystem.UTC)  # leap day
        assert (epoch + TimeSpan.from_days(1.0)).to_iso().startswith("2020-03-01T12:00:00")
        assert (epoch - TimeSpan.from_hours(13.0)).to_iso().startswith("2020-02-28T23:00:00")


# ---------------------------------------------------------------------------
# 5. TLE parsing, serialization, checksums, year windowing
# ---------------------------------------------------------------------------

class TestTleFormat:
    def test_round_trip_lines(self):
        tle = TLE.from_lines(ISS_LINE_1, ISS_LINE_2)
        l1, l2 = tle.lines
        reparsed = TLE.from_lines(l1, l2)
        assert reparsed.norad_id == tle.norad_id
        state_a = tle.get_state_at_epoch(tle.epoch)
        state_b = reparsed.get_state_at_epoch(tle.epoch)
        assert state_a.position.x == pytest.approx(state_b.position.x, abs=1e-9)

    def test_serialized_lines_have_valid_checksums(self):
        tle = TLE.from_lines(ISS_LINE_1, ISS_LINE_2)
        l1, l2 = tle.lines
        for line in (l1, l2):
            assert len(line) == 69
            assert int(line[68]) == _tle_checksum(line), f"bad checksum on: {line}"

    @pytest.mark.parametrize(
        "yy, expected_year",
        [
            ("98", 1998),
            ("00", 2000),
            ("25", 2025),
            ("49", 2049),
            ("57", 1957),
            pytest.param(
                "56",
                2056,
                marks=pytest.mark.xfail(
                    reason="DIVERGENCE: SAAL maps yy=56 to 1956 (pivot at 1956) while the "
                    "de-facto TLE standard and the reference implementation map 00-56 to "
                    "20xx; yy=50..55 raise ValueError (SAAL requires year >= 1956). "
                    "Becomes user-visible for epochs from 2050 on.",
                    strict=True,
                ),
            ),
        ],
    )
    def test_epoch_year_windowing(self, yy, expected_year):
        """Standard TLE convention: years 57-99 map to 19xx, 00-56 to 20xx."""
        line_1 = f"1 25544U 98067A   {yy}200.51605324 +.00000884  00000 0  22898-4 0 0990"
        line_1 = line_1[:68] + str(_tle_checksum(line_1))
        tle = TLE.from_lines(line_1, ISS_LINE_2)
        assert tle.epoch.to_iso().startswith(str(expected_year)), (
            f"epoch year {yy} should map to {expected_year}, got {tle.epoch.to_iso()}"
        )

    def test_python_sgp4_agrees_on_parsed_fields(self):
        sat = Satrec.twoline2rv(ISS_LINE_1, ISS_LINE_2)
        tle = TLE.from_lines(ISS_LINE_1, ISS_LINE_2)
        assert tle.norad_id == sat.satnum
        assert tle.inclination == pytest.approx(math.degrees(sat.inclo), abs=1e-6)
        assert tle.eccentricity == pytest.approx(sat.ecco, abs=1e-9)
        assert tle.raan == pytest.approx(math.degrees(sat.nodeo), abs=1e-6)


# ---------------------------------------------------------------------------
# 6. Vector math (previously zero coverage) — exact analytic references
# ---------------------------------------------------------------------------

class TestVectorMath:
    def test_distance_3_4_5(self):
        assert CartesianVector(0, 0, 0).distance(CartesianVector(3, 4, 0)) == pytest.approx(5.0)

    def test_orthogonal_angle(self):
        angle = CartesianVector(1, 0, 0).angle(CartesianVector(0, 1, 0))
        # accept either radians or degrees convention, but it must be exactly 90 deg
        assert angle == pytest.approx(math.pi / 2, abs=1e-12) or angle == pytest.approx(90.0, abs=1e-9)

    def test_spherical_round_trip(self):
        v = SphericalVector(7000.0, 45.0, 30.0)
        cart = v.to_cartesian()
        back = cart.to_spherical()
        assert back.range == pytest.approx(7000.0, abs=1e-9)
        assert back.right_ascension == pytest.approx(45.0, abs=1e-9)
        assert back.declination == pytest.approx(30.0, abs=1e-9)

    def test_spherical_to_cartesian_reference(self):
        # r=1, ra=0, dec=0 -> (1, 0, 0); r=1, ra=90, dec=0 -> (0, 1, 0)
        c = SphericalVector(1.0, 0.0, 0.0).to_cartesian()
        assert (c.x, c.y, c.z) == pytest.approx((1.0, 0.0, 0.0), abs=1e-12)
        c = SphericalVector(1.0, 90.0, 0.0).to_cartesian()
        assert (c.x, c.y, c.z) == pytest.approx((0.0, 1.0, 0.0), abs=1e-12)


# ---------------------------------------------------------------------------
# 7. Close-approach detection with simulated ephemerides (exact geometry)
# ---------------------------------------------------------------------------

def _linear_ephemeris(sat_id, norad_id, t0, duration_s, step_s, pos0, vel):
    """Ephemeris for straight-line motion: exact under Hermite interpolation."""
    def state_at(dt):
        pos = CartesianVector(
            pos0[0] + vel[0] * dt, pos0[1] + vel[1] * dt, pos0[2] + vel[2] * dt
        )
        return CartesianState(
            t0 + TimeSpan.from_seconds(dt),
            pos,
            CartesianVector(*vel),
            ReferenceFrame.TEME,
        )

    ephem = Ephemeris(sat_id, norad_id, state_at(0.0))
    dt = step_s
    while dt <= duration_s:
        ephem.add_state(state_at(dt))
        dt += step_s
    return ephem


class TestCloseApproachSimulated:
    def test_known_miss_distance_and_epoch(self):
        t0 = Epoch.from_iso("2025-01-01T00:00:00.000000Z", TimeSystem.UTC)
        # A moves along +x through the origin at t = 1200 s; B is offset 5 km in y.
        ephem_a = _linear_ephemeris("SAT-A", 90001, t0, 2400.0, 60.0, (-8.4, 0.0, 0.0), (0.007, 0.0, 0.0))
        ephem_b = _linear_ephemeris("SAT-B", 90002, t0, 2400.0, 60.0, (0.0, 5.0, 0.0), (0.0, 0.0, 0.0))

        ca = ephem_a.get_close_approach(ephem_b, 25.0)
        assert ca is not None, "close approach with 5 km miss below 25 km threshold was not detected"
        assert ca.distance == pytest.approx(5.0, abs=1e-3)
        tca_offset = (ca.epoch - t0).in_seconds()
        assert tca_offset == pytest.approx(1200.0, abs=1.0)

    def test_above_threshold_returns_none(self):
        t0 = Epoch.from_iso("2025-01-01T00:00:00.000000Z", TimeSystem.UTC)
        ephem_a = _linear_ephemeris("SAT-A", 90001, t0, 2400.0, 60.0, (-8.4, 0.0, 0.0), (0.007, 0.0, 0.0))
        ephem_b = _linear_ephemeris("SAT-B", 90002, t0, 2400.0, 60.0, (0.0, 50.0, 0.0), (0.0, 0.0, 0.0))
        ca = ephem_a.get_close_approach(ephem_b, 25.0)
        assert ca is None

    @pytest.mark.xfail(
        reason="BUG: Ephemeris.get_close_approach derives the scan window from self only "
        "(src/elements/ephemeris.rs:393) and out-of-span interpolation clamps to the "
        "boundary state keeping its own epoch (ephemeris.rs:793-798), which trips the "
        "epoch-equality guard in estimate_close_approach_epoch (ephemeris.rs:563) and "
        "silently breaks the scan loop (ephemeris.rs:442-444). A conjunction inside the "
        "overlapping span is dropped with no error.",
        strict=True,
    )
    def test_shorter_secondary_ephemeris_is_not_silently_truncated(self):
        """If the secondary ephemeris does not cover the full screening window,
        the screening should either error or still detect an approach inside the
        overlapping span — not silently return None because of the coverage gap.

        Documents a suspected defect: Ephemeris.get_close_approach derives the
        scan window from self only and breaks out of the scan loop on the first
        interpolation failure, so a conjunction occurring after the end of the
        secondary's span is silently missed (no error is raised).
        """
        t0 = Epoch.from_iso("2025-01-01T00:00:00.000000Z", TimeSystem.UTC)
        # TCA at 1200 s with 5 km miss, well inside BOTH spans here:
        ephem_a = _linear_ephemeris("SAT-A", 90001, t0, 2400.0, 60.0, (-8.4, 0.0, 0.0), (0.007, 0.0, 0.0))
        # Secondary covers only the first 900 s (ends before the coarse grid does)
        ephem_b = _linear_ephemeris("SAT-B", 90002, t0, 900.0, 60.0, (0.0, 5.0, 0.0), (0.0, 0.0, 0.0))

        # The distance at t=900 s is sqrt(2.1^2 + 5^2) ~ 5.42 km — already inside
        # the 25 km threshold within the overlapping span, so a correct
        # implementation reports an approach (or raises on partial coverage).
        ca = ephem_a.get_close_approach(ephem_b, 25.0)
        assert ca is not None, (
            "approach inside the overlapping span was dropped because the "
            "secondary ephemeris is shorter than the primary"
        )


# ---------------------------------------------------------------------------
# 8. Ephemeris interpolation sanity (simulated data)
# ---------------------------------------------------------------------------

class TestEphemerisInterpolation:
    def test_linear_motion_interpolates_exactly(self):
        t0 = Epoch.from_iso("2025-01-01T00:00:00.000000Z", TimeSystem.UTC)
        ephem = _linear_ephemeris("SAT-A", 90001, t0, 1200.0, 60.0, (100.0, -3.0, 7.0), (0.5, 0.25, -0.125))
        probe = t0 + TimeSpan.from_seconds(90.0)  # midway between grid points
        state = ephem.get_state_at_epoch(probe)
        assert state is not None
        assert state.position.x == pytest.approx(100.0 + 0.5 * 90.0, abs=1e-9)
        assert state.position.y == pytest.approx(-3.0 + 0.25 * 90.0, abs=1e-9)
        assert state.position.z == pytest.approx(7.0 - 0.125 * 90.0, abs=1e-9)

    @pytest.mark.xfail(
        reason="BUG-CANDIDATE: interpolate_state_with_grid clamps out-of-span queries to "
        "the boundary state (src/elements/ephemeris.rs:793-798) instead of returning None, "
        "so Ephemeris.get_state_at_epoch silently answers with a state at a different "
        "epoch than requested (also: get_state_at_epoch is missing from the .pyi stubs).",
        strict=True,
    )
    def test_query_outside_span_returns_none(self):
        t0 = Epoch.from_iso("2025-01-01T00:00:00.000000Z", TimeSystem.UTC)
        ephem = _linear_ephemeris("SAT-A", 90001, t0, 600.0, 60.0, (1.0, 2.0, 3.0), (0.0, 0.0, 0.0))
        outside = t0 + TimeSpan.from_seconds(3600.0)
        state = ephem.get_state_at_epoch(outside)
        assert state is None or state.epoch.days_since_1950 == outside.days_since_1950
