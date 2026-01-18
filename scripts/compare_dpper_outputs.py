#!/usr/bin/env python3
"""
Compare GPU vs CPU dpper output values.

This script documents the dpper (deep space periodics) output values
from both GPU (CUDA) and CPU (python-sgp4) implementations.
"""

print("=== GPU vs CPU DPPER Output Comparison ===\n")

# TLE used for comparison
print("TLE: GPS BIIR-2 (24876U)")
print("Line 1: 1 24876U 97035A   06236.40952540 -.00000105  00000-0  10000-3 0  3985")
print("Line 2: 2 24876  55.4467 245.5669 0123080 320.3768  38.6245  2.00569566 67521")
print("Propagation time: t = 10 minutes after epoch\n")

# CPU values (from extract_cpu_dpper_values.py output)
# Third call to dpper (init=n, during propagation)
cpu_em = 0.0122989188212686
cpu_inclm = 0.9672218867671213
cpu_nodem = 4.2861563003848557
cpu_argpm = 5.5905106038914392
cpu_mm = 0.7627849148514868

print("=== CPU (python-sgp4) DPPER Output ===")
print(f"em (eccentricity):        {cpu_em:.16f}")
print(f"inclm (inclination):      {cpu_inclm:.16f} rad")
print(f"nodem (RAAN):             {cpu_nodem:.16f} rad")
print(f"argpm (arg of perigee):   {cpu_argpm:.16f} rad")
print(f"mm (mean anomaly):        {cpu_mm:.16f} rad")
print()

# GPU values - need to run GPU test with same TLE
# For now, using placeholder values
print("=== GPU (CUDA) DPPER Output ===")
print("Need to run GPU test with matching TLE...")
print("Run: cargo test --features cuda test_intermediate_trace -- --nocapture")
print()

print("=== Next Steps ===")
print("1. Update test_intermediate_trace.rs to use the same TLE as CPU")
print("2. Re-run GPU test to capture dpper output with matching TLE")
print("3. Compare the values to find the divergence magnitude")
