# Integration matrix

| Ingredient | Pin / license | Executed capability | Boundary | Crates | Integration test | Ablation |
| --- | --- | --- | --- | --- | --- | --- |
| RuVector Core | `2.4.0`, repo `46d816d…`, MIT | Insert and nearest-neighbour search over typed obligation vectors | In-process Cargo adapter | `ch-embedding`, `ch-ruvector`, `ch-recall` | register/evaluate returns seeded nearest obligation from real `VectorDB` | `NoRuVector` removes analogue explanation and lowers completeness |
| RVF Runtime | `0.3.2`, repo `46d816d…`, MIT OR Apache-2.0 | Create, ingest, close, reopen, query durable evidence | File-backed Cargo adapter with namespace path policy | `ch-rvf`, `ch-audit`, `ch-repository` | save, process restart, exact receipt retrieval | `NoRvf` cannot claim saved/restart-durable result |
| Microsoft Regorus | `0.13.0`, `9d19b6a…`, multi-license | Parse/evaluate Rego exception authorization | In-process Cargo adapter | `ch-regorus`, `ch-policy` | long exception denied; corrected exception allowed | `NoRegorus` blocks exception authority |
| Adaptive conformal inference | Gibbs & Candès 2021 | Drift-responsive quantile correction | Owned deterministic algorithm | `ch-calibration`, `ch-forecast` | frozen coverage/drift fixtures | fixed empirical interval widens or misses under shift |
| Anytime-valid e-process | Ramdas et al. 2022 | Optional-stopping-safe sequential evidence | Owned bounded update | `ch-eprocess`, `ch-risk` | supportive/adverse/invalid streams | `NoEProcess` removes sequential evidence gate |
| Ruflo | installed runtime | Coordination and source-bound execution receipts | External build ledger, never implementation | build only | transcript readback | unavailable coordination disclosed |
| MetaHarness/Darwin/Flywheel | installed governed tools | Evaluation receipt and benchmark verification | External read-only evaluator | `ch-evaluation` outputs | exact source receipt | no promotion authority |
