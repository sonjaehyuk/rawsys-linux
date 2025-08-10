//! Architecture-specific raw syscall backends
//!
//! This module selects and re-exports the appropriate backend for the current
//! target architecture. Each backend provides a set of inline `asm!` shims that
//! implement the exact calling convention required by the kernel on that arch.
//!
//! - All exported functions are `unsafe`: you are entering the kernel directly.
//! - Backends are organized per-arch in `src/syscall/*.rs` and compiled
//!   conditionally with `cfg(target_arch=...)`.
//! - For ARM, `thumb-mode` is detected by `build.rs` and enabled via a config
//!   flag so that the correct instruction encoding is generated.
//!
//! Unless you are writing arch-specific code, prefer using the top-level
//! `syscall!`/`raw_syscall!` macros and `syscallN` wrappers re-exported by the
//! crate root; those pick the correct backend automatically.
//!
#![allow(clippy::doc_markdown, clippy::pedantic)]

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(all(
    target_arch = "arm",
    not(any(target_feature = "thumb-mode", feature = "thumb-mode"))
))]
mod arm;
#[cfg(all(
    target_arch = "arm",
    any(target_feature = "thumb-mode", feature = "thumb-mode")
))]
mod arm_thumb;
#[cfg(target_arch = "loongarch64")]
mod loongarch64;
#[cfg(target_arch = "mips")]
mod mips;
#[cfg(target_arch = "mips64")]
mod mips64;
#[cfg(target_arch = "powerpc")]
mod powerpc;
#[cfg(target_arch = "powerpc64")]
mod powerpc64;
#[cfg(target_arch = "riscv32")]
mod riscv32;
#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "s390x")]
mod s390x;
#[cfg(target_arch = "sparc")]
mod sparc;
#[cfg(target_arch = "sparc64")]
mod sparc64;
#[cfg(target_arch = "x86")]
mod x86;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(all(
    target_arch = "arm",
    not(any(target_feature = "thumb-mode", feature = "thumb-mode"))
))]
pub use arm::*;

#[cfg(all(
    target_arch = "arm",
    any(target_feature = "thumb-mode", feature = "thumb-mode")
))]
pub use arm_thumb::*;

#[cfg(target_arch = "loongarch64")]
pub use loongarch64::*;

#[cfg(target_arch = "mips")]
pub use mips::*;

#[cfg(target_arch = "mips64")]
pub use mips64::*;

#[cfg(target_arch = "powerpc")]
pub use powerpc::*;

#[cfg(target_arch = "powerpc64")]
pub use powerpc64::*;

#[cfg(target_arch = "riscv32")]
pub use riscv32::*;

#[cfg(target_arch = "riscv64")]
pub use riscv64::*;

#[cfg(target_arch = "s390x")]
pub use s390x::*;

#[cfg(target_arch = "sparc")]
pub use sparc::*;

#[cfg(target_arch = "sparc64")]
pub use sparc64::*;

#[cfg(target_arch = "x86")]
pub use x86::*;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
