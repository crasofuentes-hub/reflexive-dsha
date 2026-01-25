use agent_core::engine::{heal_to_fixpoint, verify_trace, HealConfig};

#[test]
fn demo_is_bit_stable() {
    let input = r#"
# demo input
timeout=999
mode=unsafe
timeout=10
bad key=abc
"#
    .trim()
    .to_string();

    let (st, trace) = heal_to_fixpoint(input, HealConfig { max_cycles: 16 }).expect("heal");
    verify_trace(&trace).expect("trace verify");

    let final_cfg = st.raw_input.trim().to_string();
    assert_eq!(final_cfg, "mode=safe\ntimeout=600");
}
