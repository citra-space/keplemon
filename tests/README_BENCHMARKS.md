# CPU vs GPU Performance Benchmarks

## Overview

This directory contains performance benchmarks comparing CPU (sequential) and GPU (parallel) SGP4 propagation with realistic satellite mixes.

## Running Benchmarks

### Quick Benchmark (3 sizes, 10 time points)
```bash
cargo test --features cuda --release test_quick_benchmark -- --nocapture --test-threads=1
```

### Full Benchmark (6 sizes, 24 time points)
```bash
cargo test --features cuda --release test_benchmark_cpu_vs_gpu -- --nocapture --test-threads=1
```

## Benchmark Configuration

- **Satellite Mix**: 60% LEO (Starlink, ISS) + 40% GEO (TDRS, Milstar)
- **Satellite Counts**: 10, 20, 40, 80, 160, 1000
- **Time Points**: 168 (1-hour intervals over 7 days)
- **Total Propagations**: Up to 168,000 for the largest test

## Sample Results (Release Mode)

| Satellites | CPU Time | GPU Time | Speedup | GPU Throughput |
|-----------|----------|----------|---------|----------------|
| 10        | 1.42 ms  | 3.54 ms  | 0.40x   | 474K props/sec |
| 20        | 3.03 ms  | 3.55 ms  | 0.85x   | 947K props/sec |
| 40        | 5.10 ms  | 3.57 ms  | 1.43x   | 1.88M props/sec |
| 80        | 10.0 ms  | 3.58 ms  | 2.80x   | 3.75M props/sec |
| 160       | 19.7 ms  | 7.00 ms  | 2.81x   | 3.84M props/sec |
| **1000**  | 123 ms   | 37.0 ms  | **3.33x** | **4.54M props/sec** |

**Note**: This is a stress test propagating each satellite over **7 days** (168 hours) at 1-hour intervals.
- **LEO satellites**: ~108-110 complete orbital periods
- **GEO satellites**: ~7 complete orbital periods

## Key Findings

1. **GPU Overhead**: For small batches (<20 satellites), CPU is faster due to GPU initialization and data transfer overhead.

2. **Crossover Point**: GPU becomes competitive around 20-40 satellites, achieving speedup at 40+ satellites.

3. **Large Batch Performance**: GPU achieves 3.3x speedup at 1000 satellites, demonstrating excellent scalability even with extended propagation durations.

4. **Peak Throughput**: GPU can process over 4.5 million propagations per second, maintaining high performance even when propagating for a full week.

5. **Stress Test Results**: Over 7 days of propagation (168 time points), LEO satellites complete ~108-110 orbits while GEO satellites complete ~7 orbits, demonstrating robust long-duration accuracy.

## Use Cases

- **Operational Catalogs**: GPU excels for full catalog propagation (hundreds to thousands of satellites)
- **Conjunction Screening**: Batch propagation of multiple satellites over multiple time points
- **Monte Carlo Analysis**: Parallel propagation of orbit uncertainty ensembles
- **Real-time Visualization**: Fast updates for large constellation displays

## Notes

- Benchmarks run in `--release` mode for accurate performance measurement
- Times include GPU memory transfer overhead
- Mix of LEO and GEO satellites represents realistic operational scenarios
- Results may vary based on GPU model and CUDA compute capability
