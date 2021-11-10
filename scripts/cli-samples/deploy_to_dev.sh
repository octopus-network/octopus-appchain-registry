#
export NEAR_ENV=testnet
#
near deploy --accountId dev-oct-registry.testnet --wasmFile res/appchain_registry.wasm
#
near call dev-oct-registry.testnet migrate_state '' --accountId dev-oct-registry.testnet --gas 200000000000000
#
near view dev-oct-registry.testnet get_appchains_with_state_of '{"appchain_state":null,"page_number":1,"page_size":50,"sorting_field":"AppchainId","sorting_order":"Ascending"}' --accountId dev-oct-registry.testnet
