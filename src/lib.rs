// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;
mod test;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::{U256,Address}, msg, prelude::*, storage::StorageU256};
use stylus_sdk::storage::{StorageMap, StorageUint,StorageVec};
// Define the contract's storage layout using the Solidity ABI.
// sol_storage! {
//     #[entrypoint]
//     pub struct Counter {
//         uint256 number;

//         // Mapping from address to uint256
//         mapping(address => uint256) number_mapping;

//         // Vector of uint256
//         uint256[] number_vector;
//     }


// }

// Define the contract's storage layout using `sol_storage!`.
    #[entrypoint]
    #[storage]
    pub struct Counter {
        // A single `uint256` value.
        number: StorageUint<256, 4>,

        // Mapping from address to `uint256`.
        number_mapping: StorageMap<Address, StorageUint<256, 4>>,

        // Vector of `uint256`.
        number_vector: StorageVec<StorageUint<256, 4>>,
    }

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    /// Retrieves the current number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets the number to a new user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets the number to the product of the current number and a new value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets the number to the sum of the current number and a new value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments the number by one.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    // Functions for interacting with the number_mapping storage

    /// Retrieves the mapped number for the current sender's address.
    pub fn number_mapping(&self) -> U256 {
        self.number_mapping.getter(msg::sender()).get()
    }

    /// Sets the mapped number for the current sender's address to a new value.
    pub fn set_number_mapping(&mut self, new_number: U256) {
        let sender = msg::sender();
        self.number_mapping.setter(sender).set(new_number);
    }

    /// Increments the mapped number for the current sender's address by one.
    pub fn increment_mapping(&mut self) {
        let number = self.number_mapping.getter(msg::sender()).get();
        self.set_number_mapping(number + U256::from(1));
    }

    // Functions for interacting with the number_vector storage

    /// Retrieves the entire vector of numbers.
    pub fn number_vector(&self) -> Vec<U256> {
        let length = self.number_vector.len();
        let mut result = Vec::new();
        for i in 0..length {
            result.push(self.number_vector.get(i).expect("Value not found"));
        }
        result
    }

    /// Adds a new number to the end of the vector.
    pub fn add_number_vector(&mut self, new_number: U256) {
        self.number_vector.push(new_number);
    }

    /// Sets the number at a specific index in the vector to a new value.
    pub fn set_number_vector(&mut self, new_number: U256, index: U256) {
        self.number_vector.setter(index).unwrap().set(new_number);
    }

    /// Removes the element at a specified index from the vector.
    pub fn remove_number_vector(&mut self, index: U256) {
        let length = U256::from(self.number_vector.len());

        // Validate that the index is within bounds
        if index >= length {
            panic!("Index out of bounds");
        }

        if index < length - U256::from(1) {
            // Replace the element at `index` with the last element
            if let Some(last_event) = self.number_vector.get(length - U256::from(1)) {
                self.number_vector.setter(index).unwrap().set(last_event);
            }
        }

        // Remove the last element
        self.number_vector.pop();
    }
}
