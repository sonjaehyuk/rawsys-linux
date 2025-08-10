use rawsys_linux::*;

#[test]
fn sysno_new_valid_first_last() {
    let first = Sysno::first();
    let last = Sysno::last();

    assert_eq!(Sysno::new(first.id() as usize), Some(first));
    assert_eq!(Sysno::new(last.id() as usize), Some(last));
}

#[test]
fn sysno_new_invalid_out_of_range() {
    // Negative cast to usize should be invalid
    let neg = (-1i32) as usize;
    assert_eq!(Sysno::new(neg), None);

    // One past the end should be invalid
    let past_end = (Sysno::last().id() as usize).saturating_add(1);
    assert_eq!(Sysno::new(past_end), None);

    // A very large number should be invalid (portable across arches)
    assert_eq!(Sysno::new(1024usize), None);
}

#[test]
fn sysno_new_roundtrip_all_iter() {
    // Every enumerated syscall ID must map back to the same variant.
    for s in Sysno::iter() {
        let id = s.id() as usize;
        assert_eq!(Sysno::new(id), Some(s));
    }
}

