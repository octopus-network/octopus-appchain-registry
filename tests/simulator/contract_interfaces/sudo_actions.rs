use appchain_registry::AppchainRegistryContract;

use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount};

use crate::common;

pub fn pause_asset_transfer(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
) -> ExecutionResult {
    let outcome = call!(signer, registry.pause_asset_transfer());
    common::print_outcome_result("pause_asset_transfer", &outcome);
    outcome
}

pub fn resume_asset_transfer(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
) -> ExecutionResult {
    let outcome = call!(signer, registry.resume_asset_transfer());
    common::print_outcome_result("resume_asset_transfer", &outcome);
    outcome
}
