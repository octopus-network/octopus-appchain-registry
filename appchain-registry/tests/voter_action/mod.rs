use appchain_registry::AppchainRegistryContract;
use mock_oct_token::MockOctTokenContract;
use near_sdk::serde_json::json;
use near_sdk_sim::{ContractAccount, ExecutionResult, UserAccount, DEFAULT_GAS};

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
        format!("upvote_appchain,{}", appchain_id),
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
        format!("downvote_appchain,{}", appchain_id),
        oct_token,
    )
}

pub fn withdraw_upvote_deposit_of(
    signer: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    amount: u128,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "withdraw_upvote_deposit_of",
        &json!({
            "appchain_id": appchain_id,
            "amount": amount.to_string(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("withdraw_upvote_deposit_of", &outcome);
    outcome
}

pub fn withdraw_downvote_deposit_of(
    signer: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    amount: u128,
) -> ExecutionResult {
    let outcome = signer.call(
        registry.account_id(),
        "withdraw_downvote_deposit_of",
        &json!({
            "appchain_id": appchain_id,
            "amount": amount.to_string(),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0,
    );
    common::print_outcome_result("withdraw_downvote_deposit_of", &outcome);
    outcome
}
