//! Plan: generational marksweep

pub(in crate::plan) mod global;
pub(in crate::plan) mod mutator;
pub(in crate::plan) mod gc_work;

pub use self::global::GenMarkSweep;

