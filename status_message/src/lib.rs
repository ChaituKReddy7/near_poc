use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Status {
    records: LookupMap<AccountId, String>,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl Status {
    pub fn update(&mut self, message: String) {
        let account_id = env::signer_account_id();
        let exists = self.records.contains_key(&account_id);
        if exists {
            self.records.insert(&account_id, &message);
        } else {
            log!("Key doesn't exist");
        }
    }

    pub fn create(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
        log!(
            "The message stored for the account {}, is {}",
            account_id,
            message
        );
    }

    pub fn get(&self, account_id: AccountId) -> String {
        let value = match self.records.get(&account_id) {
            Some(message) => message,
            None => String::from("No value"),
        };
        value
    }
}
