// On x86, the following registers are used for args 1-6:
// arg1: %ebx
// arg2: %ecx
// arg3: %edx
// arg4: %esi
// arg5: %edi
// arg6: %ebp
//
// eax is used for both the syscall number and the syscall return value.
//
// No other registers are clobbered. syscalls can also modify memory. With the
// `asm!()` macro, it is assumed that memory is clobbered unless the nomem
// option is specified.
use core::arch::asm;


/// System call argument/return type for x86
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
            "int 0x80",
            inlateout("eax") n => ret,
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
            "int 0x80",
            inlateout("eax") n => ret,
            in("ebx") arg1,
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
            "int 0x80",
            inlateout("eax") n => ret,
            in("ebx") arg1,
            in("ecx") arg2,
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
            "int 0x80",
            inlateout("eax") n => ret,
            in("ebx") arg1,
            in("ecx") arg2,
            in("edx") arg3,
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
            "xchg esi, {arg4}",
            "int 0x80",
            "xchg esi, {arg4}",
            // Using esi is not allowed, so we need to use another register to
            // save/restore esi. Thus, we can say that esi is not clobbered.
            arg4 = in(reg) arg4,
            inlateout("eax") n => ret,
            in("ebx") arg1,
            in("ecx") arg2,
            in("edx") arg3,
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
            "xchg esi, {arg4}",
            "int 0x80",
            "xchg esi, {arg4}",
            // Using esi is not allowed, so we need to use another register to
            // save/restore esi. Thus, we can say that esi is not clobbered.
            arg4 = in(reg) arg4,
            inlateout("eax") n => ret,
            in("ebx") arg1,
            in("ecx") arg2,
            in("edx") arg3,
            in("edi") arg5,
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
    // Since using esi and ebp are not allowed and because x86 only has 6
    // general purpose registers (excluding ESP and EBP), we need to push them
    // onto the stack and then set them using a pointer to memory (our input
    // array).
    let mut ret: SyscallWord;
    unsafe {
        asm!(
            "push ebp",
            "push esi",
            "mov esi, DWORD PTR [eax + 0]", // Set esi to arg4
            "mov ebp, DWORD PTR [eax + 4]", // Set ebp to arg6
            "mov eax, DWORD PTR [eax + 8]", // Lastly, set eax to the syscall number.
            "int 0x80",
            "pop esi",
            "pop ebp",
            // Set eax to a pointer to our input array.
            inout("eax") &[arg4, arg6, n] => ret,
            in("ebx") arg1,
            in("ecx") arg2,
            in("edx") arg3,
            in("edi") arg5,
            options(preserves_flags)
        );
    }
    ret
}
