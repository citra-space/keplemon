# PRD: Testing & Performance Audit Remediation — keplemon

<!-- pipeline-metadata
status: queued PRD
labels: ["queued PRD", "value: high", "risk: medium"]
do-not-label: ["active"]
audit-date: 2026-07-05
audit-commit: 14114699ef60b8c169c55d1b9d75645c8bb37f57
companion-branch: audit/testing-gap-verification
workflows: [bugfix, performance, refactor]  # per agents.yaml
-->

**Status:** queued (do NOT mark active — no agent has claimed this work)
**Value:** high — confirmed correctness panics in public API paths, a silent
conjunction-miss defect in the flagship screening API, and a measured 2.1×
parallel *slowdown* in batch least squares.
**Risk:** medium — fixes touch the SAAL FFI threading contract and screening
internals; regressions are contained by the verification tests delivered with
this audit (companion branch `audit/testing-gap-verification`).

---

## 1. Background

Exhaustive audit of keplemon @ `1411469` (2026-07-05) covering performance
(code-level review + empirical thread-scaling measurements) and test coverage
(public API surface vs existing 38 Rust + 23 pytest tests). Duplication check:
**the repository had zero open issues and zero open PRDs on 2026-07-05**, so no
recommendation below duplicates queued work.

Baseline at audit time: `cargo test --release` 38/38 pass; `pytest tests/`
23/23 pass. All findings below were confirmed against this green baseline.

The audit delivered two new test files (61 passed / 10 xfailed with the rest of
the suite) on branch `audit/testing-gap-verification`:

- `tests/test_algorithm_verification.py` — algorithmic verification against
  public oracles: the reference SGP4 implementation (`pip install sgp4`,
  validated against the published AIAA 2006-6753 test suite), Vallado worked
  examples (rv2coe Ex. 2-6, GMST Ex. 3-5), the public IERS leap-second table,
  real Celestrak/space-track TLE snapshots already in `tests/`, and exact
  analytic geometry with simulated ephemerides.
- `tests/test_error_handling_audit.py` — regression repros for the
  panic/silent-failure defects (subprocess-isolated where needed).

Every confirmed defect has a **strict** `xfail` marker: fixing it flips the
test to XPASS so the fix PR must remove the marker (regression test included by
construction, per `skills/testing.md`).

Positive verification results (no action needed):

- SGP4 near-earth propagation matches the reference implementation to < 1 m
  across ISS + 20 Celestrak catalog objects at epoch +90 min.
- SDP4/GEO propagation matches to < 100 m (real GEO TLE, epoch +12 h).
- Cartesian↔Keplerian round-trips are exact (< 1e-6) across LEO / Molniya /
  GEO / SSO / GTO regimes and match Vallado Ex. 2-6.
- Equinoctial→Keplerian matches standard equinoctial definitions to 1e-6.
- Hermite ephemeris interpolation is exact on linear motion; close-approach
  detection on well-formed simulated ephemerides recovers TCA to < 1 s and
  miss distance to < 1 m.
- UTC→TAI leap-second offsets match the public IERS table (2014–2024).

---

## 2. Confirmed bugs (workflow: `bugfix`)

Ordered by severity. Each has a runnable repro in the companion branch.

### BUG-1 — Silent missed conjunction when secondary ephemeris is shorter than primary
- **Severity:** critical (silent wrong answer in conjunction screening)
- **Where:** `src/elements/ephemeris.rs:393` (scan window derived from `self`
  only), `:793-798` (out-of-span interpolation clamps to boundary state,
  keeping the *boundary* epoch), `:563` (epoch-equality guard then returns
  `None`), `:442-444` (scan loop `break`s on first `None`).
- **Repro:** `tests/test_algorithm_verification.py::TestCloseApproachSimulated::test_shorter_secondary_ephemeris_is_not_silently_truncated`
  — a 5 km approach inside the overlapping span with a 25 km threshold returns
  `None` with no error.
- **Required behavior:** scan the *intersection* of both epoch ranges; if the
  ranges only partially overlap, either screen the overlap correctly or return
  a typed error — never silently truncate. Same review must cover
  `get_proximity_event` and `get_maneuver_event` which share the pattern.

### BUG-2 — Rust panics (PanicException) instead of typed Python errors in public API
Violates `agents.yaml` `no-panics-in-lib`. All confirmed by
`tests/test_error_handling_audit.py`:

| # | Call | Panic |
|---|------|-------|
| 2a | `Epoch.gst` / `to_fk4_greenwich_angle` / `to_fk5_greenwich_angle` on a UT1- or TT-system epoch | `to_system` returns `Err` for UT1→UT1/TT→TT identity and all UT1/TT sources; result is `.unwrap()`ed (`src/time/epoch.rs:83-85,153-160,186-214`) |
| 2b | `Epoch.from_iso("not-a-date", …)` | index out of bounds — no input validation |
| 2c | `TLECatalog.from_tle_file(<empty file>)` | index out of bounds `lines[1]` in chunk framing (`src/catalogs/tle_catalog.rs:68`) |
| 2d | `keplemon.set_thread_count(n)` called twice in one process | `build_global().unwrap()` → `GlobalPoolAlreadyInitialized` (`src/lib.rs:26-31`) |

- **Required behavior:** typed `ValueError`/`SAALError` (or graceful no-op for
  2d); implement identity conversions in `to_system` and replace the `-1.0`
  sentinel (see BUG-5). Audit the remaining ~30 `.unwrap()` sites in non-test
  lib code (`src/bodies/satellite.rs:38,48,200,209,226,239,415,436` etc.).

### BUG-3 — Silent NaN propagation for invalid orbital inputs
- **Severity:** high (NaN poisons screening: a NaN distance compares false
  against every threshold, so conjunctions are silently dropped)
- **Repro:** hyperbolic elements (e=1.2) → NaN cartesian state, no error;
  NaN position input → NaN keplerian elements; zero state → `a=0, e=NaN`.
  `tests/test_error_handling_audit.py::TestSilentInvalidResults`.
- **Required behavior:** validate at the boundary (KeplerianState/CartesianState
  constructors or conversion entry points): reject non-finite inputs and e≥1
  with typed errors (or explicitly support hyperbolic orbits and document).
  Related closed issue #5 ("crashing on invalid estimation of eccentricity")
  suggests BLS can *produce* e>1 estimates — add the same guard there.

### BUG-4 — Ephemeris.get_state_at_epoch silently clamps out-of-span queries
- **Severity:** medium-high
- **Where:** `src/elements/ephemeris.rs:793-798`; returned state carries the
  boundary epoch, not the requested one.
- **Repro:** query 10× past the span end returns the endpoint state.
  `tests/test_algorithm_verification.py::TestEphemerisInterpolation::test_query_outside_span_returns_none`.
- **Also:** `get_state_at_epoch` is missing from `stubs/keplemon/elements.pyi`
  (violates `require-stub-runtime-parity`). Sweep all runtime-exposed methods
  vs stubs (e.g. `TLE.propagate_batch`, `TLE.propagate_to_epochs`,
  `BatchPropagator`/`PropagationBackend` are similarly unstubbed).
- **Required behavior:** return `None`/raise outside the span (`covers_epoch`
  already exists); callers that want clamping must opt in explicitly.

### BUG-5 — TLE epoch-year windowing diverges from the standard from 2050
- **Severity:** low today, guaranteed failure window 2050–2056
- **Repro:** yy=56 → 1956 (standard + reference implementation: 2056);
  yy=50..55 → `ValueError: Invalid Year of Epoch (valid >= 1956)`.
  `tests/test_algorithm_verification.py::TestTleFormat::test_epoch_year_windowing`.
- **Required behavior:** decide and document: conform to the de-facto 57-pivot
  at the keplemon layer, or document SAAL's 1956 pivot prominently. (SAAL-side
  behavior; keplemon can pre-normalize the epoch field.)
- Related design smell: `Epoch::to_system` uses `-1.0` days-since-1950 as an
  in-band "unimplemented" sentinel — a legitimate 1949-12-30 epoch value.
  Replace with proper `Result` handling while fixing BUG-2a.

---

## 3. Performance findings (workflow: `performance`)

Empirical measurements on 4-core Linux, release build, this audit:

| Measurement | 1 thread | 4 threads | Verdict |
|---|---|---|---|
| `Constellation.get_states_at_epoch` (27,485 sats) | 0.057 s | 0.055 s | **no scaling** (FFI-bound) |
| `Constellation.get_ca_report_vs_many` (573 GEO sats, 6 h) | 2.61 s | 0.68 s | scales 3.8× (pure-Rust interpolation) |
| `BatchLeastSquares.solve` (existing test case) | 0.59 s | **1.26 s** | **2.1× SLOWER in parallel** |

### P1 (high) — BLS global mutex makes rayon a net negative — *measured 2.1× slowdown*
`src/estimation/batch_least_squares.rs:17` (`static SAAL_BLS_LOCK`), locked
per-observation inside `par_iter_mut` at `:421-451` and per (column ×
observation) at `:519-571`. Every work unit is one SAAL FFI call under a global
mutex: the fan-out adds dispatch + contention on top of serial execution.
**Fix:** remove rayon from these loops (serial baseline) or chunk work so the
lock is taken per chunk; add a criterion bench proving scaling before/after
(`skills/rust_perf.md` requires before/after evidence).

### P2 (high) — Contradictory SAAL thread-safety story
`batch_least_squares.rs:16` says SAAL calls are not thread-safe under rayon,
yet `constellation.rs:340-354,375-380,114-119,164-166,188-207,232-234,261-271`,
`satellite.rs:556-561`, `observation.rs:348-403`, `observatory.rs:129-141` call
SAAL from `par_iter` with **no** lock. Either those are data races or the BLS
lock is pure waste. The flat scaling of `get_states_at_epoch` (table above)
suggests SAAL serializes internally. **Fix:** establish the SAAL threading
contract once (the saal crate wraps thread-safe batch entry points — see P3),
then apply one policy library-wide.

### P3 (high) — Per-point FFI where SAAL batch entry points exist unused
`satellite.rs:206-222` builds ephemerides one `sgp4::get_position_velocity`
call per time step; `batch_propagator.rs:200-212` ("batch" CPU backend) is a
serial per-point loop; `ephemeris.rs:92-111` same. The saal crate already wraps
`sgp4::get_ephemeris` (`Sgp4GenEphems`, whole ephemeris in one FFI call) and
`sgp4::get_positions_velocities` (`Sgp4PropAllSats`, all sats at one epoch) —
zero call sites in keplemon. For `cache_ephemeris` over a 27k catalog this is
millions of FFI round-trips instead of 27k. **Fix:** route ephemeris build and
`get_states_at_epoch` through the batch entry points; preallocate state vecs
(`with_capacity`), bulk-insert under one lock acquisition.

### P4 (high) — O(n²) all-vs-all screening without sweep pruning
`constellation.rs:182-211` re-evaluates the apogee/perigee gate per pair inside
the n²/2 loop; `get_proximity_report_vs_many` (`:250-275`) has **no** prefilter
at all; ephemeris list build (`:383-388`) is serial; nested `into_par_iter`
(outer `:188` + inner `:193`) creates nested parallelism over tiny work items.
**Fix:** sort by perigee once, sweep candidate pairs (O(n log n + k)), single
par_iter level, sequential inner work.

### P5 (medium) — Hot-loop allocations and clones
- `ephemeris.rs:426-433`: two `String` clones per 10-min screening interval per
  pair (`refine_close_approach` takes `String` by value) — pass `&str`.
- `satellite.rs:82-92 clone_at_epoch` / `:136-161 new_with_delta_x`: full
  satellite clones incl. cached ephemeris.
- `constellation.rs:78-91 step_to_epoch`: clones every satellite each step, and
  `inertial_propagator.rs:43-53` re-epochs via TLE *line strings* + full
  re-parse + fresh UUID + 6 FFI calls per satellite per step (also truncates
  precision through the 69-char line format — accuracy defect, not just perf).
- `tle_catalog.rs:29-35,60-62` and `constellation.rs:30-42,402-404`: wholesale
  String/TLE/Satellite clones in getters at 27k-sat scale.
- `observation_collection.rs:190-212`: interpolates the same satellite state
  once per (observation × satellite) pair though the epoch is constant per
  collection — O(n_obs × n_sats) → O(n_sats).
- `epoch.rs:14-18`: `Hash for Epoch` formats an ISO string **through FFI** per
  hash — hash quantized `days_since_1950` instead.

### P6 (medium) — GIL held through long computations
`py.detach` is used for constellation reports and BLS solve but missing on:
`py_get_ephemeris`, `get_close_approach`, `get_plot_data`, `get_associations`,
`get_rms`/`get_residuals` (satellite bindings); `get_states_at_epoch`,
`step_to_epoch` (constellation bindings); `TLECatalog.from_tle_file`;
`Ephemeris.get_horizon_accesses`/`get_close_approach`/`to_observations`; BLS
`get_rms`/`get_residuals`/`get_covariance`. Blocks the whole interpreter for
seconds on catalog-scale calls.

### P7 (low) — Hygiene
Determinism: `par_iter` over `HashMap` yields non-deterministic report ordering
(violates `deterministic-results`); use BTreeMap/sorted index. `std::env::var`
checked every BLS iteration (`:98-100,332-334`) — cache in `OnceLock`. UUID
string generation per TLE parse/observation in hot paths — make lazy.
GPU path re-initializes the CUDA propagator per call and round-trips SoA→AoS
(`batch_propagator.rs:216-276`) despite resident-SoA kernels existing.

### Bench coverage gaps (add BEFORE fixing, per skills/rust_perf.md)
No criterion bench exists for: ephemeris generation throughput (P3 target),
`cache_ephemeris`/`get_ephemeris_list` at catalog scale, proximity vs_many,
association pipeline (`get_association_report`), `step_to_epoch` loop (P5),
`TLECatalog.from_tle_file`, BLS scaling vs thread count (P1 proof), BLS
maneuver estimation, `Ephemeris.add_state`/interpolation microbench. Existing
benches don't pin the rayon pool size — results are machine-dependent.

---

## 4. Testing gaps to close (workflow: `bugfix`/`refactor`, test-only)

Beyond the delivered audit tests, in priority order:

1. **Frame conversions unverified numerically.** No test converts
   TEME↔J2000↔EFG/ECR and checks values against an external reference
   (`CartesianState.to_frame` / `KeplerianState.to_frame`). Add reference-value
   tests (Vallado Ex. 3-15 or reference-implementation-generated fixtures).
2. **Estimation internals unasserted.** `Covariance` values, per-component
   `ObservationResidual`s, `BatchLeastSquares.iterate()`/`reset()`/
   `converged`/`weighted_rms`, and maneuver recovery (inject a synthetic Δv,
   assert recovered epoch/magnitude/direction) are never checked — only
   aggregate RMS thresholds.
3. **Events module has zero direct tests** (`ProximityReport`, `ManeuverEvent`/
   `ManeuverReport`, `UCTValidityReport`, `FieldOfViewReport`, …).
4. **`TLECatalog` mutation API untested** (`add`/`get`/`remove`/`clear`/
   `keys`/`fit_best_tle` — `fit_best_tle` is a nontrivial BLS fit with no
   coverage at all).
5. **`Constellation` coverage:** `get_ca_report_vs_one`, proximity vs_one/many,
   `get_uct_validity`, `get_maneuver_events`, dict protocol mutation.
6. **Zero-coverage element types:** `SphericalVector`, `CartesianVector`,
   `GeodeticPosition`, `TopocentricElements.from_j2000` (J2000 path never
   asserted), `HorizonElements`, `OrbitPlotData` (partially covered now by the
   audit files for vectors/equinoctial).
7. **Error-path tests:** documented `ValueError`s (`get_rms` empty obs, etc.)
   are never triggered in tests.
8. **Test-quality fixes:** `python/test_batch_gpu.py` has print-only "tests"
   with no assertions and a brittle `sys.path.insert(0, "target/debug")`;
   count-only magic-number assertions (`== 11305`, `== 5053`, `== 18`) verify
   nothing about correctness; exact float `==` comparisons in `test_time.py`
   and `test_bodies.py:16` compares a value to itself; cwd-relative fixture
   paths break `pytest` from any other directory (use `CARGO_MANIFEST_DIR`
   pattern already used in `batch_least_squares.rs:1006`).
9. **CPU CI blind spot:** all 26 `tests/*.rs` integration tests are
   CUDA-gated — on a CPU runner the batch-propagation numeric surface is
   uncovered. Add CPU-parity variants of the SoA/batch tests.
10. **Property-based round-trips** (proptest, already suggested by
    `skills/testing.md`): kep↔cart, cart↔spherical, epoch↔dtg, tle↔lines.

---

## 5. Acceptance criteria

1. All 10 strict-xfail tests on `audit/testing-gap-verification` flip to XPASS
   and their markers are removed (each fix lands with its regression test).
2. `cargo test`, `pip install .`, `pytest tests/` green; `cargo fmt --check`
   and clippy clean (per CLAUDE.md definition of done).
3. Stub/runtime parity sweep passes (every runtime-exposed symbol stubbed).
4. P1: criterion bench shows BLS solve ≥ 1 thread is never slower than serial;
   documented SAAL threading contract in code.
5. P3: ephemeris build for 573-sat catalog uses batch FFI (bench before/after
   in PR body, per `skills/rust_perf.md`).
6. P4: vs_many screening on the 573-sat GEO fixture produces byte-identical
   reports before/after (sorted output), with measured speedup.
7. New tests are deterministic (no network, no wall-clock dependence) and
   hermetic (no cwd-relative fixture paths).

## 6. Out of scope

- CUDA kernel optimization (separate PRD; no GPU in audit environment).
- `request_time_constants_update` network refactor (needs injection seam
  design — flag for architect review).
- SAAL-internal behavior changes (windowing pivot, checksum validation are
  wrapped, not owned; keplemon-layer normalization/documentation only).

## 7. How to run the audit deliverables

```bash
git fetch origin audit/testing-gap-verification
git checkout audit/testing-gap-verification
pip install . && pip install pytest sgp4
pytest tests/test_algorithm_verification.py tests/test_error_handling_audit.py -v
# expected: 36+ passed, 10 xfailed (strict) — xfails are the open bugs
```
