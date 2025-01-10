#[cfg(test)]
mod tests {
    use crate::Counter;
    use stylus_sdk::{alloy_primitives::U256, prelude::*};

    #[motsu::test]
    fn initial_vector_state(contract: Counter) {
        let vector = contract.number_vector();
        assert!(vector.is_empty(), "Initial vector should be empty");
    }

    #[motsu::test]
    fn can_add_to_vector(contract: Counter) {
        let value = U256::from(42);
        contract.add_number_vector(value);

        let vector = contract.number_vector();
        assert_eq!(vector.len(), 1, "Vector should have one element");
        assert_eq!(
            vector[0], value,
            "Element in vector should match the added value"
        );
    }

    #[motsu::test]
    fn can_set_vector_element(contract: Counter) {
        // Add two elements to the vector
        contract.add_number_vector(U256::from(10));
        contract.add_number_vector(U256::from(20));

        // Update the first element
        let new_value = U256::from(30);
        contract.set_number_vector(new_value, U256::from(0));

        let vector = contract.number_vector();
        assert_eq!(vector.len(), 2, "Vector should have two elements");
        assert_eq!(vector[0], new_value, "First element should be updated");
        assert_eq!(
            vector[1],
            U256::from(20),
            "Second element should remain unchanged"
        );
    }

    #[motsu::test]
    fn can_remove_vector_element(contract: Counter) {
        // Add three elements to the vector
        contract.add_number_vector(U256::from(10));
        contract.add_number_vector(U256::from(20));
        contract.add_number_vector(U256::from(30));

        // Remove the second element (index 1)
        contract.remove_number_vector(U256::from(1));

        let vector = contract.number_vector();
        assert_eq!(
            vector.len(),
            2,
            "Vector should have two elements after removal"
        );
        assert_eq!(
            vector[0],
            U256::from(10),
            "First element should remain unchanged"
        );
        assert_eq!(
            vector[1],
            U256::from(30),
            "Last element should replace the removed element"
        );
    }

    #[motsu::test]
    fn remove_last_element(contract: Counter) {
        // Add one element to the vector
        contract.add_number_vector(U256::from(10));

        // Remove the only element
        contract.remove_number_vector(U256::from(0));

        let vector = contract.number_vector();
        assert!(
            vector.is_empty(),
            "Vector should be empty after removing the last element"
        );
    }
}
