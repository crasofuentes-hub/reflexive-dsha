#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

pub mod model;
pub mod dsl;
pub mod engine;
pub mod io;
pub mod hashing;

pub use model::{Issue, Severity, State};
pub use dsl::{Op, PatchProgram};
pub use engine::{heal_to_fixpoint, verify_trace, HealConfig, TraceEvent};
