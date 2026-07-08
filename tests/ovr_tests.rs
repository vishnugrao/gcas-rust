#[cfg(test)]
mod tests {
    use gcas_rust::contract::ContractImpl;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);
        let mut ci = ContractImpl::new();
        ci.say_hello();
    }
}
