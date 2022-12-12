pub mod basic_actions;

use near_sdk::{json_types::U128, serde_json::json};
use workspaces::{result::ExecutionFinalResult, Account, Contract};

pub async fn call_ft_transfer(
    sender: &Account,
    receiver: &Account,
    amount: u128,
    ft_token_contract: &Contract,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    sender
        .call(ft_token_contract.id(), "ft_transfer")
        .args_json(json!({
            "receiver_id": receiver.id(),
            "amount": U128::from(amount),
            "memo": Option::<String>::None,
        }))
        .gas(20_000_000_000_000)
        .deposit(1)
        .transact()
        .await
}

pub async fn call_ft_transfer_call(
    sender: &Account,
    receiver: &Account,
    amount: u128,
    msg: String,
    ft_token_contract: &Contract,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    sender
        .call(ft_token_contract.id(), "ft_transfer_call")
        .args_json(json!({
            "receiver_id": receiver.id(),
            "amount": U128::from(amount),
            "memo": Option::<String>::None,
            "msg": msg.clone(),
        }))
        .gas(200_000_000_000_000)
        .deposit(1)
        .transact()
        .await
}

pub async fn get_ft_balance_of(
    user: &Account,
    ft_contract: &Contract,
) -> Result<U128, workspaces::error::Error> {
    ft_contract
        .call("ft_balance_of")
        .args_json(json!({
            "account_id": user.id()
        }))
        .view()
        .await?
        .json::<U128>()
}

pub fn to_oct_amount(amount: u128) -> u128 {
    let bt_decimals_base = (10 as u128).pow(18);
    amount * bt_decimals_base
}
