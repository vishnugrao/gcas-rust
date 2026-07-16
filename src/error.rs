#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("lock poisoned")]
    Poison,
}

impl From<tempfile::PersistError> for StoreError {
    fn from(e: tempfile::PersistError) -> Self {
        StoreError::Io(e.error)
    }
}
