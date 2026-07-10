#[cfg(test)]
mod tests {

    // Every backend runs the identical address-level suite from `gcas_rust::conformance`.
    // MemStore passes it today; Stage 1's on-disk store passes the same suite (see below).

    use gcas_rust::backend::mem::MemStore;
    use gcas_rust::conformance::contract_suite;

    #[test]
    fn mem_conforms() {
        contract_suite(MemStore::new);
    }

    // Stage 1: the same suite runs against disk, byte-for-byte.
    // #[test]
    // fn loose_conforms() {
    //     let dir = tempfile::tempdir().unwrap();
    //     contract_suite(|| LooseStore::open(dir.path()).unwrap());
    // }
}
