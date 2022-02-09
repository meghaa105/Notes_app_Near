./build.sh
near delete notes.meghaha.testnet meghaha.testnet
near create-account notes.meghaha.testnet --masterAccount meghaha.testnet --initialBalance 20
near deploy --wasmFile target/wasm32-unknown-unknown/release/test101.wasm --accountId notes.meghaha.testnet --initFunction 'new' --initArgs '{}'
near call notes.meghaha.testnet set_status '{"status" : "Trying out writing a smart contract" }' --accountId notes.meghaha.testnet
near call notes.meghaha.testnet set_status '{"status" : "Crypto is the future" }' --accountId meghaha.testnet
near call notes.meghaha.testnet set_status '{"status" : "Blockchain is cool" }' --accountId meghaha.testnet
near call notes.meghaha.testnet get_notes '{ "account_id": "meghaha.testnet" }' --accountId notes.meghaha.testnet
near call notes.meghaha.testnet get_updates --accountId notes.meghaha.testnet