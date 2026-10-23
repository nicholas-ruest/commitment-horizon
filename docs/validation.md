# Validation receipt

The exact implementation commit `f58811adf6b8f32749d08835d8363f998fa9ffe8` was exercised on the governed ruOS executor with Rust 1.99.0 and Node 22.

| Gate | Command | Observed result |
| --- | --- | --- |
| Format | `cargo fmt --check` | exit 0 |
| Strict Rust lint | `cargo clippy --locked --all-targets --all-features -- -D warnings` | exit 0, 24 crates |
| Locked Rust tests | `cargo test --locked --workspace` | exit 0, 9 focused tests plus crate/doc harnesses; the composed integration test executes RuVector, RVF and Regorus and verifies denied-write absence plus restart retrieval |
| Dependency policy | `cargo deny check` | exit 0; all categories pass; unmaintained transitive `bincode 2.0.1` from RuVector explicitly tracked because no safe upgrade exists |
| Frontend | `typecheck`, `lint`, `vitest`, contract drift, production build | exit 0; one non-failing exhaustive-deps advisory remains |
| Whole product | `scripts/verify-prototype.sh` | exit 0 |
| Browser acceptance | `playwright test` against live UI and Axum API | 3/3 passed, no first-party route interception |
| Restart durability | compare audit JSON before/after service restart | identical SHA-256 `e07b8fded590d2d4643e95f1662ed3d0d8ec72050c971d46760214c56d4346a5` |
| API benchmark | `scripts/benchmark.py --base http://127.0.0.1:18080/api/v1 --requests 50` | 50/50 successful; measured mean 69.878 ms on this executor |
| Ablations | `cargo run --locked -p ch-cli -- ablate` | combined 0.8783; without calibration 0.7766; without e-process 0.8405; Ruvnet-only analogue coverage 0.2500 (no calibrated breach decision); enterprise-only 0.7389 |
| Structural checker | checker `80f006f6960bc4e491e97f51e890ff708e81aa88` | regression suite 3/3; candidate `STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW`, 24 crates / 25 ADRs / 12 DDDs / 2 animated SVGs, zero failures |
| Workspace linkage audit | `cargo metadata --locked --no-deps --format-version 1` | 24/24 members with typed path-dependency edges; API and CLI are executable roots |

Container manifests were built into CI but Docker was not installed on the local executor. Cloud deployment was not authorized. Playwright screenshots and traces are uploaded by CI; two verified screenshots are committed under `docs/assets/`.

Ruflo 3.47.1 executed dependency/boundary analysis. Its Rust boundary detector returned zero nodes, so that output is retained as an honest tool limitation rather than semantic evidence. Manual Cargo caller-path inspection and the structural checker remain authoritative.
