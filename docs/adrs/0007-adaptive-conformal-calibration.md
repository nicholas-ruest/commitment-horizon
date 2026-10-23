# ADR-0007: Adaptive Conformal Calibration

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Static uncertainty under distribution shift becomes misleading. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Fixed quantile, Bayesian model, or adaptive conformal correction. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Use bounded online alpha correction over residuals. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Guarantees depend on documented assumptions. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Coverage/drift/learning-rate tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
