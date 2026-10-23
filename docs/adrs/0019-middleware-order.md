# ADR-0019: Middleware Order

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Auth, validation and correlation must precede effects. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Per-handler checks, proxy-only controls, or ordered middleware. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Apply correlation, tracing, auth, limits, timeout, rate, idempotency before handlers. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Local development requires documented identity headers. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Forbidden-effect and correlation tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
