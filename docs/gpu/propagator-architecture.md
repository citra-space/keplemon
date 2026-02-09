# GPU Propagator Architecture Guide

**Document Version**: 2.0
**Last Updated**: January 2026
**Keplemon Version**: 3.3.0

## Propagator Summary

Keplemon provides **5 propagators** optimized for different orbital regimes and use cases:

| Propagator | Type | Orbit Regime | GPU Speedup | Accuracy @ 7 Days | Auto-Selected |
|------------|------|--------------|-------------|-------------------|---------------|
| **CudaTlePropagator (SGP4)** | TLE-based | LEO (period < 225 min) | **83x** | 4.2m | Yes |
| **CudaSdp4InterpolatedPropagator** | TLE-based | GEO/MEO (period ≥ 225 min) | **20-50x** | 2.4 km ⚠️ | Yes |
| **CudaGeoNumericalPropagator** | ECI-based | GEO/MEO | Slower than SDP4 | ~12 km @ 7d (experimental) | No |
| **CudaTlePropagator (SDP4)** | TLE-based | GEO/MEO (legacy) | 1.6x | <0.001m | No |
| **SAAL CPU Propagator** | TLE-based | All orbits | 1x | Baseline reference | Fallback |

### Quick Start

**Recommended (Automatic Selection)**:
```rust
use keplemon::propagation::BatchPropagator;

let batch_prop = BatchPropagator::new();
let results = batch_prop.propagate_batch(&tles, &epochs)?;
// Automatically selects SGP4 (LEO) or SDP4-Interpolated (GEO)
```

**When to Use Each Propagator**:
- **CudaTlePropagator (SGP4)**: LEO satellites, large constellations (Starlink, OneWeb)
- **CudaSdp4InterpolatedPropagator**: GEO/MEO satellites when TLE compatibility + speed needed (km-level accuracy acceptable)
- **CudaGeoNumericalPropagator**: *Experimental* — GEO/MEO when you have ECI states (no TLE), short-term only (<7 days)
- **CudaTlePropagator (SDP4 standard)**: GEO/MEO when sub-meter accuracy needed (1.6x GPU speedup)
- **SAAL CPU**: Small batches (<1000 ops), GPU unavailable, validation reference, highest accuracy

---

## Table of Contents

1. [Propagator Summary](#propagator-summary) ⭐ Quick Reference
2. [Overview](#overview)
3. [Propagator Types](#propagator-types)
4. [Selection Decision Tree](#selection-decision-tree)
5. [Fallback Mechanisms](#fallback-mechanisms)
6. [Performance Comparison](#performance-comparison)
7. [Strengths and Weaknesses](#strengths-and-weaknesses)
8. [API Usage Guide](#api-usage-guide)
9. [Accuracy Verification](#accuracy-verification)
10. [Future Roadmap](#future-roadmap)

---

## Overview

Keplemon's GPU acceleration provides **5 propagators** optimized for different orbital regimes and use cases. The system automatically selects the optimal TLE-based propagator (SGP4 or SDP4-Interpolated) based on orbital characteristics, with intelligent fallback to CPU when GPU is unavailable. The GEO Numerical propagator is available as an experimental option for short-term ECI-based propagation when no TLE is available.

**Key Features**:
- Automatic backend selection (CPU vs GPU)
- Automatic propagator selection (SGP4 vs SDP4-Interpolated based on orbital period)
- Size-based thresholding for efficient small-batch handling
- Dual API modes (CPU-resident and GPU-resident results)
- Rigorous accuracy verification (SGP4: <2m, SDP4-Interpolated: 2-10 km)
- Zero-copy optimization for chained GPU operations
- Mixed-batch optimization (partitions LEO/GEO for optimal GPU utilization)

---

## Propagator Types

### 1. CudaTlePropagator (Primary - TLE-based)

**Purpose**: Universal TLE-based propagation for all orbit regimes

**Algorithms**:
- **SGP4** - Near-earth satellites (period < 225 min)
- **SDP4** - Deep-space satellites (period ≥ 225 min)

**Implementation**:
- Location: `src/gpu/cuda_tle.rs` (1,470 lines, renamed from `cuda_sgp4.rs`)
- Kernels: `tle_propagator_init.cu`, `tle_propagator_batch.cu`
- Uses Vallado's SGP4/SDP4 reference algorithms
- WGS-72 OLD (1972) gravitational constants for TLE compatibility

**Note**: Previously named `CudaSgp4Propagator`. Renamed to `CudaTlePropagator` in v3.3.0 to accurately reflect that it handles both SGP4 and SDP4 algorithms. This is a breaking change - the old name has been removed.

**Performance**:
| Orbit Type | GPU Speedup | Throughput | Notes |
|------------|-------------|------------|-------|
| LEO (SGP4) | **83x** | 439M props/sec | Excellent parallelism |
| GEO (SDP4) | **1.6x** | 5.8M props/sec | Limited by iteration loops |
| Mixed (60/40) | **3.5x** | 14.7M props/sec | Two-kernel optimization |

**Status**: ✅ Production Ready

---

### 2. CudaGeoNumericalPropagator (ECI-based)

**Purpose**: Numerical propagation for GEO/MEO satellites from ECI states (no TLE required)

**Models Included**:
- J2-J4 geopotential (secular and long-period terms)
- J22 tesseral harmonic (longitude-dependent drift)
- Lunar/solar third-body gravity (VSOP87/Brown ephemerides)
- Solar radiation pressure (SRP) with Earth shadow model
- RK4 integrator (Cowell's method) with 120s step size

**Implementation**:
- Location: `src/gpu/cuda_geo_numerical.rs` (650+ lines)
- Kernels: `geo_analytical.cu`, `geo_analytical.cuh`
- Uses EGM-96 geopotential model (higher accuracy than WGS-72)
- Operates on ECI states (not constrained to TLE format)

**Performance**:
- Slower than GPU SDP4 due to many RK4 substeps per propagation
- No iteration loops → no warp divergence, but high per-thread cost
- Suitable for short-term propagation (<7 days)

**Limitations**:
- Slower than both GPU SDP4 Standard and SDP4-Interpolated
- Cowell's method accumulates truncation error: ~12 km @ 7d, ~689 km @ 30d vs CPU SDP4
- Not recommended for propagation spans >7 days
- Not TLE-compatible (requires ECI state vectors as input)
- Not automatically selected by BatchPropagator

**Use case**: Short-term (<7 day) GEO propagation when only ECI states are available (no TLE)

**Status**: ⚠️ Experimental (short-term use only)

---

### 3. CudaSdp4InterpolatedPropagator (Resonance Interpolation)

**Purpose**: SDP4-compatible propagation eliminating iteration loops

**Innovation**:
Pre-samples resonance effects at regular intervals during initialization, then interpolates during propagation. Eliminates the iterative DSPACE loop that causes GPU warp divergence.

**Implementation**:
- Location: `src/gpu/cuda_sdp4_interpolated.rs` (500+ lines)
- Kernels: `sdp4_analytical.cu`, `sdp4_analytical.cuh`
- Pre-samples: 241 samples over ±30 days at 6-hour intervals
- Interpolation: Cubic spline for smooth resonance contribution
- **Automatic Selection**: BatchPropagator automatically selects this propagator for deep-space orbits (period ≥ 225 min)

**Performance**:
- GPU Speedup: **20-50x** (vs 1.6x for standard SDP4)
- Accuracy: 350m @ 1 day, 2.4 km @ 7 days, 10 km @ 30 days (interpolation tuning needed)
- Memory overhead: ~2 KB per satellite for resonance samples
- Throughput: ~100M+ props/sec for GEO satellites

**⚠️ Accuracy Note**: Current interpolation implementation shows growing position errors: 350m @ 1 day, 2.4 km @ 7 days, 10 km @ 30 days. This is adequate for initial mission planning but requires interpolation tuning for high-precision applications. See test warning in `test_sdp4_analytical_cpu_parity`.

**Integration** (v3.3.0):
- Fully integrated into `BatchPropagator` with automatic selection
- Mixed-batch support: LEO satellites use SGP4, GEO satellites use SDP4-Interpolated
- Intelligent partitioning eliminates warp divergence

**Status**: ✅ Production Ready

---

### 4. SGP4-XP Propagator

**Purpose**: Enhanced GEO/MEO propagation for future TLE formats

**Implementation**:
- Detection logic exists in `PropagatorType::for_tle()` (ephemeris_type == 4)
- Falls back to SDP4 when detected
- Placeholder for future Space Force TLE enhancements

**Status**: ❌ Not Implemented

---

### 5. SAAL CPU Reference (Fallback)

**Purpose**: High-accuracy CPU baseline and fallback

**Implementation**:
- Uses python-sgp4 C binding via `saal` crate
- Exact Vallado reference implementation
- WGS-72 OLD constants (matches GPU for verification)
- Rayon parallelization across satellites

**Performance**: 1x baseline (reference speed)

**Status**: ✅ Production Ready

---

## Deep-Space Propagator Technical Comparison

This section explains the fundamental differences between the three deep-space propagators and why they achieve such different performance/accuracy trade-offs.

### Standard SDP4 (CudaTlePropagator in SDP4 mode)

**Algorithm**: Direct GPU port of Vallado's SDP4 reference implementation

**Physics Model**:
- WGS-72 OLD (1972) gravitational constants
- Simplified secular lunar-solar perturbations (long-term drift)
- Periodic lunar-solar perturbations (short-term oscillations)
- Resonance effects for synchronous (GEO), 12-hour (GPS), and Molniya orbits

**Propagation Process**:
```
For each timestep:
  1. Call dspace() function
  2. Run Newton-Raphson iteration to solve for eccentric anomaly:
     while (|error| > tolerance && iterations < 10):
       E_new = E_old - f(E_old) / f'(E_old)
       // Typically 5-10 iterations
       // Different satellites converge in different iterations
  3. Calculate resonance perturbations (depends on current orbital phase)
  4. Apply perturbations to orbital elements: Δa, Δe, Δi, Δω, ΔΩ, ΔM
  5. Convert to position/velocity
```

**GPU Performance Problem - Warp Divergence**:
- GPUs execute threads in groups of 32 (warps)
- All 32 threads in a warp must execute the **same instruction** simultaneously
- When threads take different code paths, inactive threads wait → efficiency loss
- In SDP4, different satellites need different iteration counts (3-10 iterations)
- **Result**: If thread 1 needs 3 iterations and thread 2 needs 10, thread 1 idles for 7 iterations

**Example warp execution**:
```
Satellite 0: ███░░░░░░░ (3 iterations, then waits)
Satellite 1: ██████████ (10 iterations, fully utilized)
Satellite 2: █████░░░░░ (5 iterations, then waits)
...
Warp efficiency: ~50% (half the threads idle at any moment)
```

**Cost per propagation**: ~200 floating-point operations + variable iterations

**Accuracy**: <0.001m (micron-level, essentially perfect)

**GPU Speedup**: 1.6x (limited by warp divergence)

---

### SDP4-Interpolated (CudaSdp4InterpolatedPropagator)

**Algorithm**: Pre-sampled resonance interpolation scheme

**Physics Model**: **Identical to standard SDP4** (same WGS-72, same perturbations)

**Key Innovation**: Replace runtime iteration with pre-computed lookup + interpolation

**Two-Phase Approach**:

#### Phase 1: Initialization (once per satellite)
```
For t = -30 days to +30 days, every 6 hours (241 samples):
  1. Run FULL standard SDP4 algorithm (with iterations)
  2. Store resonance contributions at this time:
     samples[i] = {time, Δa(t), Δe(t), Δi(t), Δω(t), ΔΩ(t), ΔM(t)}
  3. Cost: 241 samples × 200 ops = ~48,000 ops per satellite (one-time)
```

Memory: 241 samples × 6 elements × 8 bytes = ~11.5 KB per satellite (compressed to ~2 KB)

#### Phase 2: Propagation (thousands of times)
```
For each timestep:
  1. Calculate time offset: tsince = (current_time - epoch)
  2. Binary search to find bracketing samples:
     i = find_interval(samples, tsince)  // O(log 241) = ~8 comparisons
  3. Cubic spline interpolation (NO iterations):
     t_frac = (tsince - samples[i].time) / 6_hours
     Δa = cubic(samples[i].Δa, samples[i+1].Δa, t_frac)
     Δe = cubic(samples[i].Δe, samples[i+1].Δe, t_frac)
     // ... interpolate all 6 elements
  4. Apply interpolated perturbations
  5. Convert to position/velocity
```

**GPU Performance Advantages**:
- **No iteration loops** → all threads execute exactly the same instructions
- **No warp divergence** → 100% warp efficiency
- **Fixed execution path** → GPU can maximize parallelism

**Example warp execution**:
```
Satellite 0: ██████████ (50 ops, fully utilized)
Satellite 1: ██████████ (50 ops, fully utilized)
Satellite 2: ██████████ (50 ops, fully utilized)
...
Warp efficiency: ~100% (all threads busy)
```

**Cost per propagation**: ~50 floating-point operations (4x fewer than standard SDP4)

**Speedup sources**:
- 4x fewer operations per propagation (50 vs 200)
- 2-3x from eliminating warp divergence
- **Combined**: 12-30x faster than standard SDP4

**Accuracy Trade-off - Where the Error Comes From**:

Resonance effects are **non-linear** functions of time. Cubic spline interpolation approximates them with piecewise polynomials between sample points (every 6 hours).

**Example for a GEO satellite at t = +3.5 days**:

```
True resonance (from running full SDP4):
  Δa = +0.8234 km (exact)

Pre-computed samples:
  t = 3.0 days: Δa = +0.7012 km
  t = 4.0 days: Δa = +0.9123 km

Cubic interpolation at t = 3.5 days:
  Δa ≈ +0.8109 km (approximate)

Interpolation error: 0.8234 - 0.8109 = 0.0125 km = 12.5 meters
```

**Error Accumulation**:
- Each 6-hour interval contributes ~10-50m interpolation error
- Over 1 day (4 intervals): ~349m total error
- Over 7 days (28 intervals): ~2.4 km total error
- Over 30 days (120 intervals): ~10.2 km total error
- Error grows approximately **linearly** with time

**Potential Improvements** (not yet implemented):
1. **Finer sampling**: Sample every 1 hour (6x more memory, ~6x better accuracy)
2. **Higher-order interpolation**: Quintic splines or Hermite interpolation
3. **Adaptive sampling**: Dense sampling near resonance, sparse elsewhere
4. **Shorter range**: If only propagating ±7 days, sample at higher density

**GPU Speedup**: 20-50x (10-30x faster than standard SDP4)

**Accuracy**: 349m @ 1 day, 2.4 km @ 7 days, 10 km @ 30 days

---

### GEO-Numerical (CudaGeoNumericalPropagator)

**Algorithm**: Numerical integration using RK4 (Cowell's method)

**Physics Model**: **Completely different from SDP4**
- **EGM-96** gravitational model (1996, higher resolution than WGS-72)
- J2, J3, J4 zonal harmonics (oblateness effects)
- J22 tesseral harmonic (longitude-dependent drift)
- Third-body lunar/solar gravity using **VSOP87/Brown ephemerides** (higher fidelity)
- **Solar radiation pressure (SRP)** with cylindrical Earth shadow model
- **Not TLE-compatible** (operates on ECI state vectors, different constants)

**Propagation Process**:
```
For each timestep:
  1. Calculate total acceleration (central gravity + perturbations):
     a_total = -mu/r³·r + a_J2 + a_J3 + a_J4 + a_J22 + a_sun + a_moon + a_SRP
  2. RK4 integration of full state (position, velocity):
     120s nominal step, up to 50,000 substeps
  3. Convert to output format
```

**GPU Performance**:
- **No iteration loops** → no warp divergence
- **Purely formulaic** → excellent parallelism
- Slower than GPU SDP4 due to high per-thread RK4 computation

**Accuracy** (vs CPU SDP4 reference):
- @ 1 day: ~8.5 km
- @ 7 days: ~12 km
- @ 30 days: ~689 km (not recommended for long-term propagation)

**Previous Issues (Fixed)**:
1. **RK4 step size cap** was 100 substeps max with 600s nominal step — for 7-day propagation
   the effective step was 6,048s (1.68 hours), causing catastrophic RK4 truncation error.
   Fixed: 50,000 max substeps, 120s nominal step (2 minutes per step).

**Status**: ⚠️ Experimental (short-term use only)

---

### Summary Comparison

| Aspect | SDP4 Standard | SDP4-Interpolated | GEO-Numerical |
|--------|---------------|-----------------|----------------|
| **Physics** | Vallado SDP4 | Vallado SDP4 | Custom numerical (EGM-96) |
| **Gravity** | WGS-72 OLD | WGS-72 OLD | EGM-96 |
| **Computation** | Iterative (Newton-Raphson) | Pre-sampled interpolation | RK4 (Cowell's method) |
| **Operations/prop** | ~200 + iterations | ~50 (fixed) | ~500 (RK4 substeps) |
| **Warp divergence** | ❌ High (~50% efficiency) | ✅ None (100% efficiency) | ✅ None (100% efficiency) |
| **TLE compatible** | ✅ Yes | ✅ Yes | ❌ No (ECI states) |
| **GPU speedup** | 1.6x | 20-50x | Slower than SDP4 |
| **Accuracy @ 7d** | <0.001m | 2,400m | ~12 km |
| **Status** | ✅ Production | ✅ Production | ⚠️ Experimental |

**Key Insight**: SDP4-Interpolated uses the **exact same physics** as standard SDP4, but trades accuracy for speed by replacing runtime iteration with pre-computed interpolation. GEO-Numerical uses **different physics entirely** (EGM-96, VSOP87, SRP) and integrates the full equations of motion via RK4.

---

## Selection Decision Tree

### Automatic Selection Algorithm (v3.3.0+)

```
┌─────────────────────────────────────────────────────────────┐
│ Input: TLE data, propagation times                         │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
          ┌────────────────┐
          │ GPU Available? │
          └────┬───────────┘
               │
        ┌──────┴──────┐
        │             │
       NO            YES
        │             │
        ▼             ▼
    ┌──────┐    ┌─────────────────────┐
    │ CPU  │    │ n_ops >= threshold? │
    │ SAAL │    │  (default: 1,000)   │
    └──────┘    └────┬───────┬────────┘
                     │       │
                    NO      YES
                     │       │
                     ▼       ▼
                 ┌──────┐  ┌──────────────────────────┐
                 │ CPU  │  │ Classify All Satellites  │
                 │ SAAL │  │   by Orbital Period      │
                 └──────┘  └────────┬─────────────────┘
                                    │
                         ┌──────────┼──────────┐
                         │          │          │
                    All LEO    All GEO      Mixed
                   (< 225 min) (≥ 225 min)
                         │          │          │
                         ▼          ▼          ▼
                   ┌─────────┐ ┌──────────┐ ┌─────────────┐
                   │  SGP4   │ │   SDP4   │ │ Partition & │
                   │ via TLE │ │Analytical│ │  Combine    │
                   │Propagator│ │via SDP4  │ │ (SGP4+SDP4) │
                   └─────────┘ │Analytical│ └─────────────┘
                               └──────────┘
```

**Key Changes in v3.3.0**:
- Automatic selection between SGP4 and SDP4-Interpolated based on orbital period
- Mixed-batch partitioning: LEO satellites → SGP4, GEO satellites → SDP4-Interpolated
- Eliminates manual propagator selection for most use cases
- 20-50x speedup for GEO satellites vs standard SDP4 (1.6x)

### Selection Thresholds

**Orbital Period Threshold**: **225 minutes** (6.4 rev/day mean motion)
- Below: SGP4 (near-earth propagation)
- Above: SDP4 or SDP4-Interpolated (deep-space propagation)

**GPU Batch Size Threshold**: **1,000 operations** (n_satellites × n_times)
- Below: CPU with rayon parallelization (lower overhead)
- Above: GPU (amortizes transfer costs)

**Crossover Points** (when GPU becomes faster than CPU):
- LEO only: ~1-10 satellites (SGP4 very fast on GPU)
- GEO only: ~1,000 satellites (SDP4 limited GPU benefit)
- Mixed workload: ~30-40 satellites (balanced speedup)

### Code References

**Orbit Classification**: `src/propagation/batch_propagator.rs:19-36`
```rust
// Classify orbit type based on orbital period
fn classify_orbit(mean_motion: f64) -> OrbitType {
    let period_minutes = orbital_period_minutes(mean_motion);
    if period_minutes < 225.0 {
        OrbitType::NearEarth  // Use SGP4
    } else {
        OrbitType::DeepSpace  // Use SDP4-Interpolated
    }
}
```

**Automatic Propagator Selection**: `src/propagation/batch_propagator.rs:248-268`
```rust
// Classify all satellites
let orbit_types: Vec<OrbitType> = tles.iter()
    .map(|tle| classify_orbit(tle.get_mean_motion()))
    .collect();

// Route to appropriate propagator
if all_near_earth {
    self.propagate_with_sgp4(tles, epochs)
} else if all_deep_space {
    self.propagate_with_sdp4_analytical(tles, epochs)
} else {
    self.propagate_mixed(tles, epochs, &orbit_types)
}
```

**Backend Selection**: `src/propagation/batch_propagator.rs:111-148`
```rust
match backend {
    PropagationBackend::Auto => {
        if gpu_available && total_ops >= gpu_threshold {
            SelectedBackend::Gpu
        } else {
            SelectedBackend::Cpu
        }
    }
    // ...
}
```

---

## Fallback Mechanisms

### 1. GPU Unavailability Fallback

**Trigger**: CUDA runtime not available or device count == 0

**Action**: Automatic fallback to CPU SAAL propagator

**Location**: `src/gpu/device.rs:59-61`
```rust
pub fn is_cuda_available() -> bool {
    CudarcDevice::count().is_ok()
}
```

**User notification**:
```
warn!("GPU requested but not available, falling back to CPU")
```

---

### 2. Small Batch Size Optimization

**Trigger**: `n_satellites × n_times < gpu_threshold`

**Reason**: Transfer overhead dominates computation time for small batches

**Default threshold**: 1,000 operations

**Performance rationale**:
- PCIe transfer: ~220ms for 560MB data
- SGP4 computation: ~23ms for 10M operations
- Transfer overhead: 90% of total time

**Override**: Set custom threshold with `set_gpu_threshold(n)`

---

### 3. Propagator Override (Testing)

**Purpose**: Force specific propagator for testing/analysis

**Location**: `src/gpu/cuda_tle.rs:554-566`

**Override modes**:
```rust
pub enum PropagatorOverride {
    Auto,        // Automatic selection (default)
    ForceSgp4,   // Force all satellites to SGP4
    ForceSdp4,   // Force all satellites to SDP4
}
```

**⚠️ Warning**: Forcing wrong propagator produces **severely incorrect results**:
- GEO with SGP4: Missing lunar-solar perturbations and resonance (100+ km errors)
- LEO with SDP4: Unnecessary deep-space calculations (slower, no accuracy benefit)

**Use cases**:
- Performance benchmarking pure SGP4 vs SDP4
- Algorithm validation and testing
- Debugging propagator selection logic

---

### 4. Two-Kernel Partition Strategy

**Problem**: Mixed LEO/GEO workloads cause warp divergence
- Threads in same warp execute different code paths
- 50% efficiency loss from idle threads

**Solution**: Partition satellites by type, launch separate kernels

**Location**: `src/gpu/cuda_tle.rs` (internal partitioning), `src/propagation/batch_propagator.rs:418-482` (BatchPropagator mixed routing)

**Algorithm**:
1. **Initialization Phase**: Classify each satellite as SGP4 or SDP4
2. **SGP4 Partition**: Launch kernel with only near-earth satellites
3. **SDP4 Partition**: Launch kernel with only deep-space satellites
4. **GPU-side Scatter**: Each kernel writes directly to correct output position

**Impact**: ~1.9x improvement (1.8x → 3.4x for mixed workloads)

**Memory layout**: Both kernels write to same shared SoA output buffer using original indices

---

## Performance Comparison

### Comprehensive Benchmark Results

**Test Configuration**: 1,000 satellites × 10,080 propagation times (1 week at 1-minute intervals)

#### LEO Only (SGP4)

| Metric | Value |
|--------|-------|
| CPU Time | 1,911 ms |
| GPU Kernel Time | 23 ms |
| GPU Total (kernel + transfer) | 247 ms |
| **Kernel Speedup** | **83.2x** |
| **End-to-end Speedup** | **7.7x** |
| GPU Throughput | 439M props/sec |
| Transfer Overhead | 90% of total time |

**Analysis**: SGP4 achieves excellent GPU parallelism with minimal branching and no iteration loops.

---

#### GEO Only (SDP4)

| Metric | Value |
|--------|-------|
| CPU Time | 2,860 ms |
| GPU Kernel Time | 1,739 ms |
| GPU Total (kernel + transfer) | 1,958 ms |
| **Kernel Speedup** | **1.64x** |
| **End-to-end Speedup** | **1.46x** |
| GPU Throughput | 5.8M props/sec |
| Transfer Overhead | 11% of total time |

**Analysis**: SDP4 limited by:
- Iterative Newton-Raphson solvers for resonance
- Complex conditional branches (synchronous vs 12-hour vs 24-hour orbits)
- Warp divergence from varying iteration counts
- ~200+ operations per propagation (vs ~50 for SGP4)

---

#### Mixed (60% LEO, 40% GEO) - Two-Kernel Optimization

| Metric | Value |
|--------|-------|
| CPU Time | 2,382 ms |
| GPU Kernel Time | 685 ms |
| GPU Total (kernel + transfer) | 893 ms |
| **Kernel Speedup** | **3.48x** |
| **End-to-end Speedup** | **2.67x** |
| GPU Throughput | 14.7M props/sec |
| Transfer Overhead | 23% of total time |

**Analysis**: Two-kernel optimization eliminates warp divergence, achieving balanced speedup.

---

#### GEO/Deep-Space Propagator Comparison

| Propagator | Kernel Speedup | Accuracy @ 7 Days | Status | Notes |
|------------|----------------|-------------------|--------|-------|
| SDP4 (standard) | 1.6x | <0.001m | Legacy | Iterative resonance loops |
| SDP4 Interpolated | **20-50x** | **2.4 km** ⚠️ | **Production** | Pre-sampled resonance, needs tuning |
| GEO Numerical | Slower than SDP4 | ~12 km @ 7d | **Experimental** | RK4 Cowell's method, ECI input, <7d only |

**⚠️ Important Accuracy Considerations** (measured @ 7 days):
- **SDP4 Standard (Legacy)**: Highest accuracy (<0.001m) but slow on GPU (1.6x speedup)
- **SDP4-Interpolated**: Fastest (20-50x) but lower accuracy (2.4 km @ 7 days, 10 km @ 30 days)
- **GEO Numerical**: Experimental, slower than SDP4, ~12 km @ 7d, for short-term ECI-based propagation only

**Recommendation (v3.3.0+)**:
- **Speed Priority**: BatchPropagator automatically selects SDP4-Interpolated for deep-space orbits (2-10 km accuracy)
- **Accuracy Priority**: Force CPU propagation or use standard SDP4 for sub-meter accuracy (<0.001m)
- **ECI Input (no TLE)**: Use CudaGeoNumericalPropagator for short-term (<7d) GEO propagation with ECI state vectors (experimental)

---

### Transfer Bottleneck Analysis

**PCIe Bandwidth**: ~2.5 GB/sec (below PCIe 3.0 theoretical 12 GB/sec)

**Data sizes** (1,000 satellites × 10,080 times):
- Input TLE data: 80 KB (10 doubles × 1,000 sats)
- Output states: 560 MB (7 doubles × 10.08M results)
- Transfer time: ~220 ms

**Transfer Overhead by Workload**:
| Workload | Computation | Transfer | Overhead % |
|----------|-------------|----------|------------|
| LEO (SGP4) | 23 ms | 224 ms | **90%** |
| GEO (SDP4) | 1,739 ms | 219 ms | **11%** |
| Mixed | 685 ms | 208 ms | **23%** |

**GPU-Resident Optimization**: Keep results on GPU to eliminate download overhead
- Enables chaining with other GPU operations (collision detection, visualization)
- 2-10x speedup for large batches when results stay on GPU

---

### Crossover Point Analysis

**When does GPU become faster than CPU?**

| Workload | Crossover Point | Recommendation |
|----------|----------------|----------------|
| LEO only | ~1-10 satellites | Use GPU for almost all batches |
| GEO only | ~1,000 satellites | Use CPU for <1000 satellites |
| Mixed (60/40) | ~30-40 satellites | Use GPU for typical batches |

**Factors affecting crossover**:
- PCIe bandwidth (older systems have higher crossover)
- CPU core count (more cores → higher crossover)
- Propagation time span (more times → lower crossover)

---

## Strengths and Weaknesses

### CudaTlePropagator (SGP4)

**Strengths** ✅:
- Excellent GPU parallelism (83x speedup)
- Minimal branching and iteration
- Low memory footprint (~80 bytes per satellite)
- Production-proven accuracy (<2m position error)
- Fast enough for real-time applications

**Weaknesses** ❌:
- Limited to TLE format constraints
- WGS-72 OLD constants (dated gravity model)
- Does not model solar radiation pressure
- 90% transfer overhead for small batches
- Designed for near-earth only (period < 225 min)

**Best for**: LEO satellite constellations (Starlink, OneWeb), ISS tracking, large LEO batches

---

### CudaTlePropagator (SDP4)

**Strengths** ✅:
- Standard TLE format compatibility
- Includes lunar-solar perturbations
- Handles resonance effects (GEO, GPS, Molniya)
- Production-proven accuracy (<0.5m position error)
- Required for official TLE compliance

**Weaknesses** ❌:
- Poor GPU parallelism (1.6x speedup)
- Iterative resonance loops cause warp divergence
- Complex conditional branches
- High computational cost (~200+ ops per propagation)
- Crossover at ~1,000 satellites (often not worth GPU)

**Best for**: Official TLE propagation when exact compatibility required, small GEO/MEO batches on CPU

---

### CudaGeoNumericalPropagator (Experimental)

**Strengths** ✅:
- Operates on ECI states (no TLE required)
- No iteration loops → no warp divergence
- Higher-fidelity physics model (EGM-96, VSOP87/Brown, SRP with shadow)

**Weaknesses** ❌:
- Slower than GPU SDP4 (many RK4 substeps per propagation)
- Cowell's method accumulates truncation error (~12 km @ 7d, ~689 km @ 30d)
- Not suitable for propagation spans >7 days
- Not TLE-compatible (requires ECI state vectors as input)
- No automatic selection by BatchPropagator (must be used directly)

**Best for**: GEO/MEO satellites when you have ECI state vectors, need SRP modeling, or want EGM-96 physics

---

### CudaSdp4InterpolatedPropagator

**Strengths** ✅:
- Excellent GPU parallelism (20-50x measured speedup)
- No iteration loops → no warp divergence
- Pre-sampled resonance eliminates computational bottleneck
- Fastest option for GEO/MEO propagation

**Weaknesses** ❌:
- **⚠️ Lower accuracy: 350m @ 1 day, 2.4 km @ 7 days, 10 km @ 30 days** (interpolation tuning needed)
- Memory overhead (~2 KB per satellite for samples)
- Initialization cost (pre-sampling 241 resonance points)
- Limited to ±30 days propagation span (sample range)
- GPU-resident methods not yet implemented (P1 roadmap item)
- Not suitable for high-precision applications yet

**Best for**: Large GEO/MEO batches where speed > precision, preliminary mission planning, orbit visualization

**⚠️ Use CPU SDP4 if you need sub-meter accuracy**

---

### SAAL CPU Propagator

**Strengths** ✅:
- Reference implementation (bit-for-bit python-sgp4)
- No GPU dependency
- Low overhead for small batches
- Rayon parallelization across satellites
- Exact Vallado algorithm
- Production-proven stability

**Weaknesses** ❌:
- Sequential per-satellite (1x baseline speed)
- Cannot leverage GPU acceleration
- Limited by CPU core count
- 83x slower than GPU for LEO
- 1.6x slower than GPU for GEO

**Best for**: Fallback when GPU unavailable, validation reference, small batches (<1000 ops)

---

## API Usage Guide

### 1. Automatic Backend and Propagator Selection (Recommended)

```rust
use keplemon::propagation::BatchPropagator;

// Automatic GPU/CPU selection + automatic propagator selection
let batch_prop = BatchPropagator::new();
let results = batch_prop.propagate_batch(&tles, &epochs)?;

// System automatically (v3.3.0+):
// - Checks GPU availability
// - Evaluates batch size vs threshold
// - Classifies satellites by orbital period
// - Selects optimal backend (CPU vs GPU)
// - Routes LEO satellites to SGP4 (83x speedup)
// - Routes GEO satellites to SDP4-Interpolated (20-50x speedup)
// - Handles mixed batches with partitioning
// - Falls back to CPU if GPU unavailable
```

**Example: Mixed LEO/GEO Batch** (new in v3.3.0)
```rust
// Mix of LEO (Starlink) and GEO (INTELSAT) satellites
let tles = vec![
    starlink_tle,     // period ~90 min → SGP4
    intelsat_geo_tle, // period ~1436 min → SDP4-Interpolated
];

// BatchPropagator automatically partitions and routes
let results = batch_prop.propagate_batch(&tles, &epochs)?;
// LEO: 83x speedup, GEO: 20-50x speedup
```

---

### 2. Force GPU Backend

```rust
use keplemon::propagation::{BatchPropagator, PropagationBackend};

let batch_prop = BatchPropagator::new()
    .set_backend(PropagationBackend::Gpu);

let results = batch_prop.propagate_batch(&tles, &epochs)?;

// Falls back to CPU with warning if GPU unavailable
```

---

### 3. Force CPU Backend

```rust
let batch_prop = BatchPropagator::new()
    .set_backend(PropagationBackend::Cpu);

let results = batch_prop.propagate_batch(&tles, &epochs)?;
```

---

### 4. Custom GPU Threshold

```rust
// Use GPU for batches >500 operations (instead of default 1,000)
let batch_prop = BatchPropagator::new()
    .set_gpu_threshold(500);

let results = batch_prop.propagate_batch(&tles, &epochs)?;
```

---

### 5. GPU-Resident Results (Zero-Copy)

```rust
use keplemon::gpu::CudaTlePropagator;

let mut propagator = CudaTlePropagator::new()?;
propagator.init_satellites(&tle_data)?;

// Keep results on GPU
let gpu_results = propagator.propagate_soa_resident(&times)?;

// Chain with other GPU operations (no download)
collision_kernel.launch(cfg, (
    &gpu_results.x,
    &gpu_results.y,
    &gpu_results.z
))?;

// Download only when needed
let cpu_results = gpu_results.download()?;
```

---

### 6. Propagator Override (Testing Only)

```rust
use keplemon::gpu::{CudaTlePropagator, PropagatorOverride};

let mut propagator = CudaTlePropagator::new()?;

// ⚠️ TESTING ONLY - can produce incorrect results
propagator.init_satellites_with_override(
    &tle_data,
    PropagatorOverride::ForceSgp4  // Force all to SGP4
)?;

let results = propagator.propagate_soa_arrays(&times)?;
```

---

### 7. SDP4 Interpolated Propagator (Direct Use)

```rust
use keplemon::gpu::{CudaSdp4InterpolatedPropagator, TleDataGpu};

let mut propagator = CudaSdp4InterpolatedPropagator::new()?;

// Convert TLEs to GPU format
let tle_data: Vec<TleDataGpu> = tles.iter()
    .map(|tle| TleDataGpu::from(tle))
    .collect();

// Initialize (pre-samples resonance effects)
propagator.init_satellites(&tle_data)?;

// Propagate (20-50x GPU speedup)
let times: Vec<f64> = epochs.iter()
    .map(|e| TleDataGpu::jd_from_ds50(e.days_since_1950))
    .collect();

let results = propagator.propagate(&times)?;
```

---

### 8. GEO Numerical Propagator

```rust
use keplemon::gpu::CudaGeoNumericalPropagator;
use keplemon::propagation::CartesianState;

let mut propagator = CudaGeoNumericalPropagator::new()?;

// Convert TLEs to ECI states at epoch
let eci_states: Vec<CartesianState> = tles.iter()
    .map(|tle| tle.get_cartesian_state_at_epoch(tle.epoch()))
    .collect();

// Initialize with ECI states
propagator.init_from_eci(&eci_states)?;

// Propagate
let results = propagator.propagate_soa_arrays(&times)?;
```

---

## Accuracy Verification

### Comprehensive Accuracy Benchmark Results (Jan 2026)

All GPU propagators tested against SAAL CPU reference implementation:

| Propagator | Orbit | @ 1 Day | @ 7 Days | @ 30 Days | Status |
|------------|-------|---------|----------|-----------|--------|
| **CudaTlePropagator (SGP4)** | LEO | 0.5m | 4.2m | 15.9m | ✅ Excellent |
| **CudaTlePropagator (SDP4)** | GEO/MEO | <0.001m | <0.001m | 0.034m | ✅ Near-perfect |
| **CudaSdp4InterpolatedPropagator** | GEO/MEO | 349m | 2.4 km | 10.2 km | ⚠️ Needs tuning |
| **CudaGeoNumericalPropagator** | GEO/MEO | 8.5 km | 12 km | 689 km | ⚠️ Experimental (<7d only) |

**Key Findings**:
- Standard SDP4 achieves **micron-level accuracy** (<0.001m) but only 1.6x GPU speedup
- SDP4-Interpolated trades accuracy for speed (20-50x speedup, km-level errors)
- GEO Numerical is slower than SDP4 and less accurate; only use case is short-term ECI-based propagation when no TLE is available
- SGP4 maintains excellent accuracy for LEO orbits

**Test Configuration**:
- LEO satellites: ISS, Starlink, NOAA 18
- GEO/MEO satellites: INTELSAT 902, GPS BIIR-2, GLONASS-M 736
- Time spans: 0, 1 day, 7 days, 30 days
- Reference: SAAL CPU propagator (python-sgp4 binding)
- Test file: `tests/benchmark_gpu_accuracy.rs`

---

### GPU-CPU Parity Test Results

**Test**: `tests/test_gpu_cpu_parity.rs:79-270`

**Test Configuration**:
- 9 satellites (LEO, MEO, GEO, GPS, GLONASS)
- 5 propagation times (0, 6h, 12h, 18h, 24h)
- 45 total comparisons

**Final Accuracy** (after bug fixes):

| Orbit Type | Max Position Error | Max Velocity Error | Status |
|------------|-------------------|-------------------|--------|
| ISS (LEO) | 1.649 m | 6.431 mm/s | ✅ PASS |
| Starlink (LEO) | 0.150 m | 4.605 mm/s | ✅ PASS |
| NOAA 18 (LEO) | 0.148 m | 4.503 mm/s | ✅ PASS |
| GPS (MEO) | 0.321 m | 2.366 mm/s | ✅ PASS |
| GLONASS (MEO) | 0.142 m | 2.399 mm/s | ✅ PASS |
| LES-5 (GEO) | 0.102 m | 1.926 mm/s | ✅ PASS |
| **Overall** | **1.649 m** | **6.431 mm/s** | ✅ PASS |

**Tolerance**: 2 m position, 15 mm/s velocity

---

### Accuracy Investigation History

**See**: `docs/gpu/gpu-cpu-accuracy.md` (443 lines)

**Initial Error**: 22 km position error

**Root Causes Fixed**:
1. **DPPER Baseline Bug**: Removed erroneous `dpper(init=true)` call
   - Improvement: 22 km → 32 m (99.85% reduction)

2. **WGS-72 Constant Mismatch**: Updated to WGS-72 OLD (1972)
   - J2: 0.00108262998905 → 0.001082616 (WGS-72 OLD)
   - J3: -0.00000253215306 → -0.00000253881 (WGS-72 OLD)
   - Improvement: 32 m → 0.017 m (99.99% reduction)

3. **RAAN Quadratic Term**: Added missing `p.xnodcf * t2` term

4. **Mean Motion Variable**: Fixed wrong variable in `nm` update

**Final Result**: **0.017 m - 1.649 m** position error across all regimes

---

### Dual-Mode API Equivalence

**Test**: `tests/test_gpu_cpu_parity.rs:272-390`

Verifies both GPU APIs produce identical results:
- `propagate_soa_arrays()` (downloads to CPU)
- `propagate_soa_resident()` (keeps on GPU)

**Tolerance**: 1e-10 km position, 1e-13 km/s velocity (floating-point precision)

**Result**: ✅ Bit-for-bit identical

---

### Test Coverage

| Test | Location | Purpose |
|------|----------|---------|
| `test_gpu_cpu_parity_all_regimes` | `tests/test_gpu_cpu_parity.rs:79-270` | Accuracy across LEO/MEO/GEO |
| `test_dual_mode_equivalence_leo_geo` | `tests/test_gpu_cpu_parity.rs:278-390` | API equivalence |
| `test_benchmark_cpu_vs_gpu` | `tests/benchmark_cpu_vs_gpu.rs:332-392` | Performance validation |
| `test_geo_propagation_basic` | `tests/test_geo_analytical.rs:71-104` | GEO propagator functionality |
| `test_soa_kernel` | `tests/test_soa_kernel.rs` | SoA memory layout |

---

## Future Roadmap

### Short-Term (Next Release)

**1. GPU-Resident Methods for SDP4 Interpolated** (Priority: High)
- Add `propagate_soa_resident()` to `CudaSdp4InterpolatedPropagator`
- Enable zero-copy GPU pipelines for deep-space satellites
- **Expected impact**: 2-10x additional speedup for chained GPU operations

**2. Multi-GPU Support** (Priority: Medium)
- Distribute satellites across multiple GPUs
- Automatic device selection and load balancing
- **Expected impact**: Linear scaling with GPU count

**3. Batched Transfer Optimization** (Priority: Medium)
- Pipeline CPU→GPU transfers with computation
- Reduce transfer bottleneck for large batches
- **Expected impact**: 20-30% speedup for mixed workloads

---

### Medium-Term

**4. SGP4-XP Implementation** (Priority: Low)
- Implement Space Force SGP4-XP algorithm
- Detect ephemeris_type == 4 TLEs
- Higher-order gravity model (J5, J6)
- **Expected impact**: Improved accuracy for official GEO/MEO TLEs

**5. Adaptive Propagator Selection** (Priority: Medium)
- Machine learning model to predict optimal propagator
- Consider accuracy requirements, time span, batch size
- Runtime performance profiling
- **Expected impact**: 10-20% overall speedup from better selection

**6. Kernel Fusion Optimization** (Priority: Medium)
- Fuse initialization and propagation kernels
- Reduce kernel launch overhead
- **Expected impact**: 5-10% speedup for small batches

---

### Long-Term

**7. FP16/Mixed-Precision Support** (Priority: Low)
- FP16 for non-critical computations
- FP64 for angle accumulation and high-precision terms
- **Expected impact**: 2x speedup on modern GPUs (Ampere+)

**8. Tensor Core Utilization** (Priority: Research)
- Batch matrix operations for perturbation models
- Leverage tensor cores for linear algebra
- **Expected impact**: Potential 5-10x speedup for analytical models

**9. Dynamic Compilation** (Priority: Low)
- JIT compile kernels based on batch characteristics
- Specialize for pure LEO, pure GEO, or mixed workloads
- **Expected impact**: 10-15% speedup from specialization

---

## Conclusion

Keplemon's GPU propagator architecture provides:
- **Flexibility**: 5 propagators for different use cases and input formats
- **Performance**: Up to 83x speedup for LEO, 20-50x for GEO/MEO with SDP4-Interpolated
- **Accuracy**:
  - SGP4 (LEO): 4.2m @ 7 days, 16m @ 30 days
  - SDP4 Standard (GEO): <0.001m (micron-level accuracy!)
  - SDP4-Interpolated (GEO): 2.4 km @ 7 days, 10 km @ 30 days
  - GEO Numerical: Experimental, ~12 km @ 7d (ECI input, short-term only)
- **Automation**: Automatic propagator selection based on orbital period (v3.3.0+)
- **Robustness**: Automatic fallback and intelligent selection
- **Future-proof**: Extensible design for new algorithms

**Recommended configuration**: Use `BatchPropagator::new()` with `PropagationBackend::Auto` for automatic optimization based on workload, orbital characteristics, and hardware availability.

**Key Update (v3.3.0)**: SDP4-Interpolated propagator is now fully integrated and automatically selected for deep-space orbits, providing 20-50x speedup vs the previous 1.6x standard SDP4 implementation.

For questions or contributions, see the [main README](../../README.md) and [GPU benchmark results](./gpu-benchmark-results.md).

---

**References**:
- `docs/gpu/gpu-benchmark-results.md` - Detailed performance analysis
- `docs/gpu/gpu-cpu-accuracy.md` - Accuracy investigation and fixes
- `src/gpu/` - GPU propagator implementations
- `tests/test_gpu_cpu_parity.rs` - Accuracy verification tests
- `tests/benchmark_cpu_vs_gpu.rs` - Performance benchmarks
