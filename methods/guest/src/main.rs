use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KincadeCoreReadout {
    request_id: String,
    decision: String,
    summary: String,
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
    stabilizer_converged_at: u32,
    stabilizer_k_scaled: u64,
    stabilizer_contraction_rate_scaled: u64,
    stabilizer_rate_match: bool,
    seal: String,
}

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
    let readout: KincadeCoreReadout = env::read();

    assert!(!readout.request_id.is_empty());
    assert!(readout.request_id.len() <= 128);
    assert!(!readout.decision.is_empty());
    assert!(readout.decision.len() <= 64);
    assert!(readout.complete);
    assert!(!readout.partial_call);
    assert_eq!(readout.ok_count, 36);
    assert_eq!(readout.records_read, 10);
    assert_eq!(readout.atoms_found, 90);
    assert_eq!(readout.edges_created, 72);
    assert_eq!(readout.cycles_found, 10);
    assert_eq!(readout.solver_outputs, 80);
    assert_eq!(readout.physics_outputs, 10);
    assert_eq!(readout.records_written, 90);
    assert!(readout.stabilizer_converged);
    assert_eq!(readout.stabilizer_converged_at, 25);
    assert!(readout.stabilizer_rate_match);
    assert_eq!(readout.seal.len(), 64);
    assert!(readout.seal.chars().all(|c| c.is_ascii_hexdigit()));

    let journal = KincadeCoreJournal {
        request_id: readout.request_id,
        decision: readout.decision,
        complete: readout.complete,
        partial_call: readout.partial_call,
        ok_count: readout.ok_count,
        records_read: readout.records_read,
        atoms_found: readout.atoms_found,
        edges_created: readout.edges_created,
        cycles_found: readout.cycles_found,
        solver_outputs: readout.solver_outputs,
        physics_outputs: readout.physics_outputs,
        records_written: readout.records_written,
        stabilizer_converged: readout.stabilizer_converged,
        stabilizer_rate_match: readout.stabilizer_rate_match,
        seal: readout.seal,
    };

    env::commit(&journal);
}
