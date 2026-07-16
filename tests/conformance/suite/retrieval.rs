use gcas_rust::contract::ContentStore;
use gcas_rust::id::ContentId;

pub fn stored_is_retrievable_absent_is_not<S: ContentStore>(s: &S) {
    let h = s.put(b"hello").unwrap();
    assert_eq!(s.get(&h).unwrap().as_deref(), Some(&b"hello"[..]));
    assert!(s.has(&h).unwrap());

    let absent = ContentId::of(b"never stored");
    assert!(!s.has(&absent).unwrap());
    assert!(s.get(&absent).unwrap().is_none());
}

pub fn empty_blob_is_a_value<S: ContentStore>(s: &S) {
    let h = s.put(&[]).unwrap();
    assert_eq!(h, ContentId::of(&[]));
    assert_eq!(s.get(&h).unwrap().as_deref(), Some(&[][..]));
    assert!(s.has(&h).unwrap());
}

pub fn nul_bytes_are_data_not_terminators<S: ContentStore>(s: &S) {
    let one = s.put(&[0]).unwrap();
    let three = s.put(&[0, 0, 0]).unwrap();
    assert_ne!(one, three);
    assert_eq!(s.get(&one).unwrap().as_deref(), Some(&[0u8][..]));
    assert_eq!(s.get(&three).unwrap().as_deref(), Some(&[0u8, 0, 0][..]));

    let embedded = s.put(b"a\0b").unwrap();
    assert_eq!(s.get(&embedded).unwrap().as_deref(), Some(&b"a\0b"[..]));
}
