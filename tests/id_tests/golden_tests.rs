#[cfg(test)]
mod tests {

    // Frozen golden vectors: the cross-implementation contract.
    // Two calls agreeing only proves determinism within one build; a frozen
    // constant proves the address will not drift across builds, machines, or a
    // future hash-config edit. That is the property the fleet agrees on.

    use gcas_rust::id::ContentId;

    #[test]
    fn empty_blob_address_is_frozen() {
        assert_eq!(
            ContentId::of(&[]).to_string(),
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn hello_address_is_frozen() {
        assert_eq!(
            ContentId::of(b"hello").to_string(),
            "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
        );
    }
}
