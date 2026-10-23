# Prototype acceptance map

| Journey | UI route/control | API + middleware | Application/crates | Upstream/persistence | Executed proof |
|---|---|---|---|---|---|
| Main outcome | Horizon → Evaluate selected | `POST /api/v1/commitments/{id}/evaluate`; bearer + correlation validation | evidence → calibration/e-process/forecast → risk → recall | real `ruvector-core` search; append-only `rvf-runtime` records | Playwright `register and evaluate`; correlation in JSON and API log |
| Denial + recovery | Try as viewer, then Approve as operator | `POST .../exceptions`; principal extension; consistent 403 | auth → policy → Regorus; write only after allow | Microsoft Regorus; RVF has no denied exception, then approved receipt | Playwright denial assertion followed by successful retry |
| Refresh/restart readback | Audit → refresh → Audit | `GET .../audit`; bearer + ID validation | orchestrator filters durable records | `RvfStore::open_readonly` reconstructs records | Playwright refresh journey plus restart receipt test |

The seed is deterministic and explicitly synthetic. Browser acceptance never intercepts first-party API routes. The test namespace is `prototype`; delete `data/prototype` to reset it.
