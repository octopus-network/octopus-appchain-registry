use crate::*;
use near_sdk::json_types::Base58CryptoHash;
use near_sdk::{env, Gas};

const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = Gas(15_000_000_000_000);

/// Stores attached data into blob store and returns hash of it.
/// Implemented to avoid loading the data into WASM for optimal gas usage.
#[no_mangle]
pub extern "C" fn store_wasm_of_self() {
    env::setup_panic_hook();
    let contract: AppchainRegistry = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
    contract.assert_owner();
    let input = env::input().expect("ERR_NO_INPUT");
    let sha256_hash = env::sha256(&input);

    let blob_len = input.len();
    let storage_cost = ((blob_len + 32) as u128) * env::storage_byte_cost();
    assert!(
        env::attached_deposit() >= storage_cost,
        "ERR_NOT_ENOUGH_DEPOSIT:{}",
        storage_cost
    );

    env::storage_write(&StorageKey::RegistryContractWasm.into_bytes(), &input);
    let mut blob_hash = [0u8; 32];
    blob_hash.copy_from_slice(&sha256_hash);
    let blob_hash_str = near_sdk::serde_json::to_string(&Base58CryptoHash::from(blob_hash))
        .unwrap()
        .into_bytes();

    env::value_return(&blob_hash_str);
}

#[no_mangle]
pub fn update_self() {
    env::setup_panic_hook();
    let contract: AppchainRegistry = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
    contract.assert_owner();
    let current_id = env::current_account_id();
    let input = env::storage_read(&StorageKey::RegistryContractWasm.into_bytes())
        .expect("Wasm file for deployment is not staged yet.");
    let promise_id = env::promise_batch_create(&current_id);
    env::promise_batch_action_deploy_contract(promise_id, &input);
    env::promise_batch_action_function_call(
        promise_id,
        "migrate_state",
        &[],
        0,
        env::prepaid_gas() - env::used_gas() - GAS_FOR_UPGRADE_SELF_DEPLOY,
    );
}
