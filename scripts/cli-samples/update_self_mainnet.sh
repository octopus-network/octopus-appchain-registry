#!/bin/bash
set -e
#
export NEAR_ENV=mainnet
#
export REGISTRY_ACCOUNT_ID=octopus-registry.near
#
WASM_BYTES='cat res/appchain_registry.wasm | base64'
near call $REGISTRY_ACCOUNT_ID store_wasm_of_self $(eval "$WASM_BYTES") --base64 --accountId $REGISTRY_ACCOUNT_ID --deposit 5 --gas 200000000000000
near call $REGISTRY_ACCOUNT_ID update_self '' --accountId $REGISTRY_ACCOUNT_ID --gas 200000000000000
