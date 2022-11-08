use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{json_types::U128, serde_json::json, AccountId};
use std::collections::HashMap;
use workspaces::{result::ExecutionFinalResult, Account, Contract};

pub async fn update_appchain_metadata(
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
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "update_appchain_metadata")
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
        }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn start_voting_appchain(
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "start_voting_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn pass_auditing_appchain(
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "pass_auditing_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn reject_appchain(
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "reject_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}

pub async fn start_staging_appchain(
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    let result = signer
        .call(registry.id(), "start_staging_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))
        .gas(200_000_000_000_000)
        .transact()
        .await
        .unwrap();
    println!("Result of 'start_staging_appchain': {:?}", result);
    Ok(result)
}

pub async fn remove_appchain(
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
) -> Result<ExecutionFinalResult, workspaces::error::Error> {
    signer
        .call(registry.id(), "remove_appchain")
        .args_json(json!({ "appchain_id": appchain_id }))
        .gas(200_000_000_000_000)
        .transact()
        .await
}
