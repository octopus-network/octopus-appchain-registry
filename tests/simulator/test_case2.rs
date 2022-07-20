use crate::{
    common,
    contract_interfaces::{
        appchain_lifecycle_manager, appchain_owner_actions, registry_roles, registry_settings,
        registry_viewer, sudo_actions, voter_actions,
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
        &worker,
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
    .expect("Failed in calling 'register_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    let appchain_id2 = "test_appchain2".to_string();
    let amount = common::to_oct_amount(1000);
    appchain_owner_actions::register_appchain(
        &worker,
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
    .expect("Failed in calling 'register_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    let appchain_id3 = "test_appchain3".to_string();
    let amount = common::to_oct_amount(1000);
    appchain_owner_actions::register_appchain(
        &worker,
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
    .expect("Failed in calling 'register_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id3).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[1], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[2], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[3], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &registry.as_account(), &oct_token)
            .await?
            .0,
        common::to_oct_amount(3000)
    );
    //
    appchain_lifecycle_manager::pass_auditing_appchain(&worker, &root, &registry, &appchain_id1)
        .await
        .expect_err("Should fail");
    appchain_lifecycle_manager::pass_auditing_appchain(
        &worker,
        &users[0],
        &registry,
        &appchain_id1,
    )
    .await
    .expect_err("Should fail");
    appchain_lifecycle_manager::pass_auditing_appchain(
        &worker,
        &users[1],
        &registry,
        &appchain_id1,
    )
    .await
    .expect_err("Should fail");
    //
    appchain_lifecycle_manager::start_auditing_appchain(&worker, &root, &registry, &appchain_id1)
        .await
        .expect("Failed in calling 'start_auditing_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    appchain_lifecycle_manager::pass_auditing_appchain(&worker, &root, &registry, &appchain_id1)
        .await
        .expect("Failed in calling 'pass_auditing_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
    //
    appchain_lifecycle_manager::start_auditing_appchain(&worker, &root, &registry, &appchain_id2)
        .await
        .expect("Failed in calling 'start_auditing_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    appchain_lifecycle_manager::pass_auditing_appchain(&worker, &root, &registry, &appchain_id2)
        .await
        .expect("Failed in calling 'start_auditing_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
    //
    appchain_lifecycle_manager::start_auditing_appchain(&worker, &root, &registry, &appchain_id3)
        .await
        .expect("Failed in calling 'start_auditing_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id3).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    appchain_lifecycle_manager::pass_auditing_appchain(&worker, &root, &registry, &appchain_id3)
        .await
        .expect("Failed in calling 'pass_auditing_appchain'");
    let appchain =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id3).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &worker,
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
    sudo_actions::pause_asset_transfer(&worker, &root, &registry)
        .await
        .expect("Failed in calling 'pause_asset_transfer'");
    voter_actions::upvote_appchain(
        &worker,
        &users[0],
        &oct_token,
        &registry,
        &appchain_id1,
        common::to_oct_amount(1000),
    )
    .await
    .expect("Failed in calling 'upvote_appchain'");
    voter_actions::downvote_appchain(
        &worker,
        &users[0],
        &oct_token,
        &registry,
        &appchain_id2,
        common::to_oct_amount(1500),
    )
    .await
    .expect("Failed in calling 'downvote_appchain'");
    let appchain1 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    let appchain2 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(appchain1.upvote_deposit.0, 0);
    assert_eq!(appchain2.downvote_deposit.0, 0);
    sudo_actions::resume_asset_transfer(&worker, &root, &registry)
        .await
        .expect("Failed in calling 'resume_asset_transfer'");
    //
    voter_actions::upvote_appchain(
        &worker,
        &users[0],
        &oct_token,
        &registry,
        &appchain_id1,
        common::to_oct_amount(1000),
    )
    .await
    .expect("Failed in calling 'upvote_appchain'");
    voter_actions::downvote_appchain(
        &worker,
        &users[0],
        &oct_token,
        &registry,
        &appchain_id2,
        common::to_oct_amount(1500),
    )
    .await
    .expect("Failed in calling 'downvote_appchain'");
    voter_actions::upvote_appchain(
        &worker,
        &users[4],
        &oct_token,
        &registry,
        &appchain_id2,
        common::to_oct_amount(2000),
    )
    .await
    .expect("Failed in calling 'upvote_appchain'");
    voter_actions::downvote_appchain(
        &worker,
        &users[4],
        &oct_token,
        &registry,
        &appchain_id3,
        common::to_oct_amount(800),
    )
    .await
    .expect("Failed in calling 'downvote_appchain'");
    let appchain1 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    assert_eq!(appchain1.upvote_deposit.0, common::to_oct_amount(1000));
    assert_eq!(appchain1.downvote_deposit.0, 0);
    let appchain2 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(appchain2.upvote_deposit.0, common::to_oct_amount(2000));
    assert_eq!(appchain2.downvote_deposit.0, common::to_oct_amount(1500));
    let appchain3 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id3).await?;
    assert_eq!(appchain3.upvote_deposit.0, 0);
    assert_eq!(appchain3.downvote_deposit.0, common::to_oct_amount(800));
    //
    appchain_lifecycle_manager::count_voting_score(&worker, &users[1], &registry)
        .await
        .expect_err("Should fail");
    appchain_lifecycle_manager::count_voting_score(&worker, &root, &registry)
        .await
        .expect_err("Should fail");
    registry_roles::change_operator_of_counting_voting_score(&worker, &root, &registry, &users[4])
        .await
        .expect("Failed in calling 'change_operator_of_counting_voting_score'");
    appchain_lifecycle_manager::count_voting_score(&worker, &users[4], &registry)
        .await
        .expect("Failed in calling 'count_voting_score'");
    let appchain1 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    assert_eq!(
        appchain1.voting_score.0,
        common::to_oct_amount(1000) as i128
    );
    let appchain2 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(appchain2.voting_score.0, common::to_oct_amount(500) as i128);
    let appchain3 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id3).await?;
    assert_eq!(
        appchain3.voting_score.0,
        0 - common::to_oct_amount(800) as i128
    );
    //
    voter_actions::withdraw_upvote_deposit_of(
        &worker,
        &users[0],
        &registry,
        &appchain_id1,
        common::to_oct_amount(1000) + 1,
    )
    .await
    .expect_err("Should fail");
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[0], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 2500)
    );
    voter_actions::withdraw_downvote_deposit_of(
        &worker,
        &users[4],
        &registry,
        &appchain_id3,
        common::to_oct_amount(800) + 1,
    )
    .await
    .expect_err("Should fail");
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[4], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 2800)
    );
    //
    sudo_actions::pause_asset_transfer(&worker, &root, &registry)
        .await
        .expect("Failed in calling 'pause_asset_transfer'");
    voter_actions::withdraw_upvote_deposit_of(
        &worker,
        &users[0],
        &registry,
        &appchain_id1,
        common::to_oct_amount(550),
    )
    .await
    .expect_err("Should fail");
    voter_actions::withdraw_downvote_deposit_of(
        &worker,
        &users[4],
        &registry,
        &appchain_id3,
        common::to_oct_amount(450),
    )
    .await
    .expect_err("Should fail");
    sudo_actions::resume_asset_transfer(&worker, &root, &registry)
        .await
        .expect("Failed in calling 'resume_asset_transfer'");
    //
    voter_actions::withdraw_upvote_deposit_of(
        &worker,
        &users[0],
        &registry,
        &appchain_id1,
        common::to_oct_amount(550),
    )
    .await
    .expect("Failed in calling 'withdraw_upvote_deposit_of'");
    assert_eq!(
        registry_viewer::get_upvote_deposit_of(&worker, &registry, &appchain_id1, &users[0])
            .await?
            .0,
        common::to_oct_amount(450)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[0], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1950)
    );
    voter_actions::withdraw_downvote_deposit_of(
        &worker,
        &users[4],
        &registry,
        &appchain_id3,
        common::to_oct_amount(450),
    )
    .await
    .expect("Failed in calling 'withdraw_downvote_deposit_of'");
    assert_eq!(
        registry_viewer::get_downvote_deposit_of(&worker, &registry, &appchain_id3, &users[4])
            .await?
            .0,
        common::to_oct_amount(350)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[4], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 2350)
    );
    //
    appchain_lifecycle_manager::count_voting_score(&worker, &users[2], &registry)
        .await
        .expect_err("Should fail");
    // pass a day for performing next count voting score
    worker.fast_forward(86400).await?;
    appchain_lifecycle_manager::count_voting_score(&worker, &users[4], &registry)
        .await
        .expect("Failed in calling 'count_voting_score'");
    let appchain1 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    assert_eq!(
        appchain1.voting_score.0,
        common::to_oct_amount(1450) as i128
    );
    let appchain2 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(
        appchain2.voting_score.0,
        common::to_oct_amount(1000) as i128
    );
    let appchain3 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id3).await?;
    assert_eq!(
        appchain3.voting_score.0,
        0 - common::to_oct_amount(1150) as i128
    );
    //
    voter_actions::withdraw_downvote_deposit_of(
        &worker,
        &users[0],
        &registry,
        &appchain_id2,
        common::to_oct_amount(550),
    )
    .await
    .expect("Failed in calling 'withdraw_downvote_deposit_of'");
    assert_eq!(
        registry_viewer::get_downvote_deposit_of(&worker, &registry, &appchain_id2, &users[0])
            .await?
            .0,
        common::to_oct_amount(950)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[0], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1400)
    );
    voter_actions::withdraw_upvote_deposit_of(
        &worker,
        &users[4],
        &registry,
        &appchain_id2,
        common::to_oct_amount(50),
    )
    .await
    .expect("Failed in calling 'withdraw_upvote_deposit_of'");
    assert_eq!(
        registry_viewer::get_upvote_deposit_of(&worker, &registry, &appchain_id2, &users[4])
            .await?
            .0,
        common::to_oct_amount(1950)
    );
    assert_eq!(
        common::get_ft_balance_of(&worker, &users[4], &oct_token)
            .await?
            .0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 2300)
    );
    //
    appchain_lifecycle_manager::count_voting_score(&worker, &users[3], &registry)
        .await
        .expect_err("Should fail");
    // pass a day for performing next count voting score
    worker.fast_forward(86400).await?;
    appchain_lifecycle_manager::count_voting_score(&worker, &users[4], &registry)
        .await
        .expect("Failed in calling 'count_voting_score'");
    let appchain1 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    assert_eq!(
        appchain1.voting_score.0,
        common::to_oct_amount(1900) as i128
    );
    let appchain2 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(
        appchain2.voting_score.0,
        common::to_oct_amount(2000) as i128
    );
    let appchain3 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id3).await?;
    assert_eq!(
        appchain3.voting_score.0,
        0 - common::to_oct_amount(1500) as i128
    );
    //
    registry_settings::change_voting_result_reduction_percent(&worker, &users[4], &registry, 60)
        .await
        .expect_err("Should fail");
    registry_settings::change_voting_result_reduction_percent(&worker, &root, &registry, 101)
        .await
        .expect_err("Should fail");
    registry_settings::change_voting_result_reduction_percent(&worker, &root, &registry, 60)
        .await
        .expect("Failed in calling 'change_voting_result_reduction_percent'");
    //
    appchain_lifecycle_manager::conclude_voting_score(&worker, &users[0], &registry)
        .await
        .expect_err("Should fail");
    appchain_lifecycle_manager::conclude_voting_score(&worker, &root, &registry)
        .await
        .expect("Failed in calling 'conclude_voting_score'");
    let appchain1 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id1).await?;
    assert_eq!(appchain1.voting_score.0, common::to_oct_amount(760) as i128);
    let appchain2 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id2).await?;
    assert_eq!(&appchain2.appchain_state, &AppchainState::Staging);
    assert_eq!(
        appchain2.voting_score.0,
        common::to_oct_amount(2000) as i128
    );
    let appchain3 =
        registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id3).await?;
    assert_eq!(appchain3.appchain_state, AppchainState::Dead);
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &worker,
            &registry,
            Option::Some([AppchainState::InQueue, AppchainState::Staging].to_vec()),
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Ascending
        )
        .await?,
        2
    );
    Ok(())
}
