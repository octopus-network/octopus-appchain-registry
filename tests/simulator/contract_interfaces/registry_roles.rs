use near_sdk::serde_json::json;
use workspaces::{network::Sandbox, result::CallExecutionDetails, Account, Contract, Worker};

pub async fn change_appchain_lifecycle_manager(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    account: &Account,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "start_auditing_appchain")
        .args_json(json!({ "account": account.id() }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn change_registry_settings_manager(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    account: &Account,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "change_registry_settings_manager")
        .args_json(json!({ "account": account.id() }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn change_operator_of_counting_voting_score(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    account: &Account,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(
            worker,
            registry.id(),
            "change_operator_of_counting_voting_score",
        )
        .args_json(json!({ "account": account.id() }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}
