use crate::{
    common,
    contract_interfaces::{appchain_lifecycle_manager, appchain_owner_actions, registry_viewer},
};
use appchain_registry::types::{
    AppchainSortingField, AppchainState, SubstrateTemplateType, SortingOrder,
};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::{json_types::U128, AccountId};
use std::{collections::HashMap, str::FromStr};

const TOTAL_SUPPLY: u128 = 100_000_000;

#[tokio::test]
async fn test_case3() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, _council, users) =
        common::basic_actions::initialize_contracts_and_users(&worker, total_supply, false).await?;
    //
    let mut i = 1;
    while i <= 50 {
        let appchain_id = format!("test_appchain{}", i);
        let amount = common::to_oct_amount(1000);
        assert!(appchain_owner_actions::register_appchain(
            &users[1],
            &oct_token,
            &registry,
            &appchain_id,
            Some("appchain1 description".to_string()),
            Some(SubstrateTemplateType::Barnacle),
            Some("http://ddfs.dsdfs".to_string()),
            Some("https://jldfs.yoasdfasd".to_string()),
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
        let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id).await?;
        assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
        //
        assert!(
            appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id)
                .await
                .unwrap()
                .is_success()
        );
        let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id).await?;
        assert_eq!(&appchain.appchain_state, &AppchainState::Audited);
        assert!(
            appchain_lifecycle_manager::start_voting_appchain(&root, &registry, &appchain_id)
                .await
                .unwrap()
                .is_success()
        );
        let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id).await?;
        assert_eq!(&appchain.appchain_state, &AppchainState::Voting);
        i += 1;
    }
    //
    assert_eq!(
        registry_viewer::print_appchains(
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
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::Some([AppchainState::Voting, AppchainState::Booting].to_vec()),
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
