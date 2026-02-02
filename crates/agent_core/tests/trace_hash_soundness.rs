use agent_core::{heal_to_fixpoint, HealConfig};

#[test]
fn trace_last_hash_matches_final_state_hash() {
    // Force healing: invalid mode + missing timeout; includes CRLF normalization.
    let raw = "mode=INVALID\r\na=1\r\n";

    let (final_state, trace) =
        heal_to_fixpoint(raw.to_string(), &HealConfig::default()).expect("heal_to_fixpoint");

    assert!(!trace.is_empty(), "trace must not be empty");

    let last = trace.last().expect("trace last");

    // Must be terminal fixpoint snapshot
    assert_eq!(last.mu, 0, "terminal trace event must have mu=0");

    let recomputed =
        agent_core::hashing::state_hash_sha256_from_parts(final_state.step, &final_state.canonical);

    assert_eq!(last.state_hash_sha256, recomputed);
}
