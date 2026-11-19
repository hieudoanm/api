#!/usr/bin/env bash
#
# examples.sh — Convert examples/petstore.yaml into API client collections
# for every supported format, plus browsable API documentation previews.
#
# Usage:
#   ./examples/examples.sh
#   FORMATS=postman,rest-client ./examples/examples.sh
#   PREVIEW=0 ./examples/examples.sh          # skip doc previews
#   PREVIEW_FORMATS=redoc,scalar ./examples/examples.sh
#   OUT=./tmp NAME="Petstore API" ./examples/examples.sh
#
# Environment:
#   FORMATS          comma-separated client formats or "all" (default: all)
#   PREVIEW          set to 0 to skip `api preview` (default: 1)
#   PREVIEW_FORMATS  comma-separated preview formats or "all"
#                    (default: all — swagger-ui, redoc, scalar, stoplight, rapidoc)
#   OUT              output directory (default: examples/output)
#   NAME             collection name override (default: spec info.title)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT/target/release/api"
SPEC="$ROOT/examples/petstore.yaml"
OUT="${OUT:-$ROOT/examples/output}"
FORMATS="${FORMATS:-all}"
PREVIEW="${PREVIEW:-1}"
PREVIEW_FORMATS="${PREVIEW_FORMATS:-all}"

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

if [[ "$PREVIEW" != "0" ]]; then
    pargs=(preview "$SPEC" --format "$PREVIEW_FORMATS" --out "$OUT")
    if [[ -n "${NAME:-}" ]]; then
        pargs+=(--name "$NAME")
    fi
    echo "previewing $SPEC -> $OUT (formats: $PREVIEW_FORMATS)"
    "$BIN" "${pargs[@]}"
fi

find "$OUT" -type f | sort | sed "s|$ROOT/||"
