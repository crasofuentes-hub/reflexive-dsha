use std::fs;
use agent_core::{heal_to_fixpoint, HealConfig};

#[test]
fn corpus_inputs_do_not_crash() {
    let dir = "corpus";
    if !std::path::Path::new(dir).exists() { return; }
    for e in fs::read_dir(dir).expect("read corpus") {
        let p = e.unwrap().path();
        if p.is_file() {
            let raw = fs::read_to_string(&p).expect("read corpus file");
            let _ = heal_to_fixpoint(raw, &HealConfig::default());
        }
    }
}
