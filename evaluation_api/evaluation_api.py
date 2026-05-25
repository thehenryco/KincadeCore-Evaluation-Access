#!/usr/bin/env python3
import json
import os
import re
import subprocess
import uuid
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

HOST = os.environ.get("EVALUATION_API_HOST", "0.0.0.0")
PORT = int(os.environ.get("EVALUATION_API_PORT", "8090"))

ARTIFACT_DIR = Path(os.environ.get("KINCADECORE_ARTIFACT_DIR", "verification_artifacts/records"))
ARTIFACT_DIR.mkdir(parents=True, exist_ok=True)

LATEST_LOG = Path("verification_artifacts/latest_verify.log")

ALLOWED_DECISIONS = {"approve", "review", "block", "verified_readout"}
ALLOWED_RISK_LEVELS = {"low", "medium", "high"}


def now_iso():
    return datetime.now(timezone.utc).isoformat()


def safe_id(value):
    allowed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-"
    cleaned = "".join(c for c in str(value) if c in allowed)
    return cleaned[:128] or f"kc_req_{uuid.uuid4().hex[:16]}"


def write_json(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, sort_keys=True)
        f.write("\n")


def load_json(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def parse_key_value_log(text):
    parsed = {}
    for raw in text.splitlines():
        line = raw.strip()
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        parsed[key.strip()] = value.strip()
    return parsed


def parse_bool(value):
    return str(value).lower() == "true"


def parse_int(value):
    return int(str(value).strip())


def run_live_verifier(request_id, decision):
    env = os.environ.copy()
    env["KINCADECORE_REQUEST_ID"] = request_id
    env["KINCADECORE_DECISION"] = decision

    if "GARDEN_API_KEY" not in env:
        raise RuntimeError("GARDEN_API_KEY is required to run live verification")

    env.setdefault("KINCADECORE_API_URL", "http://localhost:8088/v1/solve")

    result = subprocess.run(
        ["./verify_live_kincadecore.sh"],
        cwd=Path.cwd(),
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )

    LATEST_LOG.parent.mkdir(parents=True, exist_ok=True)
    LATEST_LOG.write_text(result.stdout, encoding="utf-8")

    if result.returncode != 0:
        raise RuntimeError("live verifier failed:\n" + result.stdout[-4000:])

    if "Live KincadeCore RISC Zero proof verified." not in result.stdout:
        raise RuntimeError("verifier did not report success")

    fields = parse_key_value_log(result.stdout)

    required = [
        "request_id",
        "decision",
        "complete",
        "ok_count",
        "records_read",
        "atoms_found",
        "edges_created",
        "cycles_found",
        "solver_outputs",
        "physics_outputs",
        "records_written",
        "stabilizer_converged",
        "stabilizer_rate_match",
        "seal",
    ]

    missing = [k for k in required if k not in fields]
    if missing:
        raise RuntimeError("verifier output missing fields: " + ", ".join(missing))

    journal = {
        "request_id": fields["request_id"],
        "decision": fields["decision"],
        "complete": parse_bool(fields["complete"]),
        "ok_count": parse_int(fields["ok_count"]),
        "records_read": parse_int(fields["records_read"]),
        "atoms_found": parse_int(fields["atoms_found"]),
        "edges_created": parse_int(fields["edges_created"]),
        "cycles_found": parse_int(fields["cycles_found"]),
        "solver_outputs": parse_int(fields["solver_outputs"]),
        "physics_outputs": parse_int(fields["physics_outputs"]),
        "records_written": parse_int(fields["records_written"]),
        "stabilizer_converged": parse_bool(fields["stabilizer_converged"]),
        "stabilizer_rate_match": parse_bool(fields["stabilizer_rate_match"]),
        "seal": fields["seal"],
    }

    return journal


def evaluate_request(payload):
    request_id = safe_id(payload.get("request_id") or f"kc_req_{uuid.uuid4().hex[:16]}")
    event_type = payload.get("event_type", "payment_review")

    decision = payload.get("decision", "approve")
    if decision not in ALLOWED_DECISIONS:
        decision = "review"

    risk_level = payload.get("risk_level", "low")
    if risk_level not in ALLOWED_RISK_LEVELS:
        risk_level = "medium"

    reason_codes = payload.get("reason_codes") or [
        "verified_readout",
        "stabilizer_converged",
        "seal_committed",
    ]

    journal = run_live_verifier(request_id=request_id, decision=decision)

    journal["event_type"] = event_type
    journal["risk_level"] = risk_level
    journal["reason_codes"] = reason_codes

    result = {
        "request_id": request_id,
        "event_type": event_type,
        "decision": decision,
        "risk_level": risk_level,
        "reason_codes": reason_codes,
        "proof_status": "verified",
        "seal": journal["seal"],
        "journal": journal,
        "created_at": now_iso(),
        "verification_url": f"/verification/{request_id}",
    }

    write_json(ARTIFACT_DIR / f"{request_id}.json", result)
    write_json(Path("verification_artifacts/live_journal.json"), journal)

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
                "mode": "live_proof_on_evaluate",
                "artifact_dir": str(ARTIFACT_DIR),
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
    print("Mode: live proof on POST /evaluate")
    HTTPServer((HOST, PORT), Handler).serve_forever()


if __name__ == "__main__":
    main()
