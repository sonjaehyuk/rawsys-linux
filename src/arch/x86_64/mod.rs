//! `x86_64` architecture syscall definitions.

pub mod v5_10;
pub mod v5_15;
pub mod v5_4;
pub mod v6_1;
pub mod v6_10;
pub mod v6_12;
pub mod v6_6;

// Select kernel version by feature; default to latest (v6.12).
#[cfg(all(not(docsrs), feature = "kernel_5_4"))]
pub use v5_4::*;
#[cfg(all(not(docsrs), feature = "kernel_5_10"))]
pub use v5_10::*;
#[cfg(all(not(docsrs), feature = "kernel_5_15"))]
pub use v5_15::*;
#[cfg(all(not(docsrs), feature = "kernel_6_1"))]
pub use v6_1::*;
#[cfg(all(not(docsrs), feature = "kernel_6_6"))]
pub use v6_6::*;
#[cfg(all(not(docsrs), feature = "kernel_6_10"))]
pub use v6_10::*;
#[cfg(all(not(docsrs), feature = "kernel_6_12"))]
pub use v6_12::*;

// Fallback if no kernel_* feature is chosen.
#[cfg(all(
    not(docsrs),
    not(any(
        feature = "kernel_5_4",
        feature = "kernel_5_10",
        feature = "kernel_5_15",
        feature = "kernel_6_1",
        feature = "kernel_6_6",
        feature = "kernel_6_10",
        feature = "kernel_6_12",
    ))
))]
pub use v6_12::*;

// On docs.rs, avoid enabling multiple versions; always show latest.
#[cfg(docsrs)]
pub use v6_12::*;
