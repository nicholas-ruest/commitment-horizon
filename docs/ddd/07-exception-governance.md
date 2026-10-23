# DDD-07: Exception Governance

## Bounded context and model

ExceptionRequest aggregate owns scope, duration, reason, requester and decision. This bounded context exposes commands and immutable query views rather than transport-specific structures. Its anti-corruption layer rejects upstream values that violate the shared language.

## Ubiquitous language, aggregates, entities and value objects

The aggregate root controls mutation and emits canonical events. Entities retain opaque identifiers; value objects validate at construction and cannot represent an invalid percentage, path, duration, sequence, or correlation identifier.

## Invariants

Denied requests have no exception side effect; allowed exceptions expire and cite policy input. Authorization and persistence are separate concerns: a valid domain command is not automatically authorized, and an authorized command is not complete until durable evidence confirms it.

## Domain events

Events: `ExceptionDenied, ExceptionGranted, ExceptionExpired`. Events carry schema version, namespace, aggregate ID, correlation ID, sequence and safe evidence hash. They never contain credentials or unbounded customer text.

## Ports, adapters and relationships

Inbound ports are used by `ch-orchestrator`; outbound ports are implemented by the specific Regorus, RuVector, RVF or interface adapter named in architecture.md. Dependencies point inward. Failure returns a typed error and cannot silently downgrade policy authority.

## Acceptance

Unit tests cover invariants and event construction. At least one J1/J2/J3 journey crosses this boundary or a documented executable evaluation path, and the prototype manifest cites the resulting evidence.
