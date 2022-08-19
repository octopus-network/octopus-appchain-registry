use crate::common;
use near_sdk::{json_types::U128, serde_json::json};
use workspaces::{network::Sandbox, result::CallExecutionDetails, Account, Contract, Worker};

pub async fn upvote_appchain(
    worker: &Worker<Sandbox>,
    signer: &Account,
    oct_token: &Contract,
    registry: &Contract,
    appchain_id: &String,
    amount: u128,
) -> anyhow::Result<CallExecutionDetails> {
    common::call_ft_transfer_call(
        worker,
        signer,
        &registry.as_account(),
        amount,
        json!({
            "UpvoteAppchain":{
                "appchain_id": appchain_id
            }
        })
        .to_string(),
        oct_token,
    )
    .await
}

pub async fn downvote_appchain(
    worker: &Worker<Sandbox>,
    signer: &Account,
    oct_token: &Contract,
    registry: &Contract,
    appchain_id: &String,
    amount: u128,
) -> anyhow::Result<CallExecutionDetails> {
    common::call_ft_transfer_call(
        worker,
        signer,
        &registry.as_account(),
        amount,
        json!({
            "DownvoteAppchain":{
                "appchain_id": appchain_id
            }
        })
        .to_string(),
        oct_token,
    )
    .await
}

pub async fn withdraw_upvote_deposit_of(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
    amount: u128,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "withdraw_upvote_deposit_of")
        .args_json(json!({
            "appchain_id": appchain_id,
            "amount": U128::from(amount)
        }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn withdraw_downvote_deposit_of(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
    amount: u128,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "withdraw_downvote_deposit_of")
        .args_json(json!({
            "appchain_id": appchain_id,
            "amount": U128::from(amount)
        }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}
