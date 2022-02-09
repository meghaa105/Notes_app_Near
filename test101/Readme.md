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

## Instructions 
1. To build the contract - run ./build.sh  <br />
2. To check out various functions of the contract <br />
   a. Create / Add notes from an account <br />
   ``` near call notes.meghaha.testnet set_status '{"status" : "Trying out writing a smart contract" }' --accountId notes.meghaha.testnet ``` <br />  <br />
   ``` near call notes.meghaha.testnet set_status '{"status" : "Crypto is the future" }' --accountId meghaha.testnet ``` <br />
   b. Fetch all notes stored in a particular account <br />
   ``` near call notes.meghaha.testnet get_notes '{ "account_id": "meghaha.testnet" }' --accountId notes.meghaha.testnet ``` <br />
   c. Fetch all the stored notes <br />
   ```near call notes.meghaha.testnet get_updates --accountId notes.meghaha.testnet ``` 
   <br />
   d. Add more notes to the same account <br />
   ``` near call notes.meghaha.testnet set_status '{"status" : "Blockchain is cool" }' --accountId meghaha.testnet ``` <br />
   e. Delete notes <br />
   ``` near call notes.meghaha.testnet delete_note --accountId meghaha.testnet ```<br />

    
   







