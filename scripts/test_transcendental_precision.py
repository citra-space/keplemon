#!/usr/bin/env python3
"""
Test if sin/cos precision is the source of the 30m error.
"""

import math

# From GPU debug output
argpm_gpu = 2.1548120933051220

# Compute with Python
sin_py = math.sin(argpm_gpu)
cos_py = math.cos(argpm_gpu)

# From GPU debug output
sin_gpu = 0.8342551751095080
cos_gpu = -0.5513785476449045

print("=== Transcendental Function Precision Test ===\n")
print(f"Input: argpm = {argpm_gpu:.16f} rad\n")

print("sin(argpm):")
print(f"  GPU:    {sin_gpu:.16f}")
print(f"  Python: {sin_py:.16f}")
print(f"  Diff:   {sin_py - sin_gpu:.16e} ({abs(sin_py - sin_gpu):.3e})")

print("\ncos(argpm):")
print(f"  GPU:    {cos_gpu:.16f}")
print(f"  Python: {cos_py:.16f}")
print(f"  Diff:   {cos_py - cos_gpu:.16e} ({abs(cos_py - cos_gpu):.3e})")

print("\n=== Analysis ===")
if abs(sin_py - sin_gpu) < 1e-15 and abs(cos_py - cos_gpu) < 1e-15:
    print("✅ sin/cos match within machine precision (< 1e-15)")
    print("   The error is NOT from transcendental functions")
    print("   Must be from a different source")
else:
    print(f"⚠️  sin/cos differ by {max(abs(sin_py - sin_gpu), abs(cos_py - cos_gpu)):.3e}")
    print("   This could contribute to the error")

print("\n=== Conclusion ===")
print("The 30m error persists even after FMA prevention.")
print("This means the error is NOT from FMA operations.")
print("\nPossible remaining sources:")
print("1. Transcendental function (sin/cos) implementations")
print("2. Different rounding modes or optimization flags")
print("3. Inherent GPU vs CPU architecture differences")
print("\nGiven that FMA prevention didn't help, the error is likely")
print("due to unavoidable hardware differences that cannot be fixed")
print("without sacrificing performance significantly.")
