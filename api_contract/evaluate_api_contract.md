# KincadeCore Evaluate API Contract

Status: DRAFT FOR EVALUATION

This document defines the intended product input/output shape for KincadeCore request-bound verification.

## Endpoint

POST /evaluate

## Purpose

Submit a payment-style or high-trust request to KincadeCore and receive a decision package that can be bound to a RISC Zero proof journal.

## Request Body

JSON:
{
  "request_id": "kc_req_demo_001",
  "event_type": "payment_review",
  "amount": 1500,
  "currency": "usd",
  "merchant_id": "demo_merchant",
  "metadata": {
    "source": "evaluation_fixture"
  }
}

## Response Body

JSON:
{
  "request_id": "kc_req_demo_001",
  "event_type": "payment_review",
  "decision": "approve",
  "risk_level": "low",
  "reason_codes": [
    "verified_readout",
    "stabilizer_converged",
    "seal_committed"
  ],
  "seal": "1e51ffa299018075833838e1dbd961a5bfdef1d0ad1c4d15c5a71eebf93e8e77",
  "proof_status": "verified",
  "journal": {
    "complete": true,
    "ok_count": 36,
    "records_read": 10,
    "atoms_found": 90,
    "edges_created": 72,
    "cycles_found": 10,
    "solver_outputs": 80,
    "physics_outputs": 10,
    "records_written": 90,
    "stabilizer_converged": true,
    "stabilizer_rate_match": true
  }
}

## Decision Values

Allowed values:

- approve
- review
- block

## Risk Levels

Allowed values:

- low
- medium
- high

## What Must Be Bound Into The Proof Journal

- request_id
- event_type
- decision
- risk_level
- reason_codes
- complete
- ok_count
- readout counts
- stabilizer convergence fields
- seal

## What Must Not Be Committed Publicly

- API keys
- customer PII
- raw payment credentials
- Stripe secrets
- private solver internals
- internal database paths
