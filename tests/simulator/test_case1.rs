use crate::{
    common,
    contract_interfaces::{
        appchain_lifecycle_manager, appchain_owner_actions, registry_settings, registry_viewer,
        sudo_actions,
    },
};
use appchain_registry::types::{
    AppchainSortingField, AppchainState, AppchainTemplateType, SortingOrder,
};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::{json_types::U128, AccountId};
use std::{collections::HashMap, str::FromStr};

const TOTAL_SUPPLY: u128 = 100_000_000;

/// Test 'register', 'update metadata', 'reject' and 'remove' actions.
#[tokio::test]
async fn test_case1() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) =
        common::basic_actions::initialize_contracts_and_users(&worker, total_supply, false).await?;
    //
    assert_eq!(
        registry_viewer::get_registry_settings(&registry)
            .await?
            .minimum_register_deposit
            .0,
        common::to_oct_amount(1000)
    );
    let amount = common::to_oct_amount(1200);
    assert!(
        registry_settings::change_minimum_register_deposit(&users[0], &registry, amount)
            .await
            .unwrap()
            .is_failure()
    );
    assert!(
        registry_settings::change_minimum_register_deposit(&root, &registry, amount)
            .await
            .unwrap()
            .is_success()
    );
    assert_eq!(
        registry_viewer::get_registry_settings(&registry)
            .await?
            .minimum_register_deposit
            .0,
        common::to_oct_amount(1200)
    );
    //
    assert_eq!(
        common::get_ft_balance_of(&registry.as_account(), &oct_token)
            .await?
            .0,
        0
    );
    //
    assert_eq!(
        common::get_ft_balance_of(&users[0], &oct_token).await?.0,
        total_supply / 10
    );
    //
    let appchain_id = String::from("test_appchain");
    let amount = common::to_oct_amount(1000);
    assert!(appchain_owner_actions::register_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
        Some("appchain1 description".to_string()),
        Some(AppchainTemplateType::Barnacle),
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
    .unwrap()
    .is_success());
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::None,
            1,
            5,
            AppchainSortingField::AppchainId,
            SortingOrder::Ascending
        )
        .await?,
        0
    );
    assert_eq!(
        common::get_ft_balance_of(&users[0], &oct_token).await?.0,
        total_supply / 10
    );
    //
    let amount = common::to_oct_amount(1200);
    assert!(sudo_actions::pause_asset_transfer(&root, &registry)
        .await
        .unwrap()
        .is_success());
    assert!(appchain_owner_actions::register_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
        Some("appchain1 description".to_string()),
        Some(AppchainTemplateType::Barnacle),
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
    .unwrap()
    .is_success());
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::None,
            1,
            5,
            AppchainSortingField::AppchainId,
            SortingOrder::Ascending
        )
        .await?,
        0
    );
    assert_eq!(
        common::get_ft_balance_of(&users[0], &oct_token).await?.0,
        total_supply / 10
    );
    //
    assert!(sudo_actions::resume_asset_transfer(&root, &registry)
        .await
        .unwrap()
        .is_success());
    assert!(appchain_owner_actions::register_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
        Some("appchain1 description".to_string()),
        Some(AppchainTemplateType::Barnacle),
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
    .unwrap()
    .is_success());
    assert_eq!(
        common::get_ft_balance_of(&users[0], &oct_token).await?.0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1200)
    );
    assert_eq!(
        common::get_ft_balance_of(&registry.as_account(), &oct_token)
            .await?
            .0,
        common::to_oct_amount(1200)
    );
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    assert!(appchain_owner_actions::transfer_appchain_ownership(
        &users[1],
        &registry,
        &appchain_id,
        &users[1],
    )
    .await
    .unwrap()
    .is_failure());
    assert!(appchain_owner_actions::transfer_appchain_ownership(
        &users[0],
        &registry,
        &appchain_id,
        &users[1],
    )
    .await
    .unwrap()
    .is_success());
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id).await?;
    assert_eq!(
        &appchain.appchain_owner,
        &AccountId::from_str(users[1].id().as_str()).unwrap()
    );
    //
    let mut custom_metadata: HashMap<String, String> = HashMap::from([
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]);
    //
    custom_metadata.clear();
    custom_metadata.insert("key3".to_string(), "value3".to_string());
    assert!(appchain_lifecycle_manager::update_appchain_metadata(
        &users[0],
        &registry,
        &appchain_id,
        Option::from(String::from("https://oct.network")),
        Option::None,
        Option::None,
        Option::None,
        Option::from(String::from("yangzhen@oct.network")),
        Option::None,
        Option::from(U128::from(10_000_000_000_000_000_000_000_000)),
        Option::from(U128::from(10_000_000_000_000_000_000_000_000)),
        Option::from(U128::from(1_000_000_000_000_000_000_000_000)),
        Option::from(U128::from(100_000_000_000_000_000_000)),
        Option::None,
        Option::from(custom_metadata.clone()),
    )
    .await
    .unwrap()
    .is_failure());
    assert!(appchain_lifecycle_manager::update_appchain_metadata(
        &root,
        &registry,
        &appchain_id,
        Option::from(String::from("https://oct.network")),
        Option::None,
        Option::None,
        Option::None,
        Option::from(String::from("yangzhen@oct.network")),
        Option::None,
        Option::from(U128::from(10_000_000_000_000_000_000_000_000)),
        Option::from(U128::from(10_000_000_000_000_000_000_000_000)),
        Option::from(U128::from(1_000_000_000_000_000_000_000_000)),
        Option::from(U128::from(100_000_000_000_000_000_000)),
        Option::None,
        Option::from(custom_metadata.clone()),
    )
    .await
    .unwrap()
    .is_success());
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id).await?;
    assert!(appchain
        .appchain_metadata
        .website_url
        .eq("https://oct.network"));
    assert!(appchain
        .appchain_metadata
        .contact_email
        .eq("yangzhen@oct.network"));
    assert!(appchain.appchain_metadata.custom_metadata.keys().len() == 1);
    //
    assert!(
        appchain_lifecycle_manager::reject_appchain(&users[4], &registry, &appchain_id)
            .await
            .unwrap()
            .is_failure()
    );
    assert!(
        appchain_lifecycle_manager::reject_appchain(&root, &registry, &appchain_id)
            .await
            .unwrap()
            .is_success()
    );
    assert_eq!(
        common::get_ft_balance_of(&users[1], &oct_token).await?.0,
        common::to_oct_amount(TOTAL_SUPPLY / 10)
    );
    assert_eq!(
        common::get_ft_balance_of(&registry.as_account(), &oct_token)
            .await?
            .0,
        common::to_oct_amount(1200)
    );
    //
    assert!(
        appchain_lifecycle_manager::remove_appchain(&users[2], &registry, &appchain_id)
            .await
            .unwrap()
            .is_failure()
    );
    assert!(
        appchain_lifecycle_manager::remove_appchain(&root, &registry, &appchain_id)
            .await
            .unwrap()
            .is_success()
    );
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::None,
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Descending
        )
        .await?,
        0
    );
    Ok(())
}
