# PyO3 wrapper skill

## Use when

- Exposing Rust core to Python, changing Python-visible API, errors, buffers.

## Hard rules

- No panics across FFI.
- Convert Rust errors to Python exceptions consistently.
- Keep GIL held only when necessary; release for heavy compute.
- Prefer zero-copy with `PyBuffer`/`PyReadonlyArray` when safe.

## Canonical patterns

- `#[pyfunction]` thin wrapper calling `core::*`
- Unified error enum -> `PyErr`
- Stub parity: update `.pyi` or typed Python surface concurrently
