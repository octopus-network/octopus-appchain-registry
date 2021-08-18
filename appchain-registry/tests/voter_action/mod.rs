use near_sdk::serde_json::json;
use near_sdk_sim::{ExecutionResult, UserAccount, DEFAULT_GAS};

use crate::common;

pub fn upvote_appchain(
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
            "msg": format!("upvote_appchain,{}", appchain_id)
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1,
    );
    common::print_outcome_result("ft_transfer_call", &outcome);
    outcome
}

pub fn downvote_appchain(
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
            "msg": format!("downvote_appchain,{}", appchain_id)
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1,
    );
    common::print_outcome_result("ft_transfer_call", &outcome);
    outcome
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
