# AI Contract (Keplemon)

You must follow `agents.yaml`.

Definition of done:

- Consider if Python bindings for Rust changes need to be exposed in bindings/ or bindings.rs
- Consider if Python bindings need to be updated in any of the python/keplemon/*.py files
- Consider if Python type hints need to be updated in any ofthe stubs/keplemon/*.pyi files
- Run `cargo make clean-all` to clear Python and Rust caches
- Run `cargo test` after changes to confirm the Rust library works
- Run `pip install .` to confirm the Python wheel still builds and installs
- Run `pytest .` to confirm Python tests pass

Formatting-only changes:

- Always run `cargo fmt` and then `cargo fmt -- --check`.

SAAL Troubleshooting:

- to use sgp4, a key must be loaded by tle and by sgp4
- tle and sgp4 remove methods must be called separately to clear a tle from saal memory
