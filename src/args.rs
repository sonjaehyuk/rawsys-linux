//! `SyscallArgs`: lightweight packing of syscall parameters
//!
//! This module provides the plain container `SyscallArgs` and a helper macro
//! `syscall_args!` for collecting up to six raw syscall arguments. It is useful
//! when you want to prepare arguments programmatically and call the generic
//! `syscall(nr, &SyscallArgs)` wrapper.
//!
//! Notes
//! - Kept intentionally untyped: the kernel ABI is in terms of machine words;
//!   `SyscallArgs` mirrors that to avoid accidental conversions or allocations.
//! - `no_std` friendly by design; no dependency on `std::io::Error`.
//!
//! Example
//! ```no_run
//! use rawsys_linux::{Sysno, SyscallArgs, syscall};
//!
//! let args = SyscallArgs::from(&[1, b"hi\n".as_ptr() as _, 3]);
//! let _ = unsafe { syscall(Sysno::write, &args) };
//! ```

use crate::SyscallWord;

/// The 6 arguments of a syscall, raw untyped version.
#[derive(PartialEq, Debug, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyscallArgs {
    pub arg0: SyscallWord,
    pub arg1: SyscallWord,
    pub arg2: SyscallWord,
    pub arg3: SyscallWord,
    pub arg4: SyscallWord,
    pub arg5: SyscallWord,
}

impl SyscallArgs {
    pub fn new(
        a0: SyscallWord,
        a1: SyscallWord,
        a2: SyscallWord,
        a3: SyscallWord,
        a4: SyscallWord,
        a5: SyscallWord,
    ) -> Self {
        SyscallArgs {
            arg0: a0,
            arg1: a1,
            arg2: a2,
            arg3: a3,
            arg4: a4,
            arg5: a5,
        }
    }
}

impl From<&[SyscallWord; 6]> for SyscallArgs {
    fn from(args: &[SyscallWord; 6]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: args[3],
            arg4: args[4],
            arg5: args[5],
        }
    }
}

impl From<&[SyscallWord; 5]> for SyscallArgs {
    fn from(args: &[SyscallWord; 5]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: args[3],
            arg4: args[4],
            arg5: 0,
        }
    }
}

impl From<&[SyscallWord; 4]> for SyscallArgs {
    fn from(args: &[SyscallWord; 4]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: args[3],
            arg4: 0,
            arg5: 0,
        }
    }
}

impl From<&[SyscallWord; 3]> for SyscallArgs {
    fn from(args: &[SyscallWord; 3]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: 0,
            arg4: 0,
            arg5: 0,
        }
    }
}

impl From<&[SyscallWord; 2]> for SyscallArgs {
    fn from(args: &[SyscallWord; 2]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: args[1],
            arg2: 0,
            arg3: 0,
            arg4: 0,
            arg5: 0,
        }
    }
}

impl From<&[SyscallWord; 1]> for SyscallArgs {
    fn from(args: &[SyscallWord; 1]) -> Self {
        SyscallArgs {
            arg0: args[0],
            arg1: 0,
            arg2: 0,
            arg3: 0,
            arg4: 0,
            arg5: 0,
        }
    }
}

impl From<&[SyscallWord; 0]> for SyscallArgs {
    fn from(_args: &[SyscallWord; 0]) -> Self {
        SyscallArgs {
            arg0: 0,
            arg1: 0,
            arg2: 0,
            arg3: 0,
            arg4: 0,
            arg5: 0,
        }
    }
}

#[macro_export]
macro_rules! syscall_args {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        $crate::SyscallArgs::new($a, $b, $c, $d, $e, $f)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        $crate::SyscallArgs::new($a, $b, $c, $d, $e, 0)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::SyscallArgs::new($a, $b, $c, $d, 0, 0)
    };
    ($a:expr, $b:expr, $c:expr) => {
        $crate::SyscallArgs::new($a, $b, $c, 0, 0, 0)
    };
    ($a:expr, $b:expr) => {
        $crate::SyscallArgs::new($a, $b, 0, 0, 0, 0)
    };
    ($a:expr) => {
        $crate::SyscallArgs::new($a, 0, 0, 0, 0, 0)
    };
    () => {
        $crate::SyscallArgs::new(0, 0, 0, 0, 0, 0)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syscall_args_macro_test() {
        assert_eq!(
            syscall_args!(1, 2, 3, 4, 5, 6),
            SyscallArgs::new(1, 2, 3, 4, 5, 6)
        );
        assert_eq!(
            syscall_args!(1, 2, 3, 4, 5),
            SyscallArgs::new(1, 2, 3, 4, 5, 0)
        );
        assert_eq!(
            syscall_args!(1, 2, 3, 4),
            SyscallArgs::new(1, 2, 3, 4, 0, 0)
        );
        assert_eq!(syscall_args!(1, 2, 3), SyscallArgs::new(1, 2, 3, 0, 0, 0));
        assert_eq!(syscall_args!(1, 2), SyscallArgs::new(1, 2, 0, 0, 0, 0));
        assert_eq!(syscall_args!(1), SyscallArgs::new(1, 0, 0, 0, 0, 0));
        assert_eq!(syscall_args!(), SyscallArgs::new(0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn syscall_args_from_u64_slice() {
        assert_eq!(
            SyscallArgs::from(&[1, 2, 3, 4, 5, 6]),
            syscall_args!(1, 2, 3, 4, 5, 6)
        );
        assert_eq!(
            SyscallArgs::from(&[1, 2, 3, 4, 5]),
            syscall_args!(1, 2, 3, 4, 5)
        );
        assert_eq!(SyscallArgs::from(&[1, 2, 3, 4]), syscall_args!(1, 2, 3, 4));
        assert_eq!(SyscallArgs::from(&[1, 2, 3]), syscall_args!(1, 2, 3));
        assert_eq!(SyscallArgs::from(&[1, 2]), syscall_args!(1, 2));
        assert_eq!(SyscallArgs::from(&[1]), syscall_args!(1));
        assert_eq!(SyscallArgs::from(&[0]), syscall_args!());
    }
}
