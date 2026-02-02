import Std

namespace ReflexiveDSHA

/-!
Basic layer (Lean 4.27, autoImplicit=false)

This file defines a terminating "heal until fixpoint" core, plus a trace API.

Key design choice (prevents the 4-hour mismatch loop):
- decreases is stated with Prop premise:
    decreases : forall s, Not (is_fixpoint ...) -> ...
  (NOT the boolean equality (!decide ...) = true)
- ASCII-only: use Not, forall, ->, etc.
-/

section

variable {State : Type} {Issue : Type}

/-- Fixpoint: the measure of detected issues is zero. -/
def is_fixpoint (detect_issues : State -> List Issue) (mu : List Issue -> Nat) (s : State) : Prop :=
  mu (detect_issues s) = 0

instance is_fixpoint_decidable
  (detect_issues : State -> List Issue) (mu : List Issue -> Nat) (s : State) :
  Decidable (is_fixpoint detect_issues mu s) := by
  unfold is_fixpoint
  simpa using (Nat.decEq (mu (detect_issues s)) 0)

/-- Trace of visited states. -/
abbrev Trace (State : Type) := List State

/-
heal_to_fixpoint: terminating recursion on Nat measure mu(detect_issues s)
-/

/-- Heal until reaching a fixpoint. -/
def heal_to_fixpoint
  (detect_issues : State -> List Issue)
  (mu : List Issue -> Nat)
  (heal : State -> State)
  (decreases :
    forall s : State,
      Not (is_fixpoint detect_issues mu s) ->
        mu (detect_issues (heal s)) < mu (detect_issues s))
  : State -> State
| s =>
    if _h : is_fixpoint detect_issues mu s then
      s
    else
      heal_to_fixpoint detect_issues mu heal decreases (heal s)
termination_by
  s => mu (detect_issues s)
decreasing_by
  -- Here h : Not (is_fixpoint ... s)
  simpa using (decreases (s := s) _h)

/-- Heal until fixpoint, returning (final_state, trace_of_states). -/
def heal_to_fixpoint_with_trace
  (detect_issues : State -> List Issue)
  (mu : List Issue -> Nat)
  (heal : State -> State)
  (decreases :
    forall s : State,
      Not (is_fixpoint detect_issues mu s) ->
        mu (detect_issues (heal s)) < mu (detect_issues s))
  : State -> Prod State (Trace State)
| s =>
    if _h : is_fixpoint detect_issues mu s then
      (s, [s])
    else
      let r := heal_to_fixpoint_with_trace detect_issues mu heal decreases (heal s)
      (r.1, s :: r.2)
termination_by
  s => mu (detect_issues s)
decreasing_by
  simpa using (decreases (s := s) _h)

/-- Convenience wrapper: run and keep trace. -/
def run_with_trace
  (detect_issues : State -> List Issue)
  (mu : List Issue -> Nat)
  (heal : State -> State)
  (decreases :
    forall s : State,
      Not (is_fixpoint detect_issues mu s) ->
        mu (detect_issues (heal s)) < mu (detect_issues s))
  (s : State) : Prod State (Trace State) :=
  heal_to_fixpoint_with_trace detect_issues mu heal decreases s

/-- Hash of a trace: hash(last state), or 0 if empty. -/
def hash_trace (hash_state : State -> Nat) (t : Trace State) : Nat :=
  match t.getLast? with
  | some st => hash_state st
  | none    => 0

end

end ReflexiveDSHA
