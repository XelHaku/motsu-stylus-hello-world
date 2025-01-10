// tests/counter_test.rs

#[cfg(test)]
mod tests {
    use crate::Counter;
    use stylus_sdk::{alloy_primitives::U256, prelude::*};

    #[motsu::test]
    fn initial_number_is_zero(contract: Counter) {
        let zero = U256::ZERO;
        let number = contract.number();
        assert_eq!(number, zero);
    }

    #[motsu::test]
    fn can_set_number(contract: Counter) {
        let new_number = U256::from(10);
        contract.set_number(new_number);
        let number = contract.number();
        assert_eq!(number, new_number);
    }

    #[motsu::test]
    fn can_increment_number(contract: Counter) {
        contract.set_number(U256::from(5));
        contract.increment();
        let number = contract.number();
        assert_eq!(number, U256::from(6));
    }

    #[motsu::test]
    fn can_add_number(contract: Counter) {
        contract.set_number(U256::from(5));
        contract.add_number(U256::from(3));
        let number = contract.number();
        assert_eq!(number, U256::from(8));
    }

    #[motsu::test]
    fn can_mul_number(contract: Counter) {
        contract.set_number(U256::from(5));
        contract.mul_number(U256::from(2));
        let number = contract.number();
        assert_eq!(number, U256::from(10));
    }
}
