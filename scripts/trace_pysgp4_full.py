#!/usr/bin/env python3
"""
Comprehensive trace of python-sgp4 intermediate values for deep space debugging.
Uses the old-style Python API to access internal state variables.

Compares with expected CUDA values to identify the source of the 18 km error.
"""

import math
from sgp4.earth_gravity import wgs72
from sgp4.io import twoline2rv
from sgp4 import propagation

# GPS BIIR-2 (PRN 13) - irez=0 non-resonant MEO satellite
LINE1 = '1 24876U 97035A   25105.50000000 -.00000012  00000+0  00000+0 0  9993'
LINE2 = '2 24876  55.4567 234.5678 0123456 123.4567 236.5432  2.00565123456789'

# Time offset in minutes for propagation  
TSINCE_MINUTES = 10.0

def trace_full_propagation():
    """Trace all intermediate values during propagation."""
    sat = twoline2rv(LINE1, LINE2, wgs72)
    
    print("=" * 70)
    print("python-sgp4 Full Intermediate Value Trace (Old API)")
    print("=" * 70)
    print(f"Satellite: GPS BIIR-2 (PRN 13)")
    print(f"TLE epoch: JD {sat.jdsatepoch}")
    print()
    
    # Print initialization parameters
    print("=" * 70)
    print("INITIALIZATION PARAMETERS")
    print("=" * 70)
    
    print("\n--- Orbital Elements ---")
    print(f"no_kozai:     {sat.no_kozai:.15f} rad/min")
    print(f"no_unkozai:   {sat.no_unkozai:.15f} rad/min")
    print(f"inclo:        {sat.inclo:.15f} rad ({math.degrees(sat.inclo):.10f}°)")
    print(f"ecco:         {sat.ecco:.15f}")
    print(f"argpo:        {sat.argpo:.15f} rad")
    print(f"nodeo:        {sat.nodeo:.15f} rad (RAAN)")
    print(f"mo:           {sat.mo:.15f} rad")
    print(f"bstar:        {sat.bstar:.15e}")
    
    print("\n--- Classification ---")
    print(f"method:       '{sat.method}' (d=deep space, n=near earth)")
    print(f"irez:         {sat.irez} (0=none, 1=sync, 2=half-day)")
    print(f"isimp:        {sat.isimp}")
    
    print("\n--- Gravitational Constants (WGS-72) ---")
    print(f"j2:           {sat.j2:.15f}")
    print(f"j3:           {sat.j3:.15e}")
    print(f"j4:           {sat.j4:.15e}")
    print(f"j3oj2:        {sat.j3oj2:.15f}")
    print(f"xke:          {sat.xke:.15f}")
    print(f"mu:           {sat.mu:.15f} km³/s²")
    
    print("\n--- Long Period Coefficients (from init) ---")
    print(f"aycof:        {sat.aycof:.15f}")
    print(f"xlcof:        {sat.xlcof:.15f}")
    
    print("\n--- Short Period Coefficients (from init) ---")
    print(f"con41:        {sat.con41:.15f}")
    print(f"x1mth2:       {sat.x1mth2:.15f}")
    print(f"x7thm1:       {sat.x7thm1:.15f}")
    
    print("\n--- Deep Space Secular Rates ---")
    print(f"dedt:         {sat.dedt:.15e}")
    print(f"didt:         {sat.didt:.15e}")
    print(f"dmdt:         {sat.dmdt:.15e}")
    print(f"domdt:        {sat.domdt:.15e}")
    print(f"dnodt:        {sat.dnodt:.15e}")
    
    print("\n--- Solar/Lunar Mean Anomalies ---")
    print(f"zmos:         {sat.zmos:.15f}")
    print(f"zmol:         {sat.zmol:.15f}")
    print(f"gsto:         {sat.gsto:.15f}")
    
    print("\n--- Deep Space Periodic Coefficients (Solar) ---")
    print(f"se2:          {sat.se2:.15e}")
    print(f"se3:          {sat.se3:.15e}")
    print(f"si2:          {sat.si2:.15e}")
    print(f"si3:          {sat.si3:.15e}")
    print(f"sl2:          {sat.sl2:.15e}")
    print(f"sl3:          {sat.sl3:.15e}")
    print(f"sl4:          {sat.sl4:.15e}")
    print(f"sgh2:         {sat.sgh2:.15e}")
    print(f"sgh3:         {sat.sgh3:.15e}")
    print(f"sgh4:         {sat.sgh4:.15e}")
    print(f"sh2:          {sat.sh2:.15e}")
    print(f"sh3:          {sat.sh3:.15e}")
    
    print("\n--- Deep Space Periodic Coefficients (Lunar) ---")
    print(f"ee2:          {sat.ee2:.15e}")
    print(f"e3:           {sat.e3:.15e}")
    print(f"xi2:          {sat.xi2:.15e}")
    print(f"xi3:          {sat.xi3:.15e}")
    print(f"xl2:          {sat.xl2:.15e}")
    print(f"xl3:          {sat.xl3:.15e}")
    print(f"xl4:          {sat.xl4:.15e}")
    print(f"xgh2:         {sat.xgh2:.15e}")
    print(f"xgh3:         {sat.xgh3:.15e}")
    print(f"xgh4:         {sat.xgh4:.15e}")
    print(f"xh2:          {sat.xh2:.15e}")
    print(f"xh3:          {sat.xh3:.15e}")
    
    print("\n--- Baseline Periodics (for dpper) ---")
    print(f"peo:          {sat.peo:.15f}")
    print(f"pinco:        {sat.pinco:.15f}")
    print(f"plo:          {sat.plo:.15f}")
    print(f"pgho:         {sat.pgho:.15f}")
    print(f"pho:          {sat.pho:.15f}")
    
    # Now propagate and trace step-by-step
    print("\n" + "=" * 70)
    print(f"PROPAGATION at t = {TSINCE_MINUTES} minutes")
    print("=" * 70)
    
    # Call propagate with tsince (old API uses .propagate())
    r, v = sat.propagate(0, 0, 0, TSINCE_MINUTES, 0)
    error_code = sat.error
    
    if error_code != 0:
        print(f"ERROR: sgp4 returned error code {error_code}")
        return
    
    print("\n--- State After Propagation ---")
    print(f"t (internal):  {sat.t:.15f} min")
    print(f"am:            {sat.am:.15f} ER")
    print(f"em:            {sat.em:.15f}")
    print(f"im:            {sat.im:.15f} rad ({math.degrees(sat.im):.10f}°)")
    print(f"Om:            {sat.Om:.15f} rad (RAAN)")
    print(f"om:            {sat.om:.15f} rad (argp)")
    print(f"mm:            {sat.mm:.15f} rad (mean anomaly)")
    print(f"nm:            {sat.nm:.15f} rad/min")
    
    print("\n--- Final Result ---")
    print(f"Position: [{r[0]:.10f}, {r[1]:.10f}, {r[2]:.10f}] km")
    print(f"Velocity: [{v[0]:.12f}, {v[1]:.12f}, {v[2]:.12f}] km/s")
    
    pos_mag = math.sqrt(r[0]**2 + r[1]**2 + r[2]**2)
    vel_mag = math.sqrt(v[0]**2 + v[1]**2 + v[2]**2)
    print(f"Position magnitude: {pos_mag:.10f} km")
    print(f"Velocity magnitude: {vel_mag:.12f} km/s")
    
    # Check t=0 (epoch) propagation
    print("\n" + "=" * 70)
    print("PROPAGATION at t = 0 minutes (EPOCH)")
    print("=" * 70)
    
    sat2 = twoline2rv(LINE1, LINE2, wgs72)
    r0, v0 = sat2.propagate(0, 0, 0, 0, 0)
    error_code = sat2.error
    
    print(f"\nPosition: [{r0[0]:.10f}, {r0[1]:.10f}, {r0[2]:.10f}] km")
    print(f"Velocity: [{v0[0]:.12f}, {v0[1]:.12f}, {v0[2]:.12f}] km/s")
    
    # Show the mm value at t=0
    print(f"\nmm at t=0:     {sat2.mm:.15f} rad")
    print(f"em at t=0:     {sat2.em:.15f}")
    print(f"im at t=0:     {sat2.im:.15f} rad ({math.degrees(sat2.im):.10f}°)")
    

def manual_propagation_trace():
    """Manually trace through the propagation to see intermediate values."""
    
    print("\n" + "=" * 70)
    print("MANUAL STEP-BY-STEP TRACE (t=10 min)")
    print("=" * 70)
    
    sat = twoline2rv(LINE1, LINE2, wgs72)
    tsince = TSINCE_MINUTES
    
    # Get constants
    xke = sat.xke
    j2 = sat.j2
    j3oj2 = sat.j3oj2
    
    # From initialization  
    no = sat.no_unkozai
    ecco = sat.ecco
    inclo = sat.inclo
    argpo = sat.argpo
    nodeo = sat.nodeo
    mo = sat.mo
    
    print("\n--- Step 1: Deep Space Secular Effects ---")
    # For irez=0, secular effects are applied as:
    # em = ecco + dedt * t
    # inclm = inclo + didt * t
    # argpm = argpo + domdt * t
    # nodem = nodeo + dnodt * t
    # mm = mo + dmdt * t + no * t
    # nm = no (unchanged for irez=0)
    
    em_secular = ecco + sat.dedt * tsince
    inclm_secular = inclo + sat.didt * tsince
    argpm_secular = argpo + sat.domdt * tsince
    nodem_secular = nodeo + sat.dnodt * tsince
    mm_secular = mo + sat.dmdt * tsince + no * tsince
    nm = no
    
    print(f"em (after secular):     {em_secular:.15f}")
    print(f"inclm (after secular):  {inclm_secular:.15f} rad")
    print(f"argpm (after secular):  {argpm_secular:.15f} rad")
    print(f"nodem (after secular):  {nodem_secular:.15f} rad")
    print(f"mm (after secular):     {mm_secular:.15f} rad")
    print(f"nm:                     {nm:.15f} rad/min")
    
    print("\n--- Step 2: Deep Space Periodic (dpper) ---")
    # This applies lunar-solar periodics
    # For now, assume small perturbations
    # The actual dpper would compute these
    
    # We need to trace the actual dpper output
    # Since we can't easily hook into it, we'll compare with the
    # values stored in sat after propagation
    
    sat_test = twoline2rv(LINE1, LINE2, wgs72)
    sat_test.propagate(0, 0, 0, tsince, 0)
    
    print(f"em (after dpper):       {sat_test.em:.15f}")
    print(f"im (after dpper):       {sat_test.im:.15f} rad")
    print(f"Om (after dpper):       {sat_test.Om:.15f} rad")
    print(f"om (after dpper):       {sat_test.om:.15f} rad")
    print(f"mm (after dpper):       {sat_test.mm:.15f} rad")
    
    # These are the key values for long period periodics
    em = sat_test.em
    inclm = sat_test.im
    argpm = sat_test.om  # Note: om = argpm after dpper
    nodem = sat_test.Om
    mm = sat_test.mm
    am = sat_test.am
    
    print("\n--- Step 3: Long Period Periodics ---")
    sinip = math.sin(inclm)
    cosip = math.cos(inclm)
    
    # python-sgp4 RECALCULATES aycof/xlcof for deep space!
    # This is the key difference we're investigating
    aycof_recalc = -0.5 * j3oj2 * sinip
    if abs(cosip + 1.0) > 1.5e-12:
        xlcof_recalc = -0.25 * j3oj2 * sinip * (3.0 + 5.0 * cosip) / (1.0 + cosip)
    else:
        xlcof_recalc = -0.25 * j3oj2 * sinip * (3.0 + 5.0 * cosip) / 1.5e-12
    
    print(f"sinip:                  {sinip:.15f}")
    print(f"cosip:                  {cosip:.15f}")
    print(f"aycof (from init):      {sat.aycof:.15f}")
    print(f"aycof (recalculated):   {aycof_recalc:.15f}")
    print(f"xlcof (from init):      {sat.xlcof:.15f}")
    print(f"xlcof (recalculated):   {xlcof_recalc:.15f}")
    
    # Continue with long period calculations
    sinargpm = math.sin(argpm)
    cosargpm = math.cos(argpm)
    
    axnl = em * cosargpm
    temp = 1.0 / (am * (1.0 - em * em))
    aynl = em * sinargpm + temp * aycof_recalc
    xl = mm + argpm + nodem + temp * xlcof_recalc * axnl
    
    print(f"\nsinargpm:               {sinargpm:.15f}")
    print(f"cosargpm:               {cosargpm:.15f}")
    print(f"axnl:                   {axnl:.15f}")
    print(f"temp (1/(am*(1-e²))):   {temp:.15f}")
    print(f"aynl:                   {aynl:.15f}")
    print(f"xl:                     {xl:.15f} rad")
    
    print("\n--- Step 4: Kepler's Equation ---")
    u = (xl - nodem) % (2.0 * math.pi)
    eo1 = u
    
    for _ in range(10):
        sineo1 = math.sin(eo1)
        coseo1 = math.cos(eo1)
        tem5 = 1.0 - coseo1 * axnl - sineo1 * aynl
        tem5 = (u - aynl * coseo1 + axnl * sineo1 - eo1) / tem5
        if abs(tem5) < 1.0e-12:
            break
        if abs(tem5) >= 0.95:
            tem5 = 0.95 if tem5 > 0 else -0.95
        eo1 = eo1 + tem5
    
    print(f"u (xl - nodem):         {u:.15f} rad")
    print(f"eo1 (eccentric anom):   {eo1:.15f} rad")
    
    print("\n--- Step 5: Short Period Periodics (sinu/cosu) ---")
    sineo1 = math.sin(eo1)
    coseo1 = math.cos(eo1)
    
    ecose = axnl * coseo1 + aynl * sineo1
    esine = axnl * sineo1 - aynl * coseo1
    el2 = axnl * axnl + aynl * aynl
    pl = am * (1.0 - el2)
    rl = am * (1.0 - ecose)
    betal = math.sqrt(1.0 - el2)
    temp = esine / (1.0 + betal)
    
    print(f"sineo1:                 {sineo1:.15f}")
    print(f"coseo1:                 {coseo1:.15f}")
    print(f"ecose:                  {ecose:.15f}")
    print(f"esine:                  {esine:.15f}")
    print(f"el2:                    {el2:.15f}")
    print(f"pl:                     {pl:.15f} ER")
    print(f"rl:                     {rl:.15f} ER")
    print(f"betal:                  {betal:.15f}")
    print(f"temp (esine/(1+betal)): {temp:.15f}")
    
    # THE CRITICAL CALCULATION
    sinu = am / rl * (sineo1 - aynl - axnl * temp)
    cosu = am / rl * (coseo1 - axnl + aynl * temp)
    su = math.atan2(sinu, cosu)
    
    print(f"\n*** CRITICAL VALUES ***")
    print(f"am/rl:                  {am/rl:.15f}")
    print(f"sinu:                   {sinu:.15f}")
    print(f"cosu:                   {cosu:.15f}")
    print(f"su (atan2):             {su:.15f} rad ({math.degrees(su):.10f}°)")
    
    # Compare with recalculated con41/x1mth2/x7thm1
    cosisq = cosip * cosip
    con41_recalc = 3.0 * cosisq - 1.0
    x1mth2_recalc = 1.0 - cosisq
    x7thm1_recalc = 7.0 * cosisq - 1.0
    
    print(f"\n--- Short Period Corrections ---")
    print(f"con41 (from init):      {sat.con41:.15f}")
    print(f"con41 (recalculated):   {con41_recalc:.15f}")
    print(f"x1mth2 (from init):     {sat.x1mth2:.15f}")
    print(f"x1mth2 (recalculated):  {x1mth2_recalc:.15f}")
    print(f"x7thm1 (from init):     {sat.x7thm1:.15f}")
    print(f"x7thm1 (recalculated):  {x7thm1_recalc:.15f}")
    

if __name__ == '__main__':
    trace_full_propagation()
    manual_propagation_trace()
