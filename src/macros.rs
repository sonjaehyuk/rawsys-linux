//! Macros: ergonomic syscall invocation helpers
//!
//! - `syscall!`: returns `Result<SyscallWord, Errno>` and is suitable for
//!   general use.
//! - `raw_syscall!`: returns the raw machine word for cases where the call is
//!   guaranteed to succeed and you do not want `Errno` conversion.
//! - `syscall_args!`: builds a `SyscallArgs` value from up to 6 expressions.
//!
//! Safety
//! - All macros expand to `unsafe` calls because invoking a syscall is unsafe.
//!   You are responsible for pointer validity, buffer sizes, and respecting the
//!   kernel ABI for the selected architecture.
//!
//! Example
//! ```no_run
//! use rawsys_linux::{Sysno, syscall};
//!
//! match unsafe { syscall!(Sysno::getpid) } {
//!     Ok(pid) => println!("pid={pid}"),
//!     Err(err) => eprintln!("getpid failed: {err}"),
//! }
//! ```
/// Performs a syscall and returns a `Result<SyscallWord, Errno>`.
///
/// Accepts a syscall number and a variable number of arguments (0 to 6).
#[macro_export]
macro_rules! syscall {
    ($nr:expr) => {
        $crate::syscall0($nr)
    };

    ($nr:expr, $a1:expr) => {
        $crate::syscall1($nr, $a1 as $crate::SyscallWord)
    };

    ($nr:expr, $a1:expr, $a2:expr) => {
        $crate::syscall2(
            $nr,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::syscall3(
            $nr,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
            $a3 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::syscall4(
            $nr,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
            $a3 as $crate::SyscallWord,
            $a4 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::syscall5(
            $nr,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
            $a3 as $crate::SyscallWord,
            $a4 as $crate::SyscallWord,
            $a5 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        $crate::syscall6(
            $nr,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
            $a3 as $crate::SyscallWord,
            $a4 as $crate::SyscallWord,
            $a5 as $crate::SyscallWord,
            $a6 as $crate::SyscallWord,
        )
    };
}

/// Performs a raw syscall and returns a `SyscallWord`.
///
/// Prefer [`syscall!`] unless you are certain the syscall cannot fail (e.g.,
/// `gettid`).
///
/// Accepts a syscall number and a variable number of arguments (0 to 6).
///
/// # Example
/// ```no_run
/// use rawsys_linux::{Sysno, raw_syscall};
/// let tid = unsafe { raw_syscall!(Sysno::gettid) };
/// println!("tid={tid}");
/// ```
#[macro_export]
macro_rules! raw_syscall {
    ($nr:expr) => {
        $crate::raw::syscall0($nr as $crate::SyscallWord)
    };

    ($nr:expr, $a1:expr) => {
        $crate::raw::syscall1(
            $nr as $crate::SyscallWord,
            $a1 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr) => {
        $crate::raw::syscall2(
            $nr as $crate::SyscallWord,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::raw::syscall3(
            $nr as $crate::SyscallWord,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
            $a3 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::raw::syscall4(
            $nr as $crate::SyscallWord,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
            $a3 as $crate::SyscallWord,
            $a4 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::raw::syscall5(
            $nr as $crate::SyscallWord,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
            $a3 as $crate::SyscallWord,
            $a4 as $crate::SyscallWord,
            $a5 as $crate::SyscallWord,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        $crate::raw::syscall6(
            $nr as $crate::SyscallWord,
            $a1 as $crate::SyscallWord,
            $a2 as $crate::SyscallWord,
            $a3 as $crate::SyscallWord,
            $a4 as $crate::SyscallWord,
            $a5 as $crate::SyscallWord,
            $a6 as $crate::SyscallWord,
        )
    };
}
