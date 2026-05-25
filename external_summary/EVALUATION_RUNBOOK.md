# KincadeCore Evaluation Runbook

## Start Servers

From WSL:

./kill_kincade_servers.sh
./start_kincade_servers.sh

## Check Health

curl -s http://localhost:8088/health -H "authorization: Bearer $GARDEN_API_KEY" | python3 -m json.tool

curl -s http://localhost:8090/health | python3 -m json.tool

## Run Evaluation

curl -s http://localhost:8090/evaluate \
  -H "content-type: application/json" \
  -d @fixtures/evaluate_request_demo.json \
  | python3 -m json.tool

## Fetch Verification Record

curl -s http://localhost:8090/verification/kc_req_demo_001 | python3 -m json.tool

## Verify Saved Receipt

./verify_saved_receipt.sh

Expected output includes:

SAVED_RECEIPT_VERIFIED=true
request_id=kc_req_demo_001
decision=approve
complete=true
ok_count=36
seal=<live seal>

## Important Files

- fixtures/evaluate_request_demo.json
- fixtures/evaluate_response_demo.json
- verification_artifacts/live_journal.json
- verification_artifacts/journal.json
- verification_artifacts/image_id.json
- verification_artifacts/records/kc_req_demo_001.json
- verify_live_kincadecore.sh
- verify_saved_receipt.sh
- evaluation_api/evaluation_api.py

## Local-Only Files

The following are intentionally not committed:

- verification_artifacts/receipt.bin
- verification_artifacts/server_logs/
- verification_artifacts/latest_verify.log
- API keys
- .env files
