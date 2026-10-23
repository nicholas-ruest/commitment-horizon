# ADR-0001: System Boundary

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Deliver a commitment-control product rather than a generic dashboard. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were CLI-only, generic ticketing, or a bounded full-stack control plane. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Build a browser/API/Rust product around measurable operational commitments. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

More integration work, but the unit of value is a usable workflow. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

All three journeys must execute against real services. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
