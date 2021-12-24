use appchain_registry::AppchainRegistryContract;

use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount};

use crate::common;

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
