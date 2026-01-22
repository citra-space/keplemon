# GPU vs CPU Crossover Point Analysis

## Executive Summary

This analysis identifies the precise conditions where GPU propagation becomes beneficial compared to CPU propagation for SGP4 satellite orbit prediction.

**Key Takeaway**: GPU becomes beneficial at surprisingly small workloads - as few as **10 satellites with 90+ time points** or **40 satellites with 45+ time points**.

## Critical Crossover Points

### LEO Satellites (~90 minute period)

| Satellite Count | dt = 1 min | dt = 10 min | dt = 1 hour |
|----------------|------------|-------------|-------------|
| **10 sats** | ✅ GPU @ 1 period (90 pts) | ❌ CPU until 10 periods (90 pts) | ❌ CPU always |
| **40 sats** | ✅ GPU @ 1 period (90 pts) | ❌ CPU until 5 periods (45 pts) | ❌ CPU until 20 periods (30 pts) |
| **100 sats** | ✅ GPU @ 1 period (90 pts) | ✅ GPU @ 1 period (9 pts) | ❌ CPU until 5 periods (8 pts) |
| **500 sats** | ✅ GPU @ 1 period (90 pts) | ✅ GPU @ 1 period (9 pts) | ✅ GPU @ 1 period (2 pts) |

### GEO Satellites (~24 hour period)

| Satellite Count | dt = 1 min | dt = 10 min | dt = 1 hour |
|----------------|------------|-------------|-------------|
| **10 sats** | ✅ GPU @ 1 period (1436 pts) | ✅ GPU @ 1 period (144 pts) | ❌ CPU @ 1 period (24 pts) |
| **40 sats** | ✅ GPU @ 1 period (1436 pts) | ✅ GPU @ 1 period (144 pts) | ✅ GPU @ 1 period (24 pts) |
| **100 sats** | ✅ GPU @ 1 period (1436 pts) | ✅ GPU @ 1 period (144 pts) | ✅ GPU @ 1 period (24 pts) |
| **500 sats** | ✅ GPU @ 1 period (1436 pts) | ✅ GPU @ 1 period (144 pts) | ✅ GPU @ 1 period (24 pts) |

## Detailed Performance Analysis

### When GPU Wins Decisively

**Best GPU Performance (>30x speedup):**
- 100 GEO sats, 1-min dt, 1 period: **41.40x speedup**
- 40 GEO sats, 1-min dt, 5 periods: **34.72x speedup**
- 500 GEO sats, 10-min dt, 5 periods: **33.48x speedup**
- 40 GEO sats, 1-min dt, 1 period: **31.79x speedup**
- 500 GEO sats, 1-hour dt, 5 periods: **31.85x speedup**

**Strong GPU Performance (10-30x speedup):**
- All scenarios with 100+ satellites
- All GEO scenarios with 40+ satellites
- LEO scenarios with 500 satellites and 10-min or 1-min dt

### When CPU Wins

**CPU is Better When:**
- **10 LEO satellites + 1-hour dt**: Any number of periods (GPU overhead > computation)
- **10 LEO satellites + 10-min dt**: Less than 10 periods (<90 time points)
- **10 GEO satellites + 1-hour dt + 1 period**: Only 24 time points

**Marginal Cases (speedup < 2x):**
- 10 LEO sats, 10-min dt, 10 periods: 1.25x speedup
- 40 LEO sats, 10-min dt, 5 periods: 1.13x speedup
- 10 GEO sats, 1-hour dt, 5 periods: 1.60x speedup

## Key Insights

### 1. Time Points Matter More Than dt

The critical factor is **total number of time points**, not the time step size:
- **90 time points** is the sweet spot for 40+ satellites (GPU wins)
- **144+ time points** is excellent for GPU even with 10 satellites
- Below 30 time points, CPU often wins unless satellite count is very high

### 2. Satellite Count Amplifies Benefits

- **10 satellites**: Need 90+ time points for GPU benefit
- **40 satellites**: Need 45+ time points for GPU benefit
- **100 satellites**: GPU wins at just 9 time points
- **500 satellites**: GPU wins at just 2 time points

### 3. GPU Overhead is ~0.15-0.20 ms

Minimum GPU execution time is approximately 0.15-0.20 ms regardless of workload:
- 10 LEO sats, 1-hour dt, 1 period (2 pts): 0.16 ms
- 500 LEO sats, 1-hour dt, 1 period (2 pts): 0.15 ms

This represents GPU memory transfer and kernel launch overhead.

### 4. Orbital Period Multiplier Impact

For LEO satellites (90-min period):
- **1 period** (~90 time points @ 1-min dt): GPU wins at 10+ satellites
- **5 periods** (~450 time points @ 1-min dt): GPU wins decisively (5-21x speedup)
- **10 periods** (~900 time points @ 1-min dt): GPU wins decisively (10-21x speedup)
- **20 periods** (~1800 time points @ 1-min dt): GPU wins decisively (11-14x speedup)

## Operational Recommendations

### Use GPU When:
1. **Tracking 40+ satellites** with any reasonable propagation scenario
2. **Tracking 10+ satellites** with 90+ time points
3. **Long-term propagation** (5+ orbital periods) even with small satellite counts
4. **High-fidelity GEO tracking** (1-min dt) - GPU wins at just 10 satellites

### Use CPU When:
1. **Single satellite** or very small counts (<10)
2. **Sparse time points** (<30 points) with small satellite counts (<40)
3. **Very coarse dt** (1-hour) with minimal propagation duration
4. **Interactive applications** where 0.15 ms overhead matters

### Optimal Configurations:
- **Best balance**: 40-100 satellites, 10-min dt, 5-10 orbital periods
- **Maximum throughput**: 500+ satellites, 1-min dt, any duration
- **Minimum viable GPU**: 40 satellites, 45 time points (any dt)

## Test Methodology

Tests ran in release mode with `--features cuda --release` to measure production performance.

Each scenario tested:
- **Satellite counts**: 10, 40, 100, 500
- **Time steps**: 1 minute, 10 minutes, 1 hour
- **Orbital periods**: 1, 5, 10, 20 periods
- **Orbit types**: LEO (~90 min) and GEO (~24 hr)

Timing measured CPU and GPU execution time separately, computing speedup as CPU_time / GPU_time.

## Conclusion

The GPU crossover point is **lower than expected**: as few as 10 satellites with moderate propagation intervals benefit from GPU acceleration. The key driver is **total propagation count** (satellites × time points), with the threshold around **900-1000 total propagations** for measurable GPU benefit.

For operational satellite tracking systems handling 40+ satellites, GPU acceleration is **strongly recommended** and provides **10-40x speedup** for typical use cases.
