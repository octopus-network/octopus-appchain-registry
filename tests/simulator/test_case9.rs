use crate::common;
use appchain_registry::{
    storage_migration::OldRegistrySettings,
    types::{AppchainSortingField, AppchainStatus, RegistryRoles, RegistrySettings, SortingOrder},
};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::{
    json_types::{U128, U64},
    serde_json::{self, json},
    AccountId,
};
use near_units::parse_near;
use std::collections::HashMap;
use workspaces::{network::Sandbox, result::ViewResultDetails, Contract, Worker};

const TOTAL_SUPPLY: u128 = 100_000_000;

#[tokio::test]
async fn test_case9() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) =
        common::basic_actions::initialize_contracts_and_users(&worker, total_supply, true).await?;
    let amount = common::to_oct_amount(1000);
    //
    // Register an appchain
    //
    let fungible_token_metadata = FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "joeToken".to_string(),
        symbol: "JOT".to_string(),
        icon: Option::None,
        reference: Option::None,
        reference_hash: Option::None,
        decimals: 18,
    };
    let result = common::call_ft_transfer_call(
        &worker,
        &users[1],
        &registry.as_account(),
        amount,
        json!({
            "RegisterAppchain":{
                "appchain_id": "appchain1".to_string(),
                "description": "appchain1 description".to_string(),
                "website_url": "http://ddfs.dsdfs".to_string(),
                "function_spec_url": "https://testchain.org/function_spec".to_string(),
                "github_address": "https://jldfs.yoasdfasd".to_string(),
                "github_release": "v1.0.0".to_string(),
                "contact_email": "joe@lksdf.com".to_string(),
                "premined_wrapped_appchain_token_beneficiary": users[1].id(),
                "premined_wrapped_appchain_token": U128::from(10000000),
                "initial_supply_of_wrapped_appchain_token": U128::from(10000000),
                "ido_amount_of_wrapped_appchain_token": U128::from(100000),
                "initial_era_reward": U128::from(100),
                "fungible_token_metadata": fungible_token_metadata,
                "custom_metadata": HashMap::from([("key1".to_string(), "value1".to_string())])
            }
        })
        .to_string(),
        &oct_token,
    )
    .await?;
    println!(
        "Result of calling 'ft_transfer_call' on OCT token account: {:?}",
        result
    );
    println!();
    assert!(result.is_success());
    //
    // Check view functions
    //
    let result = registry.call(&worker, "get_owner_pk").view().await;
    print_view_result_details::<String>("get_owner_pk", &result);
    //
    let result = registry.call(&worker, "get_oct_token").view().await;
    print_view_result_details::<AccountId>("get_oct_token", &result);
    //
    let result = registry.call(&worker, "get_registry_settings").view().await;
    print_view_result_details::<OldRegistrySettings>("get_registry_settings", &result);
    //
    let result = registry.call(&worker, "get_registry_roles").view().await;
    print_view_result_details::<RegistryRoles>("get_registry_roles", &result);
    //
    let result = registry.call(&worker, "get_total_stake").view().await;
    print_view_result_details::<U128>("get_total_stake", &result);
    //
    let result = registry.call(&worker, "get_appchain_ids").view().await;
    print_view_result_details::<Vec<String>>("get_appchain_ids", &result);
    //
    let result = registry
        .call(&worker, "get_appchains_count_of")
        .args_json(json!({ "appchain_state": null }))?
        .view()
        .await;
    print_view_result_details::<U64>("get_appchains_count_of", &result);
    //
    let result = registry
        .call(&worker, "get_appchains_with_state_of")
        .args_json(json!({
            "appchain_state": null,
            "page_number": 1,
            "page_size": 5,
            "sorting_field": AppchainSortingField::RegisteredTime,
            "sorting_order": SortingOrder::Descending,
        }))?
        .view()
        .await;
    print_view_result_details::<Vec<AppchainStatus>>("get_appchains_with_state_of", &result);
    //
    let result = registry
        .call(&worker, "get_appchain_status_of")
        .args_json(json!({
            "appchain_id": "appchain1",
        }))?
        .view()
        .await;
    print_view_result_details::<AppchainStatus>("get_appchain_status_of", &result);
    //
    // perform migration
    //
    root.call(&worker, registry.id(), "store_wasm_of_self")
        .args(std::fs::read(format!("res/appchain_registry.wasm"))?)
        .gas(300_000_000_000_000)
        .deposit(parse_near!("6 N"))
        .transact()
        .await
        .expect("Failed in calling 'store_wasm_of_self'");
    root.call(&worker, registry.id(), "set_owner")
        .args_json(json!({
            "owner": registry.id(),
        }))?
        .gas(200_000_000_000_000)
        .transact()
        .await
        .expect("Failed in calling 'set_owner'");
    let result = registry
        .call(&worker, "update_self")
        .gas(300_000_000_000_000)
        .transact()
        .await?;
    println!("Result of calling 'update_self': {:?}", result);
    println!();
    assert!(result.is_success());
    //
    print_view_function_results(&worker, &registry).await;
    //
    Ok(())
}

async fn print_view_function_results(worker: &Worker<Sandbox>, registry: &Contract) {
    //
    let result = registry.call(&worker, "get_owner_pk").view().await;
    print_view_result_details::<String>("get_owner_pk", &result);
    //
    let result = registry.call(&worker, "get_oct_token").view().await;
    print_view_result_details::<AccountId>("get_oct_token", &result);
    //
    let result = registry.call(&worker, "get_registry_settings").view().await;
    print_view_result_details::<RegistrySettings>("get_registry_settings", &result);
    //
    let result = registry.call(&worker, "get_registry_roles").view().await;
    print_view_result_details::<RegistryRoles>("get_registry_roles", &result);
    //
    let result = registry.call(&worker, "get_total_stake").view().await;
    print_view_result_details::<U128>("get_total_stake", &result);
    //
    let result = registry.call(&worker, "get_appchain_ids").view().await;
    print_view_result_details::<Vec<String>>("get_appchain_ids", &result);
    //
    let result = registry
        .call(&worker, "get_appchains_count_of")
        .args_json(json!({ "appchain_state": null }))
        .unwrap()
        .view()
        .await;
    print_view_result_details::<U64>("get_appchains_count_of", &result);
    //
    let result = registry
        .call(&worker, "get_appchains_with_state_of")
        .args_json(json!({
            "appchain_state": null,
            "page_number": 1,
            "page_size": 5,
            "sorting_field": AppchainSortingField::RegisteredTime,
            "sorting_order": SortingOrder::Descending,
        }))
        .unwrap()
        .view()
        .await;
    print_view_result_details::<Vec<AppchainStatus>>("get_appchains_with_state_of", &result);
    //
    let result = registry
        .call(&worker, "get_appchain_status_of")
        .args_json(json!({
            "appchain_id": "appchain1",
        }))
        .unwrap()
        .view()
        .await;
    print_view_result_details::<AppchainStatus>("get_appchain_status_of", &result);
}

fn print_view_result_details<
    T: near_sdk::serde::Serialize + for<'de> near_sdk::serde::Deserialize<'de>,
>(
    function_name: &str,
    result: &Result<ViewResultDetails, anyhow::Error>,
) {
    match result {
        Ok(result) => println!(
            "{}: {}",
            function_name,
            serde_json::to_string(&result.json::<T>().unwrap()).unwrap()
        ),
        Err(error) => println!("{}: {:?}", function_name, error),
    }
    println!();
}
