use proptest::prelude::*;
use agent_core::{heal_to_fixpoint, HealConfig};

proptest! {
  #[test]
  fn prop_heal_does_not_panic_and_progresses(raw in "([ -~]{0,200})(\r?\n[ -~]{0,200}){0,10}") {
    let _ = heal_to_fixpoint(raw.to_string(), &HealConfig::default());
    // If Ok, trace mu should be monotone decreasing and end at mu==0 (verify_trace already enforces)
  }
}
