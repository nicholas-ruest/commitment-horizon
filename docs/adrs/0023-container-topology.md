# ADR-0023: Container Topology

Status: accepted for the 2026-10-22 design checkpoint.

## Context

A clean checkout needs reproducible launch. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Host scripts only, one opaque image, or backend/frontend containers. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Provide Dockerfiles and Compose plus Linux bootstrap/start/stop. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Two images cost more but isolate runtime responsibilities. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Containerized health and Playwright CI. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
