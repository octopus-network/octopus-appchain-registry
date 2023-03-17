#!/bin/bash
set -e
#
export NEAR_ENV=testnet
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
#
near deploy --accountId $REGISTRY_ACCOUNT_ID --initFunction 'migrate_state' --initArgs '{}' --wasmFile res/appchain_registry.wasm --force
#
# view functions
#
near view $REGISTRY_ACCOUNT_ID version
near view $REGISTRY_ACCOUNT_ID get_owner
near view $REGISTRY_ACCOUNT_ID get_appchain_ids
near view $REGISTRY_ACCOUNT_ID get_registry_roles
near view $REGISTRY_ACCOUNT_ID get_registry_settings
near view $REGISTRY_ACCOUNT_ID get_appchains_with_state_of '{"appchain_state":null,"page_number":1,"page_size":50,"sorting_field":"AppchainId","sorting_order":"Ascending"}'
