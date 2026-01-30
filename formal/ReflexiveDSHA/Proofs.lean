import Std
import ReflexiveDSHA.Basic

namespace ReflexiveDSHA

/-
  Proofs layer (Lean 4.27, autoImplicit=false)

  Verified state:
  - ReflexiveDSHA.Basic exports only:
      * is_fixpoint
      * is_fixpoint_decidable
  - It does NOT define or export:
      * heal_to_fixpoint, heal_to_fixpoint_with_trace, run_with_trace, hash_trace

  Therefore, this Proofs layer must not reference those names.
-/

-- Si quieres, aquí van teoremas SOBRE lo que sí existe:

theorem is_fixpoint_iff_measure_zero
  {State : Type} {Issue : Type}
  (detect_issues : State -> List Issue)
  (mu : List Issue -> Nat)
  (s : State) :
  is_fixpoint (State := State) (Issue := Issue) (detect_issues := detect_issues) (mu := mu) s
    ↔ mu (detect_issues s) = 0 := by
  rfl

end ReflexiveDSHA
