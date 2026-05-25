use methods::KINCADECORE_GUEST_ID;
use risc0_zkvm::Receipt;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KincadeCoreJournal {
    request_id: String,
    decision: String,
    complete: bool,
    partial_call: bool,
    ok_count: u32,
    records_read: u32,
    atoms_found: u32,
    edges_created: u32,
    cycles_found: u32,
    solver_outputs: u32,
    physics_outputs: u32,
    records_written: u32,
    stabilizer_converged: bool,
    stabilizer_rate_match: bool,
    seal: String,
}

fn main() {
    let receipt_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "verification_artifacts/receipt.bin".to_string());

    let bytes = fs::read(&receipt_path).expect("failed to read receipt file");
    let receipt: Receipt = bincode::deserialize(&bytes).expect("failed to decode receipt");

    receipt
        .verify(KINCADECORE_GUEST_ID)
        .expect("receipt verification failed");

    let journal: KincadeCoreJournal = receipt
        .journal
        .decode()
        .expect("failed to decode receipt journal");

    println!("SAVED_RECEIPT_VERIFIED=true");
    println!("receipt_path={}", receipt_path);
    println!("request_id={}", journal.request_id);
    println!("decision={}", journal.decision);
    println!("complete={}", journal.complete);
    println!("ok_count={}", journal.ok_count);
    println!("seal={}", journal.seal);
}
