#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::inline_always,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::ptr_as_ptr,
    clippy::unsafe_derive_deserialize
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    // These architectures require nightly to use inline assembly.
    // See https://github.com/rust-lang/rust/issues/93335
    any(
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "s390x",
        target_arch = "powerpc",
        target_arch = "powerpc64",
    ),
    feature(asm_experimental_arch)
)]

#[macro_use]
mod macros;

mod arch;
mod args;
mod errno;
mod map;
mod set;
mod syscall;

pub use arch::*;
pub use args::SyscallArgs;
pub use errno::{Errno, ErrnoSentinel};
pub use map::*;
pub use set::*;
pub use syscall::SyscallWord;

pub mod raw {
    //! Exposes raw syscalls that simply return a `SyscallWord` instead of a `Result`.

    pub use super::syscall::syscall0;
    pub use super::syscall::syscall1;
    pub use super::syscall::syscall2;
    pub use super::syscall::syscall3;
    pub use super::syscall::syscall4;
    pub use super::syscall::syscall5;
    pub use super::syscall::syscall6;
}

// NOTE on x86_64 x32 ABI
// -----------------------
// Some targets use 32-bit pointers but still return syscall results in a
// 64-bit register width. In particular, the x86_64 x32 ABI has
// `target_arch = "x86_64"` with `target_pointer_width = "32"`, but the
// syscall return value is still delivered in a 64-bit register (RAX), and the
// kernel reports errors by returning negative values truncated to the machine
// word size. If we naïvely keyed off pointer width alone (treating all 32-bit
// targets as returning a 32-bit value), negative return codes on x32 would be
// misinterpreted and error conversion (Errno) would be wrong.
//
// To handle this correctly, we special-case x86_64 + 32-bit pointer-width to
// convert using Errno::from_ret_u64, while other 32-bit targets continue to
// use Errno::from_ret_u32 and 64-bit targets use Errno::from_ret_u64.
//
// Test status: we have not run CI on an actual x32 target here. The logic is
// based on the ABI specification and should be correct, but x32-specific
// testing remains outstanding.

/// Issues a system call with 0 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall0(nr: Sysno) -> Result<SyscallWord, Errno> {
    let ret = unsafe { raw::syscall0(nr as SyscallWord) };

    // x86_64 x32 ABI: 32-bit pointers with 64-bit syscall return width.
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "64"
    ))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "32"
    ))]
    return Errno::from_ret_u32(ret as u32);
}

/// Issues a system call with 1 argument.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall1(
    nr: Sysno,
    a1: SyscallWord,
) -> Result<SyscallWord, Errno> {
    let ret = unsafe { raw::syscall1(nr as SyscallWord, a1) };

    // x86_64 x32 ABI
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "64"
    ))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "32"
    ))]
    return Errno::from_ret_u32(ret as u32);
}

/// Issues a system call with 2 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall2(
    nr: Sysno,
    a1: SyscallWord,
    a2: SyscallWord,
) -> Result<SyscallWord, Errno> {
    let ret = unsafe { raw::syscall2(nr as SyscallWord, a1, a2) };

    // x86_64 x32 ABI
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "64"
    ))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "32"
    ))]
    return Errno::from_ret_u32(ret as u32);
}

/// Issues a system call with 3 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall3(
    nr: Sysno,
    a1: SyscallWord,
    a2: SyscallWord,
    a3: SyscallWord,
) -> Result<SyscallWord, Errno> {
    let ret = unsafe { raw::syscall3(nr as SyscallWord, a1, a2, a3) };

    // x86_64 x32 ABI
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "64"
    ))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "32"
    ))]
    return Errno::from_ret_u32(ret as u32);
}

/// Issues a system call with 4 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall4(
    nr: Sysno,
    a1: SyscallWord,
    a2: SyscallWord,
    a3: SyscallWord,
    a4: SyscallWord,
) -> Result<SyscallWord, Errno> {
    let ret = unsafe { raw::syscall4(nr as SyscallWord, a1, a2, a3, a4) };

    // x86_64 x32 ABI
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "64"
    ))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "32"
    ))]
    return Errno::from_ret_u32(ret as u32);
}

/// Issues a system call with 5 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall5(
    nr: Sysno,
    a1: SyscallWord,
    a2: SyscallWord,
    a3: SyscallWord,
    a4: SyscallWord,
    a5: SyscallWord,
) -> Result<SyscallWord, Errno> {
    let ret = unsafe { raw::syscall5(nr as SyscallWord, a1, a2, a3, a4, a5) };

    // x86_64 x32 ABI
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "64"
    ))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "32"
    ))]
    return Errno::from_ret_u32(ret as u32);
}

/// Issues a system call with 6 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall6(
    nr: Sysno,
    a1: SyscallWord,
    a2: SyscallWord,
    a3: SyscallWord,
    a4: SyscallWord,
    a5: SyscallWord,
    a6: SyscallWord,
) -> Result<SyscallWord, Errno> {
    let ret =
        unsafe { raw::syscall6(nr as SyscallWord, a1, a2, a3, a4, a5, a6) };

    // x86_64 x32 ABI
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "64"
    ))]
    return Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "32"
    ))]
    return Errno::from_ret_u32(ret as u32);
}

/// Does a raw syscall.
///
/// # Arguments
///  - `nr`: The syscall number.
///  - `args`: packed arguments
///
/// # Returns
///  - `Ok` on success,
///  - `Err` when the syscall failed (with errno).
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
pub unsafe fn syscall(
    nr: Sysno,
    args: &SyscallArgs,
) -> Result<SyscallWord, Errno> {
    unsafe {
        syscall6(
            nr, args.arg0, args.arg1, args.arg2, args.arg3, args.arg4,
            args.arg5,
        )
    }
}
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall1_syscall4() {
        let fd = unsafe {
            let at_fdcwd = -100isize;
            syscall!(Sysno::openat, at_fdcwd, "/dev/zero\0".as_ptr(), 0)
        }
        .unwrap();

        let mut buffer1: [u8; 64] = unsafe { core::mem::zeroed() };
        let mut buffer2: [u8; 64] = unsafe { core::mem::zeroed() };

        let r1 =
            unsafe { libc::read(fd as i32, buffer1.as_mut_ptr() as _, 64) };

        let s1 = unsafe {
            core::slice::from_raw_parts(
                buffer1.as_mut_ptr() as *const u8,
                r1 as usize,
            )
        };
        let r2 = unsafe { syscall!(Sysno::read, fd, buffer2.as_mut_ptr(), 64) };
        let s2 = unsafe {
            core::slice::from_raw_parts(
                buffer1.as_mut_ptr() as *const u8,
                r2.unwrap_or(0) as usize,
            )
        };

        assert_eq!(r2, Ok(r1 as SyscallWord));
        assert_eq!(s1, s2);

        let closed = unsafe { syscall!(Sysno::close, fd) };
        assert!(closed.is_ok());
    }

    #[test]
    fn test_syscall1_syscall4_2() {
        let fd = unsafe {
            let at_fdcwd = -100isize;
            syscall!(Sysno::openat, at_fdcwd, "/dev/zero\0".as_ptr(), 0)
        }
        .unwrap();

        let mut buffer1: [u8; 64] = unsafe { core::mem::zeroed() };
        let mut buffer2: [u8; 64] = unsafe { core::mem::zeroed() };

        let args = SyscallArgs::from(&[
            fd as SyscallWord,
            buffer1.as_mut_ptr() as _,
            64,
        ]);
        let r1 = unsafe { syscall(Sysno::read, &args) }.expect("read failed");

        let s1 = unsafe {
            core::slice::from_raw_parts(
                buffer1.as_mut_ptr() as *const u8,
                r1 as usize,
            )
        };
        let r2 = unsafe { syscall!(Sysno::read, fd, buffer2.as_mut_ptr(), 64) };
        let s2 = unsafe {
            core::slice::from_raw_parts(
                buffer1.as_mut_ptr() as *const u8,
                r2.unwrap_or(0) as usize,
            )
        };

        assert_eq!(r2, Ok(r1));
        assert_eq!(s1, s2);

        let closed = unsafe { syscall!(Sysno::close, fd) };
        assert!(closed.is_ok());
    }

    #[test]
    fn test_name() {
        assert_eq!(Sysno::write.name(), "write");
        assert_eq!(Sysno::fsopen.name(), "fsopen");
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_syscallno() {
        assert_eq!(Sysno::from(2), Sysno::open);
        assert_eq!(Sysno::new(2), Some(Sysno::open));
        assert_eq!(Sysno::new(-1i32 as usize), None);
        assert_eq!(Sysno::new(1024), None);
    }

    #[test]
    fn test_first() {
        #[cfg(target_arch = "x86_64")]
        assert_eq!(Sysno::first(), Sysno::read);

        #[cfg(target_arch = "x86")]
        assert_eq!(Sysno::first(), Sysno::restart_syscall);
    }

    #[test]
    fn test_syscall_len() {
        assert!(Sysno::table_size() > 300);
        assert!(Sysno::table_size() < 1000);
    }
}
