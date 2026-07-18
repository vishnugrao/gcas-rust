pub mod addressing;
pub mod concurrency;
pub mod errors;
pub mod retrieval;
pub mod support;

macro_rules! conformance_suite {
    ($make:expr) => {
        #[test]
        fn stored_is_retrievable_absent_is_not() {
            crate::suite::retrieval::stored_is_retrievable_absent_is_not(&($make)());
        }

        #[test]
        fn empty_blob_is_a_value() {
            crate::suite::retrieval::empty_blob_is_a_value(&($make)());
        }

        #[test]
        fn nul_bytes_are_data_not_terminators() {
            crate::suite::retrieval::nul_bytes_are_data_not_terminators(&($make)());
        }

        #[test]
        fn dedup_is_address_stable() {
            crate::suite::addressing::dedup_is_address_stable(&($make)());
        }

        #[test]
        fn isolation_between_stores() {
            crate::suite::addressing::isolation_between_stores(&($make)(), &($make)());
        }

        #[test]
        fn address_is_pure_function_of_content() {
            crate::suite::addressing::address_is_pure_function_of_content(&($make)());
        }

        #[test]
        fn address_is_stable_across_sizes() {
            crate::suite::addressing::address_is_stable_across_sizes(&($make)());
        }

        #[test]
        fn address_is_stable_across_reader_chunking() {
            crate::suite::addressing::address_is_stable_across_reader_chunking(&($make)());
        }

        #[test]
        fn reader_io_errors_propagate() {
            crate::suite::errors::reader_io_errors_propagate(&($make)());
        }

        #[test]
        fn concurrent_puts_agree_on_address() {
            crate::suite::concurrency::concurrent_puts_agree_on_address(std::sync::Arc::new(($make)()));
        }

        #[test]
        fn interrupted_reads_are_retried() {
            crate::suite::errors::interrupted_reads_are_retried(&($make)());
        }
    };
}

pub(crate) use conformance_suite;
