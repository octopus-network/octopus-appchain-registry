#
export NEAR_ENV=testnet
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
export OWNER_ACCOUNT_ID=test_oct.testnet
#
near deploy --accountId $REGISTRY_ACCOUNT_ID --wasmFile res/appchain_registry.wasm
#
near call $REGISTRY_ACCOUNT_ID migrate_state '' --accountId $OWNER_ACCOUNT_ID --gas 200000000000000
#
# view functions
#
near view $REGISTRY_ACCOUNT_ID get_appchain_ids '{}'
#
near view $REGISTRY_ACCOUNT_ID get_appchains_with_state_of '{"appchain_state":null,"page_number":1,"page_size":50,"sorting_field":"AppchainId","sorting_order":"Ascending"}'
#
near view $REGISTRY_ACCOUNT_ID get_appchain_status_of '{"appchain_id":"easydeal"}'
