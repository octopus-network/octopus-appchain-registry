#
export NEAR_ENV=mainnet
export REGISTRY_ACCOUNT_ID=octopus-registry.near
export OWNER_ACCOUNT_ID=octopus-registry.near
#
near deploy --accountId $REGISTRY_ACCOUNT_ID --wasmFile res/appchain_registry.wasm
#
near call $REGISTRY_ACCOUNT_ID new '{"oct_token":"f5cfbc74057c610c8ef151a439252680ac68c6dc.factory.bridge.near"}' --accountId $OWNER_ACCOUNT_ID --gas 200000000000000
#
near call $REGISTRY_ACCOUNT_ID change_operator_of_counting_voting_score '{"operator_account":"octopus-counter.near"}' --accountId $OWNER_ACCOUNT_ID --gas 200000000000000
#
near call $REGISTRY_ACCOUNT_ID migrate_state '{}' --accountId $OWNER_ACCOUNT_ID --gas 200000000000000
#
# view functions
#
near view $REGISTRY_ACCOUNT_ID get_appchain_ids '{}'
#
near view $REGISTRY_ACCOUNT_ID get_appchains_with_state_of '{"appchain_state":null,"page_number":1,"page_size":50,"sorting_field":"AppchainId","sorting_order":"Ascending"}'
#
near view $REGISTRY_ACCOUNT_ID get_appchain_status_of '{"appchain_id":"debionetwork"}'
