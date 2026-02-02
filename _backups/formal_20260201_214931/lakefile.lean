import Lake
open Lake DSL

package reflexive_dsha where
  moreLeanArgs := #["-DautoImplicit=false"]

@[default_target]
lean_lib ReflexiveDSHA
