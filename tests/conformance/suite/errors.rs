use super::support::{FailingReader, PartialThenFailReader, InterruptingReader};
use gcas_rust::contract::ContentStore;
use gcas_rust::error::StoreError;
use gcas_rust::id::ContentId;
use crate::suite::support::ramp;

pub fn reader_io_errors_propagate<S: ContentStore>(s: &S) {
    let immediate = s.put_reader(&mut FailingReader).unwrap_err();
    assert!(matches!(immediate, StoreError::Io(_)), "got {immediate:?}");

    let partial = s
        .put_reader(&mut PartialThenFailReader { sent: false })
        .unwrap_err();
    assert!(matches!(partial, StoreError::Io(_)), "got {partial:?}");
}

pub fn interrupted_reads_are_retried<S: ContentStore>(s: &S) {
    let data = ramp(10_000);
    let mut reader = InterruptingReader{data: &data, chunk_size: 7, armed: true};
    assert_eq!(s.put_reader(&mut reader).unwrap(), ContentId::of(&data));
}
