# Finding the DPPER Divergence

## Summary

Python recomputation using GPU values gives **IDENTICAL** result to GPU:
- GPU aynl:    0.0105420611915136
- Python aynl: 0.0105420611915136
- Difference:  8.67e-18 (machine epsilon)

But comparing GPU to previous CPU trace:
- GPU aynl:    0.0105420611915136
- CPU aynl:    0.010542703088
- Difference:  **6.419e-7** ← THE PROBLEM

**Conclusion**: The divergence is NOT in the aynl calculation. It's in the INPUT values coming from dpper.

## GPU Output After DPPER

```
--- Deep Space Periodics ---
em (pp):    0.0123591075
inclm (pp): 0.9682660568 rad
nodem (pp): 4.0936741457 rad
argpm (pp): 2.1548120933 rad
mm (pp):    4.2167450727 rad
```

##Next Steps

1. **Get CPU values after dpper** for comparison
   - Need to instrument python-sgp4 or SAAL
   - Focus on: em, argpm, inclm after dpper

2. **Check dpper implementation** for differences
   - Compare CUDA `sgp4_deepspace.cuh` dpper function
   - With python-sgp4 dpper implementation
   - Look for FMA, transcendental functions, order of operations

3. **Hypothesis**: The difference comes from:
   - Periodic term calculations in dpper (sin/cos precision)
   - Inclination perturbation (pinc) calculation
   - Argument of perigee perturbation calculation

## Key Values to Extract from CPU

Need these values AFTER dpper call at t=10 min:
- `em` (eccentricity)
- `argpm` (argument of perigee)
- `inclm` (inclination)

Then compare:
```
           GPU            CPU          Difference
em:      0.0123591075     ?            ?
argpm:   2.1548120933     ?            ?
inclm:   0.9682660568     ?            ?
```

Once we find which value differs, we can trace into the dpper function
to find the exact calculation causing the divergence.

## Previous Investigation Notes

From deep-space-debugging.md lines 504-519:
- The dpper baseline periodics bug was already FIXED
- That fix reduced error from 22 km to 40 m
- Remaining 30-50m is likely normal FP precision

The current 30m error is likely due to normal floating-point precision
differences in the dpper periodic term calculations, not a fixable bug.
