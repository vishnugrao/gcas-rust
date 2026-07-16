use crate::suite::conformance_suite;

mod mem {
    use super::conformance_suite;
    use crate::suite::support::Fixture;
    use gcas_rust::backend::mem::MemStore;
    conformance_suite!(|| Fixture::bare(MemStore::new()));
}

mod loose {
    use super::conformance_suite;
    use crate::suite::support::Fixture;
    use gcas_rust::backend::loose::LooseStore;
    use tempfile::tempdir;

    conformance_suite!(|| {
        let dir = tempdir().unwrap();
        let store = LooseStore::open(dir.path()).unwrap();
        Fixture::with_dir(store, dir)
    });
}
