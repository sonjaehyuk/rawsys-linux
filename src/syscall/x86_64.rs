// On x86-64, the following registers are used for args 1-6:
// arg1: %rdi
// arg2: %rsi
// arg3: %rdx
// arg4: %r10
// arg5: %r8
// arg6: %r9
//
// rax is used for both the syscall number and the syscall return value.
//
// rcx and r11 are always clobbered. syscalls can also modify memory. With the
// `asm!()` macro, it is assumed that memory is clobbered unless the nomem
// option is specified.
use core::arch::asm;

/// System call argument/return type for x86_64 (64-bit)
pub type SyscallWord = u64;

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
            "syscall",
            inlateout("rax") n => ret,
            out("rcx") _, // rcx is used to store old rip
            out("r11") _, // r11 is used to store old rflags
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
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") arg1,
            out("rcx") _, // rcx is used to store old rip
            out("r11") _, // r11 is used to store old rflags
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
pub unsafe fn syscall2(n: SyscallWord, arg1: SyscallWord, arg2: SyscallWord) -> SyscallWord {
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            out("rcx") _, // rcx is used to store old rip
            out("r11") _, // r11 is used to store old rflags
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
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            out("rcx") _, // rcx is used to store old rip
            out("r11") _, // r11 is used to store old rflags
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
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            out("rcx") _, // rcx is used to store old rip
            out("r11") _, // r11 is used to store old rflags
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
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8")  arg5,
            out("rcx") _, // rcx is used to store old rip
            out("r11") _, // r11 is used to store old rflags
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
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8")  arg5,
            in("r9")  arg6,
            out("rcx") _, // rcx is used to store old rip
            out("r11") _, // r11 is used to store old rflags
            options(nostack, preserves_flags)
        );
    }
    ret
}
