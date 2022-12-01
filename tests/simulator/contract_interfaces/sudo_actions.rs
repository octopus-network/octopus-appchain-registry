use workspaces::{result::ExecutionFinalResult, Account, Contract};

pub async fn pause_asset_transfer(
    signer: &Account,
    registry: &Contract,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "pause_asset_transfer")
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn resume_asset_transfer(
    signer: &Account,
    registry: &Contract,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "resume_asset_transfer")
        .gas(200_000_000_000_000)
        .transact()
        .await
}
