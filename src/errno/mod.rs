//! Errno: minimal, no_std-friendly Linux error codes
//!
//! `Errno` wraps Linux error numbers and offers lightweight conversions from
//! raw syscall return values. It intentionally avoids depending on `std` by
//! default so it can be used in `no_std` environments.
//!
//! - Use `Errno::from_ret_u32`/`from_ret_u64` to convert raw returns into
//!   `Result` without panicking or allocating.
//! - With the `std` feature, `Errno` integrates with `std::io::Error` and can
//!   retrieve the thread-local errno via `Errno::last()`.
//! - For convenience, aliases such as `EWOULDBLOCK` map to canonical variants.
//!
//! Design intent
//! - Avoid conflating OS errors with richer I/O errors: conversion from
//!   `std::io::Error` is fallible (`from_io_error`) because not every I/O error
//!   is a plain errno.
//! - Keep formatting cheap: `Display`/`Debug` prefer static names and short
//!   messages when available.
//!
#[macro_use]
mod macros;

mod generated;

#[cfg(feature = "std")]
mod last;

use core::fmt;

pub use self::generated::Errno;

impl Errno {
    /// Operation would block. This is the same as [`Errno::EAGAIN`].
    pub const EWOULDBLOCK: Self = Self::EAGAIN;

    /// Same as [`Errno::EDEADLK`].
    pub const EDEADLOCK: Self = Self::EDEADLK;

    /// Creates a new `Errno`.
    pub fn new(num: i32) -> Self {
        Self(num)
    }

    /// Converts the `Errno` into a raw `i32`.
    pub fn into_raw(self) -> i32 {
        self.0
    }

    /// Returns true if the error code is valid (i.e., less than 4096).
    pub fn is_valid(&self) -> bool {
        self.0 < 4096
    }

    /// Converts a raw syscall return value to a result.
    ///
    /// > Please use [`Errno::from_ret_u32`] or [`Errno::from_ret_u64`].
    /// > Refactored [`Errno::from_ret`] to handle u32 and u64 explicitly, instead of using usize.
    #[inline(always)]
    #[deprecated = "It is recommended to explicitly use u32 or u64."]
    pub fn from_ret(value: usize) -> Result<usize, Errno> {
        if value > -4096isize as usize {
            // Truncation of the error value is guaranteed to never occur due to
            // the above check. This is the same check that musl uses:
            // https://git.musl-libc.org/cgit/musl/tree/src/internal/syscall_ret.c?h=v1.1.15
            Err(Self(-(value as i32)))
        } else {
            Ok(value)
        }
    }

    /// Rewriting of [`Errno::from_ret`] to use a u32 for pointer width. This function is for platforms where a pointer has a size of 32 bits.
    #[inline(always)]
    pub fn from_ret_u32(value: u32) -> Result<u32, Errno> {
        const THRESHOLD: u32 = u32::MAX - 4095; // == (u32)(-4096)
        if value > THRESHOLD {
            // Restore -ret to positive errno code (1..=4095).
            let code = (u32::MAX - value + 1) as i32;
            Err(Errno(code))
        } else {
            Ok(value)
        }
    }

    /// Rewriting of [`Errno::from_ret`] to use a u64 for register width. This function is for platforms where the syscall return register is 64 bits.
    #[inline(always)]
    pub fn from_ret_u64(value: u64) -> Result<u64, Errno> {
        const THRESHOLD: u64 = u64::MAX - 4095; // == (u64)(-4096)
        if value > THRESHOLD {
            // Restore -ret to positive errno code (1..=4095).
            let code = (u64::MAX - value + 1) as i32;
            Err(Errno(code))
        } else {
            Ok(value)
        }
    }
    /// Returns the last error that occurred.
    #[cfg(feature = "std")]
    pub fn last() -> Self {
        Self(unsafe { *last::errno() })
    }

    /// Converts a value into an `Errno`.
    #[cfg(feature = "std")]
    pub fn result<T>(value: T) -> Result<T, Errno>
    where
        T: ErrnoSentinel + PartialEq<T>,
    {
        if value == T::sentinel() {
            Err(Self::last())
        } else {
            Ok(value)
        }
    }

    /// Returns the name of the error. If the internal error code is unknown or
    /// invalid, `None` is returned.
    pub fn name(&self) -> Option<&'static str> {
        self.name_and_description().map(|x| x.0)
    }

    /// Returns the error description. If the internal error code is unknown or
    /// invalid, `None` is returned.
    pub fn description(&self) -> Option<&'static str> {
        self.name_and_description().map(|x| x.1)
    }

    /// Converts an `std::io::Error` into an `Errno` if possible. Since an error
    /// code is just one of the few possible error types that `std::io::Error`
    /// can represent, this will return `None` if the conversion is not possible.
    ///
    /// A `From<std::io::Error>` implementation is not provided because this
    /// conversion can fail. However, the reverse is possible, so that is
    /// provided as a `From` implementation.
    #[cfg(feature = "std")]
    pub fn from_io_error(err: std::io::Error) -> Option<Self> {
        err.raw_os_error().map(Self::new)
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.name_and_description() {
            Some((name, description)) => {
                write!(f, "{} {name} ({description})", -self.0)
            }
            None => {
                if self.is_valid() {
                    write!(f, "{}", -self.0)
                } else {
                    write!(f, "Invalid errno {:#x}", self.0)
                }
            }
        }
    }
}

impl fmt::Debug for Errno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "Errno({})", self.0),
        }
    }
}

#[cfg(feature = "std")]
impl From<Errno> for std::io::Error {
    fn from(err: Errno) -> Self {
        std::io::Error::from_raw_os_error(err.into_raw())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Errno {}

pub trait ErrnoSentinel: Sized {
    fn sentinel() -> Self;
}

impl ErrnoSentinel for isize {
    fn sentinel() -> Self {
        -1
    }
}

impl ErrnoSentinel for i32 {
    fn sentinel() -> Self {
        -1
    }
}

impl ErrnoSentinel for i64 {
    fn sentinel() -> Self {
        -1
    }
}

impl ErrnoSentinel for *mut core::ffi::c_void {
    fn sentinel() -> Self {
        -1isize as *mut core::ffi::c_void
    }
}

impl ErrnoSentinel for usize {
    fn sentinel() -> Self {
        usize::MAX
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Errno::ENOENT.name(), Some("ENOENT"));
        assert_eq!(
            Errno::ENOENT.description(),
            Some("No such file or directory")
        );
        #[cfg(feature = "std")]
        {
            assert_eq!(
                format!("{}", Errno::ENOENT),
                "-2 ENOENT (No such file or directory)"
            );
            assert_eq!(format!("{:?}", Errno::ENOENT), "ENOENT");
        }
    }

    #[allow(deprecated)]
    #[test]
    fn from_ret() {
        assert_eq!(Errno::from_ret(-2isize as usize), Err(Errno::ENOENT));
        assert_eq!(Errno::from_ret_u64(-2isize as u64), Err(Errno::ENOENT));
        assert_eq!(Errno::from_ret_u32(-2isize as u32), Err(Errno::ENOENT));
        assert_eq!(Errno::from_ret(2), Ok(2));
        assert_eq!(Errno::from_ret_u32(2), Ok(2));
        assert_eq!(Errno::from_ret_u64(2), Ok(2));
    }

    #[cfg(feature = "std")]
    #[test]
    fn io_error() {
        use std::io;

        assert_eq!(
            io::Error::from(Errno::ENOENT).kind(),
            io::ErrorKind::NotFound
        );

        assert_eq!(
            Errno::from_io_error(io::Error::from_raw_os_error(2)),
            Some(Errno::ENOENT)
        );

        assert_eq!(
            Errno::from_io_error(io::Error::new(io::ErrorKind::Other, "")),
            None
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn last_errno() {
        assert_eq!(
            Errno::result(unsafe {
                libc::open(
                    b"this_should_not_exist\0".as_ptr() as *const _,
                    libc::O_RDONLY,
                )
            }),
            Err(Errno::ENOENT)
        );
    }
}
