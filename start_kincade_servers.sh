#!/usr/bin/env bash
set -euo pipefail

export GARDEN_API_KEY="${GARDEN_API_KEY:-kincadecore-local-admin-test}"
export KINCADECORE_API_URL="${KINCADECORE_API_URL:-http://localhost:8088/v1/solve}"

GARDEN_DIR="/mnt/c/Users/jerod/eden/garden"
EVAL_DIR="/mnt/c/Users/jerod/eden/kincadecore-evaluation-access/KincadeCore-Evaluation-Access"

LOG_DIR="$EVAL_DIR/verification_artifacts/server_logs"
mkdir -p "$LOG_DIR"

echo "Starting KincadeCore API on 8088..."
cd "$GARDEN_DIR"
nohup python3 kincadecore_benchmark_api.py > "$LOG_DIR/kincadecore_8088.log" 2>&1 &
KC_PID=$!
echo "$KC_PID" > "$LOG_DIR/kincadecore_8088.pid"

sleep 2

echo "Checking KincadeCore API health..."
curl -s http://localhost:8088/health \
  -H "authorization: Bearer $GARDEN_API_KEY" \
  | python3 -m json.tool

echo "Starting Evaluation API on 8090..."
cd "$EVAL_DIR"
nohup env \
  GARDEN_API_KEY="$GARDEN_API_KEY" \
  KINCADECORE_API_URL="$KINCADECORE_API_URL" \
  python3 evaluation_api/evaluation_api.py > "$LOG_DIR/evaluation_8090.log" 2>&1 &
EVAL_PID=$!
echo "$EVAL_PID" > "$LOG_DIR/evaluation_8090.pid"

sleep 2

echo "Checking Evaluation API health..."
curl -s http://localhost:8090/health | python3 -m json.tool

echo "Servers started."
echo "KincadeCore API PID: $KC_PID"
echo "Evaluation API PID: $EVAL_PID"
echo "Logs:"
echo "$LOG_DIR/kincadecore_8088.log"
echo "$LOG_DIR/evaluation_8090.log"
