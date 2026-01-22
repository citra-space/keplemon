# SGP4 vs SDP4: Propagator Selection and Error Analysis

## Quick Reference

| Orbit Type | Period | Mean Motion | Propagator | Altitude |
|------------|--------|-------------|------------|----------|
| **LEO** | < 225 min | > 6.4 rev/day | **SGP4** | < 5,878 km |
| **MEO/GEO/HEO** | ≥ 225 min | ≤ 6.4 rev/day | **SDP4** | ≥ 5,878 km |

**Critical Rule**: The threshold is **225 minutes orbital period** (or **6.4 revolutions/day**)

## What Each Propagator Includes

### SGP4 (Simplified General Perturbations 4)
**For near-earth orbits (period < 225 min)**

Includes:
- ✅ J2 gravitational harmonic (Earth oblateness)
- ✅ J3 gravitational harmonic (pear-shaped Earth)
- ✅ J4 gravitational harmonic (higher-order oblateness)
- ✅ Atmospheric drag (via B* drag coefficient)
- ✅ Short-period perturbations

**Does NOT include:**
- ❌ Lunar gravity perturbations
- ❌ Solar gravity perturbations
- ❌ Resonance effects
- ❌ Long-period perturbation integrator

### SDP4 (Simplified Deep-Space Perturbations 4)
**For deep-space orbits (period ≥ 225 min)**

Includes everything SGP4 has, PLUS:
- ✅ **Lunar gravity** (secular and periodic terms)
- ✅ **Solar gravity** (secular and periodic terms)
- ✅ **Resonance effects** (12-hour and 24-hour orbits)
- ✅ **Deep-space integrator** (720-minute time steps)
- ✅ **Lyddane coordinate system** (better for high-altitude orbits)

## Error from Using the Wrong Propagator

### Using SGP4 at GEO Altitude ❌

**Expected Position Errors:**

| Time Period | Estimated Error | Cause |
|-------------|----------------|-------|
| 1 day | **5-10 km** | Missing lunar/solar daily effects |
| 7 days | **30-70 km** | Long-period perturbations accumulate |
| 15 days | **100-200 km** | Compare to ≤40 km with proper SDP4 |
| 30 days | **200-500 km** | Resonance errors dominate |
| 1 year | **1000+ km** | Secular drift, inclination errors |

**Published Research Findings:**
- Proper **SDP4 at GEO**: ~1.9 km average accuracy
- **15-day SDP4 predictions**: ≤40 km error
- Using **wrong propagator**: "significantly larger errors"

### Why Errors Grow So Large

1. **Comparable Force Magnitudes**
   - At GEO altitude (~35,786 km), Earth's oblateness and lunisolar gravitational forces are **comparable in magnitude**
   - SGP4 only models Earth's oblateness
   - Missing 50% of the dominant forces!

2. **Lunar Effects** (Moon at ~384,400 km from Earth)
   - Causes large amplitude, long-period perturbations
   - Periodic effects with ~27-day lunar month cycle
   - Unmodeled → errors accumulate linearly

3. **Solar Effects** (Sun at ~150M km from Earth)
   - Causes secular drift in orbital elements
   - Perigee height changes at ~1 km/day over months
   - Inclination drifts ~0.03° after 11 years
   - Unmodeled → quadratic error growth

4. **24-Hour Resonance**
   - GEO satellites are in **1:1 resonance** with Earth's rotation
   - Resonance amplifies certain perturbations
   - SDP4 includes specific resonance terms for 24-hour orbits
   - Without these → rapid orbital element drift

### Using SDP4 for LEO ✅ (Safe but Slower)

Using SDP4 for LEO orbits is **safe** but **computationally expensive**:
- ✅ Still produces correct results (lunar/solar effects are small at LEO)
- ❌ ~4x more computation than SGP4
- ❌ GPU performance: 5.8M props/sec (vs 439M props/sec with SGP4)

The keplemon implementation automatically chooses the correct propagator based on mean motion.

## Physical Intuition

### Why 225 minutes?

At orbital periods ≥ 225 minutes:
- Satellite is far enough from Earth that lunar/solar gravity becomes significant
- Earth's gravitational harmonics (J2, J3, J4) decrease with altitude
- Third-body effects increase with altitude
- The crossover point is where these effects become comparable

### Force Scaling

| Altitude | J2 Effect | Lunar/Solar Effect | Ratio |
|----------|-----------|-------------------|-------|
| LEO (400 km) | Dominant | Negligible | 1000:1 |
| MEO (20,000 km) | Strong | Weak | 100:1 |
| **GEO (35,786 km)** | **Moderate** | **Moderate** | **1:1** |

## Implementation in Keplemon

The library automatically selects the correct propagator:

```rust
use keplemon::elements::TLE;
use keplemon::bodies::Satellite;

// LEO satellite (ISS)
let leo_tle = TLE::from_lines(
    "1 25544U 98067A   25105.52083333  .00012345  00000+0  22013-3 0  9991",
    "2 25544  51.6456 339.5765 0003456  35.8734  85.9834 15.48919755123456",
    Some("ISS")
)?;
// Mean motion: 15.49 rev/day → SGP4 automatically selected ✓

// GEO satellite (TDRS 3)
let geo_tle = TLE::from_lines(
    "1 19548U 88091B   26008.31539492 -.00000299  00000+0  00000+0 0  9994",
    "2 19548  12.7229 342.0612 0044050 345.9566 204.1506  1.00262839123781",
    Some("TDRS 3")
)?;
// Mean motion: 1.00 rev/day → SDP4 automatically selected ✓
```

No manual selection needed - the library handles it correctly.

## GPU Performance Impact

From benchmark results (`docs/gpu-benchmark-results.md`):

| Propagator | GPU Speedup | GPU Throughput | Complexity |
|------------|-------------|----------------|------------|
| **SGP4 (LEO)** | **83x** | **439M props/sec** | Low |
| **SDP4 (GEO)** | **1.6x** | **5.8M props/sec** | High |

**Why is SDP4 75x slower on GPU?**
- More complex calculations (~4x more operations)
- Deep conditional branching (resonance detection, coordinate conversion)
- Iterative solvers (Newton-Raphson)
- Poor GPU parallelism (warp divergence)

## References

1. **"Analysis on the Accuracy of the SGP4/SDP4 Model"**
   - Comprehensive accuracy analysis across orbit regimes

2. **"High-Integrity TLE Error Models for MEO and GEO Satellites"** (Virginia Tech)
   - Statistical characterization of 148,984 MEO TLEs and 5,813 GEO TLEs
   - [PDF available](https://www.aoe.vt.edu/content/dam/aoe_vt_edu/people/faculty/joerger/publications/2018_aiaa_space_forum_paper_racelis-joerger.pdf)

3. **"Lunar and solar perturbations on geosynchronous satellite orbits"**
   - Analysis of third-body effects at GEO altitude

4. **Celestrak - "Revisiting Spacetrack Report #3" (AIAA 2006-6753-Rev2)**
   - Official SGP4/SDP4 specification and implementation guide

5. **Wikipedia: Simplified Perturbations Models**
   - [Good overview](https://en.wikipedia.org/wiki/Simplified_perturbations_models) of the model family
