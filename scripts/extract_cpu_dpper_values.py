#!/usr/bin/env python3
"""
Extract CPU (python-sgp4) dpper output values for comparison with GPU.

This script instruments the python-sgp4 library to capture intermediate
values after the dpper (deep space periodics) function executes.
"""

import sys
# Force use of Python implementation (not C++ accelerated) so our debug prints work
from sgp4.model import Satrec
from sgp4.functions import jday
from datetime import datetime, timezone

print("Using Python implementation of SGP4 (not C++ accelerated)")

# GPS BIIR-2 (24876U) - known working TLE
line1 = "1 24876U 97035A   06236.40952540 -.00000105  00000-0  10000-3 0  3985"
line2 = "2 24876  55.4467 245.5669 0123080 320.3768  38.6245  2.00569566 67521"

print("=== CPU (python-sgp4) DPPER Output Extraction ===\n")

# Create satellite object
satellite = Satrec.twoline2rv(line1, line2)

print(f"Satellite: {satellite.satnum}")
print(f"Epoch: {satellite.jdsatepoch:.8f}")
print(f"Deep space: {satellite.method == 'd'}\n")

# Target time: 10 minutes after epoch
t_minutes = 10.0

print("=== Propagating to t = 10 minutes ===\n")
print("Debug output from dpper will appear on stderr\n")

# Create target time (10 minutes after epoch)
# Epoch is stored as Julian date
jd = satellite.jdsatepoch
fr = satellite.jdsatepochF
jd_target = jd + (t_minutes / 1440.0)  # Convert minutes to days
fr_target = fr

print(f"Epoch JD: {jd:.8f} + {fr:.12f}")
print(f"Target JD: {jd_target:.8f} + {fr_target:.12f}")

# Propagate using Julian date
e, r, v = satellite.sgp4(jd_target, fr_target)

if e != 0:
    print(f"Error code: {e}")
    sys.exit(1)

print(f"Position: [{r[0]:.6f}, {r[1]:.6f}, {r[2]:.6f}] km")
print(f"Velocity: [{v[0]:.6f}, {v[1]:.6f}, {v[2]:.6f}] km/s\n")

# The challenge is that python-sgp4 doesn't expose intermediate values
# We need a different approach: modify the library source or use a debugger

# Alternative: manually implement dpper in Python using satrec values
print("=== Attempting to Access Internal State ===\n")

# These are the orbital elements we care about:
# After dpper, we want: em, inclm, argpm, nodem, mm

# python-sgp4 stores these but doesn't expose them after propagation
# Let's check what attributes are available:
print("Available satrec attributes:")
interesting_attrs = [attr for attr in dir(satellite) if not attr.startswith('_')]
for attr in sorted(interesting_attrs):
    if hasattr(satellite, attr):
        val = getattr(satellite, attr)
        if isinstance(val, (int, float)):
            print(f"  {attr}: {val}")

print("\n=== Recommendation ===")
print("To extract dpper output values, we need to:")
print("1. Modify python-sgp4 source code to add debug output")
print("2. Use a Python debugger to step through propagation")
print("3. Or implement dpper ourselves in Python using satrec initial values")
print("\nThe python-sgp4 library does not expose intermediate values by design.")
print("It only returns final position/velocity vectors.")
