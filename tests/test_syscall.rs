use rawsys_linux::*;

#[test]
fn test_syscall() {
    // Fixed an issue where the STDOUT pipe would break.
    let s = "Hello\0";
    let fd = unsafe {
        let at_fdcwd = -100isize;
        syscall!(Sysno::openat, at_fdcwd, "/dev/null\0".as_ptr(), 2) // The mode value is system-dependent. If your test fails, try changing the mode value first.
    }
    .unwrap();

    assert_eq!(
        unsafe { syscall!(Sysno::write, fd, s.as_ptr() as *const _, 6) },
        Ok(6)
    );

    let _ = unsafe { syscall!(Sysno::close, fd) };
}

#[test]
fn test_syscall_map() {
    // Make sure the macro exports are ok
    let mut map = SysnoMap::new();
    assert!(map.is_empty());
    assert_eq!(map.count(), 0);
    assert_eq!(map.get(Sysno::write), None);
    map.insert(Sysno::write, 42);
    assert_eq!(map.get(Sysno::write), Some(&42));
    assert_eq!(map.count(), 1);
    assert!(!map.is_empty());
    map.remove(Sysno::write);
    assert_eq!(map.count(), 0);
    assert!(map.is_empty());
}
