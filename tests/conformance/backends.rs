use gcas_rust::backend::mem::MemStore;
use gcas_rust::conformance::contract_suite;

#[test]
fn mem_conforms() {
    contract_suite(MemStore::new);
}

// #[test]
// fn loose_conforms() {
//     let dir = tempfile::tempdir().unwrap();
//     contract_suite(|| LooseStore::open(dir.path()).unwrap());
// }
