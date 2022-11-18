use crate::{
    common,
    contract_interfaces::{
        appchain_lifecycle_manager, appchain_owner_actions, registry_roles, registry_viewer,
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

/// Test 'pass auditing', 'upvote', 'downvote', 'withdraw upvote' and 'withdraw downvote' actions.
#[tokio::test]
async fn test_case2() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) =
        common::basic_actions::initialize_contracts_and_users(&worker, total_supply, false).await?;
    //
    let appchain_id1 = "test_appchain1".to_string();
    let amount = common::to_oct_amount(1000);
    appchain_owner_actions::register_appchain(
        &users[1],
        &oct_token,
        &registry,
        &appchain_id1,
        Some("appchain1 description".to_string()),
        Some(AppchainTemplateType::Barnacle),
        Some("http://ddfs.dsdfs".to_string()),
        Some("https://testchain.org/function_spec".to_string()),
        Some("https://jldfs.yoasdfasd".to_string()),
        Some("v1.await?.0.await?.0".to_string()),
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
    .expect("Failed in calling 'register_appchain'")
    .unwrap();
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id1).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    let appchain_id2 = "test_appchain2".to_string();
    let amount = common::to_oct_amount(1000);
    appchain_owner_actions::register_appchain(
        &users[2],
        &oct_token,
        &registry,
        &appchain_id2,
        Some("appchain2 description".to_string()),
        Some(AppchainTemplateType::Barnacle),
        Some("http://ddfs.dsdfs".to_string()),
        Some("https://testchain.org/function_spec".to_string()),
        Some("https://jldfs.yoasdfasd".to_string()),
        Some("v1.await?.0.await?.0".to_string()),
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
    .expect("Failed in calling 'register_appchain'")
    .unwrap();
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id2).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    let appchain_id3 = "test_appchain3".to_string();
    let amount = common::to_oct_amount(1000);
    appchain_owner_actions::register_appchain(
        &users[3],
        &oct_token,
        &registry,
        &appchain_id3,
        Some("appchain3 description".to_string()),
        Some(AppchainTemplateType::Barnacle),
        Some("http://ddfs.dsdfs".to_string()),
        Some("https://testchain.org/function_spec".to_string()),
        Some("https://jldfs.yoasdfasd".to_string()),
        Some("v1.await?.0.await?.0".to_string()),
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
    .expect("Failed in calling 'register_appchain'")
    .unwrap();
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id3).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    assert_eq!(
        common::get_ft_balance_of(&users[1], &oct_token).await?.0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        common::get_ft_balance_of(&users[2], &oct_token).await?.0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        common::get_ft_balance_of(&users[3], &oct_token).await?.0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        common::get_ft_balance_of(&registry.as_account(), &oct_token)
            .await?
            .0,
        common::to_oct_amount(3000)
    );
    //
    assert!(appchain_lifecycle_manager::pass_auditing_appchain(
        &users[0],
        &registry,
        &appchain_id1
    )
    .await
    .unwrap()
    .is_failure());
    assert!(appchain_lifecycle_manager::pass_auditing_appchain(
        &users[1],
        &registry,
        &appchain_id1
    )
    .await
    .unwrap()
    .is_failure());
    //
    assert!(
        appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id1)
            .await
            .unwrap()
            .is_success()
    );
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id1).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Audited);
    assert!(
        appchain_lifecycle_manager::start_voting_appchain(&root, &registry, &appchain_id1)
            .await
            .unwrap()
            .is_success()
    );
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id1).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Voting);
    //
    assert!(
        appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id2)
            .await
            .unwrap()
            .is_success()
    );
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id2).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Audited);
    assert!(
        appchain_lifecycle_manager::start_voting_appchain(&root, &registry, &appchain_id2)
            .await
            .unwrap()
            .is_success()
    );
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id2).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Voting);
    //
    assert!(
        appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id3)
            .await
            .unwrap()
            .is_success()
    );
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id3).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Audited);
    assert!(
        appchain_lifecycle_manager::start_voting_appchain(&root, &registry, &appchain_id3)
            .await
            .unwrap()
            .is_success()
    );
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id3).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Voting);
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::None,
            1,
            10,
            AppchainSortingField::AppchainId,
            SortingOrder::Descending
        )
        .await?,
        3
    );
    //
    assert!(sudo_actions::pause_asset_transfer(&root, &registry)
        .await
        .unwrap()
        .is_success());
    let appchain1 = registry_viewer::get_appchain_status_of(&registry, &appchain_id1).await?;
    let appchain2 = registry_viewer::get_appchain_status_of(&registry, &appchain_id2).await?;
    assert_eq!(appchain1.upvote_deposit.0, 0);
    assert_eq!(appchain2.downvote_deposit.0, 0);
    assert!(sudo_actions::resume_asset_transfer(&root, &registry)
        .await
        .unwrap()
        .is_success());
    //
    assert!(
        appchain_lifecycle_manager::start_booting_appchain(&root, &registry, &appchain_id3)
            .await
            .unwrap()
            .is_failure()
    );
    assert!(appchain_lifecycle_manager::start_booting_appchain(
        &users[4],
        &registry,
        &appchain_id3
    )
    .await
    .unwrap()
    .is_failure());
    assert!(appchain_lifecycle_manager::start_booting_appchain(
        &users[5],
        &registry,
        &appchain_id3
    )
    .await
    .unwrap()
    .is_failure());
    assert!(
        registry_roles::change_octopus_council(&root, &registry, &users[5])
            .await
            .unwrap()
            .is_success()
    );
    assert!(appchain_lifecycle_manager::start_booting_appchain(
        &users[5],
        &registry,
        &appchain_id3
    )
    .await
    .unwrap()
    .is_success());
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id3).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Booting);
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::Some([AppchainState::Voting].to_vec()),
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Ascending
        )
        .await?,
        2
    );
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::Some([AppchainState::Booting].to_vec()),
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Ascending
        )
        .await?,
        1
    );
    //
    Ok(())
}
