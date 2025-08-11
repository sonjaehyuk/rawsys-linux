//! `mips64` architecture syscall definitions.

pub mod v5_10;
pub mod v5_15;
pub mod v5_4;
pub mod v6_1;
pub mod v6_10;
pub mod v6_12;
pub mod v6_6;

// Select kernel version by feature; default to latest (v6.12).
#[cfg(all(not(docsrs), feature = "default_kernel_5_4"))]
pub use v5_4::*;
#[cfg(all(not(docsrs), feature = "default_kernel_5_10"))]
pub use v5_10::*;
#[cfg(all(not(docsrs), feature = "default_kernel_5_15"))]
pub use v5_15::*;
#[cfg(all(not(docsrs), feature = "default_kernel_6_1"))]
pub use v6_1::*;
#[cfg(all(not(docsrs), feature = "default_kernel_6_6"))]
pub use v6_6::*;
#[cfg(all(not(docsrs), feature = "default_kernel_6_10"))]
pub use v6_10::*;
#[cfg(all(not(docsrs), feature = "default_kernel_6_12"))]
pub use v6_12::*;

// Fallback if no default_kernel_* feature is chosen.
#[cfg(all(
    not(docsrs),
    not(any(
        feature = "default_kernel_5_4",
        feature = "default_kernel_5_10",
        feature = "default_kernel_5_15",
        feature = "default_kernel_6_1",
        feature = "default_kernel_6_6",
        feature = "default_kernel_6_10",
        feature = "default_kernel_6_12",
    ))
))]
pub use v6_12::*;

// On docs.rs, avoid enabling multiple versions; always show latest.
#[cfg(docsrs)]
pub use v6_12::*;
