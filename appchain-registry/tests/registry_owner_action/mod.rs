use appchain_registry::AppchainRegistryContract;
use mock_oct_token::MockOctTokenContract;
use std::collections::HashMap;

use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount, DEFAULT_GAS};

use crate::common;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ParamOfUpdateAppchainMetadata {
    appchain_id: String,
    website_url: Option<String>,
    github_address: Option<String>,
    github_release: Option<String>,
    commit_id: Option<String>,
    contact_email: Option<String>,
    custom_metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ParamOfPassAuditingAppchain {
    appchain_id: String,
    appchain_anchor_code: Vec<u8>,
}

pub fn update_appchain_metadata(
    signer: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    website_url: Option<String>,
    github_address: Option<String>,
    github_release: Option<String>,
    commit_id: Option<String>,
    contact_email: Option<String>,
    custom_metadata: Option<HashMap<String, String>>,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "update_appchain_metadata",
        &serde_json::to_string(&ParamOfUpdateAppchainMetadata {
            appchain_id: appchain_id.clone(),
            website_url,
            github_address,
            github_release,
            commit_id,
            contact_email,
            custom_metadata,
        })
        .unwrap()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("update_appchain_metadata", &outcome);
    outcome
}

pub fn change_minimum_register_deposit(
    signer: &UserAccount,
    registry: &UserAccount,
    value: u128,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "change_minimum_register_deposit",
        &json!({
            "value": value.to_string(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("change_minimum_register_deposit", &outcome);
    outcome
}

pub fn start_auditing_appchain(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.start_auditing_appchain(appchain_id.clone())
    );
    common::print_outcome_result("start_auditing_appchain", &outcome);
    outcome
}

pub fn pass_auditing_appchain(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
) -> ExecutionResult {
    let outcome = call!(signer, registry.pass_auditing_appchain(appchain_id.clone()));
    common::print_outcome_result("pass_auditing_appchain", &outcome);
    outcome
}

pub fn reject_appchain(
    signer: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    refund_percent: u8,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "reject_appchain",
        &json!({
            "appchain_id": appchain_id,
            "refund_percent": refund_percent.to_string(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("reject_appchain", &outcome);
    outcome
}

pub fn count_voting_score(signer: &UserAccount, registry: &UserAccount) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "count_voting_score",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("count_voting_score", &outcome);
    outcome
}

pub fn conclude_voting_score(
    signer: &UserAccount,
    registry: &UserAccount,
    voting_result_reduction_percent: u8,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "conclude_voting_score",
        &json!({
            "voting_result_reduction_percent": voting_result_reduction_percent.to_string(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("conclude_voting_score", &outcome);
    outcome
}

pub fn remove_appchain(
    signer: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "remove_appchain",
        &json!({
            "appchain_id": appchain_id,
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("remove_appchain", &outcome);
    outcome
}
