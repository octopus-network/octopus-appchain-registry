use near_sdk::{json_types::U128, serde_json::json};
use workspaces::{result::ExecutionFinalResult, Account, Contract};

pub async fn change_minimum_register_deposit(
    signer: &Account,
    registry: &Contract,
    value: u128,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "change_minimum_register_deposit")
        .args_json(json!({ "value": U128::from(value) }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}
