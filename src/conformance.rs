// Backend-agnostic conformance suite for `ContentStore`.
// The address-level contract is identical for every backend, so it lives here
// once and is reused: `MemStore` runs it now, the on-disk store runs the same
// suite later. `make` must return a fresh, empty store and is called repeatedly.

use crate::contract::ContentStore;
use crate::id::ContentId;

pub fn contract_suite<S: ContentStore>(make: impl Fn() -> S) {
    presence_and_absence(&make());
    empty_blob_is_a_value(&make());
    dedup_is_address_stable(&make());
    byte_transparency(&make());
    isolation_between_stores(&make(), &make());
    address_is_pure_function_of_content(&make());
}

fn presence_and_absence<S: ContentStore>(s: &S) {
    let h = s.put(b"hello").unwrap();
    assert_eq!(s.get(&h).unwrap().as_deref(), Some(&b"hello"[..]));
    assert!(s.has(&h).unwrap());

    let absent = ContentId::of(b"never stored");
    assert!(!s.has(&absent).unwrap());
    assert!(s.get(&absent).unwrap().is_none());
}

fn empty_blob_is_a_value<S: ContentStore>(s: &S) {
    let h = s.put(&[]).unwrap();
    assert_eq!(h, ContentId::of(&[]));
    assert_eq!(s.get(&h).unwrap().as_deref(), Some(&[][..]));
    assert!(s.has(&h).unwrap());
}

fn dedup_is_address_stable<S: ContentStore>(s: &S) {
    let a = s.put(b"same content").unwrap();
    let b = s.put(b"same content").unwrap();
    assert_eq!(a, b);
    assert_eq!(s.get(&a).unwrap().as_deref(), Some(&b"same content"[..]));
}

fn byte_transparency<S: ContentStore>(s: &S) {
    // NUL bytes are data, not terminators; length is part of identity.
    let one = s.put(&[0]).unwrap();
    let three = s.put(&[0, 0, 0]).unwrap();
    assert_ne!(one, three);
    assert_eq!(s.get(&one).unwrap().as_deref(), Some(&[0u8][..]));
    assert_eq!(s.get(&three).unwrap().as_deref(), Some(&[0u8, 0, 0][..]));

    let embedded = s.put(b"a\0b").unwrap();
    assert_eq!(s.get(&embedded).unwrap().as_deref(), Some(&b"a\0b"[..]));
}

fn isolation_between_stores<S: ContentStore>(a: &S, b: &S) {
    let h = a.put(b"only in a").unwrap();
    assert!(a.has(&h).unwrap());
    assert!(!b.has(&h).unwrap());
    assert_eq!(h, ContentId::of(b"only in a"));
}

fn address_is_pure_function_of_content<S: ContentStore>(s: &S) {
    let content = b"deterministic address";
    assert_eq!(s.put(content).unwrap(), ContentId::of(content));
}
