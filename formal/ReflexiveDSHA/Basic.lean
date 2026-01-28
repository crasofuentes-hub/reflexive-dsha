namespace ReflexiveDSHA

/-
  Minimal formal scaffold (Lean 4).
  Purpose: keep the project building while we incrementally formalize:
  - termination (well-founded measure)
  - fixpoint idempotence
  - trace/hash soundness
-/

section
  variable {State : Type} {Issue : Type}
  variable (detect_issues : State -> List Issue)
  variable (mu : List Issue -> Nat)

  def is_fixpoint (s : State) : Prop :=
    mu (detect_issues s) = 0

  instance is_fixpoint_decidable (s : State) :
      Decidable (is_fixpoint (detect_issues := detect_issues) (mu := mu) s) := by
    unfold is_fixpoint
    simpa using (Nat.decEq (mu (detect_issues s)) 0)
end
