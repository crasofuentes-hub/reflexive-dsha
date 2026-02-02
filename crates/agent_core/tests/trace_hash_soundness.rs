use agent_core::{heal_to_fixpoint, HealConfig};

#[test]
fn trace_last_hash_matches_final_state_hash() {
    // Forzamos healing: modo invalido + faltante de timeout
    let raw = "mode=INVALID\r\na=1\r\n";

    let (final_state, trace) =
        heal_to_fixpoint(raw.to_string(), &HealConfig::default()).expect("heal_to_fixpoint");

    assert!(!trace.is_empty(), "trace must not be empty");

    let last = trace.last().expect("trace last");
    let trace_hash = last.state_hash_sha256.clone();

    let recomputed =
        agent_core::hashing::state_hash_sha256_from_parts(final_state.step, &final_state.canonical);

    assert_eq!(trace_hash, recomputed);
}
