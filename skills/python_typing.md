# Python typing skill

## Use when

- Any public Python API, wrappers, glue code, tests that hit bindings.

## Hard rules

- Strict typing: no implicit Optional, no `Any` in public surface.
- `mypy` must pass; add `py.typed` if distributing typed package.
- Prefer small typed dataclasses / pydantic models for boundaries.

## Canonical patterns

- Typed exceptions, typed result objects
- `Protocol` for injectable dependencies
