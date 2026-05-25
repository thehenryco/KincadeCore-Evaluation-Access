# Next Product Steps

The evaluation repo now demonstrates request-bound KincadeCore verification.

## Completed

- Live KincadeCore API readout fetched
- RISC Zero guest proof completed
- Receipt verified
- request_id committed
- decision committed
- payment_review demo fixture added
- request-bound journal artifact added

## Next Build Step

Build a small evaluation API wrapper with:

- POST /evaluate
- GET /verification/:request_id

## POST /evaluate Should

1. Accept a request payload.
2. Assign or validate request_id.
3. Call the KincadeCore solve/readout path.
4. Produce decision, risk_level, and reason_codes.
5. Run the RISC Zero proof.
6. Store or return seal, journal, and receipt metadata.

## GET /verification/:request_id Should Return

- request_id
- decision
- risk_level
- reason_codes
- seal
- proof_status
- public journal
- verifier instructions

## Production Hardening Later

- deployable KincadeCore endpoint
- stable artifact storage
- receipt export
- CI verification test
- Stripe PaymentIntent or session binding
- secret management
- customer-data redaction
