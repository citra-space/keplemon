# Deep Space (SDP4) CUDA Implementation Debugging

This document tracks the investigation into the 12-22 km position errors between the CUDA GPU implementation and the reference CPU propagator (SAAL/python-sgp4) for deep space satellites.

## Summary

- **Current Status**: CUDA deep space shows 5-22 km errors vs CPU
- **Reference Propagator**: SAAL v1.3.3 (matches python-sgp4 to < 0.01 km)
- **Error Pattern**: Constant at t=0 (~18 km) → bug is in **propagation step**, not initialization
- **Gravity Model**: WGS-72 (both CUDA and python-sgp4)
- **Error Traced To**: Argument of latitude (`su`) differs by 0.048° — root cause still unknown
- **Last Updated**: January 17, 2026

---

## Refactoring Completed ✅

### Constants Consolidation
All CUDA constants are now in a single header file: `kernels/sgp4_constants.cuh`

The file is well-documented with:
- Clear WGS-72 designation (NOT WGS-84!)
- Comparison table of WGS-72 vs WGS-84 values
- Organized sections for mathematical, gravitational, deep space, and resonance constants
- Reference to python-sgp4 and Vallado papers

**Key WGS-72 Constants** (updated to match python-sgp4 exactly):
| Constant | WGS-72 Value | Notes |
|----------|--------------|-------|
| RE | 6378.135 km | Earth radius |
| J2 | 0.00108262998905 | Second zonal harmonic |
| J3 | -0.00000253215306 | Third zonal harmonic |
| J4 | -0.00000165597 | Fourth zonal harmonic |
| XKE | 0.0743669161 | sqrt(GM) in canonical units |
| MU | 398600.8 km³/s² | Gravitational parameter |

### Mean Anomaly 2π Offset (Not a Bug)

**Observation**: Mean anomaly differs by exactly 2π between implementations:
- CUDA: `mm = 4.2159707029 rad`
- python-sgp4: `mm = -2.0672146049 rad`
- Difference: `6.2831853078 = 2π`

**Conclusion**: This is a normalization difference, NOT a bug. Both values are mathematically equivalent since sin/cos are 2π-periodic. Verified:
- `sin(mm_cuda) = sin(mm_pysgp4)` ✅
- `cos(mm_cuda) = cos(mm_pysgp4)` ✅

---

## Current Investigation: Argument of Latitude Divergence 🔍

### Key Finding (January 17, 2026)

**python-sgp4 recalculates `aycof` and `xlcof` for deep space satellites during propagation!**

From python-sgp4 `propagation.py` (lines 156-165):
```python
if satrec.method == 'd':
    sinip = sin(xincp);
    cosip = cos(xincp);
    satrec.aycof = -0.5*satrec.j3oj2*sinip;
    if fabs(cosip+1.0) > 1.5e-12:
        satrec.xlcof = -0.25 * satrec.j3oj2 * sinip * (3.0 + 5.0 * cosip) / (1.0 + cosip);
```

**CUDA's current behavior**: Uses static `aycof`/`xlcof` from initialization
**python-sgp4's behavior**: Recalculates using dpper-modified inclination (`xincp`)

### Impact Analysis

The Z component error is consistently ~18 km:
- At t=0: CUDA Z = -460.14 km, python-sgp4 Z = -442.06 km (diff = 18.07 km)
- At t=10: CUDA Z = 1440.54 km, python-sgp4 Z = 1459.04 km (diff = 18.50 km)

The implied effective inclination difference is **~1.08 degrees**:
- CUDA effective xinc: 55.458°
- python-sgp4 implied: 56.540°

### Trace of Intermediate Values at t=+10 min

| Stage | Variable | CUDA | Expected | Status |
|-------|----------|------|----------|--------|
| Deep Space Secular | nm | 0.0087513189 | 0.0087513189 | ✅ |
| Deep Space Secular | em | 0.0123455901 | 0.0123455901 | ✅ |
| Deep Space Secular | inclm | 0.9679019662 | 0.9679019662 | ✅ |
| Deep Space Periodics | em_pp | 0.0123455805 | 0.0123455805 | ✅ |
| Deep Space Periodics | inclm_pp | 0.9679020102 | 0.9679020102 | ✅ |
| Deep Space Periodics | argpm_pp | 2.1547294661 | 2.1547294661 | ✅ |
| Long Period | axnl | -0.0068062372 | -0.0068062372 | ✅ |
| Long Period | aynl | 0.0105318918 | 0.0105318918 | ✅ |
| Kepler Solution | eo1 | 0.0764891879 | 0.0764891878 | ✅ |
| Short Period | su | 0.0655005374 | 0.0663416118* | ❌ |
| Short Period | xinc | 0.9679236989 | ~0.9868* | ❌ |
| Orientation | U_z | 0.0539138415 | 0.0546061866* | ❌ |

*Values back-calculated from python-sgp4's final position

### Argument of Latitude Divergence (Observed Symptom)

**Error Traced To**: The argument of latitude (`su`) calculation produces different results:

| Metric | CUDA | python-sgp4 | Difference |
|--------|------|-------------|------------|
| Implied sinuk | 0.0654562263 | 0.0662939486 | 0.00084 |
| Implied uk | 3.753° | 3.801° | **0.048°** |
| Unit vector U_z | 0.0539 | 0.0546 | 0.0007 |
| Radius magnitude | 26718.13 km | 26719.34 km | 1.21 km |
| Angle between U vectors | — | — | **0.0396°** |

The formula `su = atan2(sinu, cosu)` is identical in both implementations, so the divergence must be in the **inputs to sinu/cosu**:
```
sinu = am / rl * (sineo1 - aynl - axnl * temp)
cosu = am / rl * (coseo1 - axnl + aynl * temp)
```

**Suspect Variables** (need comparison):
- `sineo1`, `coseo1` - eccentric anomaly sin/cos
- `axnl`, `aynl` - long period periodic terms
- `temp` = esine / (1 + betal)
- `am`, `rl` - semi-major axis and radius

### Position Error Breakdown

```
Position Error at t=+10 min:
  X: -14649.11 vs -14645.23 = -3.88 km
  Y: -22299.19 vs -22300.48 = +1.29 km  
  Z:  1440.54 vs  1459.04  = -18.50 km  ← Main error!
  Total: 18.94 km
```

The error is dominated by the Z component, which depends on `sin(xinc) * sin(su)`.

### Fix Attempt: Deep Space aycof/xlcof/con41 Recalculation ❌

**Hypothesis**: python-sgp4 recalculates `aycof`, `xlcof`, `con41`, `x1mth2`, `x7thm1` for deep space satellites using the dpper-modified inclination.

**Implementation**: Added recalculation in `sgp4_batch.cu`:
```cuda
// Long period periodics - recalculate aycof/xlcof for deep space
if (p.is_deep_space) {
    aycof_eff = -0.5 * J3OJ2 * sinip;
    xlcof_eff = -0.25 * J3OJ2 * sinip * (3.0 + 5.0 * cosip) / (1.0 + cosip);
}

// Short period periodics - recalculate con41/x1mth2/x7thm1 for deep space
if (p.is_deep_space) {
    double cosisq = cosip * cosip;
    con41_eff = 3.0 * cosisq - 1.0;
    x1mth2_eff = 1.0 - cosisq;
    x7thm1_eff = 7.0 * cosisq - 1.0;
}
```

**Result**: No change in error (18.62 km → 18.62 km)

**Reason**: The dpper-modified inclination (`inclm_pp = 0.9679020102`) is essentially identical to the original (`inclo = 0.9679020073`). The difference of ~3e-9 rad is too small to cause the observed 18 km error.

### Constants Fix: J2/J3 Updated to Match WGS-72 ✅

**Issue Found**: CUDA was using slightly different J2/J3 values than python-sgp4.

| Constant | CUDA (old) | python-sgp4 (WGS-72) | Difference |
|----------|------------|----------------------|------------|
| J2 | 0.001082616 | 0.00108262998905 | -1e-8 |
| J3 | -2.53881e-6 | -2.53215306e-6 | -6.7e-9 |
| J3/J2 | -0.0023450697 | -0.0023388906 | -6.2e-5 |

**Fix Applied**: Updated `sgp4_constants.cuh` to use exact WGS-72 values.

**Result**: Minimal impact (18.65 km → 18.62 km) - not the root cause.

---

## Test Satellites

| Satellite | Period (min) | Mean Motion (rad/min) | ecc | irez | Path | Error (km) |
|-----------|--------------|----------------------|-----|------|------|------------|
| LES-5 (GEO) | ~1317 | 0.00477 | 0.0055 | 1 | Lyddane | 5-17 |
| OPS 3811 (DSP 2) | ~1469 | 0.00428 | 0.0022 | 1 | Lyddane | 10-17 |
| ANIK A2 | ~1436 | 0.00437 | 0.0003 | 1 | Lyddane | 3-15 |
| GPS BIIR-2 | ~718 | 0.00875 | 0.0123 | 0 | Standard | **20-22** |
| NAVSTAR 62 | ~718 | 0.00875 | 0.0089 | 0 | Standard | **17-20** |
| GLONASS-M 736 | ~675 | 0.00929 | 0.0012 | 0 | Standard | **13-18** |
| LAGEOS 1 | ~226 | 0.02783 | 0.0046 | 0 | Standard | 1.3-1.7 |

**Key Observation**: Non-resonant satellites (irez=0) have the highest errors, especially GPS/MEO orbits.

### Latest Test Results (January 17, 2026)

```
=== Deep Space Accuracy Summary ===
Successful comparisons: 49
Max position error: 22.2240 km (GPS BIIR-2 (PRN 13))
Max velocity error: 0.002898 km/s (2.898 m/s)
```

| Satellite | t=0h | t=1h | t=6h | t=12h | t=24h | t=168h |
|-----------|------|------|------|-------|-------|--------|
| LES-5 (GEO) | 7.4 km | 11.2 km | 14.6 km | 11.8 km | 14.4 km | 16.2 km |
| OPS 3811 | 10.6 km | 7.2 km | 15.8 km | 10.8 km | 12.2 km | 18.3 km |
| ANIK A2 | 6.9 km | 5.3 km | 14.8 km | 6.8 km | 6.8 km | 5.9 km |
| **GPS BIIR-2** | **22.2 km** | **21.6 km** | **21.0 km** | **22.2 km** | **22.2 km** | **22.1 km** |
| NAVSTAR 62 | 19.0 km | 19.1 km | 17.9 km | 19.0 km | 19.0 km | 19.0 km |
| GLONASS-M 736 | 14.3 km | 14.9 km | 14.4 km | 14.6 km | 15.5 km | 15.2 km |
| LAGEOS 1 | 1.8 km | 1.5 km | 1.6 km | 1.5 km | 1.7 km | 1.4 km |

**Pattern**: GPS/NAVSTAR (irez=0, ~718 min period) have consistently highest errors (~20-22 km).

---

## Verified Equivalent ✅

### 1. Epoch Conversion
```
epoch = jd - 2433281.5  (days since Jan 0, 1950)
```
- CUDA: `p.epoch_jd - 2433281.5` ✅
- python-sgp4: `epoch = jd - 2433281.5` ✅

### 2. Day Calculation (dscom)
```
day = epoch + 18261.5 + tc / 1440.0
```
- CUDA: `day = epoch + 18261.5 + tc / 1440.0` ✅
- python-sgp4: `day = epoch + 18261.5 + tc / 1440.0` ✅
- Verified: `day = 45761.0` for GPS BIIR-2 ✅

### 3. GST at Epoch (gsto) - AFSPC Mode
```python
ts70 = epoch - 7305.0
ds70 = floor(ts70 + 1e-8)
tfrac = ts70 - ds70
gsto = (thgr70 + c1*ds70 + c1p2p*tfrac + ts70*ts70*fk5r) % TWOPI
```
- CUDA: Matches exactly ✅
- python-sgp4: `gsto = 0.4171287729` ✅
- CUDA computed: `gsto = 0.4171287730` ✅ (< 1e-10 difference)

### 4. Un-Kozai Mean Motion
```python
no_kozai = 0.008751304422 rad/min
no_unkozai = 0.008751318946 rad/min
```
- CUDA: Uses same formula ✅
- python-sgp4: Matches ✅

### 5. Deep Space Constants (dscom)
| Constant | Value | Status |
|----------|-------|--------|
| zes | 0.01675 | ✅ |
| zel | 0.05490 | ✅ |
| c1ss | 2.9864797e-6 | ✅ |
| c1l | 4.7968065e-7 | ✅ |
| zsinis | 0.39785416 | ✅ |
| zcosis | 0.91744867 | ✅ |
| zcosgs | 0.1945905 | ✅ |
| zsings | -0.98088458 | ✅ |

### 6. Resonance Detection (dsinit)
```python
irez = 0  # default
if 0.0034906585 < nm < 0.0052359877:
    irez = 1  # synchronous (GEO)
if 0.00826 <= nm <= 0.00924 and em >= 0.5:
    irez = 2  # half-day (Molniya)
```
- GPS BIIR-2: nm=0.00875, ecc=0.0123 → irez=0 ✅
- LES-5: nm=0.00477 → irez=1 ✅

### 7. Resonance Constants (dsinit)
| Constant | Value | Status |
|----------|-------|--------|
| q22 | 1.7891679e-6 | ✅ |
| q31 | 2.1460748e-6 | ✅ |
| q33 | 2.2123015e-7 | ✅ |
| root22 | 1.7891679e-6 | ✅ |
| root32 | 3.7393792e-7 | ✅ |
| root44 | 7.3636953e-9 | ✅ |
| root52 | 1.1428639e-7 | ✅ |
| root54 | 2.1765803e-9 | ✅ |
| rptim | 4.37526908801129966e-3 | ✅ |
| znl | 1.5835218e-4 | ✅ |
| zns | 1.19459e-5 | ✅ |

### 8. Solar/Lunar Mean Anomalies
```
zmos = (6.2565837 + 0.017201977 * day) % TWOPI
zmol = (4.7199672 + 0.22997150 * day - gam) % TWOPI
```
- Verified formula matches python-sgp4 ✅

### 9. Inclination Path Selection
```
if inclp < 0.2 rad (~11.5°):
    use Lyddane modification
else:
    use Standard modification
```
- GEO satellites (incl < 5°) → Lyddane ✅
- GPS satellites (incl ~55°) → Standard ✅

### 10. Simple Keplerian Position
At t=0 with no perturbations:
- python-sgp4: `[-15755.83, -21605.80, -442.06] km`
- Simple Kepler: `[-15753.54, -21606.76, -449.94] km`
- Difference: 8.26 km (expected from perturbations)

---

## Fixes Applied

### 1. Lyddane nodep Modulo (Did NOT fix the issue)
**Changed**: nodep modulo calculation in Lyddane path
```cuda
// Old (CUDA normalized to positive):
nodep = fmod(nodep, TWOPI);
if (nodep < 0.0) nodep += TWOPI;

// New (python-sgp4 signed modulo):
if (nodep >= 0.0) {
    nodep = fmod(nodep, TWOPI);
} else {
    nodep = -fmod(-nodep, TWOPI);
}
```
**Result**: No change in error - GPS satellites use Standard path anyway.

### 2. C1L Constant Definition (Already correct in code)
**Issue Found**: Global define was wrong but local variable was correct
```cuda
// sgp4_constants.cuh (was wrong, fixed):
#define C1L 4.7968065e-7  // was incorrectly 0.00015835218

// sgp4_deepspace.cuh (was already correct):
const double c1l = 4.7968065e-7;
```
**Result**: No change - local variable was already correct.

### 3. JD_1950 Reference Point (FIXED)
**Issue Found**: Tests were using wrong JD_1950 constant
- **Wrong**: `JD_1950 = 2433282.5` (Jan 1, 1950 00:00 UTC)
- **Correct**: `JD_1950 = 2433281.5` (Dec 31, 1949 00:00 UTC = Jan 0.0, 1950)

SAAL's `days_since_1950` uses **Jan 0.0, 1950** (Dec 31, 1949 midnight) as reference, NOT Jan 1, 1950.

**Verification**:
- TLE epoch `25105.50000000` = April 15, 2025 12:00 UTC = JD 2460781.0
- SAAL: `days_since_1950 = 27499.5`
- With JD_1950 = 2433281.5: `epoch_jd = 27499.5 + 2433281.5 = 2460781.0` ✅
- With JD_1950 = 2433282.5: `epoch_jd = 27499.5 + 2433282.5 = 2460782.0` ❌

**Result**: Epoch now correct! After fix:
- `epoch_jd: 2460781.0` ✅
- `zmos: CUDA=1.7549044924, expected=1.7549044924` ✅ (diff=2.78e-11)
- `zmol: CUDA=3.5467933339, expected=3.5467933339` ✅ (diff=1.40e-11)
- Secular rates now match to ~1e-17 ✅

**BUT**: Position errors remain at 5-22 km! This means the bug is in the **propagation step**, not initialization.

---

## Under Investigation 🔍

### 1. dspace Function for irez=0

**Observation**: For non-resonant satellites (irez=0), the dspace function:
1. Applies secular rates: `em += dedt * t`, `inclm += didt * t`, etc.
2. Does NOT enter the resonance integration loop
3. Does NOT update `nm` (mean motion) explicitly

**python-sgp4 behavior**:
```python
if irez != 0:
    # ... resonance integration ...
    nm = no + dndt
# For irez=0: nm stays as input value (no change)
```

**Question**: Is my CUDA correctly preserving `nm` for irez=0 case?

### 2. Secular Rate Values (dedt, didt, dmdt, domdt, dnodt)

These are computed in dsinit from:
- Solar terms (ss1-ss7, sz1-sz33)
- Lunar terms (s1-s7, z1-z33)

**Need to verify**: Are the computed secular rates identical between CUDA and python-sgp4?

### 3. DPPER Periodic Contributions

The dpper function applies lunar-solar periodic corrections to:
- eccentricity (ep)
- inclination (inclp)
- RAAN (nodep)
- argument of perigee (argpp)
- mean anomaly (mp)

**Current implementation**:
- At init (t=0): Stores baseline periodics (peo, pinco, plo, pgho, pho)
- At propagation: Subtracts baseline and applies delta

**Need to verify**: Are the periodic coefficients (se2, se3, si2, si3, etc.) correct?

### 4. Position Error at t=0

**CRITICAL FINDING**: The position error exists at t=0 (epoch propagation):
- **CPU (SAAL)**: `[-15755.832, -21605.800, -442.064]` km (matches python-sgp4 exactly)
- **GPU (CUDA)**: `[-15759.388, -21602.840, -460.136]` km
- **Delta at t=0**: X=-3.56 km, Y=+2.96 km, Z=-18.07 km → **~18.7 km error at epoch!**

This proves the bug is in the **propagation step**, not in the initialization parameters (which all match).

### 5. Initialization Parameters Verified ✅

After the JD_1950 fix, all initialization parameters now match python-sgp4:

| Parameter | CUDA Value | Expected Value | Status |
|-----------|------------|----------------|--------|
| epoch_jd | 2460781.0 | 2460781.0 | ✅ |
| no_kozai | 0.008751304402544 | 0.008751304402544 | ✅ |
| no_unkozai | 0.008751318925699 | 0.008751318925887 | ✅ (1.88e-13) |
| zmos | 1.7549044924 | 1.7549044924 | ✅ (2.78e-11) |
| zmol | 3.5467933339 | 3.5467933339 | ✅ (1.40e-11) |
| dedt | -9.940528783e-10 | -9.940528760e-10 | ✅ (2.26e-18) |
| didt | -4.110889571e-9 | -4.110889562e-9 | ✅ (9.36e-18) |
| dmdt | -4.994053207e-10 | -4.994053196e-10 | ✅ (1.14e-18) |
| domdt | +2.621064689e-8 | +2.621064683e-8 | ✅ (5.97e-17) |
| dnodt | -1.194592402e-8 | -1.194592399e-8 | ✅ (2.72e-17) |

**Conclusion**: Initialization is correct. Bug must be in dspace, dpper, or post-processing.

---

## Key Equations to Verify

### Secular Rates (dsinit)
```python
# Solar terms
ses = ss1 * zns * ss5
sis = ss2 * zns * (sz11 + sz13)
sls = -zns * ss3 * (sz1 + sz3 - 14.0 - 6.0 * emsq)
sghs = ss4 * zns * (sz31 + sz33 - 6.0)
shs = -zns * ss2 * (sz21 + sz23)

# Lunar terms
dedt = ses + s1 * znl * s5
didt = sis + s2 * znl * (z11 + z13)
dmdt = sls - znl * s3 * (z1 + z3 - 14.0 - 6.0 * emsq)
# etc.
```

### Mean Motion Update in dspace (irez=0 case)
For non-resonant satellites, after dspace:
- `nm` should remain equal to `no_unkozai`
- Only secular changes to em, inclm, argpm, nodem, mm are applied

---

## Next Steps

1. ✅ ~~**Add debug output** to CUDA kernel showing intermediate values~~ (Done)

2. ⏳ **Compare dscom output coefficients** - These 24 coefficients haven't been verified:
   - Solar: se2, se3, si2, si3, sl2, sl3, sl4, sgh2, sgh3, sgh4, sh2, sh3
   - Lunar: ee2, e3, xi2, xi3, xl2, xl3, xl4, xgh2, xgh3, xgh4, xh2, xh3

3. ✅ ~~**Trace single satellite step-by-step**~~ - Traced to `su` divergence

4. ✅ ~~**Check if error is in orbital elements or Cartesian conversion**~~ - Confirmed error is in orbital element `su`, not coordinate conversion

5. 🔍 **NEW: Trace sinu/cosu calculation inputs** - Since `su = atan2(sinu, cosu)` formula matches, the inputs must differ:
   ```
   sinu = am / rl * (sineo1 - aynl - axnl * temp)
   cosu = am / rl * (coseo1 - axnl + aynl * temp)
   ```
   Need to compare each input variable between CUDA and python-sgp4:
   - [ ] `sineo1` - sin of eccentric anomaly
   - [ ] `coseo1` - cos of eccentric anomaly  
   - [ ] `axnl` - e * cos(argp) with long period corrections
   - [ ] `aynl` - e * sin(argp) with long period corrections
   - [ ] `temp` = esine / (1 + betal)
   - [ ] `am` - semi-major axis
   - [ ] `rl` - radius (am * (1 - ecose))

6. 🔍 **NEW: Check if axnl/aynl calculation differs** - These are the long period periodic terms:
   ```
   axnl = em * cos(argpm)
   aynl = em * sin(argpm) + temp * aycof
   ```
   The `argpm` value after dpper should be checked.

---

## Test Commands

```bash
# Run deep space accuracy test
cargo test --features cuda test_deep_space_gpu_vs_cpu_accuracy -- --nocapture

# Compare python-sgp4 output
python3 << 'EOF'
from sgp4.api import Satrec
line1 = '1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993'
line2 = '2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789'
sat = Satrec.twoline2rv(line1, line2)
e, r, v = sat.sgp4(sat.jdsatepoch, sat.jdsatepochF)
print(f"Position: {r}")
print(f"Velocity: {v}")
EOF
```

---

## References

- [python-sgp4 source](https://github.com/brandon-rhodes/python-sgp4)
- [Vallado SGP4 paper](https://celestrak.org/publications/AIAA/2006-6753/)
- [SAAL library](https://github.com/citra-space/saal) (Rust FFI wrapper)
