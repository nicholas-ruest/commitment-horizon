# Architecture

## Product topology

The browser loads a React/TypeScript application on port 4173 and uses a generated typed client for `/api/v1`. Axum middleware authenticates the documented development identity, assigns or preserves an opaque correlation ID, enforces request/body/rate/idempotency limits, and returns RFC 9457-style problems. `ch-orchestrator` owns application sagas; domain crates remain transport-free. RuVector supplies intelligence and RVF supplies restart-durable append-only state. Regorus is the only exception-policy evaluator.

The container topology is two stateless processes over one authorized RVF volume in local mode: `commitment-horizon-api` and the static frontend server. Cloud Run configuration keeps the API stateless and expects an authorized external RVF/RuVector service or Cloud SQL-backed governing adapter before production; this prototype does not claim that deployment.

## Browser-to-storage call path

`React control -> generated client -> Axum route -> auth/correlation/idempotency/rate middleware -> HorizonService -> commitment/evidence/calibration/e-process/forecast/risk -> RuVector recall -> Regorus policy -> audit -> RVF archive -> JSON response -> React result and audit views`.

## Persistence and restart

Each namespace has a validated directory below the configured data root. A completed mutation writes an RVF bundle and an append-only canonical projection; startup verifies bundles, reconstructs typed records, and rehydrates RuVector. No SQLite or service-owned database exists. Tests use a unique namespace and delete only that explicit directory after validation.

## Crate map (24 meaningful Rust crates)

| Crate | Responsibility | Context | Typed API | Caller | Traceability | Tests | Acceptance |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ch-types` | Shared IDs, percentages, windows, decisions, and API-neutral values | Shared Kernel | `CommitmentId, EvidencePoint, RiskBand` | all domain crates | ADR-0001; DDD-01 | Unit + vertical slice | participates in tested call path |
| `ch-errors` | Stable error taxonomy and public problem details | Shared Kernel | `HorizonError, Problem` | middleware, API, adapters | ADR-0002; DDD-02 | Unit + vertical slice | participates in tested call path |
| `ch-config` | Validated environment configuration, paths, limits, and timeouts | Runtime | `RuntimeConfig::load` | API and orchestrator | ADR-0003; DDD-03 | Unit + vertical slice | participates in tested call path |
| `ch-commitment` | Commitment aggregate, lifecycle, target and invariant enforcement | Registry | `Commitment::register, suspend, activate` | orchestrator, repository | ADR-0004; DDD-04 | Unit + vertical slice | participates in tested call path |
| `ch-evidence` | Evidence batch validation, idempotency keys, ordering and summaries | Evidence Intake | `EvidenceBatch::new, summary` | orchestrator, e-process | ADR-0005; DDD-05 | Unit + vertical slice | participates in tested call path |
| `ch-calibration` | Adaptive conformal residual window and quantile correction | Risk Intelligence | `Calibrator::observe, interval` | forecast, evaluation | ADR-0006; DDD-06 | Unit + vertical slice | participates in tested call path |
| `ch-eprocess` | Anytime-valid sequential breach evidence | Risk Intelligence | `EProcess::observe, value` | risk, evaluation | ADR-0007; DDD-07 | Unit + vertical slice | participates in tested call path |
| `ch-forecast` | Breach-horizon projection with uncertainty band | Risk Intelligence | `Forecaster::project` | risk, orchestrator | ADR-0008; DDD-08 | Unit + vertical slice | participates in tested call path |
| `ch-risk` | Fuses forecast, e-value, budget and analogue severity | Risk Intelligence | `RiskEngine::assess` | policy, orchestrator | ADR-0009; DDD-09 | Unit + vertical slice | participates in tested call path |
| `ch-embedding` | Deterministic obligation/evidence vector encoding | Risk Intelligence | `embed_commitment` | RuVector adapter | ADR-0010; DDD-10 | Unit + vertical slice | participates in tested call path |
| `ch-ruvector` | Real RuVector index boundary with bounded search | Risk Intelligence | `ObligationIndex::insert, search` | recall, repository | ADR-0011; DDD-11 | Unit + vertical slice | participates in tested call path |
| `ch-recall` | Ranks and explains comparable obligations | Risk Intelligence | `RecallService::nearest` | orchestrator | ADR-0012; DDD-12 | Unit + vertical slice | participates in tested call path |
| `ch-regorus` | Real Regorus exception-policy adapter | Governance | `RegoPolicy::evaluate` | policy | ADR-0013; DDD-01 | Unit + vertical slice | participates in tested call path |
| `ch-policy` | Fail-closed evaluation and exception decisions | Governance | `PolicyEngine::evaluate, request_exception` | orchestrator | ADR-0014; DDD-02 | Unit + vertical slice | participates in tested call path |
| `ch-audit` | Append-only domain audit events and receipt hashes | Audit & Recovery | `AuditLedger::append, verify` | orchestrator, RVF | ADR-0015; DDD-03 | Unit + vertical slice | participates in tested call path |
| `ch-rvf` | RVF create/ingest/reopen/query adapter | Audit & Recovery | `RvfArchive::persist, verify` | repository | ADR-0016; DDD-04 | Unit + vertical slice | participates in tested call path |
| `ch-repository` | Restart-durable RVF projection and RuVector rehydration | Audit & Recovery | `Repository::save, load, list` | orchestrator, API | ADR-0017; DDD-05 | Unit + vertical slice | participates in tested call path |
| `ch-auth` | Development identity extraction and role model | Product Interface | `Identity::from_header` | middleware | ADR-0018; DDD-06 | Unit + vertical slice | participates in tested call path |
| `ch-middleware` | Correlation, rate limit, idempotency and policy context | Product Interface | `RequestContext, Guard` | API | ADR-0019; DDD-07 | Unit + vertical slice | participates in tested call path |
| `ch-orchestrator` | Register/evaluate/exception/retrieval application sagas | Application | `HorizonService` | API and evaluation | ADR-0020; DDD-08 | Unit + vertical slice | participates in tested call path |
| `ch-api` | Versioned Axum HTTP service, OpenAPI and health/readiness | Product Interface | `router, serve` | browser and tests | ADR-0021; DDD-09 | Unit + vertical slice | participates in tested call path |
| `ch-seed` | Deterministic synthetic fixture ingestion through application ports | Operations | `seed_namespace` | API startup, tests | ADR-0022; DDD-10 | Unit + vertical slice | participates in tested call path |
| `ch-evaluation` | Benchmarks, baselines, ablations and frozen workload | Evaluation | `Evaluator::run` | CI executable | ADR-0023; DDD-11 | Unit + vertical slice | participates in tested call path |
| `ch-cli` | Operator seed/evaluate/verify interface | Product Interface | `commitment-horizon` | operator and CI | ADR-0024; DDD-12 | Unit + vertical slice | participates in tested call path |

## Dependency direction

Shared types/errors/config flow inward to domain aggregates and algorithms, then adapters, application orchestration, and interfaces. Domain crates cannot import Axum, Regorus, RuVector, or RVF. Only adapter crates import their upstream. `ch-api`, `ch-cli`, and `ch-evaluation` are executable roots. The UI imports only the generated client package.

## Failure model

Invalid identity or payload fails before domain execution. Regorus denial produces no exception mutation. RVF failure makes the request incomplete and never returns a saved receipt. RuVector failure permits a visible degraded assessment but blocks an exception approval. Timeout/cancellation returns a retryable problem with the correlation ID. Duplicate idempotency keys return the first committed result.
