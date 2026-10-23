#!/usr/bin/env python3
"""Reproducible end-to-end benchmark against a running prototype."""

import argparse
import json
import time
import urllib.request


def request(url: str, token: str, body: bytes | None = None):
    headers = {"authorization": f"Bearer {token}", "content-type": "application/json"}
    method = "POST" if body is not None else "GET"
    return urllib.request.urlopen(
        urllib.request.Request(url, data=body, headers=headers, method=method), timeout=10
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", default="http://127.0.0.1:8080/api/v1")
    parser.add_argument("--token", default="local-operator")
    parser.add_argument("--requests", type=int, default=50)
    args = parser.parse_args()
    rows = json.load(request(f"{args.base}/commitments", args.token))
    if not rows:
        raise SystemExit("seed/register a commitment before benchmarking")
    commitment_id = rows[0]["id"]
    body = json.dumps(
        {
            "commitment_id": commitment_id,
            "source": "synthetic-benchmark-v1",
            "points": [
                {"sequence": index + 1, "observed": value, "timestamp_ms": 1_700_000_000_000 + index * 60_000}
                for index, value in enumerate([0.99, 0.96, 0.88, 0.81])
            ],
        }
    ).encode()
    started = time.perf_counter()
    codes = [
        request(f"{args.base}/commitments/{commitment_id}/evaluate", args.token, body).status
        for _ in range(args.requests)
    ]
    elapsed_ms = (time.perf_counter() - started) * 1000
    print(
        json.dumps(
            {
                "dataset": "synthetic-benchmark-v1",
                "requests": args.requests,
                "success": sum(code == 201 for code in codes),
                "total_ms": round(elapsed_ms, 3),
                "mean_ms": round(elapsed_ms / args.requests, 3),
            }
        )
    )


if __name__ == "__main__":
    main()
