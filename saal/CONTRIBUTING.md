# Contributing

This repository is Rust-first with Python bindings built via PyO3 and distributed as wheels.
The goal of this document is to make development and review deterministic for both humans and AI.

---

## Requirements

- Rust (stable toolchain) with `rustfmt` and `clippy`
- Python 3.10+
- Virtual environment recommended
- `cargo-make`
- `maturin`
- Python dev dependencies: `pytest`, `mypy`

---

## Definition of Done

Before opening a PR, all changes must pass:

    cargo make check

If the change impacts performance (hot loops, algorithms, parallelism, vectorization), also run:

    cargo make perf

---

## Core Workflows

### Formatting (Rust)

    cargo fmt

Formatting differences will fail CI.

### Linting and Tests (Rust)

    cargo fmt -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-features

### Typing and Tests (Python)

    python -m mypy python/pysaal
    pytest -v --tb=short tests

All public Python APIs must be fully typed.
`mypy` must pass with no errors.

---

## Building and Testing Wheels

Platform-specific build tasks are defined in `Makefile.toml`.

### Build a wheel

    cargo make build-mac-arm
    # build-mac-x86
    # build-linux-arm
    # build-linux-x86
    # build-windows

### Install the built wheel

    cargo make install-mac-arm

### Run tests against the installed wheel

    cargo make test-mac-arm

---

## Release / Publish (Maintainers Only)

The publish flow is strictly ordered:

    cargo make publish-mac-arm
    # publish-mac-x86
    # publish-linux-arm
    # publish-linux-x86
    # publish-windows

Publishing requires:

- Passing wheel tests for the platform
- Correct versioning
- Stub/runtime parity

---

## Repository Conventions

### Rust

- Stable Rust only (no nightly)
- No panics across FFI boundaries
- `clippy` warnings are treated as errors
- Performance changes require benchmarks
- Parallelism must be deterministic and bounded

### Python

- Public APIs must be fully typed
- No `Any` in the public surface
- `mypy` must pass
- Stubs must match runtime behavior

The following directories are kept in sync:

- `stubs/pysaal/`
- `python/pysaal/`

Stub files are copied during `cargo make clean-all`.

---

## FFI and ABI Changes

Any change affecting PyO3 signatures, error translation, memory ownership, lifetimes, or buffer layout requires:

- Rust and Python changes in the same PR
- Updated stubs
- Wheel tests on at least one platform

---

## Troubleshooting

### `cargo fmt -- --check` fails

    cargo fmt

### Wheel tests failing

    cargo make clean-all
    cargo make build-mac-arm
    cargo make install-mac-arm
    cargo make test-mac-arm

---

## Review Expectations

Reviewers may block changes that:

- Break determinism
- Reduce type safety
- Introduce undocumented behavior
- Regress performance without evidence
- Violate FFI safety

This is intentional.
