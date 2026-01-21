# Rust performance skill

## Use when

- Hot loops, image/array ops, parsing, hashing, numerics, SIMD/parallelism candidates.

## Hard rules

- No performance change without a Criterion benchmark and before/after numbers.
- Prefer layout-first optimizations (SoA, contiguous buffers) before micro-opts.
- No allocations in hot loops; preallocate and reuse buffers.
- Parallelism must be deterministic; bound threads (Rayon global pool config).

## Canonical patterns

- Slice-based APIs: `fn f(x: &[T], out: &mut [U])`
- Iterator avoidance in hot loops if it blocks autovectorization.
- Use `rayon::slice::ParallelSlice` only when workload is large enough.

## Deliverables

- `benches/*.rs` + before/after table + short profiling note.
