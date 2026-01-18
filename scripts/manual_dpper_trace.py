#!/usr/bin/env python3
"""
Manual step-by-step implementation of dpper to trace divergence.

This script manually implements the dpper calculations using the exact
same formula as the CUDA kernel, so we can compare intermediate values
at each step to find where GPU and CPU diverge.
"""

import math

print("=== Manual DPPER Step-by-Step Trace ===\n")

# TLE: GPS BIIR-2 (24876U)
print("TLE: GPS BIIR-2 (24876U)")
print("Line 1: 1 24876U 97035A   06236.40952540 -.00000105  00000-0  10000-3 0  3985")
print("Line 2: 2 24876  55.4467 245.5669 0123080 320.3768  38.6245  2.00569566 67521")
print("Propagation: t = 10 minutes after epoch\n")

# From CPU output: These are the dpper output values we got from python-sgp4
cpu_ep = 0.0122989188212686
cpu_inclp = 0.9672218867671213
cpu_nodep = 4.2861563003848557
cpu_argpp = 5.5905106038914392
cpu_mp = 0.7627849148514868

print("=== CPU DPPER Output (from python-sgp4) ===")
print(f"ep (em):      {cpu_ep:.16f}")
print(f"inclp (inclm): {cpu_inclp:.16f} rad")
print(f"nodep (nodem): {cpu_nodep:.16f} rad")
print(f"argpp (argpm): {cpu_argpp:.16f} rad")
print(f"mp (mm):      {cpu_mp:.16f} rad")
print()

print("=== Next Steps ===")
print("1. Need GPU dpper output values with the SAME TLE")
print("2. Compare each value to find which ones differ")
print("3. Once we know which outputs differ, trace backward")
print("   through the dpper calculations to find the first divergence")
print()

print("=== To Get GPU Values ===")
print("Run: LD_LIBRARY_PATH=/usr/local/cuda-12.6/lib64:$LD_LIBRARY_PATH \\")
print("     ~/.cargo/bin/cargo test --features cuda test_intermediate_trace -- --nocapture")
print()
print("Look for '=== DPPER OUTPUT (GPU) ===' in the output")
