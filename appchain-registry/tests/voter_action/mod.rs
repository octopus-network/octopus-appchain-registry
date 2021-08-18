use near_sdk::serde_json::json;
use near_sdk_sim::{ExecutionResult, UserAccount, DEFAULT_GAS};
use num_format::{Locale, ToFormattedString};

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
    println!(
        "Gas burnt of function 'ft_transfer_call': {}",
        outcome.gas_burnt().to_formatted_string(&Locale::en)
    );
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
    println!(
        "Gas burnt of function 'ft_transfer_call': {}",
        outcome.gas_burnt().to_formatted_string(&Locale::en)
    );
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
    println!(
        "Gas burnt of function 'withdraw_upvote_deposit_of': {}",
        outcome.gas_burnt().to_formatted_string(&Locale::en)
    );
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
    println!(
        "Gas burnt of function 'withdraw_downvote_deposit_of': {}",
        outcome.gas_burnt().to_formatted_string(&Locale::en)
    );
    outcome
}
