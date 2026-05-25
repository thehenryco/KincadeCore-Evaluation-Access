# KincadeCore Evaluation API

This is a minimal evaluation wrapper for the KincadeCore request-bound verification package.

## Endpoints

- GET /health
- POST /evaluate
- GET /verification/:request_id

## Current Mode

POST /evaluate now runs the live verifier.

Flow:

request payload
-> KincadeCore live readout
-> RISC Zero proof
-> verified journal
-> stored verification record
-> response returned to caller

## Required Environment

GARDEN_API_KEY must be set.

Optional:

KINCADECORE_API_URL defaults to http://localhost:8088/v1/solve
EVALUATION_API_PORT defaults to 8090

## Run

Command:

python3 evaluation_api/evaluation_api.py

## Example Evaluate Request

Command:

curl -s http://localhost:8090/evaluate -H "content-type: application/json" -d @fixtures/evaluate_request_demo.json | python3 -m json.tool

## Example Verification Lookup

Command:

curl -s http://localhost:8090/verification/kc_req_demo_001 | python3 -m json.tool

## What Is Verified

The live RISC Zero proof path verifies that the guest received and committed the request-bound KincadeCore readout fields.

The response includes:

- request_id
- event_type
- decision
- risk_level
- reason_codes
- proof_status
- seal
- public journal
- verification_url

## What Is Still Evaluation Only

This wrapper is still evaluation infrastructure. It does not yet include production auth, deployed storage, customer PII handling, Stripe webhook handling, or receipt export hosting.
