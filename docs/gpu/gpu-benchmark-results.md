# GPU Benchmark Results: Orbit Regime Comparison

## Executive Summary

Benchmark results reveal dramatic performance differences between orbit regimes:

| Regime | Kernel Speedup (1000 sats) | GPU Throughput | Key Finding |
|--------|---------------------------|----------------|-------------|
| **LEO Only** | **83.2x** | 439M props/sec | GPU excels at near-earth propagation |
| **GEO Only** | **1.6x** | 5.8M props/sec | Deep-space propagation barely benefits |
| **Mixed (60/40)** | **3.5x** | 14.7M props/sec | Two-kernel optimization critical |

## Full Benchmark Results

### Test Configuration
- **Time points**: 10,080 (7 days at 1-minute intervals)
- **Satellite counts**: 10, 50, 100, 1000
- **Orbit regimes**: LEO only, GEO only, Mixed (60% LEO / 40% GEO)

---

## LEO Only (SGP4 Near-Earth Propagation)

| Satellites | CPU Time | GPU Kernel | GPU + Transfer | Kernel Speedup | GPU Throughput |
|------------|----------|------------|----------------|----------------|----------------|
| 10 | 19ms | 0.4ms | 1.1ms | **46.9x** | 248M/sec |
| 50 | 96ms | 1.6ms | 12ms | **61.6x** | 325M/sec |
| 100 | 191ms | 2.7ms | 23ms | **71.4x** | 377M/sec |
| 1000 | 1,911ms | 23ms | 247ms | **83.2x** | **439M/sec** |

### Key Observations - LEO

1. **Massive GPU advantage**: 83x speedup for kernel-only at 1000 satellites
2. **Transfer bottleneck**: 90%+ overhead for large batches
   - For 1000 sats: 224ms transfer vs 23ms computation
3. **Scaling**: Speedup increases with satellite count (46x → 83x)
4. **Throughput**: GPU achieves **439 million propagations/sec**
5. **CPU performance**: Consistent ~5.3M props/sec regardless of batch size

**Recommendation**: LEO-only workloads are ideal for GPU acceleration. Use GPU-resident mode to eliminate transfer overhead.

---

## GEO Only (SDP4 Deep-Space Propagation)

| Satellites | CPU Time | GPU Kernel | GPU + Transfer | Kernel Speedup | GPU Throughput |
|------------|----------|------------|----------------|----------------|----------------|
| 10 | 28ms | 29ms | 29ms | **0.99x** | 3.5M/sec |
| 50 | 143ms | 112ms | 122ms | **1.28x** | 4.5M/sec |
| 100 | 286ms | 194ms | 215ms | **1.48x** | 5.2M/sec |
| 1000 | 2,860ms | 1,739ms | 1,958ms | **1.64x** | **5.8M/sec** |

### Key Observations - GEO

1. **Minimal GPU advantage**: Only 1.64x speedup even at 1000 satellites
2. **Low transfer overhead**: Only 11% (SDP4 is compute-heavy, transfer is relatively small)
3. **Limited scaling**: Speedup barely improves with satellite count (0.99x → 1.64x)
4. **Throughput**: GPU achieves only 5.8M props/sec (75x slower than LEO!)
5. **CPU performance**: ~3.5M props/sec (33% slower than LEO on CPU too)

**Why GEO is slow on GPU:**
- Deep-space propagation (SDP4) has complex perturbation calculations
- More branching and conditional logic → warp divergence
- Lunar/solar perturbations require iterative solvers
- Higher computational intensity per satellite

**Recommendation**: For GEO-only workloads, consider CPU parallelism (rayon) instead of GPU unless batch size is very large (1000+).

---

## Mixed (60% LEO, 40% GEO)

| Satellites | CPU Time | GPU Kernel | GPU + Transfer | Kernel Speedup | GPU Throughput |
|------------|----------|------------|----------------|----------------|----------------|
| 10 | 24ms | 29ms | 29ms | **0.83x** | 3.5M/sec |
| 50 | 119ms | 57ms | 67ms | **2.11x** | 8.9M/sec |
| 100 | 238ms | 86ms | 107ms | **2.78x** | 11.8M/sec |
| 1000 | 2,382ms | 685ms | 893ms | **3.48x** | **14.7M/sec** |

### Key Observations - Mixed

1. **Moderate speedup**: 3.48x at 1000 satellites
2. **Transfer overhead**: 23% (moderate, 209ms transfer vs 685ms computation)
3. **Two-kernel benefit**: Without partition, would suffer warp divergence
4. **Throughput**: 14.7M props/sec (between LEO and GEO as expected)
5. **Crossover point**: GPU becomes faster than CPU at ~30-40 satellites

**Performance composition:**
- 60% LEO contribution: ~439M/sec × 0.6 × (600 sats / 1000) = ~158M props/sec on LEO portion
- 40% GEO contribution: ~5.8M/sec × 0.4 × (400 sats / 1000) = ~0.9M props/sec on GEO portion
- GEO satellites dominate execution time despite being only 40% of the constellation

**Recommendation**: Mixed workloads benefit significantly from two-kernel optimization. Without partitioning, LEO threads would be blocked waiting for GEO threads in the same warp.

---

## Analysis: Why LEO is 75x Faster than GEO on GPU

### SGP4 (Near-Earth / LEO)
- Simple perturbations (J2, J3, J4 gravitational harmonics)
- Minimal branching
- No iterative solvers
- ~50 double-precision operations per propagation
- Excellent GPU parallelism

### SDP4 (Deep-Space / GEO)
- Complex perturbations (lunar, solar, resonance effects)
- Deep conditional branches for:
  - Synchronous vs non-synchronous orbits
  - Resonance detection and handling
  - Lyddane coordinate conversion
- Iterative Newton-Raphson solvers
- ~200+ double-precision operations per propagation
- Poor GPU parallelism due to divergence

## Transfer Overhead Analysis

| Regime | 1000 sats | Transfer Time | % Overhead | Data Size |
|--------|-----------|---------------|------------|-----------|
| LEO | 10.08M props | 224ms | **90.7%** | 560MB |
| GEO | 10.08M props | 219ms | **11.2%** | 560MB |
| Mixed | 10.08M props | 209ms | **23.3%** | 560MB |

**Key insight**: Transfer time is constant (~220ms for 560MB), but appears as higher overhead when kernel time is low (LEO).

**PCIe bandwidth**: 560MB / 220ms = **2.5 GB/sec**
- This is below PCIe 3.0 theoretical bandwidth (~12 GB/sec)
- Likely due to non-contiguous memory access patterns and cudarc overhead

## Recommendations by Use Case

### Use GPU When:
1. **LEO-heavy workloads** (>50% LEO satellites)
2. **Large batches** (100+ satellites)
3. **GPU-resident pipelines** (collision detection, visualization, etc.)
4. **Need to free CPU** for other tasks

### Use CPU When:
1. **GEO-heavy workloads** (>70% GEO satellites)
2. **Small batches** (<30 satellites)
3. **Single propagations** (one satellite, one time)
4. **No GPU available** (graceful fallback)

### Optimization Opportunities

| Optimization | Target Regime | Potential Gain |
|--------------|---------------|----------------|
| **GPU-resident mode** | LEO | Eliminate 90% transfer overhead |
| **Optimize SDP4 branch reduction** | GEO | 1.5-2x improvement possible |
| **Half-precision for LEO** | LEO | 2x throughput (if accuracy acceptable) |
| **CPU parallelism (rayon)** | GEO | 4-8x with multi-core CPU |
| **Hybrid CPU+GPU** | Mixed | Process LEO on GPU, GEO on CPU |

## Conclusion

The benchmark reveals that:

1. **LEO propagation is GPU's sweet spot** with 83x speedup
2. **GEO propagation barely benefits** from GPU parallelism (1.6x)
3. **Two-kernel optimization is critical** for mixed workloads
4. **Transfer overhead dominates** for fast operations (LEO)

The implementation successfully addresses mixed-constellation performance through intelligent partitioning, achieving 3.5x speedup despite the GEO bottleneck. For LEO-only workloads, the GPU provides transformative performance (83x), while GEO-only workloads may be better served by CPU parallelism.
