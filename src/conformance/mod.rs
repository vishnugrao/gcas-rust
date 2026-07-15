use std::sync::Arc;

use crate::contract::ContentStore;

mod addressing;
mod concurrency;
mod errors;
mod retrieval;
mod support;

pub fn contract_suite<S: ContentStore + 'static>(make: impl Fn() -> S) {
    retrieval::stored_is_retrievable_absent_is_not(&make());
    retrieval::empty_blob_is_a_value(&make());
    retrieval::nul_bytes_are_data_not_terminators(&make());

    addressing::dedup_is_address_stable(&make());
    addressing::isolation_between_stores(&make(), &make());
    addressing::address_is_pure_function_of_content(&make());
    addressing::address_is_stable_across_sizes(&make());
    addressing::address_is_stable_across_reader_chunking(&make());

    errors::reader_io_errors_propagate(&make());

    concurrency::concurrent_puts_agree_on_address(Arc::new(make()));
}
