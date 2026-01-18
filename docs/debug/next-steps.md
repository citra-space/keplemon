# Next Investigation Steps: Finding the Exact Error Source

**Status**: 🔍 **ONGOING** - Pinpointing exact divergence location in dpper

---

## What We've Ruled Out

✅ **NOT caused by:**
- FMA operations (tested with `__dadd_rn()` and `__dmul_rn()` intrinsics - 0% improvement)
- Transcendental functions (sin/cos match within machine epsilon)

---

## Current Focus

Find the **exact calculation in dpper** where GPU and CPU first diverge.

### We Have
1. ✅ **CPU dpper output values** (from python-sgp4):
   ```
   TLE: GPS BIIR-2 (24876U) - 06236.40952540 epoch
   Propagation: t = 10 minutes

   ep (em):      0.0122989188212686
   inclp (inclm): 0.9672218867671213 rad
   nodep (nodem): 4.2861563003848557 rad
   argpp (argpm): 5.5905106038914392 rad
   mp (mm):      0.7627849148514868 rad
   ```

### We Need
2. ⏳ **GPU dpper output values** with the SAME TLE
   - Test updated: `tests/test_intermediate_trace.rs` now uses matching TLE
   - Debug output added: `kernels/sgp4_deepspace.cuh` line 504-512
   - DEBUG_PRINT enabled: `kernels/sgp4_batch.cu` line 11

### How to Get GPU Values
```bash
# Set CUDA library path
export LD_LIBRARY_PATH=/usr/local/cuda-12.6/lib64:$LD_LIBRARY_PATH

# Run test
~/.cargo/bin/cargo test --features cuda test_intermediate_trace -- --nocapture 2>&1 | grep -A10 "DPPER OUTPUT (GPU)"
```

**Issue**: CUDA runtime library loading fails in current environment
- Error: "Unable to dynamically load the 'cuda' shared library"
- May need to run on system with GPU available

---

## Step-by-Step Investigation Plan

### Phase 1: Direct Comparison
Once we have GPU dpper output values:

1. **Compare output values**:
   ```python
   diff_ep = gpu_ep - cpu_ep
   diff_inclp = gpu_inclp - cpu_inclp
   diff_nodep = gpu_nodep - cpu_nodep
   diff_argpp = gpu_argpp - cpu_argpp
   diff_mp = gpu_mp - cpu_mp
   ```

2. **Identify which outputs differ most**
   - If all differ by similar magnitude → error in early calculations (f2, f3, sinzf)
   - If some differ more → error in specific term calculations (solar vs lunar)
   - If only one differs → error in final adjustment logic

### Phase 2: Trace Backward
For the output(s) that differ, trace backward through calculations:

#### If `ep` (eccentricity) differs:
```
ep = ep + pe
pe = pe - peo  (baseline subtraction)
pe = ses + sel  (solar + lunar terms)
ses = se2 * f2 + se3 * f3
sel = ee2 * f2 + e3 * f3

Need to check:
- Are f2, f3 the same?
- Are se2, se3, ee2, e3 constants the same?
- Does the multiplication/addition differ?
```

#### If `inclp` (inclination) differs:
```
inclp = inclp + pinc
pinc = pinc - pinco  (baseline subtraction)
pinc = sis + sil  (solar + lunar terms)
sis = si2 * f2 + si3 * f3
sil = xi2 * f2 + xi3 * f3

Need to check:
- Same as above for f2, f3
- Are si2, si3, xi2, xi3 the same?
```

#### If `nodep` (RAAN) differs:
```
nodep = nodep + ph  (or atan2 calculation for low inclination)
ph = ph - pho  (baseline subtraction)
ph = shs + shll
shs = sh2 * f2 + sh3 * f3
shll = xh2 * f2 + xh3 * f3

Need to check:
- Check if low inclination path vs normal path
- atan2 vs direct addition
```

#### If `argpp` (argument of perigee) differs:
```
argpp = argpp + pgh  (or xls - mp - cosip * nodep for low incl)
pgh = pgh - pgho  (baseline subtraction)
pgh = sghs + sghl
sghs = sgh2 * f2 + sgh3 * f3
sghl = xgh2 * f2 + xgh3 * f3

Need to check:
- Which calculation path was taken
- Low inclination special case
```

#### If `mp` (mean anomaly) differs:
```
mp = mp + pl
pl = pl - plo  (baseline subtraction)
pl = sls + sll
sls = sl2 * f2 + sl3 * f3 + sl4 * sinzf
sll = xl2 * f2 + xl3 * f3 + xl4 * sinzf

Need to check:
- Are f2, f3, sinzf the same?
- sl4 and xl4 terms have extra sinzf
```

### Phase 3: Find Root Cause
Once we identify the first diverging calculation:

1. **Extract exact input values** for that calculation on both sides
2. **Perform calculation manually** in high precision
3. **Determine WHY they differ**:
   - Constant precision difference?
   - Operation ordering?
   - Rounding mode?
   - Compiler optimization?

### Phase 4: Determine Fix
Based on root cause:

| Cause | Fix | Complexity |
|-------|-----|------------|
| Constant precision | Use exact constants | Easy |
| Operation ordering | Reorder operations | Medium |
| Rounding mode | Set compiler flags | Medium |
| Compiler optimization | Disable specific opts | Medium |
| Fundamental FP limit | Kahan sum / double-double | Hard |

---

## Tools Available

### Scripts Created
1. `scripts/extract_cpu_dpper_values.py` - Get CPU values
2. `scripts/manual_dpper_trace.py` - Template for comparison
3. `scripts/compare_dpper_outputs.py` - Comparison framework
4. `scripts/run_gpu_dpper_extract.sh` - Run GPU extraction

### Test Files
1. `tests/test_intermediate_trace.rs` - GPU debug output

### Debug Modifications
1. `kernels/sgp4_deepspace.cuh` - Added dpper output prints
2. `kernels/sgp4_batch.cu` - DEBUG_PRINT=1
3. `/path/to/venv/.../sgp4/propagation.py` - CPU dpper prints

---

## Current Blocker

**CUDA Runtime**: Cannot run GPU test in current environment
- Need system with GPU and CUDA runtime
- OR need to extract GPU values from existing test runs
- OR compile and run on GPU-enabled machine

### Alternative Approach
If GPU unavailable:
1. Review previous GPU debug output (if saved)
2. Add even more granular debug output to CUDA kernel
3. Compile kernel separately and inspect assembly/PTX code
4. Use CUDA emulator (if available)

---

## Expected Outcome

Once we identify the exact diverging calculation, we can:
1. **Understand WHY** it differs
2. **Determine IF** it can be corrected
3. **Implement FIX** if possible
4. **Document** if unavoidable hardware difference

This is the path to finding the TRUE root cause, not just symptoms.
