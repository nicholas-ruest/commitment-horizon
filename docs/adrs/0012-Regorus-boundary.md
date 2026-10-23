# ADR-0012: Regorus Boundary

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Exception rules must be reviewable and deterministic. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Hand-coded conditionals, OPA sidecar, or embedded Regorus. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Load Rego into `regorus::Engine` and evaluate typed input. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Policy syntax becomes an operational dependency. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Real allow/deny/malformed-policy tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
