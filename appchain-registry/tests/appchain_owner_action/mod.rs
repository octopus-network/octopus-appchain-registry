use std::collections::HashMap;

use appchain_registry::AppchainRegistryContract;
use mock_oct_token::MockOctTokenContract;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount};

use crate::common;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ParamOfUpdateAppchainCustomMetadata {
    appchain_id: String,
    custom_metadata: HashMap<String, String>,
}

pub fn register_appchain(
    signer: &UserAccount,
    oct_token: &ContractAccount<MockOctTokenContract>,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    amount: u128,
) -> ExecutionResult {
    common::ft_transfer_call_oct_token(
        signer,
        &registry.user_account,
        amount,
        format!("register_appchain,{},website_url_string,github_address_string,github_release_string,commit_id,email_string,\"10000000000000000000000000\",\"1000000000000000000000000\",\"100000000000000000000\"", appchain_id),
        oct_token)
}

pub fn update_appchain_custom_metadata(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    custom_metadata: &HashMap<String, String>,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.update_appchain_custom_metadata(appchain_id.clone(), custom_metadata.clone())
    );
    common::print_outcome_result("update_appchain_custom_metadata", &outcome);
    outcome
}

pub fn transfer_appchain_ownership(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    new_owner: &UserAccount,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.transfer_appchain_ownership(appchain_id.clone(), new_owner.account_id())
    );
    common::print_outcome_result("transfer_appchain_ownership", &outcome);
    outcome
}
