# ADR-0013: Exception Authority

Status: accepted for the 2026-10-22 design checkpoint.

## Context

An exception changes obligation interpretation and needs bounded scope. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Free-form note, permanent override, or scoped expiring command. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Require owner role, reason, maximum duration, correlation and evidence receipt. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Some legitimate long exceptions need a higher policy tier. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Denied mutation and corrected retry tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
