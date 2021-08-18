use appchain_registry::AppchainRegistryContract;
use std::collections::HashMap;

use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount};

use crate::common;

pub fn update_appchain_metadata(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    website_url: Option<String>,
    github_address: Option<String>,
    github_release: Option<String>,
    commit_id: Option<String>,
    contact_email: Option<String>,
    custom_metadata: Option<HashMap<String, String>>,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.update_appchain_metadata(
            appchain_id.clone(),
            website_url,
            github_address,
            github_release,
            commit_id,
            contact_email,
            custom_metadata
        )
    );
    common::print_outcome_result("update_appchain_metadata", &outcome);
    outcome
}

pub fn change_minimum_register_deposit(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    value: u128,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.change_minimum_register_deposit(value.into())
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
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    refund_percent: u64,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.reject_appchain(appchain_id.clone(), refund_percent.into())
    );
    common::print_outcome_result("reject_appchain", &outcome);
    outcome
}

pub fn count_voting_score(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
) -> ExecutionResult {
    let outcome = call!(signer, registry.count_voting_score());
    common::print_outcome_result("count_voting_score", &outcome);
    outcome
}

pub fn conclude_voting_score(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    voting_result_reduction_percent: u64,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.conclude_voting_score(voting_result_reduction_percent.into())
    );
    common::print_outcome_result("conclude_voting_score", &outcome);
    outcome
}

pub fn remove_appchain(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
) -> ExecutionResult {
    let outcome = call!(signer, registry.remove_appchain(appchain_id.clone()));
    common::print_outcome_result("remove_appchain", &outcome);
    outcome
}
