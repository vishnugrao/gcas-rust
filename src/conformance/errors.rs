use super::support::{FailingReader, PartialThenFailReader};
use crate::contract::ContentStore;
use crate::error::StoreError;

pub(super) fn reader_io_errors_propagate<S: ContentStore>(s: &S) {
    let immediate = s.put_reader(&mut FailingReader).unwrap_err();
    assert!(matches!(immediate, StoreError::Io(_)), "got {immediate:?}");

    let partial = s
        .put_reader(&mut PartialThenFailReader { sent: false })
        .unwrap_err();
    assert!(matches!(partial, StoreError::Io(_)), "got {partial:?}");
}
