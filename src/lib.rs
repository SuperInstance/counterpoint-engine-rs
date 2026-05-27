//! # counterpoint-engine
//!
//! Species counterpoint engine — interval arithmetic, voice leading,
//! constraint-based composition, Laman graph rigidity, and holonomy harmony.
//!
//! Rust port of [counterpoint-engine](https://github.com/SuperInstance/counterpoint-engine).

pub mod interval;
pub mod exceptions;
pub mod rules;
pub mod voice_leading;
pub mod generator;
pub mod laman;
pub mod tensor_output;
pub mod harmony;
