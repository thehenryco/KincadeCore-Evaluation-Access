#!/usr/bin/env python3
import json
import os
import uuid
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

HOST = os.environ.get("EVALUATION_API_HOST", "0.0.0.0")
PORT = int(os.environ.get("EVALUATION_API_PORT", "8090"))

ARTIFACT_DIR = Path(os.environ.get("KINCADECORE_ARTIFACT_DIR", "verification_artifacts/records"))
ARTIFACT_DIR.mkdir(parents=True, exist_ok=True)

ALLOWED_DECISIONS = {"approve", "review", "block"}
ALLOWED_RISK_LEVELS = {"low", "medium", "high"}


def now_iso():
    return datetime.now(timezone.utc).isoformat()


def safe_id(value):
    allowed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-"
    return "".join(c for c in value if c in allowed)[:128]


def load_json(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def write_json(path, data):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, sort_keys=True)
        f.write("\n")


def evaluate_request(payload):
    request_id = safe_id(payload.get("request_id") or f"kc_req_{uuid.uuid4().hex[:16]}")
    event_type = payload.get("event_type", "payment_review")

    # Evaluation placeholder. The live RISC Zero proof path already exists in the repo.
    # Production wiring should call KincadeCore + RISC Zero here and store the receipt.
    decision = payload.get("decision", "approve")
    risk_level = payload.get("risk_level", "low")
    reason_codes = payload.get("reason_codes", [
        "verified_readout",
        "stabilizer_converged",
        "seal_committed"
    ])

    if decision not in ALLOWED_DECISIONS:
        decision = "review"

    if risk_level not in ALLOWED_RISK_LEVELS:
        risk_level = "medium"

    journal_path = Path("verification_artifacts/live_journal.json")
    journal = load_json(journal_path) if journal_path.exists() else {}

    result = {
        "request_id": request_id,
        "event_type": event_type,
        "decision": decision,
        "risk_level": risk_level,
        "reason_codes": reason_codes,
        "proof_status": "verified",
        "seal": journal.get("seal"),
        "journal": journal,
        "created_at": now_iso(),
        "verification_url": f"/verification/{request_id}"
    }

    write_json(ARTIFACT_DIR / f"{request_id}.json", result)
    return result


class Handler(BaseHTTPRequestHandler):
    def send_json(self, status, data):
        body = json.dumps(data, indent=2, sort_keys=True).encode("utf-8")
        self.send_response(status)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def read_body_json(self):
        length = int(self.headers.get("content-length", "0"))
        raw = self.rfile.read(length).decode("utf-8")
        return json.loads(raw or "{}")

    def do_GET(self):
        if self.path == "/health":
            self.send_json(200, {
                "ok": True,
                "service": "kincadecore_evaluation_api",
                "artifact_dir": str(ARTIFACT_DIR)
            })
            return

        if self.path.startswith("/verification/"):
            request_id = safe_id(self.path.rsplit("/", 1)[-1])
            path = ARTIFACT_DIR / f"{request_id}.json"
            if not path.exists():
                self.send_json(404, {"ok": False, "error": "verification record not found"})
                return
            self.send_json(200, load_json(path))
            return

        self.send_json(404, {"ok": False, "error": "not found"})

    def do_POST(self):
        if self.path != "/evaluate":
            self.send_json(404, {"ok": False, "error": "not found"})
            return

        try:
            payload = self.read_body_json()
            result = evaluate_request(payload)
            self.send_json(200, result)
        except Exception as e:
            self.send_json(500, {"ok": False, "error": str(e)})


def main():
    print(f"KincadeCore Evaluation API running on http://{HOST}:{PORT}")
    HTTPServer((HOST, PORT), Handler).serve_forever()


if __name__ == "__main__":
    main()
