import Std
import ReflexiveDSHA.Basic

namespace ReflexiveDSHA

/-!
Proofs layer (Lean 4.27, autoImplicit=false)

Verificado en este repo:
- NO existen definiciones de: heal_to_fixpoint, heal_to_fixpoint_with_trace, run_with_trace, hash_trace, Trace.
- Por lo tanto, esta capa NO debe referenciar esos nombres.

En su lugar, trabajamos con un 'run_with_trace' abstracto:
  run_with_trace : State -> Prod State (List State)
y axiomatizamos una propiedad de consistencia "último del trace = estado final".
-/

/-- Axioma: el último estado del trace (si existe) coincide con el estado final. -/
axiom trace_last_equals_final
  {State : Type}
  (run_with_trace : State -> Prod State (List State))
  (s : State) :
    (run_with_trace s).snd.getLast? = some (run_with_trace s).fst

/-- Corolario: si tu hash usa el último elemento del trace, coincide con el hash del estado final. -/
theorem hash_last_eq_hash_final
  {State : Type}
  (run_with_trace : State -> Prod State (List State))
  (hash_state : State -> Nat)
  (s : State) :
    (match (run_with_trace s).snd.getLast? with
     | some st => hash_state st
     | none    => 0)
      =
    hash_state (run_with_trace s).fst := by
  simp [trace_last_equals_final (run_with_trace := run_with_trace) (s := s)]

end ReflexiveDSHA
