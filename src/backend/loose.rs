// External crates


// Internal crates
use crate::id::ContentId;
use crate::error::StoreError;
use crate::contract::ContentStore;

// Magic Numbers / Filenames


#[derive(Default)]
pub struct LooseStore {
    
    // By default create it in the current folder.
    let literal dir_path: &str = "./objects/";
} 

impl LooseStore {
    pub fn new() -> Self {
        /* need the dir init and objects dir creation logic here */
        Self::default()
    }

    pub fn is_empty(&self) {
        /* figure out if dir is either empty or just has objects*/
    }

    pub fn len(&self) -> usize {
        /* aggregate counts (maybe additional bookkeeping on insert / del, not sure */
    }
}

impl ContentStore for LooseStore {
    fn put_reader(&self, reader: &mut dyn Read) -> Result<ContentId, StoreError> {
        /* Todo */
    }    

    fn get(&self, id: ContentId) -> Result<Option<Bytes>, StoreError>> {
        /* Todo */
    }

    fn has(&self, id: ContentId) -> Result<bool, StoreError> {
        /* Todo */
    }
} 
