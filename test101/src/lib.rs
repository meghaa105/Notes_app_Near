// // To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::{env, near_bindgen, setup_alloc};
// use near_sdk::collections::LookupMap;

// setup_alloc!();

// // Structs in Rust are similar to other languages, and may include impl keyword as shown below
// // Note: the names of the structs are not important when calling the smart contract, but the function names are
// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]

// pub struct Welcome {
//     records: LookupMap<String, String>,
// }

// impl Default for Welcome {
//   fn default() -> Self {
//     Self {
//       records: LookupMap::new(b"a".to_vec()),
//     }
//   }
// }

// #[near_bindgen]
// impl Welcome {
//     pub fn set_greeting(&mut self, message: String) {
//         let account_id = env::signer_account_id();

//         // Use env::log to record logs permanently to the blockchain!
//         env::log(format!("Saving greeting '{}' for account '{}'", message, account_id,).as_bytes());

//         self.records.insert(&account_id, &message);
//     }

//     // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
//     // self.records.get(&account_id) is not yet defined.
//     // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
//     pub fn get_greeting(&self, account_id: String) -> String {
//         match self.records.get(&account_id) {
//             Some(greeting) => greeting,
//             None => "Hello".to_string(),
//         }
//     }
// }


use near_sdk::{near_bindgen, env };
use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::Timestamp;
// Try these other ones, too!
// See: https://docs.rs/near-sdk/2.0.0/near_sdk/collections/index.html
use near_sdk::collections::{ UnorderedMap, TreeMap, Vector};
// use near_sdk::collections::{ TreeMap };
use near_sdk::json_types::U128;
use near_sdk::Promise; 
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

// This isn't required, but a nice way to essentially alias a type
pub type UPC = u128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Produce {
    pub veggies_taste: TreeMap<UPC, String>,
    pub status_updates: UnorderedMap<AccountId, String>,
    pub notes : UnorderedMap<AccountId,Vec<String>>,
}

impl Default for Produce {
    fn default() -> Self {
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl Produce {
    /// Init attribute used for instantiation.
    #[init]
    pub fn new() -> Self {
        // Useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // Note this is an implicit "return" here
        Self {
            veggies_taste: TreeMap::new(b"v".to_vec()),
            status_updates: UnorderedMap::new(b"s".to_vec()),
            notes: UnorderedMap::new(b"w".to_vec()),
        }
    }

    // This functions changes state, so 1st param uses `&mut self`
    /// Add a veggie and its taste
    pub fn add_veggie_taste(&mut self, upc: U128, taste: String) {
        let existing_veggie: Option<String> = self.veggies_taste.get(&upc.into());
        if existing_veggie.is_some() {
            env::panic(b"Sorry, already added this UPC.")
        }
        self.veggies_taste.insert(&upc.into(), &taste);
    }

    // This functions simple returns state, so 1st param uses `&self`
    /// Return the stored taste for a veggie
    pub fn get_taste(&self, upc: U128) -> String {
        match self.veggies_taste.get(&upc.into()) {
            Some(stored_taste) => {
                let log_message = format!("{}", stored_taste.clone());
                env::log(log_message.as_bytes());
                // found account user in map, return the taste
                stored_taste
            },
            // did not find the veggie
            // note: curly brackets after arrow are optional in simple cases, like other languages
            None => "No note found.".to_string()
        }
    }

    /// Throw out all veggies. (reset the data structure)
    pub fn perish_all(&mut self) {
        assert_eq!(env::current_account_id(), env::predecessor_account_id(), "To cause all veggies to perish, this method must be called by the (implied) contract owner.");
        self.veggies_taste.clear();
        env::log(b"All notes removed, time to add more!");
    }

    pub fn view_all(&mut self) {
        // System.out.println("TreeMap: ");
        // for value in self.veggies_taste.values() {
        //     println!("{}", value);
        // }
        
    }

    pub fn set_status(&mut self, status: String) {
        self.status_updates.insert(&env::predecessor_account_id(), &status);
        if self.notes.get(&env::predecessor_account_id()).is_none() {
            let mut vec = Vec:: new();
            vec.push(status);
            self.notes.insert(&env::predecessor_account_id(), &vec);
        }
        else {
            let mut vec1 = self.notes.get(&env::predecessor_account_id()).unwrap();
            vec1.push(status);
            self.notes.insert(&env::predecessor_account_id(), &vec1);
        }

        // Note, don't need to check size, since `UnorderedMap` doesn't store all data in memory.
    }

    pub fn delete_status(&mut self) {
        self.status_updates.remove(&env::predecessor_account_id());
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<String> {
        self.status_updates.get(&account_id)
    }
    pub fn get_notes(&self, account_id: AccountId) -> Vec<std::string::String> {
        match self.notes.get(&account_id) {
            Some(x) => x,
            None => vec![],
        }
        // self.notes.get(&account_id).unwrap()
    }

    // pub fn get_updates(&self) -> Vec<(AccountId, std::string::String)> {
    pub fn get_updates(&self)-> Vec<Vec<std::string::String>> {
      
        let _keys = self.notes.keys_as_vector();
        let _values = self.notes.values_as_vector();
        let v1_iter = _values.iter();
        println!("{_values:?}");
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
    fn add_veggie() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Produce::new();
        let cucumber_upc = U128(679508051007679508);
        let soso = "so-so".to_string();
        contract.add_veggie_taste(cucumber_upc.clone(), soso.clone());
        // we can do println! in tests, but reminder to use env::log outside of tests
        let returned_taste = contract.get_taste(cucumber_upc);
        println!("Taste returned: {}", returned_taste.clone());
        // confirm
        assert_eq!(soso, returned_taste);
    }
}