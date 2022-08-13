use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u8,
}

#[near_bindgen]
impl Counter {
    pub fn get_value(&self) -> u8 {
        self.val
    }

    pub fn increment(&mut self) {
        self.val += 1;
        log!("The incremented value is {}", self.val);
    }

    pub fn decrement(&mut self) {
        if self.val > 0 {
            self.val -= 1;
        } else {
            log!("Cannot go below zero");
        }
    }

    pub fn reset(&mut self) {
        self.val = 0;
    }
}
