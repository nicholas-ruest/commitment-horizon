# ADR-0024: Evaluation Contract

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Ingredient claims require measured baselines and ablations. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Demo screenshots, unit tests only, or frozen evaluator. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Run combined, enterprise-only, Ruvnet-only and ingredient-removal variants. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Synthetic workload limits external validity and is disclosed. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Raw JSON results with dataset hash. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
