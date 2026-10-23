#!/usr/bin/env bash
set -euo pipefail
mkdir -p .run evidence/prototype data
export HORIZON_BIND="${HORIZON_BIND:-127.0.0.1:8080}"
export HORIZON_DATA_ROOT="${HORIZON_DATA_ROOT:-$PWD/data}"
export HORIZON_NAMESPACE="${HORIZON_NAMESPACE:-prototype}"
export HORIZON_DEV_TOKEN="${HORIZON_DEV_TOKEN:-local-operator}"
setsid cargo run --locked -p ch-api --bin commitment-horizon-api >evidence/prototype/api.log 2>&1 & echo $! >.run/api.pid
setsid npm --prefix frontend run dev -- --port 14173 --strictPort >evidence/prototype/frontend.log 2>&1 & echo $! >.run/frontend.pid
for _ in $(seq 1 90); do curl -fsS "${HORIZON_HEALTH_URL:-http://127.0.0.1:8080}/readyz" >/dev/null && curl -fsS http://127.0.0.1:14173 >/dev/null && exit 0; sleep 1; done
echo "prototype readiness timeout" >&2; exit 1
