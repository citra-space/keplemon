# Testing skill (SAAL)

## Use when

- Any behavior change (bugfix, feature, refactor)
- Any FFI / PyO3 API change
- Any performance change (add regression tests + benches)
- Any reported bug without an existing repro

## Hard rules

- No PR without tests unless explicitly justified in the PR description.
- Prefer small, deterministic tests (no network, no wall-clock sleeps).
- Every bugfix must include a regression test that fails before the fix.
- Keep Rust-core tests independent of Python (no PyO3 in `cargo test`).
- Public Python surface must have at least one end-to-end binding test.

## What to add (priority order)

1) Rust unit tests for pure logic (fast, deterministic)
2) Rust property tests for invariants (if appropriate)
3) Python wrapper tests that call into the wheel (only for binding behavior)
4) Integration tests for multi-component behavior

## Canonical patterns

### Rust unit test pattern

- Arrange / Act / Assert
- Use table-driven cases for coverage
- Add edge cases: empty, singleton, large values, NaNs (if applicable), time bounds

### Property tests (optional)

- Use proptest for invariants (e.g., monotonicity, bounds, reversibility)
- Keep generators small and seeded-friendly

### Python binding tests

- Test the Python signature + error behavior + returned types
- Ensure stub/runtime parity (types and behavior match)

## Deliverables

- New tests added in the same PR as the change
- `cargo make check` passes
- If relevant: brief note of what coverage/branch was added (what was untested before)

## Anti-patterns (reject)

- “Tests not run”
- Adding only snapshot tests for numerical logic without assertions
- Flaky tests (time-based, random without seed, reliance on OS ordering)
