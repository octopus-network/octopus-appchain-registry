use appchain_registry::AppchainRegistryContract;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{json_types::U128, AccountId, Timestamp};
use std::collections::HashMap;

use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount};

use crate::common;

pub fn update_appchain_metadata(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    website_url: Option<String>,
    function_spec_url: Option<String>,
    github_address: Option<String>,
    github_release: Option<String>,
    commit_id: Option<String>,
    contact_email: Option<String>,
    premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
    premined_wrapped_appchain_token: Option<U128>,
    ido_amount_of_wrapped_appchain_token: Option<U128>,
    initial_era_reward: Option<U128>,
    fungible_token_metadata: Option<FungibleTokenMetadata>,
    custom_metadata: Option<HashMap<String, String>>,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.update_appchain_metadata(
            appchain_id.clone(),
            website_url,
            function_spec_url,
            github_address,
            github_release,
            commit_id,
            contact_email,
            premined_wrapped_appchain_token_beneficiary,
            premined_wrapped_appchain_token,
            ido_amount_of_wrapped_appchain_token,
            initial_era_reward,
            fungible_token_metadata,
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

pub fn change_voting_result_reduction_percent(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    value: u64,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.change_voting_result_reduction_percent(value.into())
    );
    common::print_outcome_result("change_voting_result_reduction_percent", &outcome);
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
) -> ExecutionResult {
    let outcome = call!(signer, registry.reject_appchain(appchain_id.clone()));
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
) -> ExecutionResult {
    let outcome = call!(signer, registry.conclude_voting_score());
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

pub fn stage_code(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    contract_code: Vec<u8>,
    staging_timestamp: Timestamp,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.stage_code(contract_code, staging_timestamp)
    );
    common::print_outcome_result("stage_code", &outcome);
    outcome
}
