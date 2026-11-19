#!/usr/bin/env bash
#
# examples.sh — Convert examples/petstore.yaml into API client collections
# for every supported format.
#
# Usage:
#   ./examples/examples.sh
#   FORMATS=postman,rest-client ./examples/examples.sh
#   OUT=./tmp NAME="Petstore API" ./examples/examples.sh
#
# Environment:
#   FORMATS  comma-separated formats or "all" (default: all)
#   OUT      output directory (default: examples/output)
#   NAME     collection name override (default: spec info.title)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT/target/release/api"
SPEC="$ROOT/examples/petstore.yaml"
OUT="${OUT:-$ROOT/examples/output}"
FORMATS="${FORMATS:-all}"

if [[ ! -x "$BIN" ]]; then
    echo "error: $BIN not found; run 'cargo build --release -p api-cli' first" >&2
    exit 1
fi

if [[ ! -f "$SPEC" ]]; then
    echo "error: $SPEC not found" >&2
    exit 1
fi

args=(convert "$SPEC" --format "$FORMATS" --out "$OUT")
if [[ -n "${NAME:-}" ]]; then
    args+=(--name "$NAME")
fi

echo "converting $SPEC -> $OUT (formats: $FORMATS)"
"$BIN" "${args[@]}"

find "$OUT" -type f | sort | sed "s|$ROOT/||"
