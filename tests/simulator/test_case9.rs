use crate::{
    common,
    contract_interfaces::{appchain_owner_actions, registry_viewer},
};
use appchain_registry::types::{
    AppchainSortingField, RegistryRoles, RegistrySettings, SortingOrder,
};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::{json_types::U128, serde_json, AccountId};
use std::{collections::HashMap, str::FromStr};

const TOTAL_SUPPLY: u128 = 100_000_000;

#[tokio::test]
async fn test_case9() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) =
        common::basic_actions::initialize_contracts_and_users(&worker, total_supply, true).await?;
    let amount = common::to_oct_amount(1000);
    appchain_owner_actions::register_appchain(
        &worker,
        &users[0],
        &oct_token,
        &registry,
        &"appchain1".to_string(),
        Some("appchain1 description".to_string()),
        Some("http://ddfs.dsdfs".to_string()),
        Some("https://testchain.org/function_spec".to_string()),
        Some("https://jldfs.yoasdfasd".to_string()),
        Some("v1.0.0".to_string()),
        Some("joe@lksdf.com".to_string()),
        Some(AccountId::from_str(users[1].id().as_str()).unwrap()),
        Some(U128::from(10000000)),
        Some(U128::from(10000000)),
        Some(U128::from(1000000)),
        Some(U128::from(100)),
        Some(FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "joeToken".to_string(),
            symbol: "JOT".to_string(),
            icon: Option::None,
            reference: Option::None,
            reference_hash: Option::None,
            decimals: 18,
        }),
        Some(HashMap::from([("key1".to_string(), "value1".to_string())])),
        amount,
    )
    .await
    .expect("Failed in calling 'register_appchain'");
    //
    // perform migration
    //
    common::basic_actions::deploy_new_appchain_registry(&worker, &registry).await?;
    root.call(&worker, registry.id(), "migrate_state")
        .gas(300_000_000_000_000)
        .transact()
        .await?;
    //
    let registry_settings = registry_viewer::get_registry_settings(&worker, &registry).await?;
    println!(
        "Registry settings: {}",
        serde_json::to_string::<RegistrySettings>(&registry_settings).unwrap()
    );
    let registry_roles = registry_viewer::get_registry_roles(&worker, &registry).await?;
    println!(
        "Registry roles: {}",
        serde_json::to_string::<RegistryRoles>(&registry_roles).unwrap()
    );
    assert_eq!(
        registry_viewer::print_appchains(
            &worker,
            &registry,
            Option::None,
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Descending
        )
        .await?,
        1
    );
    Ok(())
}
