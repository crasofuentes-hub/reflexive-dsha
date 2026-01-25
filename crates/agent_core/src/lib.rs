#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

pub mod dsl;
pub mod engine;
pub mod hashing;
pub mod io;
pub mod model;

pub use dsl::{Op, PatchProgram};
pub use engine::{heal_to_fixpoint, verify_trace, HealConfig, TraceEvent};
pub use model::{Issue, Severity, State};
