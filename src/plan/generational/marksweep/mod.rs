//! Plan: generational marksweep

pub(in crate::plan) mod global;
pub(in crate::plan) mod mutator;

pub use self::global::GenMarkSweep;

