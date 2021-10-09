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
        format!("{{\"RegisterAppchain\":{{\"appchain_id\":\"{}\",\"website_url\":\"http://ddfs.dsdfs\",\"github_address\":\"https://jldfs.yoasdfasd\",\"github_release\":\"v1.0.0\",\"commit_id\":\"commit_id\",\"contact_email\":\"joe@lksdf.com\",\"premined_wrapped_appchain_token\":\"10000000\",\"ido_amount_of_wrapped_appchain_token\":\"1000000\",\"initial_era_reward\":\"100\",\"custom_metadata\":{{\"key1\":\"value1\"}}}}}}", appchain_id),
        oct_token)
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
