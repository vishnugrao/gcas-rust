use crate::suite::conformance_suite;

mod mem {
    use super::conformance_suite;
    use gcas_rust::backend::mem::MemStore;
    conformance_suite!(MemStore::new);
}

// mod loose {
//     use super::conformance_suite;
//     use gcas_rust::backend::loose::LooseStore;
//     conformance_suite!(|| LooseStore::open(tempfile::tempdir().unwrap().path()).unwrap());
// }
