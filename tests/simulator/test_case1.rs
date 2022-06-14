use crate::{
    common,
    contract_interfaces::{
        appchain_lifecycle_manager, appchain_owner_actions, registry_settings, registry_viewer,
        sudo_actions, voter_actions,
    },
};
use appchain_registry::types::{AppchainSortingField, AppchainState, SortingOrder};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::{json_types::U128, AccountId};
use std::{collections::HashMap, str::FromStr};

const TOTAL_SUPPLY: u128 = 100_000_000;

/// Test 'register', 'update metadata', 'start auditing', 'reject' and 'remove' actions.
#[tokio::test]
async fn test_case1() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) =
        common::basic_actions::initialize_contracts_and_users(&worker, total_supply, false).await?;
    //
    assert_eq!(
        registry_viewer::get_registry_settings(&worker, &registry)
            .await?
            .minimum_register_deposit
            .0,
        common::to_oct_amount(1000)
    );
    let amount = common::to_oct_amount(1200);
    registry_settings::change_minimum_register_deposit(&worker, &users[0], &registry, amount)
        .await
        .expect_err("Should fail");
    registry_settings::change_minimum_register_deposit(&worker, &root, &registry, amount)
        .await
        .expect("Failed in calling 'change_minimum_register_deposit'");
    assert_eq!(
        registry_viewer::get_registry_settings(&worker, &registry)
            .await?
            .minimum_register_deposit
            .0,
        common::to_oct_amount(1200)
    );
    //
    assert_eq!(
        common::get_ft_balance_of(&worker, &registry.as_account(), &oct_token)
            .await?
            .0,
        0
    );
    //
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[0], &oct_token)
            .await?
            .0,
        total_supply / 10
    );
    //
    let appchain_id = String::from("test_appchain");
    let amount = common::to_oct_amount(1000);
    appchain_owner_actions::register_appchain(
        &worker,
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
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
    assert_eq!(
        registry_viewer::print_appchains(
            &worker,
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
        common::get_ft_balance_of(&worker, &users[0], &oct_token)
            .await?
            .0,
        total_supply / 10
    );
    //
    let amount = common::to_oct_amount(1200);
    sudo_actions::pause_asset_transfer(&worker, &root, &registry)
        .await
        .expect("Failed in calling 'pause_asset_transfer'");
    appchain_owner_actions::register_appchain(
        &worker,
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
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
    assert_eq!(
        registry_viewer::print_appchains(
            &worker,
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
        common::get_ft_balance_of(&worker, &users[0], &oct_token)
            .await?
            .0,
        total_supply / 10
    );
    //
    sudo_actions::resume_asset_transfer(&worker, &root, &registry)
        .await
        .expect("Failed in calling 'resume_asset_transfer'");
    appchain_owner_actions::register_appchain(
        &worker,
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
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
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[0], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1200)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &registry.as_account(), &oct_token)
            .await?
            .0,
        common::to_oct_amount(1200)
    );
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    appchain_owner_actions::transfer_appchain_ownership(
        &worker,
        &users[1],
        &registry,
        &appchain_id,
        &users[1],
    )
    .await
    .expect_err("Should fail");
    appchain_owner_actions::transfer_appchain_ownership(
        &worker,
        &users[0],
        &registry,
        &appchain_id,
        &users[1],
    )
    .await
    .expect("Failed in calling 'transfer_appchain_ownership'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id).await?;
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
    appchain_lifecycle_manager::update_appchain_metadata(
        &worker,
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
    .expect_err("Should fail");
    appchain_lifecycle_manager::update_appchain_metadata(
        &worker,
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
    .expect("Failed in calling 'update_appchain_metadata'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id).await?;
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
    appchain_lifecycle_manager::start_auditing_appchain(
        &worker,
        &users[1],
        &registry,
        &appchain_id,
    )
    .await
    .expect_err("Should fail");
    appchain_lifecycle_manager::start_auditing_appchain(&worker, &root, &registry, &appchain_id)
        .await
        .expect("Failed in calling 'start_auditing_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    voter_actions::upvote_appchain(
        &worker,
        &users[2],
        &oct_token,
        &registry,
        &appchain_id,
        common::to_oct_amount(300),
    )
    .await
    .expect("Failed in calling 'upvote_appchain'");
    assert_eq!(
        registry_viewer::get_upvote_deposit_of(&worker, &registry, &appchain_id, &users[2])
            .await?
            .0,
        0
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[2], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &registry.as_account(), &oct_token)
            .await?
            .0,
        common::to_oct_amount(1200)
    );
    voter_actions::downvote_appchain(
        &worker,
        &users[3],
        &oct_token,
        &registry,
        &appchain_id,
        common::to_oct_amount(200),
    )
    .await
    .expect("Failed in calling 'downvote_appchain'");
    assert_eq!(
        registry_viewer::get_downvote_deposit_of(&worker, &registry, &appchain_id, &users[3])
            .await?
            .0,
        0
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[3], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &registry.as_account(), &oct_token)
            .await?
            .0,
        common::to_oct_amount(1200)
    );
    //
    appchain_lifecycle_manager::reject_appchain(&worker, &users[4], &registry, &appchain_id)
        .await
        .expect_err("Should faile");
    appchain_lifecycle_manager::reject_appchain(&worker, &root, &registry, &appchain_id)
        .await
        .expect("Failed in calling 'reject_appchain'");
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[1], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &registry.as_account(), &oct_token)
            .await?
            .0,
        common::to_oct_amount(1200)
    );
    //
    appchain_lifecycle_manager::remove_appchain(&worker, &users[2], &registry, &appchain_id)
        .await
        .expect_err("Should fail");
    appchain_lifecycle_manager::remove_appchain(&worker, &root, &registry, &appchain_id)
        .await
        .expect("Failed to call 'remove_appchain'");
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
        0
    );
    Ok(())
}
