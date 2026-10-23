#!/usr/bin/env bash
set -euo pipefail
cargo fetch --locked
npm --prefix frontend ci
npx --prefix frontend playwright install chromium
