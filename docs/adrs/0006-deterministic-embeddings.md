# ADR-0006: Deterministic Embeddings

Status: accepted for the 2026-10-22 design checkpoint.

## Context

Analogue recall must be reproducible without a model service. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Hosted embedding, bag-of-words, or fixed typed vector. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Encode target, window, observed rate, volatility, domain and severity into eight dimensions. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Semantic richness is limited but source-bound. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Golden-vector and dimension tests. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
