use std::collections::HashMap;

#[derive(Default)]
pub struct MemStore {
    map: RwLock<HashMap<ContentId, Bytes>>,
}
