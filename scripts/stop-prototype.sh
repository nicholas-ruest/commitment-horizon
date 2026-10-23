#!/usr/bin/env bash
set -euo pipefail
for file in .run/frontend.pid .run/api.pid; do
  if [[ -f "$file" ]]; then
    pid=$(<"$file")
    kill -- "-$pid" 2>/dev/null || kill "$pid" 2>/dev/null || true
    rm -f "$file"
  fi
done
