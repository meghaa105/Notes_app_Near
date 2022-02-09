use near_sdk::{near_bindgen, env };
use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// Try these other ones, too!
// See: https://docs.rs/near-sdk/2.0.0/near_sdk/collections/index.html
use near_sdk::collections::{ UnorderedMap, TreeMap, Vector};
// use near_sdk::json_types::U128;
use near_sdk::Promise; 
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

// This isn't required, but a nice way to essentially alias a type
pub type UPC = u128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Produce {
    // Just trying out data types and nested data types
    pub status_updates: UnorderedMap<AccountId, String>,
    pub notes : UnorderedMap<AccountId,Vec<String>>,
}

impl Default for Produce {
    fn default() -> Self {
        // Check incase the contract is not initialized
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl Produce {
    /// Init attribute used for instantiation.
    #[init]
    pub fn new() -> Self {
        // Useful snippet,making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // Note this is an implicit "return" here i.e if the last line of a code block doesn't end with a semi colon it is considered as an implicit return
        Self {
            // I don't understand by do we have to do "b'SOME VARIABLE'" for instantiation
            status_updates: UnorderedMap::new(b"s".to_vec()),
            notes: UnorderedMap::new(b"w".to_vec()),
        }
    }
    pub fn set_status(&mut self, status: String) {
        // Generic implementation 
        self.status_updates.insert(&env::predecessor_account_id(), &status);
        // Checks if the notes contains the vector associated with a given account ID
        if self.notes.get(&env::predecessor_account_id()).is_none() {
            // If there is no such entry, we initialize a new Vector and insert the status and then push the vector into the map
            let mut vec = Vec:: new();
            vec.push(status);
            self.notes.insert(&env::predecessor_account_id(), &vec);
        }
        else {
            // If the account id exists in the map, we just retrieve the vector, insert the note and insert it back again. 
            // NOTE: I was wondering if there was a more efficient way of just appending a note to the existing vector
            let mut vec1 = self.notes.get(&env::predecessor_account_id()).unwrap();
            vec1.push(status);
            self.notes.insert(&env::predecessor_account_id(), &vec1);
        }

        // Note, don't need to check size, since `UnorderedMap` doesn't store all data in memory.
    }
    // Following are the generic map applications
    pub fn delete_status(&mut self) {
        self.status_updates.remove(&env::predecessor_account_id());
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<String> {
        self.status_updates.get(&account_id)
    }

    // Retrives the vector with the given account ID
    pub fn get_notes(&self, account_id: AccountId) -> Vec<std::string::String> {
        match self.notes.get(&account_id) {
            Some(x) => x,
            None => vec![],
        }
        // self.notes.get(&account_id).unwrap() # Converts it from optional to vector
    }

    // Deletes notes associated to the account id through which the function is called
    pub fn delete_note(&mut self) {
        self.notes.remove(&env::predecessor_account_id());
    }
    
    // Retreives all the notes from the map
    pub fn get_updates(&self)-> Vec<Vec<std::string::String>> {
        
        let _keys = self.notes.keys_as_vector();
        let _values = self.notes.values_as_vector();
        let v1_iter = _values.iter();
        // println!("{_values:?}");
        let mut ans = Vec:: new();
        for i in v1_iter {
            ans.push(i);
        }
        ans
    }

    pub fn transfer_money(&mut self, account_id: AccountId, amount: f64) {
        Promise::new(account_id).transfer(amount as u128);
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test -- --nocapture
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, AccountId};

    // part of writing unit tests is setting up a mock context
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool, predecessor: AccountId) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "mike.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: predecessor,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    // unlike other frameworks, the function names don't need to be special or have "test" in it
    #[test]
    fn add_notes() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Produce::new();
        let note_id = U128(679508051007679508);
        let soso = "Hello".to_string();
        contract.add_veggie_taste(cucumber_upc.clone(), soso.clone());
        // we can do println! in tests, but reminder to use env::log outside of tests
        let returned_taste = contract.get_taste(note_id);
        println!("Note returned: {}", returned_taste.clone());
        // confirm
        assert_eq!(hello, returned_taste);
    }
}