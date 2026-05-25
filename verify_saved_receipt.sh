#!/usr/bin/env bash
set -euo pipefail

RECEIPT_PATH="${1:-verification_artifacts/receipt.bin}"

cargo run --release --bin verify_saved_receipt -- "$RECEIPT_PATH"
