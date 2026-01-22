# GPU Benchmarks

This directory contains Criterion-based statistical benchmarks for CUDA-accelerated SGP4 propagation.

## Prerequisites

- NVIDIA GPU with CUDA support
- CUDA 12.6 or later
- Build with the `cuda` feature: `cargo bench --features cuda`

## Benchmarks

### propagation.rs
Statistical performance benchmark for GPU batch propagation using Criterion.

**Run:**
```bash
cargo bench --features cuda --bench gpu_propagation
```

**Measures:**
- GPU propagation throughput for various batch sizes
- Scaling characteristics with satellite count
- Statistical analysis with warmup, outlier detection, and regression

**Note:** This is a proper Criterion benchmark that runs many iterations for statistical accuracy. It takes several minutes to complete but provides rigorous performance measurements.

## GPU Performance Analysis Tools

For **quick performance analysis and comparison tools** (single-run, custom output), see `tests/`:

- **`tests/benchmark_gpu.rs`** - Basic GPU performance with custom table output
- **`tests/benchmark_cpu_vs_gpu.rs`** - CPU vs GPU comparison with speedup calculations
- **`tests/benchmark_gpu_crossover.rs`** - Crossover point analysis ("GPU WINS" vs "CPU WINS")
- **`tests/quick_soa_timing.rs`** - AoS vs SoA kernel comparison
- **`tests/test_sgp4_geo_empirical_error.rs`** - SGP4 vs SDP4 accuracy analysis at GEO

These are **ignored tests** that provide immediate feedback:

```bash
# Run specific analysis tool
cargo test --features cuda --release benchmark_gpu_propagation -- --ignored --nocapture

# Run CPU vs GPU comparison
cargo test --features cuda --release test_benchmark_cpu_vs_gpu -- --ignored --nocapture

# Run crossover analysis
cargo test --features cuda --release test_gpu_crossover_analysis -- --ignored --nocapture

# Run quick SoA timing
cargo test --features cuda --release quick_timing_comparison -- --ignored --nocapture
```

### Why Two Approaches?

**Criterion Benchmarks (`benches/`):**
- Statistical microbenchmarking
- 100+ iterations per measurement
- Regression detection
- Takes minutes to run
- Use when: You need rigorous, statistically significant performance data

**Analysis Tools (`tests/`):**
- Single or few-run measurements
- Custom formatted output (tables, speedup factors)
- Decision guidance ("GPU WINS" vs "CPU WINS")
- Takes seconds to run
- Use when: You need quick feedback to make architectural decisions

## Running All GPU Tests

```bash
# Run all GPU benchmarks (statistical, slow)
cargo bench --features cuda --bench gpu_propagation

# Run all GPU analysis tools (fast, immediate feedback)
cargo test --features cuda --release -- benchmark --ignored --nocapture
cargo test --features cuda --release -- gpu_crossover --ignored --nocapture
```

## Interpreting Criterion Results

Criterion provides:
- **Mean time:** Average execution time across iterations
- **Std. dev:** Standard deviation of measurements
- **Outliers:** Number of measurements outside expected range
- **Change:** Performance change vs previous run (if available)

### Typical Performance Characteristics

- **Small batches (<100 satellites):** CPU may be faster due to GPU overhead
- **Medium batches (100-1000 satellites):** GPU begins to show advantage
- **Large batches (>1000 satellites):** GPU shows significant speedup (5-50x)
- **Memory transfer:** Can add 10-30% overhead vs GPU-resident computation

## CI Considerations

GPU benchmarks require:
- CUDA compiler (nvcc)
- NVIDIA GPU hardware
- `cuda` feature enabled

CI typically does NOT run these. To skip in CI:
```yaml
# GitHub Actions example
- name: Run benchmarks
  run: cargo bench --bench satellite --bench constellation  # Skip GPU benches
```

## Troubleshooting

### "CUDA not available"
- Ensure NVIDIA drivers are installed
- Check `nvidia-smi` works
- Verify CUDA toolkit is in PATH

### Compilation errors
- Check CUDA version matches `Cargo.toml` (`cuda-12060`)
- Ensure `nvcc` is available: `which nvcc`
- Try cleaning: `cargo clean`

### Poor performance
- Check GPU isn't in use by other processes
- Ensure GPU isn't thermal throttling
- Verify problem size is large enough for GPU advantage
- Check if debugging symbols are disabled (use `--release` mode)
