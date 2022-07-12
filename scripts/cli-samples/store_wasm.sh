#!/bin/bash
set -e
#
export NEAR_ENV=testnet
#
export REGISTRY_ACCOUNT_ID=registry.test_oct.testnet
#
WASM_BYTES='cat res/appchain_registry.wasm | base64'
near call $REGISTRY_ACCOUNT_ID store_wasm_of_self $(eval "$WASM_BYTES") --base64 --accountId $REGISTRY_ACCOUNT_ID --deposit 5 --gas 200000000000000
