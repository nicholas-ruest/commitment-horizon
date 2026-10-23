# Core Memory guidance map

Repository guidance source: `nicholas-ruest/core-memory@4312b00fb7056482fbfb5d8dab93f09adedfef92`.

| Source | Instruction | Decision | Evidence state |
| --- | --- | --- | --- |
| AGENTS.md + canonical standard | Real full-stack prototype; 20+ meaningful Rust crates; design first | 24-crate backend plus React app; design committed before implementation | APPLIED when design commit exists and journeys pass |
| canonical standard | Three UI-to-storage journeys and restart retrieval | J1/J2/J3 in prototype acceptance | APPLIED by Playwright/API/RVF evidence |
| CLAUDE.md | one writer, focused files, typed boundaries, no secrets | single integration worktree; files <500 lines; `.env.example` only | APPLIED by source review |
| learning-loop.md + ADR-0026 | verified reviewed digest only; feedback is proposal | Connector verified no `memory/lessons/` tree at source SHA; `NONE_AVAILABLE`; lesson map empty | APPLIED; no learned text injected |
| explicit task_start | distinguish empty unscreened response | returned empty arrays with `unscreened=true` | `LIVE_MEMORY_EMPTY_UNSCREENED` |
