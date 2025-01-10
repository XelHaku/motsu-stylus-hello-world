// tests/counter_test.rs

#[cfg(test)]
mod tests {
    use crate::Counter;
    use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};
    #[motsu::test]
    fn initial_mapping_value(contract: Counter) {
        // let sender = msg::sender();
        let value = contract.number_mapping();
        assert_eq!(value, U256::ZERO);
    }

    #[motsu::test]
    fn can_set_mapping_value(contract: Counter) {
        let value = U256::from(10);
        contract.set_number_mapping(value);
        let retrieved = contract.number_mapping();
        assert_eq!(retrieved, value);
    }

    #[motsu::test]
    fn can_increment_mapping(contract: Counter) {
        contract.set_number_mapping(U256::from(5));
        contract.increment_mapping();
        let value = contract.number_mapping();
        assert_eq!(value, U256::from(6));
    }

    #[motsu::test]
    fn overflow_increment(contract: Counter) {
        contract.set_number(U256::MAX);
        contract.increment();
        let number = contract.number();
        assert_eq!(number, U256::ZERO);
    }
}
