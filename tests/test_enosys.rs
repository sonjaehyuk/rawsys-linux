use rawsys_linux;

// Intentionally invoke an invalid syscall number and ensure ENOSYS is returned.
//
// We pick `last_id + 100` for the current arch table which should be invalid on
// all supported kernels/arches.
// Note: The additional value of 100 is just a heuristic.
#[test]
fn invalid_syscall_returns_enosys() {
    let invalid_id = rawsys_linux::Sysno::last().id() + 100;

    // Call the raw backend directly with the invalid number.
    let ret = unsafe {
        rawsys_linux::raw::syscall0(invalid_id as rawsys_linux::SyscallWord)
    };

    // Convert according to the ABI return width.
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    let res = rawsys_linux::Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "64"
    ))]
    let res = rawsys_linux::Errno::from_ret_u64(ret as u64);

    #[cfg(all(
        not(all(target_arch = "x86_64", target_pointer_width = "32")),
        target_pointer_width = "32"
    ))]
    let res = rawsys_linux::Errno::from_ret_u32(ret as u32);

    assert_eq!(res, Err(rawsys_linux::Errno::ENOSYS));
}
