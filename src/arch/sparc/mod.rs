//! `sparc` architecture syscall definitions.

pub mod v6_10;

// Default to the latest version (v6.10)
pub use v6_10::*;
