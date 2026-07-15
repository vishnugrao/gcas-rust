use gcas_rust::id::{ContentId, ParseIdError};

#[test]
fn round_trip_is_lossless() {
    let id = ContentId::of(b"hello world");
    let parsed = ContentId::from_hex(&id.to_string()).unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn rejects_non_hex_characters() {
    let err = ContentId::from_hex(&"z".repeat(64)).unwrap_err();
    assert!(matches!(err, ParseIdError::Hex(_)), "got {err:?}");
}

#[test]
fn rejects_odd_length_string() {
    let err = ContentId::from_hex(&"a".repeat(63)).unwrap_err();
    assert!(matches!(err, ParseIdError::Hex(_)), "got {err:?}");
}

#[test]
fn rejects_too_few_bytes() {
    let err = ContentId::from_hex(&"a".repeat(60)).unwrap_err();
    assert!(matches!(err, ParseIdError::BadLength(30)), "got {err:?}");
}

#[test]
fn rejects_too_many_bytes() {
    let err = ContentId::from_hex(&"a".repeat(66)).unwrap_err();
    assert!(matches!(err, ParseIdError::BadLength(33)), "got {err:?}");
}

#[test]
fn rejects_empty_string() {
    let err = ContentId::from_hex("").unwrap_err();
    assert!(matches!(err, ParseIdError::BadLength(0)), "got {err:?}");
}

#[test]
fn one_char_short_of_valid_is_rejected() {
    let mut s = ContentId::of(b"anchor").to_string();
    s.pop();
    let err = ContentId::from_hex(&s).unwrap_err();
    assert!(matches!(err, ParseIdError::Hex(_)), "got {err:?}");
}
