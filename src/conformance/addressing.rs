use super::support::{ChunkedReader, ramp};
use crate::contract::ContentStore;
use crate::id::ContentId;

pub(super) fn dedup_is_address_stable<S: ContentStore>(s: &S) {
    let a = s.put(b"same content").unwrap();
    let b = s.put(b"same content").unwrap();
    assert_eq!(a, b);
    assert_eq!(s.get(&a).unwrap().as_deref(), Some(&b"same content"[..]));
}

pub(super) fn isolation_between_stores<S: ContentStore>(a: &S, b: &S) {
    let h = a.put(b"only in a").unwrap();
    assert!(a.has(&h).unwrap());
    assert!(!b.has(&h).unwrap());
    assert_eq!(h, ContentId::of(b"only in a"));
}

pub(super) fn address_is_pure_function_of_content<S: ContentStore>(s: &S) {
    let content = b"deterministic address";
    assert_eq!(s.put(content).unwrap(), ContentId::of(content));
}

pub(super) fn address_is_stable_across_sizes<S: ContentStore>(s: &S) {
    for size in [4096, 65_535, 65_536, 65_537, 3 * 65_536, 256 * 65_536] {
        let data = ramp(size);
        assert_eq!(s.put(&data).unwrap(), ContentId::of(&data), "size={size}");
    }
}

pub(super) fn address_is_stable_across_reader_chunking<S: ContentStore>(s: &S) {
    let data = ramp(10_000);
    let mut reader = ChunkedReader { data: &data, chunk_size: 7 };
    assert_eq!(s.put_reader(&mut reader).unwrap(), ContentId::of(&data));
}
