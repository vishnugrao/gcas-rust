#[cfg(test)]
mod tests {

    // Negative tests for ContentId::from_hex:
    // - non-hex characters          -> ParseIdError::Hex
    // - odd-length string           -> ParseIdError::Hex (hex crate rejects it)
    // - valid hex, too few bytes    -> ParseIdError::BadLength
    // - valid hex, too many bytes   -> ParseIdError::BadLength
    // - empty string                -> ParseIdError::BadLength(0)
    // Plus a positive round-trip so the parser is anchored.

    use gcas_rust::id::{ContentId, ParseIdError};

    /// A known-good 64 hex-char (32 byte) string.
    fn valid_hex() -> String {
        ContentId::of(b"anchor").to_string()
    }

    #[test]
    fn round_trip_is_lossless() {
        let id = ContentId::of(b"hello world");
        let parsed = ContentId::from_hex(&id.to_string()).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn rejects_non_hex_characters() {
        // 64 chars but 'z' is not a hex digit.
        let bad = "z".repeat(64);
        let err = ContentId::from_hex(&bad).unwrap_err();
        assert!(
            matches!(err, ParseIdError::Hex(_)),
            "expected ParseIdError::Hex, got {err:?}"
        );
    }

    #[test]
    fn rejects_odd_length_string() {
        // 63 chars: valid hex digits but an odd count, which hex::decode rejects.
        let bad = "a".repeat(63);
        let err = ContentId::from_hex(&bad).unwrap_err();
        assert!(
            matches!(err, ParseIdError::Hex(_)),
            "expected ParseIdError::Hex for odd length, got {err:?}"
        );
    }

    #[test]
    fn rejects_too_few_bytes() {
        // 60 hex chars -> 30 bytes -> not 32.
        let bad = "a".repeat(60);
        let err = ContentId::from_hex(&bad).unwrap_err();
        assert!(
            matches!(err, ParseIdError::BadLength(30)),
            "expected ParseIdError::BadLength(30), got {err:?}"
        );
    }

    #[test]
    fn rejects_too_many_bytes() {
        // 66 hex chars -> 33 bytes -> not 32.
        let bad = "a".repeat(66);
        let err = ContentId::from_hex(&bad).unwrap_err();
        assert!(
            matches!(err, ParseIdError::BadLength(33)),
            "expected ParseIdError::BadLength(33), got {err:?}"
        );
    }

    #[test]
    fn rejects_empty_string() {
        let err = ContentId::from_hex("").unwrap_err();
        assert!(
            matches!(err, ParseIdError::BadLength(0)),
            "expected ParseIdError::BadLength(0), got {err:?}"
        );
    }

    #[test]
    fn one_char_short_of_valid_is_rejected() {
        // Truncating a genuinely-valid id by one hex char makes it odd-length.
        let mut s = valid_hex();
        s.pop();
        let err = ContentId::from_hex(&s).unwrap_err();
        assert!(
            matches!(err, ParseIdError::Hex(_)),
            "expected ParseIdError::Hex, got {err:?}"
        );
    }
}
