# KincadeCore Request-Bound Verification

Status: PASSED

This checkpoint binds a live KincadeCore RISC Zero proof to a specific evaluation request.

## Demo Request

The included fixture is:

- File: fixtures/payment_review_demo.json
- Request ID: kc_req_demo_001
- Event type: payment_review
- Decision: approve
- Risk level: low

## Verified Journal Fields

The RISC Zero journal commits:

- request_id
- event_type
- decision
- risk_level
- reason_codes
- KincadeCore readout counts
- stabilizer convergence fields
- live seal hash

## What This Means

The live KincadeCore API produced a readout, the host passed selected fields into the RISC Zero guest, the guest asserted invariants, and the host verified the receipt.

This creates a request-bound verification record.

## What This Does Not Yet Include

This evaluation fixture does not include real customer data, real Stripe PaymentIntent data, production credentials, or private payment records.

## Next Production Step

Replace the demo request with a real payment/session/request identifier from the production workflow, then store:

- request ID
- decision
- risk level
- reason codes
- seal
- receipt
- public journal
