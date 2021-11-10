#
export NEAR_ENV=testnet
#
near deploy --accountId registry.test_oct.testnet --wasmFile res/appchain_registry.wasm
#
near call registry.test_oct.testnet migrate_state '' --accountId test_oct.testnet --gas 200000000000000
#
# view functions
#
near view registry.test_oct.testnet get_appchains_with_state_of '{"appchain_state":null,"page_number":1,"page_size":50,"sorting_field":"AppchainId","sorting_order":"Ascending"}' --accountId test_oct.testnet
#
near view registry.test_oct.testnet get_appchain_ids '{}' --accountId test_oct.testnet
#
near view registry.test_oct.testnet get_appchains_with_state_of '{"appchain_state":null,"page_number":1,"page_size":50,"sorting_field":"AppchainId","sorting_order":"Ascending"}' --accountId test_oct.testnet
#
near view registry.test_oct.testnet get_appchain_status_of '{"appchain_id":"easydeal"}' --accountId test_oct.testnet
