/-
CORE TERMINATION PROOFS (Lean-first, minimal but real)

We model a deterministic self-healing loop as:
- State carries a finite, decidable list of "issues" to heal.
- mu(s) := number of issues (Nat)
- healStep removes exactly one issue if any exist; otherwise it is a fixpoint.

This matches the Rust core invariant:
  if issues exist, next_mu < current_mu  (strict decrease)
  else healStep(s) = s                  (fixpoint)

From this we prove:
1) healStep strictly decreases mu when mu > 0
2) applying healStep exactly mu(s) times reaches a fixpoint (mu = 0)
3) fixpoint is stable: healStep(s) = s when mu(s) = 0
-/

namespace ReflexiveDSHAProofs

structure State where
  issues : List Nat
deriving Repr, DecidableEq

def mu (s : State) : Nat :=
  s.issues.length

def healStep (s : State) : State :=
  match s.issues with
  | []      => s
  | _ :: tl => { issues := tl }

theorem healStep_fixpoint (s : State) (h : mu s = 0) : healStep s = s := by
  cases hs : s.issues with
  | nil =>
      simp [mu, healStep, hs] at *
  | cons hd tl =>
      simp [mu, hs] at h

theorem healStep_strict_decrease (s : State) (h : mu s > 0) : mu (healStep s) < mu s := by
  cases hs : s.issues with
  | nil =>
      simp [mu, hs] at h
  | cons hd tl =>
      simp [mu, healStep, hs]

def healN : Nat -> State -> State
  | 0,     s => s
  | n + 1, s => healN n (healStep s)

theorem mu_healN_le (n : Nat) (s : State) : mu (healN n s) ≤ mu s := by
  induction n generalizing s with
  | zero =>
      simp [healN]
  | succ n ih =>
      have : mu (healN n (healStep s)) ≤ mu (healStep s) := ih (s := healStep s)
      have h2 : mu (healStep s) ≤ mu s := by
        cases hs : s.issues with
        | nil =>
            simp [mu, healStep, hs]
        | cons hd tl =>
            simp [mu, healStep, hs]
      exact le_trans this h2

/--
Main termination theorem (bounded domain):
Applying healStep exactly mu(s) times reaches a fixpoint (mu = 0).
-/
theorem termination_by_mu (s : State) : mu (healN (mu s) s) = 0 := by
  cases hs : s.issues with
  | nil =>
      simp [mu, healN, healStep, hs]
  | cons hd tl =>
      have ih : mu (healN (List.length tl) { issues := tl }) = 0 := by
        simpa [mu] using (termination_by_mu { issues := tl })
      simp [mu, healN, healStep, hs] at *
      exact ih

/--
Fixpoint stability: once mu=0, healStep is idempotent.
-/
theorem stable_at_fixpoint (s : State) (h : mu s = 0) : healStep (healStep s) = healStep s := by
  have : healStep s = s := healStep_fixpoint s h
  simp [this]

end ReflexiveDSHAProofs