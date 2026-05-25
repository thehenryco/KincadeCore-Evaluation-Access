#!/usr/bin/env bash
set -euo pipefail

echo "Stopping KincadeCore servers..."

for PORT in 8088 8090; do
  PID="$(ss -ltnp | awk -v port=":$PORT" '$0 ~ port {print $NF}' | sed -n 's/.*pid=\([0-9]*\).*/\1/p' | head -1 || true)"
  if [ -n "${PID:-}" ]; then
    echo "Killing process on port $PORT: PID=$PID"
    kill "$PID" 2>/dev/null || true
    sleep 1
  else
    echo "No process on port $PORT"
  fi
done

for PORT in 8088 8090; do
  PID="$(ss -ltnp | awk -v port=":$PORT" '$0 ~ port {print $NF}' | sed -n 's/.*pid=\([0-9]*\).*/\1/p' | head -1 || true)"
  if [ -n "${PID:-}" ]; then
    echo "Force killing process on port $PORT: PID=$PID"
    kill -9 "$PID" 2>/dev/null || true
    sleep 1
  fi
done

echo "Remaining listeners:"
ss -ltnp | grep -E ':8088|:8090' || true

echo "Done."
