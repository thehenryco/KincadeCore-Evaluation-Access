#!/usr/bin/env bash
set -euo pipefail

: "${GARDEN_API_KEY:?GARDEN_API_KEY is required}"
: "${KINCADECORE_API_URL:=http://localhost:8088/v1/solve}"
: "${KINCADECORE_REQUEST_ID:=kc_req_demo_001}"
: "${KINCADECORE_PAYMENT_ID:=pi_demo_001}"
: "${KINCADECORE_SESSION_ID:=cs_demo_001}"
: "${KINCADECORE_EVENT_TYPE:=payment_review}"
: "${KINCADECORE_DECISION:=approve}"
: "${KINCADECORE_RISK_LEVEL:=low}"
: "${KINCADECORE_REASON_CODES:=verified_readout,stabilizer_converged,seal_committed}"

export KINCADECORE_API_URL
export KINCADECORE_REQUEST_ID
export KINCADECORE_PAYMENT_ID
export KINCADECORE_SESSION_ID
export KINCADECORE_EVENT_TYPE
export KINCADECORE_DECISION
export KINCADECORE_RISK_LEVEL
export KINCADECORE_REASON_CODES

mkdir -p verification_artifacts

echo "Running live KincadeCore RISC Zero verification..."
echo "KINCADECORE_API_URL=$KINCADECORE_API_URL"

RISC0_DEV_MODE=0 cargo run --release --bin host 2>&1 | tee verification_artifacts/latest_verify.log

grep -q "Fetched live KincadeCore readout." verification_artifacts/latest_verify.log
grep -q "Live KincadeCore RISC Zero proof verified." verification_artifacts/latest_verify.log
grep -q "request_id=" verification_artifacts/latest_verify.log
grep -q "payment_id=" verification_artifacts/latest_verify.log
grep -q "session_id=" verification_artifacts/latest_verify.log
grep -q "decision=" verification_artifacts/latest_verify.log
grep -q "risk_level=" verification_artifacts/latest_verify.log
grep -q "complete=true" verification_artifacts/latest_verify.log
grep -q "ok_count=36" verification_artifacts/latest_verify.log
grep -q "seal=" verification_artifacts/latest_verify.log
grep -q "exported_receipt=verification_artifacts/receipt.bin" verification_artifacts/latest_verify.log

echo "PASS: live KincadeCore RISC Zero verification completed."
