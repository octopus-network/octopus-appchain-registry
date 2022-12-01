use near_sdk::serde_json::json;
use workspaces::{result::ExecutionFinalResult, Account, Contract};

pub async fn change_appchain_lifecycle_manager(
    signer: &Account,
    registry: &Contract,
    account: &Account,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "start_auditing_appchain")
        .args_json(json!({ "account": account.id() }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn change_registry_settings_manager(
    signer: &Account,
    registry: &Contract,
    account: &Account,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "change_registry_settings_manager")
        .args_json(json!({ "account": account.id() }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn change_octopus_council(
    signer: &Account,
    registry: &Contract,
    account: &Account,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "change_octopus_council")
        .args_json(json!({ "account": account.id() }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}
