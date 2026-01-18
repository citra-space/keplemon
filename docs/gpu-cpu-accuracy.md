# GPU vs CPU SGP4 Accuracy

**Status**: Resolved
**Date**: 2026-01-18
**Final Error**: 0.017 m (17 mm)

## Summary

The CUDA GPU implementation of SGP4 deep space propagation initially showed 22 km position errors compared to the CPU reference (python-sgp4). After investigation and fixes, accuracy improved to **17 millimeters**.

| Phase | Error | Improvement |
|-------|-------|-------------|
| Initial | 22 km | - |
| After dpper baseline fix | 32 m | 99.85% |
| After WGS-72 constant fix | **0.017 m** | **99.99%** |

---

## Root Causes Found

### Issue 1: DPPER Baseline Periodics Bug (Fixed 2026-01-17)

**Problem**: CUDA initialization incorrectly called `dpper(init=true)`, storing non-zero baseline periodics that were subtracted during propagation.

**Solution**: Removed the `dpper(init=true)` call from initialization. Baseline periodics must remain 0 (as set by `dscom`).

**Impact**: Error reduced from 22 km to 32 m.

### Issue 2: WGS-72 Gravitational Constants (Fixed 2026-01-18)

**Problem**: GPU used WGS-72 (1976 revised) constants while python-sgp4 uses WGS-72 OLD (1972 original).

| Constant | GPU (before) | python-sgp4 | Difference |
|----------|--------------|-------------|------------|
| J2 | 0.00108262998905 | 0.001082616 | 0.001292% |
| J3 | -0.00000253215306 | -0.00000253881 | -0.262% |

**Solution**: Updated `kernels/sgp4_constants.cuh` to use WGS-72 OLD values.

**Impact**: Error reduced from 32 m to 0.017 m.

---

## How Constants Affect SGP4

Small gravitational constant differences propagate through the calculation chain:

1. **Initialization**: aycof, xlcof coefficients (J2/J3 dependent)
2. **Long-period periodics**: Uses aycof, xlcof
3. **Short-period periodics**: All formulas use J2 perturbations
4. **Final position**: Accumulates all corrections

At GPS orbit altitudes (~26,000 km), a ~0.001% J2 difference causes 10-30 m position errors.

---

## Diagnostic Tests Performed

| Test | Result | Finding |
|------|--------|---------|
| GPU Determinism | 0.000 mm variance (100 runs) | Error is deterministic |
| Error Growth Rate | 9.62% variation over 7 days | Initialization-related, not drift |
| Component-Wise | 99.9% in-track error | Angular position, not radial |
| Satellite Pattern | Systematic across all satellites | Independent of orbital parameters |
| FMA Prevention | No improvement | FMA not the cause |
| Transcendental Functions | Exact match | sin/cos not the cause |

---

## Investigation Timeline

1. **Initial observation**: 5-22 km errors for deep space satellites
2. **Epoch fix**: Corrected JD_1950 reference (2433281.5, not 2433282.5)
3. **DPPER baseline fix**: Removed erroneous `dpper(init=true)` call - error dropped to 32 m
4. **FMA investigation**: Tested explicit intrinsics (`__dadd_rn`, `__dmul_rn`) - no improvement
5. **Transcendental test**: Confirmed sin/cos match within machine epsilon
6. **DPPER comparison**: Confirmed outputs match within 1e-11 rad
7. **Constants extraction**: Created scripts to compare all GPU/CPU constants
8. **Root cause found**: J2/J3 differ between WGS-72 versions
9. **Fix applied**: Updated GPU to WGS-72 OLD constants
10. **Verification**: Error reduced to 0.017 m

---

## Files Modified

### Production Code
- `kernels/sgp4_constants.cuh` - Updated J2/J3 to WGS-72 OLD values, improved documentation
- `kernels/sgp4_batch.cu` - Disabled DEBUG_PRINT
- `kernels/sgp4_deepspace.cuh` - Fixed dpper initialization (earlier fix)

### Investigation Scripts
- `scripts/extract_gpu_constants.py` - Extracts constants from CUDA headers
- `scripts/extract_cpu_constants.py` - Extracts constants from python-sgp4
- `scripts/compare_gpu_cpu_constants.py` - Systematic comparison (identified root cause)

### Tests
- `tests/test_gpu_determinism.rs` - Verifies bit-identical GPU output
- `tests/test_error_growth_rate.rs` - Analyzes temporal error behavior
- `tests/test_component_wise_error.rs` - RIC frame error decomposition
- `tests/test_intermediate_trace.rs` - Debug output extraction

---

## Verification

Run this test to verify GPU/CPU accuracy:

```bash
cargo test --release --features cuda --test test_intermediate_trace -- --nocapture
```

Expected output:
```
Position difference:
  dx: ~0.01 m
  dy: ~0.01 m
  dz: ~0.01 m
  Total: <0.02 m
```

---

## Key Lessons

1. **Gravitational constants matter**: Even 0.001% differences in J2 cause meter-level position errors
2. **Standards versions matter**: WGS-72 OLD (1972) vs WGS-72 (1976) have different values
3. **Match reference implementation**: python-sgp4 uses WGS-72 OLD, GPU must match
4. **Systematic debugging works**: Extract and compare actual values, don't assume constants match

---

## WGS-72 Constant Reference

The GPU now uses these values (matching python-sgp4):

| Constant | Value | Description |
|----------|-------|-------------|
| RE | 6378.135 km | Earth radius |
| J2 | 0.001082616 | Second zonal harmonic (WGS-72 OLD) |
| J3 | -0.00000253881 | Third zonal harmonic (WGS-72 OLD) |
| J4 | -0.00000165597 | Fourth zonal harmonic |
| XKE | 0.0743669161 | sqrt(GM) in canonical units |
| MU | 398600.8 km³/s² | Gravitational parameter |

---

## References

- Hoots & Roehrich, "Spacetrack Report No. 3" (1980)
- Vallado et al., "Revisiting Spacetrack Report #3" AIAA 2006-6753
- python-sgp4: https://github.com/brandon-rhodes/python-sgp4
