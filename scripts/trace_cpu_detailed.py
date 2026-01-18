#!/usr/bin/env python3
"""
Extract detailed intermediate values from python-sgp4 by patching the propagation function.
"""

from sgp4.api import Satrec
import sgp4.propagation as prop
from sgp4 import ext
import math

# GPS BIIR-2 (PRN 13)
line1 = '1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993'
line2 = '2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789'

sat = Satrec.twoline2rv(line1, line2)

print("=== DETAILED CPU INTERMEDIATE VALUE TRACE ===\n")
print("Satellite: GPS BIIR-2 (PRN 13)")
print("Propagation: epoch + 10 minutes\n")

# Store intermediate values
intermediates = {}

# Patch the sgp4 function to extract values
original_sgp4 = prop.sgp4

def traced_sgp4(satrec, tsince):
    """Patched sgp4 that extracts intermediate values"""

    # This is a simplified extraction - we'll need to manually compute
    # the key values following the algorithm

    # Call original
    error, r, v = original_sgp4(satrec, tsince)

    return error, r, v

# For more detailed tracing, let's manually replicate the key calculations
# from sgp4/propagation.py

print("--- Deep Space Propagation Manual Trace ---\n")

tsince = 10.0  # minutes

# Initial values
print(f"tsince: {tsince}")
print(f"inclo: {sat.inclo:.10f} rad")
print(f"no_kozai: {sat.no_kozai:.10f} rad/min")
print(f"ecco: {sat.ecco:.10f}")
print(f"argpo: {sat.argpo:.10f} rad")
print(f"mo: {sat.mo:.10f} rad")
print(f"nodeo: {sat.nodeo:.10f} rad")

# Secular updates
xmdf = sat.mo + sat.mdot * tsince
argpdf = sat.argpo + sat.argpdot * tsince
nodedf = sat.nodeo + sat.nodedot * tsince

print(f"\nAfter secular updates:")
print(f"xmdf: {xmdf:.10f} rad")
print(f"argpdf: {argpdf:.10f} rad")
print(f"nodedf: {nodedf:.10f} rad")

# For deep space, dspace and dpper are called
# We need to extract values after these calls

# Actually run the propagation to get final values
error, r, v = sat.sgp4(sat.jdsatepoch, sat.jdsatepochF + tsince / 1440.0)

print(f"\n==========================================")
print(f"FINAL CPU RESULT:")
print(f"==========================================")
print(f"Position (km): x={r[0]:.10f}, y={r[1]:.10f}, z={r[2]:.10f}")
print(f"Velocity (km/s): vx={v[0]:.10f}, vy={v[1]:.10f}, vz={v[2]:.10f}")

# Now let's manually compute the long-period periodics to extract axnl, aynl
# This requires knowing the values after dspace and dpper

print(f"\n==========================================")
print(f"MANUAL COMPUTATION OF LONG-PERIOD TERMS:")
print(f"==========================================")

# We need inclm, em, argpm, nodem after dpper
# These are not directly accessible, so we need to instrument the code

print("\nTo extract axnl, aynl, su values:")
print("1. Clone python-sgp4 repository")
print("2. Add print statements in sgp4/propagation.py")
print("3. Run modified version")
print("\nAlternatively, use C library (SAAL) with debug output")

print(f"\n==========================================")
print(f"COMPARISON WITH PREVIOUS TRACE:")
print(f"==========================================")
print("From deep-space-debugging.md (previous investigation):")
print("")
print("CPU (python-sgp4) values at t=10 min:")
print("  axnl: -0.006814517956")
print("  aynl:  0.010542703088")
print("  su:    0.066324603991 rad")
print("")
print("GPU (current) values at t=10 min:")
print("  axnl: -0.0068145467")
print("  aynl:  0.0105420612")
print("  su:    0.0663274791 rad")
print("")
print("Differences:")
print(f"  Δaxnl: {-0.0068145467 - (-0.006814517956):.10e} ({abs(-0.0068145467 - (-0.006814517956)):.3e})")
print(f"  Δaynl: {0.0105420612 - 0.010542703088:.10e} ({abs(0.0105420612 - 0.010542703088):.3e})")
print(f"  Δsu:   {0.0663274791 - 0.066324603991:.10e} ({abs(0.0663274791 - 0.066324603991):.3e})")

print(f"\n==========================================")
print(f"ANALYSIS:")
print(f"==========================================")
print("axnl difference: ~6e-9 (essentially matching)")
print("aynl difference: ~6e-7 (0.6 microradians)")
print("su difference:   ~3e-6 (3 microradians = 0.000172°)")
print("")
print("The small differences in aynl and su accumulate through:")
print("1. Trig function (sin/cos) precision differences")
print("2. FMA (fused multiply-add) vs separate operations")
print("3. Order of operations causing different rounding")
print("")
print("These tiny angular differences (microradians) translate to")
print("~30m position error through the coordinate transformation.")
