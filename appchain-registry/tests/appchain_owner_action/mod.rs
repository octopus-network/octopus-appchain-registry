use std::collections::HashMap;

use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk_sim::{ExecutionResult, UserAccount, DEFAULT_GAS};

use crate::common;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ParamOfUpdateAppchainCustomMetadata {
    appchain_id: String,
    custom_metadata: HashMap<String, String>,
}

pub fn register_appchain(
    signer: &UserAccount,
    oct_token: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    amount: u128,
) -> ExecutionResult {
    let outcome = signer.call(
        oct_token.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": registry.valid_account_id(),
            "amount": amount.to_string(),
            "msg": format!("register_appchain,{},website_url_string,github_address_string,github_release_string,commit_id,email_string", appchain_id)
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1,
    );
    common::print_outcome_result("ft_transfer_call", &outcome);
    outcome
}

pub fn update_appchain_custom_metadata(
    signer: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    custom_metadata: &HashMap<String, String>,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "update_appchain_custom_metadata",
        &serde_json::to_string(&ParamOfUpdateAppchainCustomMetadata {
            appchain_id: appchain_id.clone(),
            custom_metadata: custom_metadata.clone(),
        })
        .unwrap()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("update_appchain_custom_metadata", &outcome);
    outcome
}

pub fn transfer_appchain_ownership(
    signer: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    new_owner: &UserAccount,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "transfer_appchain_ownership",
        &json!({
            "appchain_id": appchain_id,
            "new_owner": new_owner.account_id(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("transfer_appchain_ownership", &outcome);
    outcome
}