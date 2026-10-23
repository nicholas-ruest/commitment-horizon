# ADR-0025: Publication And Learning

Status: accepted for the 2026-10-22 design checkpoint.

## Context

A checker or feedback issue must not self-authorize release or learned policy. The decision must preserve the complete browser-to-storage acceptance path, typed boundaries, failure visibility, and the authority limits in the canonical build standard.

## Alternatives

The viable alternatives considered were Automatic merge, chat claim, or governed gates. Each was evaluated for domain fit, reproducibility, operational failure behavior, and whether it could be validated without fabricated live results.

## Decision

Require checker + semantic review + PR CI; create feedback proposal only after exact outcome. Ownership and dependency direction are recorded in the crate map; adapters cannot create policy authority and the UI cannot bypass middleware.

## Tradeoffs and consequences

Human review remains manual. This choice is reversible through the stated adapter or contract boundary, but existing evidence remains append-only. A failure is explicit and cannot be converted into an optimistic default.

## Validation

Readback of commit, CI, Gists, outcome and issue. The exact source, commands, exit codes, and artifacts are bound in the prototype manifest. A structural checker pass alone does not satisfy this ADR.
