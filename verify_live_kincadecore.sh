#!/usr/bin/env bash
set -euo pipefail

: "${GARDEN_API_KEY:?GARDEN_API_KEY is required}"
: "${KINCADECORE_API_URL:=http://localhost:8088/v1/solve}"

mkdir -p verification_artifacts

echo "Running live KincadeCore RISC Zero verification..."
echo "KINCADECORE_API_URL=$KINCADECORE_API_URL"

RISC0_DEV_MODE=0 cargo run --release --bin host 2>&1 | tee verification_artifacts/latest_verify.log

grep -q "Fetched live KincadeCore readout." verification_artifacts/latest_verify.log
grep -q "Live KincadeCore RISC Zero proof verified." verification_artifacts/latest_verify.log
grep -q "complete=true" verification_artifacts/latest_verify.log
grep -q "ok_count=36" verification_artifacts/latest_verify.log
grep -q "seal=" verification_artifacts/latest_verify.log
grep -q "exported_receipt=verification_artifacts/receipt.bin" verification_artifacts/latest_verify.log

echo "PASS: live KincadeCore RISC Zero verification completed."
