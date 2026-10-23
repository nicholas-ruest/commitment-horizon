#!/usr/bin/env bash
set -euo pipefail
payload=$(cargo run --quiet --locked -p ch-cli -- seed payments-slo)
curl -fsS -X POST http://127.0.0.1:8080/api/v1/commitments -H 'authorization: Bearer local-operator' -H 'content-type: application/json' -H 'idempotency-key: seed-payments-v1' -d '{"id":"payments-slo","name":"Payments success rate","owner":"payments-platform","target":0.95,"window_minutes":60}' >/dev/null || true
curl -fsS -X POST http://127.0.0.1:8080/api/v1/commitments/payments-slo/evaluate -H 'authorization: Bearer local-operator' -H 'content-type: application/json' -d "$payload"
