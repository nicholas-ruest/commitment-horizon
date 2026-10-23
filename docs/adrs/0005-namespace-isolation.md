# ADR-0005: Namespace Isolation

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Browser tests and tenants must not share durable data. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Global file, database per service, or validated namespace directory. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Use validated namespace roots and explicit retention/reset commands. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Many small bundles require compaction later. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Cross-namespace and safe-delete tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
