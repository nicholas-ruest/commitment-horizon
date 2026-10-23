# ADR-0004: Evidence Contract

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Risk depends on ordered, idempotent observations. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Accept arbitrary JSON, server timestamps, or typed sequence records. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Require finite typed values, monotonic sequence, source and idempotency key. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Some telemetry must be normalized upstream. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Duplicate/out-of-order/nonfinite tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
