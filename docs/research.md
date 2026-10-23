# Research and selection

## Candidate comparison

| Candidate | System capability | Rejection or selection |
| --- | --- | --- |
| Commitment Horizon | Live operational-commitment registry, risk horizon, governed exceptions, durable audit UI | Selected: product-shaped, multi-context, and distinct from recent builds |
| Prompt License Scanner | Scores prompt bundles for licensing risk | Rejected as validator-shaped utility |
| Agent Cost Threshold CLI | Warns when model spend exceeds a limit | Rejected as single-feature monitor |

## Primary sources and pins

- RuVector repository `46d816ddc4c2a8a06cd3613534a79b1d557d858b`, MIT: <https://github.com/ruvnet/RuVector/tree/46d816ddc4c2a8a06cd3613534a79b1d557d858b>. `ruvector-core 2.4.0` executes insert/search for comparable obligations.
- RVF in the same repository, MIT OR Apache-2.0. `rvf-runtime 0.3.2` executes create, ingest, reopen, and query for durable audit bundles.
- Microsoft Regorus `0.13.0`, commit `9d19b6a578d22d6a223c48ba37e632d7d74ee0db`, MIT AND Apache-2.0 AND BSD-3-Clause: <https://github.com/microsoft/regorus/tree/9d19b6a578d22d6a223c48ba37e632d7d74ee0db>. The real `Engine` parses and evaluates exception policy.
- Gibbs and Candès, “Adaptive Conformal Inference Under Distribution Shift,” NeurIPS 2021: <https://proceedings.neurips.cc/paper/2021/hash/0d441de75945e5acbc865406fc9a2559-Abstract.html>. A bounded online correction updates the target quantile after each miss.
- Ramdas et al., “Game-Theoretic Statistics and Safe Anytime-Valid Inference,” arXiv:2210.01948: <https://arxiv.org/abs/2210.01948>. A nonnegative evidence process guards repeated inspection of breach evidence.

## Frontier mechanism

The service stores a declared target miscoverage, deterministic residual calibration window, bounded learning rate, and sequential e-value. The conformal interval estimates the plausible breach horizon while adaptive correction responds to drift. The e-process separately answers whether continuing observations support an unacceptable breach-rate null under optional stopping. The UI reports both interval and evidence rather than collapsing uncertainty into a single score.

This is a deliberately bounded prototype: scalar commitment evidence, exchangeability diagnostics, deterministic residuals, and synthetic observations. It does not claim universal contractual interpretation, arbitrary causal attribution, or production calibration without domain validation.

## Novelty ledger

| Date | Project | Capability | Distinction |
| --- | --- | --- | --- |
| 2026-10-16 | Provenance Firebreak | Artifact containment | Horizon manages ongoing commitments and exceptions, not artifact quarantine. |
| 2026-10-17 | Continuity Atlas | Regional failover | Horizon evaluates obligation health, not recovery topology. |
| 2026-10-18 | Context Ferry | Context transfer | Horizon does not migrate agent state. |
| 2026-10-19 | Counterfactual Shield | Pre-action authorization | Horizon monitors contractual outcomes over time. |
| 2026-10-20 | Canary Covenant | Runtime rollout control | Horizon is not release/cohort promotion. |
| 2026-10-21 | Causal Tapestry | Incident reconstruction | Horizon forecasts commitments prospectively and governs exceptions. |
