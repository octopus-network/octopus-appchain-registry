use crate::common;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId,
};
use std::collections::HashMap;
use workspaces::{network::Sandbox, result::CallExecutionDetails, Account, Contract, Worker};

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ParamOfUpdateAppchainCustomMetadata {
    appchain_id: String,
    custom_metadata: HashMap<String, String>,
}

pub async fn register_appchain(
    worker: &Worker<Sandbox>,
    signer: &Account,
    oct_token: &Contract,
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
    amount: u128,
) -> anyhow::Result<CallExecutionDetails> {
    common::call_ft_transfer_call(
        worker,
        signer,
        &registry.as_account(),
        amount,
        json!({
            "RegisterAppchain":{
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
                "custom_metadata": custom_metadata
            }
        })
        .to_string(),
        oct_token,
    ).await
}

pub async fn transfer_appchain_ownership(
    worker: &Worker<Sandbox>,
    signer: &Account,
    registry: &Contract,
    appchain_id: &String,
    new_owner: &Account,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(worker, registry.id(), "transfer_appchain_ownership")
        .args_json(json!({
            "appchain_id": appchain_id,
            "new_owner": new_owner.id()
        }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
}
