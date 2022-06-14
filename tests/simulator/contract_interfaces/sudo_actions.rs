use workspaces::{network::Sandbox, result::CallExecutionDetails, Account, Contract, Worker};

pub async fn pause_asset_transfer(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "pause_asset_transfer")
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn resume_asset_transfer(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "resume_asset_transfer")
        .gas(200_000_000_000_000)
        .transact()
        .await
}
