#!/usr/bin/env python3
"""
Trace intermediate values from python-sgp4 for deep space debugging.
Focus on the short period periodics: sinu, cosu, su calculation.

This script patches python-sgp4 to print intermediate values for comparison
with the CUDA implementation.
"""

import math
from sgp4 import api as sgp4_api

# GPS BIIR-2 (PRN 13) - irez=0 non-resonant MEO satellite
LINE1 = '1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993'
LINE2 = '2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789'

# Time offset in minutes for propagation
TSINCE_MINUTES = 10.0

def trace_propagation():
    """Modified propagation that prints intermediate values."""
    sat = sgp4_api.Satrec.twoline2rv(LINE1, LINE2)
    
    print("=" * 70)
    print("python-sgp4 Intermediate Value Trace")
    print("=" * 70)
    print(f"Satellite: GPS BIIR-2 (PRN 13)")
    print(f"TLE epoch: JD {sat.jdsatepoch} + {sat.jdsatepochF}")
    print(f"Propagation time: t = {TSINCE_MINUTES} minutes")
    print()
    
    # Access internal state
    print("--- Initial Parameters (from sgp4init) ---")
    print(f"no_kozai:   {sat.no_kozai:.15f} rad/min")
    print(f"inclo:      {sat.inclo:.10f} rad ({math.degrees(sat.inclo):.6f}°)")
    print(f"ecco:       {sat.ecco:.10f}")
    print(f"argpo:      {sat.argpo:.10f} rad")
    print(f"nodeo:      {sat.nodeo:.10f} rad")
    print(f"mo:         {sat.mo:.10f} rad")
    print(f"bstar:      {sat.bstar:.10e}")
    print()
    
    print("--- Deep Space Parameters ---")
    print(f"is_deep_space (method): '{sat.method}'")
    print(f"irez:       {getattr(sat, 'irez', 'N/A')}")
    print(f"gsto:       {sat.gsto:.10f} rad")
    print(f"aycof:      {sat.aycof:.15f}")
    print(f"xlcof:      {sat.xlcof:.15f}")
    print(f"con41:      {sat.con41:.15f}")
    print(f"x1mth2:     {sat.x1mth2:.15f}")
    print(f"x7thm1:     {sat.x7thm1:.15f}")
    print(f"j3oj2:      {getattr(sat, 'j3oj2', 'N/A')}")
    print()
    
    print("--- Secular Rate Coefficients ---")
    print(f"dedt:       {getattr(sat, 'dedt', 0):.15e}")
    print(f"didt:       {getattr(sat, 'didt', 0):.15e}")
    print(f"dmdt:       {getattr(sat, 'dmdt', 0):.15e}")
    print(f"domdt:      {getattr(sat, 'domdt', 0):.15e}")
    print(f"dnodt:      {getattr(sat, 'dnodt', 0):.15e}")
    print()
    
    # Now propagate and check final result
    error_code, r, v = sat.sgp4(sat.jdsatepoch, sat.jdsatepochF + TSINCE_MINUTES / 1440.0)
    
    if error_code != 0:
        print(f"ERROR: sgp4 returned error code {error_code}")
        return
    
    print("--- Final Result ---")
    print(f"Position: [{r[0]:.6f}, {r[1]:.6f}, {r[2]:.6f}] km")
    print(f"Velocity: [{v[0]:.9f}, {v[1]:.9f}, {v[2]:.9f}] km/s")
    print()
    
    # Now manually trace the propagation step-by-step
    # Import the propagation module to access internal functions
    from sgp4 import propagation
    from sgp4 import model
    
    print("=" * 70)
    print("Step-by-Step Propagation Trace")
    print("=" * 70)
    
    # Convert time to minutes since epoch
    tsince = TSINCE_MINUTES
    
    # Get initial elements
    xmo = sat.mo
    xno = sat.no_kozai
    em = sat.ecco
    inclm = sat.inclo
    argpm = sat.argpo
    nodem = sat.nodeo
    mm = sat.mo
    
    # Apply deep space secular effects (from propagation.py)
    print("\n--- After dspace (secular effects) ---")
    # These would be modified by dspace for t > 0
    # For accurate trace, we need to look at python-sgp4's actual computation
    
    # The key variables we need to trace:
    # 1. After dpper (periodic effects): inclm, argpm, nodem, em, mm
    # 2. Long period: axnl, aynl, xl
    # 3. Kepler: u, eo1
    # 4. Short period: sineo1, coseo1, am, rl, sinu, cosu, su
    
    print("\nNOTE: For detailed tracing, we need to modify python-sgp4 source")
    print("or use a patched version with debug output.")
    print()
    print("Key intermediate values to compare with CUDA:")
    print("  - axnl = em * cos(argpm)")
    print("  - aynl = em * sin(argpm) + temp * aycof_eff")
    print("  - sinu = am/rl * (sineo1 - aynl - axnl*temp)")
    print("  - cosu = am/rl * (coseo1 - axnl + aynl*temp)")
    print()


def create_patched_sgp4():
    """Create a patched version of sgp4 propagation for debugging."""
    import sgp4.propagation as prop
    import sys
    import os
    
    # Get the path to sgp4 propagation.py
    prop_path = prop.__file__
    print(f"\npython-sgp4 propagation module: {prop_path}")
    
    # Read the source
    with open(prop_path, 'r') as f:
        source = f.read()
    
    # Find the section after Kepler's equation that computes sinu/cosu
    # Look for the pattern
    if 'sinu = temp2 * (sineo1 - ay - ax * temp3)' in source:
        print("Found sinu/cosu calculation in python-sgp4 source")
        
        # Show the relevant section
        lines = source.split('\n')
        for i, line in enumerate(lines):
            if 'sinu = temp2' in line or 'cosu = temp2' in line or 'su = atan2' in line:
                print(f"  Line {i+1}: {line}")
    else:
        print("Could not find sinu/cosu calculation pattern")


def trace_at_epoch():
    """Trace values at t=0 (epoch propagation)."""
    sat = sgp4_api.Satrec.twoline2rv(LINE1, LINE2)
    
    print("=" * 70)
    print("Epoch (t=0) Propagation")
    print("=" * 70)
    
    error_code, r, v = sat.sgp4(sat.jdsatepoch, sat.jdsatepochF)
    
    if error_code != 0:
        print(f"ERROR: sgp4 returned error code {error_code}")
        return
    
    print(f"Position: [{r[0]:.6f}, {r[1]:.6f}, {r[2]:.6f}] km")
    print(f"Velocity: [{v[0]:.9f}, {v[1]:.9f}, {v[2]:.9f}] km/s")
    
    # Calculate magnitude
    pos_mag = math.sqrt(r[0]**2 + r[1]**2 + r[2]**2)
    vel_mag = math.sqrt(v[0]**2 + v[1]**2 + v[2]**2)
    print(f"Position magnitude: {pos_mag:.6f} km")
    print(f"Velocity magnitude: {vel_mag:.9f} km/s")
    print()


if __name__ == '__main__':
    trace_at_epoch()
    print()
    trace_propagation()
    create_patched_sgp4()
