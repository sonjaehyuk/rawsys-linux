// On arm, the following registers are used for args 1-6:
// arg1: %r0
// arg2: %r1
// arg3: %r2
// arg4: %r3
// arg5: %r4
// arg6: %r5
//
// %r7 is used for the syscall number.
//
// %r0 is reused for the syscall return value.
//
// No other registers are clobbered.
use core::arch::asm;

/// System call argument/return type for ARM (32-bit)
pub type SyscallWord = u32;

/// Issues a raw system call with 0 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall0(n: SyscallWord) -> SyscallWord {
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "svc 0",
            in("r7") n,
            lateout("r0") ret,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Issues a raw system call with 1 argument.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall1(n: SyscallWord, arg1: SyscallWord) -> SyscallWord {
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "svc 0",
            in("r7") n,
            inlateout("r0") arg1 => ret,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Issues a raw system call with 2 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall2(
    n: SyscallWord,
    arg1: SyscallWord,
    arg2: SyscallWord,
) -> SyscallWord {
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "svc 0",
            in("r7") n,
            inlateout("r0") arg1 => ret,
            in("r1") arg2,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Issues a raw system call with 3 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall3(
    n: SyscallWord,
    arg1: SyscallWord,
    arg2: SyscallWord,
    arg3: SyscallWord,
) -> SyscallWord {
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "svc 0",
            in("r7") n,
            inlateout("r0") arg1 => ret,
            in("r1") arg2,
            in("r2") arg3,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Issues a raw system call with 4 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall4(
    n: SyscallWord,
    arg1: SyscallWord,
    arg2: SyscallWord,
    arg3: SyscallWord,
    arg4: SyscallWord,
) -> SyscallWord {
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "svc 0",
            in("r7") n,
            inlateout("r0") arg1 => ret,
            in("r1") arg2,
            in("r2") arg3,
            in("r3") arg4,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Issues a raw system call with 5 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall5(
    n: SyscallWord,
    arg1: SyscallWord,
    arg2: SyscallWord,
    arg3: SyscallWord,
    arg4: SyscallWord,
    arg5: SyscallWord,
) -> SyscallWord {
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "svc 0",
            in("r7") n,
            inlateout("r0") arg1 => ret,
            in("r1") arg2,
            in("r2") arg3,
            in("r3") arg4,
            in("r4") arg5,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Issues a raw system call with 6 arguments.
///
/// # Safety
///
/// Running a system call is inherently unsafe. It is the caller's
/// responsibility to ensure safety.
#[inline]
pub unsafe fn syscall6(
    n: SyscallWord,
    arg1: SyscallWord,
    arg2: SyscallWord,
    arg3: SyscallWord,
    arg4: SyscallWord,
    arg5: SyscallWord,
    arg6: SyscallWord,
) -> SyscallWord {
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "svc 0",
            in("r7") n,
            inlateout("r0") arg1 => ret,
            in("r1") arg2,
            in("r2") arg3,
            in("r3") arg4,
            in("r4") arg5,
            in("r5") arg6,
            options(nostack, preserves_flags)
        );
    }
    ret
}
