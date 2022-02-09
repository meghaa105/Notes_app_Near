## Notes storage App in Rust
Functionalities implemented
1. Create note from an account
2. Fetch all the notes from that account
3. Append more notes in the same account
4. Fetch all the stored notes on the database
5. Delete notes 

Currently working on 
1. Edit notes

The smart contract is deployed on notes.meghaha.testnet

Instructions 
1. To build the contract - run ./build.sh
2. To check out various functions of the contract 
   a. Create / Add notes from an account 
   near call notes.meghaha.testnet set_status '{"status" : "Trying out writing a smart contract" }' --accountId notes.meghaha.testnet
   near call notes.meghaha.testnet set_status '{"status" : "Crypto is the future" }' --accountId meghaha.testnet
   b. Fetch all notes stored in a particular account
   near call notes.meghaha.testnet get_notes '{ "account_id": "meghaha.testnet" }' --accountId notes.meghaha.testnet
   c. Fetch all the stored notes
   near call notes.meghaha.testnet get_updates --accountId notes.meghaha.testnet
   d. Add more notes to the same account
   near call notes.meghaha.testnet set_status '{"status" : "Blockchain is cool" }' --accountId meghaha.testnet
   e. Delete notes
   near call notes.meghaha.testnet delete_note --accountId meghaha.testnet

    
   







