use appchain_registry::AppchainRegistryContract;
use mock_oct_token::MockOctTokenContract;
use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount};

use crate::common;

pub fn upvote_appchain(
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
        format!(
            "{{\"UpvoteAppchain\":{{\"appchain_id\":\"{}\"}}}}",
            appchain_id
        ),
        oct_token,
    )
}

pub fn downvote_appchain(
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
        format!(
            "{{\"DownvoteAppchain\":{{\"appchain_id\":\"{}\"}}}}",
            appchain_id
        ),
        oct_token,
    )
}

pub fn withdraw_upvote_deposit_of(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    amount: u128,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.withdraw_upvote_deposit_of(appchain_id.clone(), amount.into())
    );
    common::print_outcome_result("withdraw_upvote_deposit_of", &outcome);
    outcome
}

pub fn withdraw_downvote_deposit_of(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    amount: u128,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.withdraw_downvote_deposit_of(appchain_id.clone(), amount.into())
    );
    common::print_outcome_result("withdraw_downvote_deposit_of", &outcome);
    outcome
}
