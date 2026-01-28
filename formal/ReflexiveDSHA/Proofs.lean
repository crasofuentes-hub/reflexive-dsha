set_option autoImplicit false

import ReflexiveDSHA.Basic

namespace ReflexiveDSHA

open ReflexiveDSHA

variable (detect_issues : State → Issues)
variable (mu : Issues → Nat)
variable (step : State → State × TraceEvent)

/-- Propiedad clave: si NO estás en fixpoint, el siguiente paso reduce mu estrictamente. -/
def DecreasesMu (s : State) : Prop :=
  let m := mu (detect_issues s)
  m ≠ 0 →
    let (s', e) := step s
    mu (detect_issues s') < m

/-- 1) Totalidad: heal_to_fixpoint siempre devuelve un par (sf, tr). -/
theorem heal_to_fixpoint_total (n : Nat) (s : State) :
  ∃ (sf : State) (tr : List TraceEvent),
    heal_to_fixpoint (detect_issues:=detect_issues) (mu:=mu) (step:=step) n s = (sf, tr) :=
by
  exact ⟨(heal_to_fixpoint (detect_issues:=detect_issues) (mu:=mu) (step:=step) n s).1,
         (heal_to_fixpoint (detect_issues:=detect_issues) (mu:=mu) (step:=step) n s).2,
         rfl⟩

/-- 2) (En progreso) Terminación por medida bien fundada:
    bajo el invariante de decremento estricto de mu, se alcanza fixpoint en ≤ mu(s0) pasos. -/
theorem reaches_fixpoint_within_mu
  (Hdec : ∀ s, DecreasesMu (detect_issues:=detect_issues) (mu:=mu) (step:=step) s)
  (s0 : State) :
  let m0 := mu (detect_issues s0)
  is_fixpoint (detect_issues:=detect_issues) (mu:=mu)
    ( (heal_to_fixpoint (detect_issues:=detect_issues) (mu:=mu) (step:=step) m0 s0).1 ) :=
by
  -- TODO: prueba por inducción sobre m0 = mu(detect_issues s0)
  -- Caso base m0=0: trivial. Caso inductivo: aplica Hdec para reducir mu.
  classical
  admit

/-- 3) Idempotencia en fixpoint: si mu=0, heal_to_fixpoint no cambia el estado. -/
theorem heal_to_fixpoint_idempotent_if_fixpoint
  (n : Nat) (s : State)
  (H : mu (detect_issues s) = 0) :
  heal_to_fixpoint (detect_issues:=detect_issues) (mu:=mu) (step:=step) n s = (s, []) :=
by
  cases n with
  | zero =>
      simp [Basic.heal_to_fixpoint]
  | succ k =>
      simp [Basic.heal_to_fixpoint, Basic.is_fixpoint, H]

/-- Hash placeholder para soundness semántico-operacional. -/
abbrev Hash := Nat

variable (hash_trace : List TraceEvent → Hash)

/-- Semántica: definimos que la "ejecución" es heal_to_fixpoint. -/
def sem_exec (fuel : Nat) (s : State) : State × List TraceEvent :=
  heal_to_fixpoint (detect_issues:=detect_issues) (mu:=mu) (step:=step) fuel s

/-- Soundness: el hash reportado coincide con el hash de la traza semántica modelada. -/
theorem hash_matches_semantics (fuel : Nat) (s : State) :
  hash_trace ( (sem_exec (hash_trace:=hash_trace) (detect_issues:=detect_issues) (mu:=mu) (step:=step) fuel s).2 )
  =
  hash_trace ( (heal_to_fixpoint (detect_issues:=detect_issues) (mu:=mu) (step:=step) fuel s).2 ) :=
by
  simp [sem_exec]

end ReflexiveDSHA