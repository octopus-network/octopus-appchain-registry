use near_sdk::{json_types::U128, serde_json::json};
use workspaces::{network::Sandbox, result::CallExecutionDetails, Account, Contract, Worker};

pub async fn change_minimum_register_deposit(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    value: u128,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "change_minimum_register_deposit")
        .args_json(json!({ "value": U128::from(value) }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn change_voting_result_reduction_percent(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    value: u128,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(
            worker,
            registry.id(),
            "change_voting_result_reduction_percent",
        )
        .args_json(json!({ "value": U128::from(value) }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}
