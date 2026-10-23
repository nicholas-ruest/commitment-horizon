# ADR-0018: Http Api

Status: accepted for the 2026-10-22 design checkpoint.

## Context

The frontend needs a stable typed contract. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were GraphQL, ad-hoc JSON, or versioned REST/OpenAPI. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Expose `/api/v1` Axum routes and committed OpenAPI. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Versioning adds maintenance overhead. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Schema validation and response snapshots. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
