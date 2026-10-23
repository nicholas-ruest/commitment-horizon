# ADR-0002: Commitment Aggregate

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Targets, windows, owners and lifecycle must change atomically. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Loose records, mutable maps, or one aggregate. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Use a typed Commitment aggregate with explicit transitions. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Stricter commands reject convenient partial updates. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Unit invariants and API mutation tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
