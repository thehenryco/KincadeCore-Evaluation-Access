use methods::{KINCADECORE_GUEST_ELF, KINCADECORE_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

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

fn as_u32(v: &Value, path: &[&str]) -> u32 {
    let mut cur = v;
    for key in path {
        cur = cur.get(*key).unwrap_or_else(|| panic!("missing field: {}", path.join(".")));
    }
    cur.as_u64()
        .unwrap_or_else(|| panic!("field is not u64: {}", path.join("."))) as u32
}

fn as_bool(v: &Value, path: &[&str]) -> bool {
    let mut cur = v;
    for key in path {
        cur = cur.get(*key).unwrap_or_else(|| panic!("missing field: {}", path.join(".")));
    }
    cur.as_bool()
        .unwrap_or_else(|| panic!("field is not bool: {}", path.join(".")))
}

fn as_str(v: &Value, path: &[&str]) -> String {
    let mut cur = v;
    for key in path {
        cur = cur.get(*key).unwrap_or_else(|| panic!("missing field: {}", path.join(".")));
    }
    cur.as_str()
        .unwrap_or_else(|| panic!("field is not string: {}", path.join(".")))
        .to_string()
}

fn as_scaled_f64(v: &Value, path: &[&str]) -> u64 {
    let mut cur = v;
    for key in path {
        cur = cur.get(*key).unwrap_or_else(|| panic!("missing field: {}", path.join(".")));
    }
    let x = cur
        .as_f64()
        .unwrap_or_else(|| panic!("field is not f64: {}", path.join(".")));
    (x * 1_000_000_000_000_000.0).round() as u64
}

fn fetch_live_readout() -> KincadeCoreReadout {
    let api_key = env::var("GARDEN_API_KEY")
        .expect("GARDEN_API_KEY must be set in the shell running cargo");

    let url = env::var("KINCADECORE_API_URL")
        .unwrap_or_else(|_| "http://localhost:8088/v1/solve".to_string());

    let body = serde_json::json!({
        "prompt": "Run a sealed KincadeCore system readout for this RISC Zero proof task."
    });

    let response: Value = ureq::post(&url)
        .set("authorization", &format!("Bearer {}", api_key))
        .set("content-type", "application/json")
        .send_json(body)
        .expect("KincadeCore API request failed")
        .into_json()
        .expect("KincadeCore API returned invalid JSON");

    let answer = response
        .get("answer")
        .expect("missing answer object");

    let request_id = env::var("KINCADECORE_REQUEST_ID")
        .unwrap_or_else(|_| "kc_eval_2026_05_25_live_001".to_string());

    let decision = env::var("KINCADECORE_DECISION")
        .unwrap_or_else(|_| "verified_readout".to_string());

    KincadeCoreReadout {
        request_id,
        decision,
        summary: as_str(answer, &["summary"]),
        complete: as_bool(answer, &["complete"]),
        partial_call: as_bool(answer, &["partial_call"]),
        ok_count: as_u32(answer, &["counts", "OK"]),
        records_read: as_u32(answer, &["computing_summary", "records_read"]),
        atoms_found: as_u32(answer, &["computing_summary", "atoms_found"]),
        edges_created: as_u32(answer, &["computing_summary", "edges_created"]),
        cycles_found: as_u32(answer, &["computing_summary", "cycles_found"]),
        solver_outputs: as_u32(answer, &["computing_summary", "solver_outputs"]),
        physics_outputs: as_u32(answer, &["computing_summary", "physics_outputs"]),
        records_written: as_u32(answer, &["computing_summary", "records_written"]),
        stabilizer_converged: as_bool(answer, &["computing_summary", "stabilizer", "converged"]),
        stabilizer_converged_at: as_u32(answer, &["computing_summary", "stabilizer", "converged_at"]),
        stabilizer_k_scaled: as_scaled_f64(answer, &["computing_summary", "stabilizer", "K"]),
        stabilizer_contraction_rate_scaled: as_scaled_f64(
            answer,
            &["computing_summary", "stabilizer", "contraction_rate"],
        ),
        stabilizer_rate_match: as_bool(answer, &["computing_summary", "stabilizer", "rate_match"]),
        seal: as_str(answer, &["seal"]),
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let readout = fetch_live_readout();

    println!("Fetched live KincadeCore readout.");
    println!("live_request_id={}", readout.request_id);
    println!("live_decision={}", readout.decision);
    println!("live_complete={}", readout.complete);
    println!("live_ok_count={}", readout.ok_count);
    println!("live_seal={}", readout.seal);

    let env = ExecutorEnv::builder()
        .write(&readout)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, KINCADECORE_GUEST_ELF).unwrap();
    let receipt = prove_info.receipt;

    let journal: KincadeCoreJournal = receipt.journal.decode().unwrap();

    assert_eq!(journal.request_id, readout.request_id);
    assert_eq!(journal.decision, readout.decision);
    assert!(journal.complete);
    assert!(!journal.partial_call);
    assert_eq!(journal.ok_count, readout.ok_count);
    assert_eq!(journal.records_read, readout.records_read);
    assert_eq!(journal.atoms_found, readout.atoms_found);
    assert_eq!(journal.edges_created, readout.edges_created);
    assert_eq!(journal.cycles_found, readout.cycles_found);
    assert_eq!(journal.solver_outputs, readout.solver_outputs);
    assert_eq!(journal.physics_outputs, readout.physics_outputs);
    assert_eq!(journal.records_written, readout.records_written);
    assert_eq!(journal.stabilizer_converged, readout.stabilizer_converged);
    assert_eq!(journal.stabilizer_rate_match, readout.stabilizer_rate_match);
    assert_eq!(journal.seal, readout.seal);

    receipt.verify(KINCADECORE_GUEST_ID).unwrap();

    println!("Live KincadeCore RISC Zero proof verified.");
    println!("request_id={}", journal.request_id);
    println!("decision={}", journal.decision);
    println!("complete={}", journal.complete);
    println!("ok_count={}", journal.ok_count);
    println!("records_read={}", journal.records_read);
    println!("atoms_found={}", journal.atoms_found);
    println!("edges_created={}", journal.edges_created);
    println!("cycles_found={}", journal.cycles_found);
    println!("solver_outputs={}", journal.solver_outputs);
    println!("physics_outputs={}", journal.physics_outputs);
    println!("records_written={}", journal.records_written);
    println!("stabilizer_converged={}", journal.stabilizer_converged);
    println!("stabilizer_rate_match={}", journal.stabilizer_rate_match);
    println!("seal={}", journal.seal);
}
