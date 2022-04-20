use appchain_registry::AppchainRegistryContract;

use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount};

use crate::common;

pub fn change_appchain_lifecycle_manager(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    account: &UserAccount,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.change_appchain_lifecycle_manager(account.account_id())
    );
    common::print_outcome_result("change_appchain_lifecycle_manager", &outcome);
    outcome
}

pub fn change_registry_settings_manager(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    account: &UserAccount,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.change_registry_settings_manager(account.account_id())
    );
    common::print_outcome_result("change_registry_settings_manager", &outcome);
    outcome
}

pub fn change_operator_of_counting_voting_score(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    account: &UserAccount,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.change_operator_of_counting_voting_score(account.account_id())
    );
    common::print_outcome_result("change_operator_of_counting_voting_score", &outcome);
    outcome
}
