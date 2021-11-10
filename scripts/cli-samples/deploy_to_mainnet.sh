#
export NEAR_ENV=mainnet
#
near deploy --accountId octopus-registry.near --wasmFile res/appchain_registry.wasm
#
near call octopus-registry.near new '{"oct_token":"f5cfbc74057c610c8ef151a439252680ac68c6dc.factory.bridge.near"}' --accountId octopus-registry.near --gas 200000000000000
#
near call octopus-registry.near change_operator_of_counting_voting_score '{"operator_account":"octopus-counter.near"}' --accountId octopus-registry.near --gas 200000000000000
#
near call octopus-registry.near migrate_state '{}' --accountId octopus-registry.near --gas 200000000000000
#
# view functions
#
near view octopus-registry.near get_appchains_with_state_of '{"appchain_state":null,"page_number":1,"page_size":50,"sorting_field":"AppchainId","sorting_order":"Ascending"}'
#
near view octopus-registry.near get_appchain_status_of '{"appchain_id":"debionetwork"}'
