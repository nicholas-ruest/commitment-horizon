# ADR-0017: Application Saga

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Evaluation crosses algorithms, policy and two upstream stores. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Fat HTTP handler, event bus, or explicit saga. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

`HorizonService` owns ordered application steps and rollback semantics. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Central orchestration is larger but testable. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Vertical-slice and partial-failure tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
