#!/usr/bin/env bash
set -euo pipefail
trap './scripts/stop-prototype.sh' EXIT
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo deny check
npm --prefix frontend run typecheck
npm --prefix frontend run lint
npm --prefix frontend test
npm --prefix frontend run contract
npm --prefix frontend run build
if [[ -d data/prototype ]]; then find data/prototype -mindepth 1 -delete; fi
./scripts/start-prototype.sh
npm --prefix frontend run e2e
cargo run --locked -p ch-cli -- ablate | tee evidence/prototype/ablation.txt
