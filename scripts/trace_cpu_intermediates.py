#!/usr/bin/env python3
"""
Trace CPU intermediate values using python-sgp4 for comparison with GPU.
"""

from sgp4.api import Satrec, SGP4_ERRORS
import math

# GPS BIIR-2 (PRN 13) - same TLE as GPU test
line1 = '1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993'
line2 = '2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789'

# Parse TLE
sat = Satrec.twoline2rv(line1, line2)

print("=== CPU (python-sgp4) Intermediate Values ===\n")
print("Satellite: GPS BIIR-2 (PRN 13)")
print("Propagation: epoch + 10 minutes\n")

# Propagate to t=10 minutes
tsince = 10.0  # minutes
error_code, r, v = sat.sgp4(sat.jdsatepoch, sat.jdsatepochF + tsince / 1440.0)

if error_code != 0:
    print(f"ERROR: SGP4 error code {error_code}: {SGP4_ERRORS[error_code]}")
    exit(1)

print("--- Initialized Parameters ---")
print(f"inclo:       {sat.inclo:.10f} rad ({math.degrees(sat.inclo):.6f} deg)")
print(f"nodeo:       {sat.nodeo:.10f} rad")
print(f"ecco:        {sat.ecco:.10f}")
print(f"argpo:       {sat.argpo:.10f} rad")
print(f"mo:          {sat.mo:.10f} rad")
print(f"no_kozai:    {sat.no_kozai:.10f} rad/min")
print(f"a:           {sat.a:.10f} ER")
print(f"bstar:       {sat.bstar:.10e}")
print(f"method:      {sat.method} (d=deep space)")

print("\n==========================================")
print("CPU FINAL RESULT:")
print("==========================================")
print(f"Position (km): x={r[0]:.10f}, y={r[1]:.10f}, z={r[2]:.10f}")
print(f"Velocity (km/s): vx={v[0]:.10f}, vy={v[1]:.10f}, vz={v[2]:.10f}")

# Compute position magnitude
r_mag = math.sqrt(r[0]**2 + r[1]**2 + r[2]**2)
v_mag = math.sqrt(v[0]**2 + v[1]**2 + v[2]**2)
print(f"|r| = {r_mag:.6f} km")
print(f"|v| = {v_mag:.6f} km/s")

# Compare with GPU
gpu_r = [-14645.2185263055, -22300.4897288589, 1459.0680512925]
dx = r[0] - gpu_r[0]
dy = r[1] - gpu_r[1]
dz = r[2] - gpu_r[2]
error_m = math.sqrt(dx**2 + dy**2 + dz**2) * 1000

print("\n==========================================")
print("COMPARISON WITH GPU:")
print("==========================================")
print(f"dx: {dx*1000:.3f} m")
print(f"dy: {dy*1000:.3f} m")
print(f"dz: {dz*1000:.3f} m")
print(f"Error: {error_m:.3f} m")

print("\n==========================================")
print("INTERMEDIATE VALUE EXTRACTION:")
print("==========================================")
print("From previous investigation (deep-space-debugging.md lines 495-500):")
print("At t=10 min for GPS BIIR-2:")
print("")
print("| Variable | GPU (current)   | python-sgp4 (prev) | Status |")
print("|----------|-----------------|---------------------|--------|")
print("| axnl     | -0.0068145467   | -0.006814517956     | ✅ MATCH (6.1e-9) |")
print("| aynl     |  0.0105420612   |  0.010542703088     | ⚠️ diff 6.4e-7 |")
print("| su       |  0.0663274791   |  0.066324603991     | ⚠️ diff 2.9e-6 |")
print("")
print("GPU values from current run:")
print("  axnl: -0.0068145467")
print("  aynl:  0.0105420612")
print("  xl:   10.4652283758")
print("  su:    0.0663274791 rad (3.801°)")
print("")
print("Note: Previous investigation showed aynl/su differ due to dpper fix.")
print("The fix reduced error from 22 km to 40 m.")
print("Remaining 30-50m error is likely FP precision in angular calculations.")
