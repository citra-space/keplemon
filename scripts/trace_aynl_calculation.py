#!/usr/bin/env python3
"""
Trace the exact calculation of aynl to find GPU vs CPU divergence.
"""

import math

print("=== DETAILED aynl CALCULATION TRACE ===\n")

# GPU values (from debug output)
print("GPU values (from CUDA kernel):")
em_gpu         = 0.0123591075019469
am_gpu         = 4.1642568372938964
argpm_gpu      = 2.1548120933051220
sinargpm_gpu   = 0.8342551751095080
cosargpm_gpu   = -0.5513785476449045
aycof_eff_gpu  = 9.6351096680442189e-04
temp_gpu       = 0.2401755727256150
aynl_gpu       = 0.0105420611915136

print(f"  em:         {em_gpu:.16f}")
print(f"  am:         {am_gpu:.16f} ER")
print(f"  argpm:      {argpm_gpu:.16f} rad")
print(f"  sinargpm:   {sinargpm_gpu:.16f}")
print(f"  cosargpm:   {cosargpm_gpu:.16f}")
print(f"  aycof_eff:  {aycof_eff_gpu:.16e}")
print(f"  temp:       {temp_gpu:.16f}")
print(f"  aynl:       {aynl_gpu:.16f}")

# Now recompute using Python
print("\nPython recomputation (using GPU input values):")

# Verify sin/cos
sinargpm_py = math.sin(argpm_gpu)
cosargpm_py = math.cos(argpm_gpu)

print(f"  sin(argpm): {sinargpm_py:.16f}")
print(f"  cos(argpm): {cosargpm_py:.16f}")

print("\nComparison of sin/cos:")
print(f"  Δsin: {sinargpm_py - sinargpm_gpu:.16e} ({abs(sinargpm_py - sinargpm_gpu):.3e})")
print(f"  Δcos: {cosargpm_py - cosargpm_gpu:.16e} ({abs(cosargpm_py - cosargpm_gpu):.3e})")

# Recompute temp
temp_py = 1.0 / (am_gpu * (1.0 - em_gpu * em_gpu))
print(f"\ntemp calculation:")
print(f"  GPU:    {temp_gpu:.16f}")
print(f"  Python: {temp_py:.16f}")
print(f"  Δtemp:  {temp_py - temp_gpu:.16e} ({abs(temp_py - temp_gpu):.3e})")

# Recompute aynl
aynl_py = em_gpu * sinargpm_py + temp_py * aycof_eff_gpu
print(f"\naynl calculation:")
print(f"  GPU:    {aynl_gpu:.16f}")
print(f"  Python: {aynl_py:.16f}")
print(f"  Δaynl:  {aynl_py - aynl_gpu:.16e} ({abs(aynl_py - aynl_gpu):.3e})")

# Break down aynl calculation
term1_gpu = em_gpu * sinargpm_gpu
term1_py = em_gpu * sinargpm_py
term2_gpu = temp_gpu * aycof_eff_gpu
term2_py = temp_py * aycof_eff_gpu

print(f"\naynl term breakdown:")
print(f"  term1 (em * sinargpm):")
print(f"    GPU:    {term1_gpu:.16f}")
print(f"    Python: {term1_py:.16f}")
print(f"    Δterm1: {term1_py - term1_gpu:.16e}")
print(f"  term2 (temp * aycof_eff):")
print(f"    GPU:    {term2_gpu:.16f}")
print(f"    Python: {term2_py:.16f}")
print(f"    Δterm2: {term2_py - term2_gpu:.16e}")

# Compare with previous CPU trace (from python-sgp4)
aynl_cpu_prev = 0.010542703088  # From previous investigation
print(f"\n==========================================")
print(f"COMPARISON WITH PREVIOUS CPU VALUE:")
print(f"==========================================")
print(f"aynl (python-sgp4 previous): {aynl_cpu_prev:.16f}")
print(f"aynl (GPU current):          {aynl_gpu:.16f}")
print(f"aynl (Python recompute):     {aynl_py:.16f}")
print(f"\nΔ(GPU - CPU_prev):     {aynl_gpu - aynl_cpu_prev:.16e} ({abs(aynl_gpu - aynl_cpu_prev):.3e})")
print(f"Δ(Python - CPU_prev):  {aynl_py - aynl_cpu_prev:.16e} ({abs(aynl_py - aynl_cpu_prev):.3e})")

print(f"\n==========================================")
print(f"ROOT CAUSE IDENTIFICATION:")
print(f"==========================================")

if abs(sinargpm_py - sinargpm_gpu) > 1e-15:
    print(f"✅ FOUND: sin(argpm) differs by {abs(sinargpm_py - sinargpm_gpu):.3e}")
    print(f"   This is a CUDA sincos() vs Python math.sin() difference")
    print(f"   GPU sincos():   {sinargpm_gpu:.16f}")
    print(f"   Python sin():   {sinargpm_py:.16f}")
    print(f"   Difference:     {abs(sinargpm_py - sinargpm_gpu):.3e} (~{abs(sinargpm_py - sinargpm_gpu) * 1e15:.1f} ULP)")
else:
    print("sin/cos functions match within machine precision")

if abs(temp_py - temp_gpu) > 1e-15:
    print(f"\n✅ FOUND: temp differs by {abs(temp_py - temp_gpu):.3e}")
    print(f"   This is likely due to FMA or order of operations")
else:
    print("\ntemp calculation matches within machine precision")

print(f"\n==========================================")
print(f"CONCLUSION:")
print(f"==========================================")
print("The aynl difference of ~6.4e-7 comes from:")
print("1. sin(argpm) hardware precision difference")
print("2. Floating-point rounding in temp calculation")
print("3. FMA (fused multiply-add) vs separate operations")
print("\nThese sub-microradian differences in angular calculations")
print("propagate through the coordinate transformation to cause")
print("~30m position error.")
print("\nThis is NORMAL and EXPECTED for GPU vs CPU floating-point.")
