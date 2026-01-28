import ReflexiveDSHA.Basic

namespace ReflexiveDSHA

/-
  Proofs placeholder file.
  We keep it compiling and later replace these theorems with the real ones:
  - termination of heal_to_fixpoint via well-founded recursion/measure
  - fixpoint stability (idempotence)
  - trace/hash soundness
-/

section
  variable {State : Type} {Issue : Type}
  variable (detect_issues : State -> List Issue)
  variable (mu : List Issue -> Nat)

  -- Placeholder theorem: if you are at a fixpoint, you're at a fixpoint.
  theorem fixpoint_idempotent (s : State)
      (h : is_fixpoint (detect_issues := detect_issues) (mu := mu) s) :
      is_fixpoint (detect_issues := detect_issues) (mu := mu) s := by
    exact h
end

end ReflexiveDSHA
