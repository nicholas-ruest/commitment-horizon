# Specification

## Problem and users

Platform owners promise latency, availability, recovery, data-residency, and safety outcomes to internal and external stakeholders. Those commitments are usually scattered across tickets, dashboards, and prose. Evidence arrives continuously, exceptions are approved inconsistently, and auditors cannot reproduce why an obligation was considered healthy or at risk. SRE leads, service owners, compliance reviewers, and account teams need one governed product that turns a measurable commitment and incoming observations into an explainable risk horizon and durable decision trail.

## System capability

Commitment Horizon registers a typed operational commitment, accepts deterministic evidence through a versioned API, computes an anytime-valid and conformal breach-risk horizon, recalls analogous obligations through RuVector, evaluates exception authority through Microsoft Regorus, persists append-only evidence through RVF, and exposes all results through a responsive React application.

No ingredient provides the combination. RuVector retrieves operational analogues but does not own obligations. RVF durably packages evidence but does not compute risk. Regorus evaluates policy but does not forecast breach. The research mechanism supplies calibrated uncertainty but no workflow, persistence, UI, or authorization.

## Bounded contexts

1. Commitment Registry owns commitments, thresholds, windows, and lifecycle.
2. Evidence Intake validates observations and preserves idempotent ordering.
3. Risk Intelligence performs conformal horizon estimation, anytime-valid monitoring, and RuVector analogue recall.
4. Exception Governance evaluates Regorus policy, authority, scope, and expiry.
5. Audit & Recovery owns RVF persistence, replay, restart hydration, and visible history.
6. Product Interface owns authenticated API flows, correlation, accessibility, and browser journeys.

## Acceptance journeys

### J1 — Register and evaluate a commitment

An operator signs in with the documented development identity, creates a 99.9% availability commitment, submits a 30-point evidence window, and selects Evaluate. The browser calls the typed client, middleware validates identity/idempotency/correlation, Rust composes registry, calibration, e-process, RuVector recall, Regorus policy, and RVF persistence. The UI renders the risk band, breach horizon, comparable obligations, and audit receipt.

### J2 — Policy denial and recovery

An operator requests an exception beyond the permitted duration. Regorus denies it, no exception event is persisted, and the UI shows a structured denial with a corrective duration. The operator shortens the duration and retries with the same correlation context; the allowed exception is persisted exactly once and appears in history.

### J3 — Refresh and restart retrieval

After a saved evaluation, the browser refreshes and retrieves the audit record. The backend process is then restarted against the same test namespace and RVF directory. It rehydrates the commitment/evaluation index, returns the identical receipt hash, and the UI displays the saved result without synthetic fallback data.

## Success criteria

- One command starts backend and frontend and reports both health checks.
- All three journeys cross UI, middleware, Rust application crates, RuVector/RVF, Regorus where applicable, and visible browser results.
- Invalid thresholds and duplicate evidence are rejected without forbidden effects.
- Restart retrieval returns the identical receipt hash from RVF-backed state.
- Combined evaluation differs from no-RuVector, no-Regorus, no-calibration, and no-RVF ablations.
- At least 20 meaningful Rust crates participate in backend call paths or executable evaluation.
- Locked Rust and Node tests, strict lint, production frontend build, contract drift, container checks, and Playwright E2E pass against real services.

## Constraints

Synthetic fixtures are labeled. The prototype never mutates production infrastructure. Browser code contains no service credential. Files remain under 500 lines, paths and payloads are bounded, errors are typed, timeouts and cancellation are explicit, and the service owns no shadow database. RVF is the durable record; RuVector is rebuilt from verified RVF projections after restart.
