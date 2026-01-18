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
- **Time Points**: 24 (1-hour intervals over 24 hours)
- **Total Propagations**: Up to 24,000 for the largest test

## Sample Results (Release Mode)

| Satellites | CPU Time | GPU Time | Speedup | GPU Throughput |
|-----------|----------|----------|---------|----------------|
| 10        | 0.30 ms  | 3.56 ms  | 0.08x   | 67K props/sec  |
| 20        | 0.48 ms  | 3.54 ms  | 0.14x   | 135K props/sec |
| 40        | 0.93 ms  | 3.54 ms  | 0.26x   | 271K props/sec |
| 80        | 1.84 ms  | 3.54 ms  | 0.52x   | 543K props/sec |
| 160       | 4.03 ms  | 4.29 ms  | 0.94x   | 894K props/sec |
| **1000**  | 23.0 ms  | 5.44 ms  | **4.23x** | **4.4M props/sec** |

## Key Findings

1. **GPU Overhead**: For small batches (<80 satellites), CPU is faster due to GPU initialization and data transfer overhead.

2. **Crossover Point**: GPU becomes competitive around 80-160 satellites.

3. **Large Batch Performance**: GPU achieves 4.2x speedup at 1000 satellites, demonstrating excellent scalability.

4. **Peak Throughput**: GPU can process over 4 million propagations per second in optimal conditions.

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
