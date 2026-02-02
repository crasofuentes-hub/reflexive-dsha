# reflexive-dsha

Deterministic self-healing agent core (Rust-first), offline, reproducible, trace-hashed, with formal-verification scaffolding.

## Quickstart (PowerShell)
\\\powershell
cargo build
cargo run -p agent_cli -- run-demo --in demo/inputs/config.txt --out out
cargo run -p agent_cli -- verify-trace --trace out/trace.json
Get-Content out/final.config
\\\

## Determinism Guarantees (Demo Scope)
- No RNG
- No floating point
- Canonical serialization (BTreeMap ordered keys)
- SHA-256 hash per step in trace

## Next: Formal proofs
- proofs/lean: DSL semantics + termination/soundness theorems
- proofs/tla: composability + deadlock-freedom model


## Estado de las pruebas formales (Lean)

Ubicacion: `formal/ReflexiveDSHA/*`

**Demostrado / compilando:**
- `is_fixpoint` definido como `mu (detect_issues s) = 0`.
- Terminacion de `heal_to_fixpoint` y `heal_to_fixpoint_with_trace` basada en una medida bien fundada `mu` con hipotesis `decreases` (compila en Lean).

**En progreso (prioridad):**
- Idempotencia en fixpoint: si `is_fixpoint ... s`, entonces `heal_to_fixpoint ... s = s` (en el API Lean actual).
- Soundness trace/hash: el hash derivado del ultimo estado del trace coincide con el hash del estado final, y es consistente con `verify-trace` (Rust).

**Pendiente (futuro cercano):**
- Vincular formalmente el modelo Lean con la implementacion Rust (modelo/axiomas vs extraccion).
- Invariantes adicionales: estabilidad del hash, propiedades del trace, etc.

> Nota: la capa Lean se mantiene minimalista y ASCII-only para maximizar portabilidad y estabilidad del build.
