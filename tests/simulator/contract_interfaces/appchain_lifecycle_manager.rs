use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{json_types::U128, serde_json::json, AccountId};
use std::collections::HashMap;
use workspaces::{network::Sandbox, result::CallExecutionDetails, Account, Contract, Worker};

pub async fn update_appchain_metadata(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
    website_url: Option<String>,
    function_spec_url: Option<String>,
    github_address: Option<String>,
    github_release: Option<String>,
    contact_email: Option<String>,
    premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
    premined_wrapped_appchain_token: Option<U128>,
    initial_supply_of_wrapped_appchain_token: Option<U128>,
    ido_amount_of_wrapped_appchain_token: Option<U128>,
    initial_era_reward: Option<U128>,
    fungible_token_metadata: Option<FungibleTokenMetadata>,
    custom_metadata: Option<HashMap<String, String>>,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "update_appchain_metadata")
        .args_json(json!({
            "appchain_id": appchain_id,
            "website_url": website_url,
            "function_spec_url": function_spec_url,
            "github_address": github_address,
            "github_release": github_release,
            "contact_email": contact_email,
            "premined_wrapped_appchain_token_beneficiary": premined_wrapped_appchain_token_beneficiary,
            "premined_wrapped_appchain_token": premined_wrapped_appchain_token,
            "initial_supply_of_wrapped_appchain_token": initial_supply_of_wrapped_appchain_token,
            "ido_amount_of_wrapped_appchain_token": ido_amount_of_wrapped_appchain_token,
            "initial_era_reward": initial_era_reward,
            "fungible_token_metadata": fungible_token_metadata,
            "custom_metadata": custom_metadata,
        }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn start_auditing_appchain(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "start_auditing_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn pass_auditing_appchain(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "pass_auditing_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn reject_appchain(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "reject_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn count_voting_score(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
) -> anyhow::Result<CallExecutionDetails> {
    let result = signer
        .call(worker, registry.id(), "count_voting_score")
        .gas(200_000_000_000_000)
        .transact()
        .await;
    if result.is_ok() {
        println!("{:?}", result.as_ref().unwrap());
    }
    result
}

pub async fn conclude_voting_score(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "conclude_voting_score")
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn remove_appchain(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "remove_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}
