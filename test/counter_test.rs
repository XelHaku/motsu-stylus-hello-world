// tests/counter_test.rs

#[cfg(test)]
mod tests {
    use stylus_sdk::{alloy_primitives::U256, prelude::*};
    use crate::Counter;
    use alloy_sol_types::uint;

    #[motsu::test]
    fn initial_number_is_zero(contract: Counter) {
        let zero = U256::zero();
        let number = contract.number();
        assert_eq!(number, zero);
    }

    #[motsu::test]
    fn can_set_number(contract: Counter) {
        let new_number = uint!(10_U256);
        contract.set_number(new_number);
        let number = contract.number();
        assert_eq!(number, new_number);
    }

    #[motsu::test]
    fn can_increment_number(contract: Counter) {
        contract.set_number(uint!(5_U256));
        contract.increment();
        let number = contract.number();
        assert_eq!(number, uint!(6_U256));
    }

    #[motsu::test]
    fn can_add_number(contract: Counter) {
        contract.set_number(uint!(5_U256));
        contract.add_number(uint!(3_U256));
        let number = contract.number();
        assert_eq!(number, uint!(8_U256));
    }

    #[motsu::test]
    fn can_mul_number(contract: Counter) {
        contract.set_number(uint!(5_U256));
        contract.mul_number(uint!(2_U256));
        let number = contract.number();
        assert_eq!(number, uint!(10_U256));
    }
}