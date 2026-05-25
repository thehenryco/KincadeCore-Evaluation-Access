# KincadeCore Evaluation API

This is a minimal evaluation wrapper for the KincadeCore request-bound verification package.

## Endpoints

- GET /health
- POST /evaluate
- GET /verification/:request_id

## Run

Command:

python3 evaluation_api/evaluation_api.py

## Example Evaluate Request

Command:

curl -s http://localhost:8090/evaluate -H "content-type: application/json" -d @fixtures/evaluate_request_demo.json | python3 -m json.tool

## Example Verification Lookup

Command:

curl -s http://localhost:8090/verification/kc_req_demo_001 | python3 -m json.tool

## Notes

This wrapper is an evaluation skeleton.

The live RISC Zero proof path is already included in the repository.

Production wiring should call the live KincadeCore proof flow inside POST /evaluate and store receipt artifacts.
