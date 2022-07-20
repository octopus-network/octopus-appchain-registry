use crate::{
    common,
    contract_interfaces::{
        appchain_lifecycle_manager, appchain_owner_actions, registry_roles, registry_viewer,
        voter_actions,
    },
};
use appchain_registry::types::{
    AppchainSortingField, AppchainState, AppchainTemplateType, SortingOrder,
};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::{json_types::U128, AccountId};
use std::{collections::HashMap, str::FromStr};

const TOTAL_SUPPLY: u128 = 100_000_000;

#[tokio::test]
async fn test_case3() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) =
        common::basic_actions::initialize_contracts_and_users(&worker, total_supply, false).await?;
    //
    let mut i = 1;
    while i <= 50 {
        let appchain_id = format!("test_appchain{}", i);
        let amount = common::to_oct_amount(1000);
        appchain_owner_actions::register_appchain(
            &worker,
            &users[1],
            &oct_token,
            &registry,
            &appchain_id,
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
            registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id).await?;
        assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
        //
        appchain_lifecycle_manager::start_auditing_appchain(
            &worker,
            &root,
            &registry,
            &appchain_id,
        )
        .await
        .expect("Failed in calling 'start_auditing_appchain'");
        let appchain =
            registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id).await?;
        assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
        appchain_lifecycle_manager::pass_auditing_appchain(&worker, &root, &registry, &appchain_id)
            .await
            .expect("Failed in calling 'pass_auditing_appchain'");
        let appchain =
            registry_viewer::get_appchain_status_of(&worker, &registry, &appchain_id).await?;
        assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
        //
        voter_actions::upvote_appchain(
            &worker,
            &users[0],
            &oct_token,
            &registry,
            &appchain_id,
            common::to_oct_amount(i * 10),
        )
        .await
        .expect("Failed in calling 'upvote_appchain'");
        i += 1;
    }
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &worker,
            &registry,
            Option::None,
            1,
            50,
            AppchainSortingField::AppchainId,
            SortingOrder::Descending
        )
        .await?,
        50
    );
    //
    registry_roles::change_operator_of_counting_voting_score(&worker, &root, &registry, &users[3])
        .await
        .expect("Failed in calling 'change_operator_of_counting_voting_score'");
    //
    appchain_lifecycle_manager::count_voting_score(&worker, &users[3], &registry)
        .await
        .expect("Failed in calling 'count_voting_score'");
    // pass a day for performing next count voting score
    worker.fast_forward(86400).await?;
    //
    appchain_lifecycle_manager::count_voting_score(&worker, &users[3], &registry)
        .await
        .expect("Failed in calling 'count_voting_score'");
    // pass a day for performing next count voting score
    worker.fast_forward(86400).await?;
    //
    appchain_lifecycle_manager::count_voting_score(&worker, &users[3], &registry)
        .await
        .expect("Failed in calling 'count_voting_score'");
    //
    appchain_lifecycle_manager::conclude_voting_score(&worker, &root, &registry)
        .await
        .expect("Failed in calling 'conclude_voting_score'");
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &worker,
            &registry,
            Option::Some([AppchainState::InQueue, AppchainState::Staging].to_vec()),
            1,
            50,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Ascending
        )
        .await?,
        50
    );
    Ok(())
}
