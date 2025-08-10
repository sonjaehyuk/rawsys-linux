// On riscv32, the following registers are used for args 1-6:
// arg1: %a0
// arg2: %a1
// arg3: %a2
// arg4: %a3
// arg5: %a4
// arg6: %a5
//
// %a7 is used for the syscall number.
//
// %a0 is reused for the syscall return value.
//
// No other registers are clobbered.
use core::arch::asm;

/// System call argument/return type for riscv
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
            "ecall",
            in("a7") n,
            out("a0") ret,
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
            "ecall",
            in("a7") n,
            inlateout("a0") arg1 => ret,
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
            "ecall",
            in("a7") n,
            inlateout("a0") arg1 => ret,
            in("a1") arg2,
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
            "ecall",
            in("a7") n,
            inlateout("a0") arg1 => ret,
            in("a1") arg2,
            in("a2") arg3,
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
            "ecall",
            in("a7") n,
            inlateout("a0") arg1 => ret,
            in("a1") arg2,
            in("a2") arg3,
            in("a3") arg4,
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
            "ecall",
            in("a7") n,
            inlateout("a0") arg1 => ret,
            in("a1") arg2,
            in("a2") arg3,
            in("a3") arg4,
            in("a4") arg5,
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
            "ecall",
            in("a7") n,
            inlateout("a0") arg1 => ret,
            in("a1") arg2,
            in("a2") arg3,
            in("a3") arg4,
            in("a4") arg5,
            in("a5") arg6,
            options(nostack, preserves_flags)
        );
    }
    ret
}
