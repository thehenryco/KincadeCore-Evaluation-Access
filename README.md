# KincadeCore Evaluation Access

This repository contains a non-confidential evaluation package for KincadeCore proof verification.

It includes a RISC Zero zkVM scaffold that demonstrates how a KincadeCore readout can be passed into a guest program, checked against expected invariants, committed into the public journal, and verified by the host.

## Current Checkpoint

- KincadeCore local API verified with `/health` returning `ok=true`
- `/v1/solve` returned `complete=true`
- RISC Zero project scaffold created and compiled
- Guest receives a KincadeCore readout structure
- Guest asserts selected invariants
- Guest commits a public journal
- Host verifies the receipt
- Real proving mode completed with `EXIT_CODE=0`

## Public Journal Example

```text
complete=true
ok_count=36
records_read=10
atoms_found=90
edges_created=72
cycles_found=10
solver_outputs=80
physics_outputs=10
records_written=90
stabilizer_converged=true
stabilizer_rate_match=true
seal=00aba5cb6bc00496c0482eb0fe7ee210caae937c1658c67a117d87c97cd14617
