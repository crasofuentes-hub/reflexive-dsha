set_option autoImplicit false

namespace ReflexiveDSHA

/-- Modelo mínimo de estado. Ajusta campos cuando quieras. -/
structure State where
  raw : String
deriving DecidableEq, Repr

/-- Issues abstractos (puede ser Unit, o una estructura). -/
abbrev Issues := Nat

/-- Evento de traza mínimo: capturamos mu antes/después. -/
structure TraceEvent where
  mu_before : Nat
  mu_after  : Nat
deriving DecidableEq, Repr

/-- Funciones abstractas: en Proofs.lean se razona sobre propiedades. -/
variable (detect_issues : State → Issues)
variable (mu : Issues → Nat)

/-- Un paso de healing: produce estado siguiente + evento. -/
variable (step : State → State × TraceEvent)

/-- Ejecuta 
 pasos, siempre termina por recursión en Nat. -/
def heal (n : Nat) (s : State) : State × List TraceEvent :=
  match n with
  | 0     => (s, [])
  | n+1   =>
      let (s', e) := step s
      let (sf, tr) := heal (detect_issues:=detect_issues) (mu:=mu) (step:=step) n s'
      (sf, e :: tr)

/-- Fixpoint check: mu(issues(s)) == 0 -/
def is_fixpoint (s : State) : Prop :=
  mu (detect_issues s) = 0

/-- Ejecuta hasta fixpoint o hasta consumir fuel.
    Devuelve (estado_final, traza). -/
def heal_to_fixpoint (max_cycles : Nat) (s : State) : State × List TraceEvent :=
  match max_cycles with
  | 0 => (s, [])
  | n+1 =>
      if h : is_fixpoint (detect_issues:=detect_issues) (mu:=mu) s then
        (s, [])
      else
        let (s', e) := step s
        let (sf, tr) := heal_to_fixpoint (detect_issues:=detect_issues) (mu:=mu) (step:=step) n s'
        (sf, e :: tr)

end ReflexiveDSHA