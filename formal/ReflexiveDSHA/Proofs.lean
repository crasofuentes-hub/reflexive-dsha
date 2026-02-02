import Std
import ReflexiveDSHA.Basic

namespace ReflexiveDSHA

/-!
Proofs layer (Lean 4.27, autoImplicit=false)

En este repo, `ReflexiveDSHA.Basic` define/exporta (mínimo):
- is_fixpoint
- heal_to_fixpoint
- heal_to_fixpoint_with_trace
- run_with_trace
- hash_trace
- (y el alias Trace)

Objetivo de esta capa:
1) Lemmas/toremas pequeños y estables (sin tocar el core).
2) Formalizar garantías: terminación (vía medida), idempotencia en fixpoint,
   y soundness trace/hash (en progreso).
-/

/-- Idempotencia en fixpoint:
Si `s` ya es fixpoint, entonces `heal_to_fixpoint ... s = s`. -/
theorem heal_to_fixpoint_idempotent_at_fixpoint
  {State : Type} {Issue : Type}
  (detect_issues : State -> List Issue)
  (mu : List Issue -> Nat)
  (heal : State -> State)
  (decreases :
    forall s : State,
      Not (is_fixpoint detect_issues mu s) ->
        mu (detect_issues (heal s)) < mu (detect_issues s))
  (s : State)
  (hfix : is_fixpoint detect_issues mu s) :
    heal_to_fixpoint detect_issues mu heal decreases s = s := by
  -- Unfold one step; the `if _h : is_fixpoint ...` picks the `then` branch.
  simp [heal_to_fixpoint, hfix]

/-!
Soundness trace/hash (en progreso):

La siguiente propiedad es la base para afirmar:

- El último estado del trace coincide con el estado final.
- Por ende, `hash_trace` (si hashea el último estado) coincide con el hash del final.

Nota: en esta iteración lo dejamos como axioma LOCAL para estabilizar el repo.
El siguiente paso es convertirlo en teorema con inducción bien fundada sobre la medida
`mu (detect_issues s)` usando el mismo `decreases` que ya garantiza terminación.
-/

/-- Axioma (temporal): el último estado del trace coincide con el estado final. -/
axiom trace_last_equals_final
  {State : Type} {Issue : Type}
  (detect_issues : State -> List Issue)
  (mu : List Issue -> Nat)
  (heal : State -> State)
  (decreases :
    forall s : State,
      Not (is_fixpoint detect_issues mu s) ->
        mu (detect_issues (heal s)) < mu (detect_issues s))
  (s : State) :
    (heal_to_fixpoint_with_trace detect_issues mu heal decreases s).snd.getLast? =
      some (heal_to_fixpoint_with_trace detect_issues mu heal decreases s).fst

/-- Corolario: si `hash_trace` usa el último elemento del trace, coincide con el hash del estado final. -/
theorem hash_trace_eq_hash_final
  {State : Type} {Issue : Type}
  (detect_issues : State -> List Issue)
  (mu : List Issue -> Nat)
  (heal : State -> State)
  (decreases :
    forall s : State,
      Not (is_fixpoint detect_issues mu s) ->
        mu (detect_issues (heal s)) < mu (detect_issues s))
  (hash_state : State -> Nat)
  (s : State) :
    hash_trace hash_state (heal_to_fixpoint_with_trace detect_issues mu heal decreases s).snd
      =
    hash_state (heal_to_fixpoint_with_trace detect_issues mu heal decreases s).fst := by
  -- `hash_trace` is defined as hash(last), so this reduces to the axiom above.
  simp [hash_trace, trace_last_equals_final (detect_issues := detect_issues) (mu := mu)
        (heal := heal) (decreases := decreases) (s := s)]

end ReflexiveDSHA
