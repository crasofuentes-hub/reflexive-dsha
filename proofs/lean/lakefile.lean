import Lake
open Lake DSL

/--
Reflexive.DSHA proofs (Lean-first).

This package is intentionally minimal:
- formal termination via a well-founded measure (mu : State -> Nat)
- fixpoint properties for the healing step

The Rust engine enforces mu to strictly decrease at runtime.
Here we prove the abstract model properties.
-/
package ReflexiveDSHAProofs where
  moreLeanArgs := #["-DwarningAsError=true"]

lean_lib ReflexiveDSHAProofs where
  roots := #[`ReflexiveDSHAProofs]