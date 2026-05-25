use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KincadeCoreReadout {
    request_id: String,
    payment_id: String,
    session_id: String,
    event_type: String,
    decision: String,
    risk_level: String,
    reason_codes: Vec<String>,
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
    payment_id: String,
    session_id: String,
    event_type: String,
    decision: String,
    risk_level: String,
    reason_codes: Vec<String>,
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

fn assert_short_nonempty(label: &str, value: &str, max_len: usize) {
    assert!(!value.is_empty(), "{} is empty", label);
    assert!(value.len() <= max_len, "{} too long", label);
}

fn main() {
    let readout: KincadeCoreReadout = env::read();

    assert_short_nonempty("request_id", &readout.request_id, 128);
    assert_short_nonempty("payment_id", &readout.payment_id, 128);
    assert_short_nonempty("session_id", &readout.session_id, 128);
    assert_short_nonempty("event_type", &readout.event_type, 64);
    assert_short_nonempty("decision", &readout.decision, 64);
    assert_short_nonempty("risk_level", &readout.risk_level, 64);

    assert!(matches!(readout.decision.as_str(), "approve" | "review" | "block" | "verified_readout"));
    assert!(matches!(readout.risk_level.as_str(), "low" | "medium" | "high"));

    assert!(!readout.reason_codes.is_empty());
    assert!(readout.reason_codes.len() <= 16);
    for code in &readout.reason_codes {
        assert_short_nonempty("reason_code", code, 64);
    }

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
        payment_id: readout.payment_id,
        session_id: readout.session_id,
        event_type: readout.event_type,
        decision: readout.decision,
        risk_level: readout.risk_level,
        reason_codes: readout.reason_codes,
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
