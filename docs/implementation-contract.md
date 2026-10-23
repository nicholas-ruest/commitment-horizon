# Implementation contract

## Ordered delivery

1. Commit design and the three browser journeys before code.
2. Build the first vertical slice: browser registers a commitment through the real API and RVF repository, then reads it back.
3. Add evidence evaluation through calibration, e-process, RuVector recall, Regorus, audit and visible risk results.
4. Add denied/corrected exception flow and restart retrieval.
5. Add benchmarks, ablations, container launch, browser/mobile proof, structural and semantic review, then governed publication.

## API and frontend

OpenAPI is the source contract. The TypeScript client is generated deterministically and checked for drift. Required routes are health/readiness, commitments create/list/get, evidence evaluation, exception request, and audit retrieval. The React product has Overview, Commitments, Evaluation, Exceptions and Audit routes with loading, empty, validation, success, error, narrow viewport, and keyboard focus states. No primary control uses hardcoded response data.

## Runtime and middleware

The API binds only configured addresses, limits request bodies, uses a 10-second application timeout, propagates cancellation, emits structured tracing with correlation IDs, accepts a documented development bearer identity, rate-limits mutations, and deduplicates idempotent requests. Error bodies never include filesystem paths or secrets.

## Verification

`scripts/verify-prototype.sh` starts the real API and frontend, waits for both health endpoints, runs Rust format/Clippy/tests/security policy, frontend format/type/lint/unit/build, contract drift, API integration, Playwright desktop/mobile journeys, restart retrieval, benchmark and ablations. It writes a manifest bound to the exact source. CI uploads browser report, traces, screenshots and correlation evidence. Any missing layer forbids BUILT.

## Publication

The indexed candidate must pass the pinned checker at `80f006f6960bc4e491e97f51e890ff708e81aa88`, then independent call-path/evidence review, then a non-draft PR and exact-main CI. Repository and two Gists are read back. Core Memory receives the exact outcome and one unapproved feedback proposal issue; no self-review or automatic merge occurs.
