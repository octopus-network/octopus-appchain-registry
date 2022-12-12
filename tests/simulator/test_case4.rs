use crate::{
    common,
    contract_interfaces::{
        appchain_lifecycle_manager, appchain_owner_actions, registry_roles, registry_viewer,
    },
};
use appchain_anchor::types::ProtocolSettings;
use appchain_registry::types::{
    AppchainSortingField, AppchainState, AppchainTemplateType, SortingOrder,
};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::{json_types::U128, serde_json::json, AccountId};
use std::{collections::HashMap, str::FromStr};
use workspaces::Account;

const TOTAL_SUPPLY: u128 = 100_000_000;

#[tokio::test]
async fn test_case4() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, council, users) =
        common::basic_actions::initialize_contracts_and_users(&worker, total_supply, false).await?;
    //
    let appchain_id = String::from("appchain1");
    let amount = common::to_oct_amount(1000);
    assert!(appchain_owner_actions::register_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
        Some("appchain1 description".to_string()),
        Some(AppchainTemplateType::Barnacle),
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
        1
    );
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
    assert!(
        registry_roles::change_octopus_council(&root, &registry, &council)
            .await
            .unwrap()
            .is_success()
    );
    assert!(
        appchain_lifecycle_manager::start_booting_appchain(&council, &registry, &appchain_id)
            .await
            .unwrap()
            .is_success()
    );
    let appchain = registry_viewer::get_appchain_status_of(&registry, &appchain_id).await?;
    assert_eq!(&appchain.appchain_state, &AppchainState::Booting);
    //
    let anchor_account_id =
        workspaces::AccountId::try_from(format!("appchain1.{}", registry.id())).unwrap();
    let anchor = Account::from_secret_key(anchor_account_id, root.secret_key().clone(), &worker);
    let anchor = anchor
        .deploy(&std::fs::read(format!("res/appchain_anchor_v2.5.0.wasm"))?)
        .await
        .expect("Failed to deploy anchor contract.")
        .unwrap();
    assert!(anchor
        .call("new")
        .args_json(json!({
            "appchain_id": "appchain1",
            "appchain_template_type": AppchainTemplateType::Barnacle,
            "appchain_registry": registry.id(),
            "oct_token": oct_token.id(),
        }))
        .gas(200_000_000_000_000)
        .transact()
        .await
        .expect("Failed to call function 'new' of anchor contract.")
        .is_success());
    //
    let new_protocol_settings = anchor
        .call("get_protocol_settings")
        .view()
        .await
        .expect("Failed in calling 'get_registry_settings'")
        .json::<ProtocolSettings>()
        .unwrap();
    assert!(new_protocol_settings.minimum_validator_deposit.0 == 5000000000000000000000 as u128);
    let result = council
        .call(registry.id(), "call_anchor_function")
        .args_json(json!({
            "appchain_id": "appchain1",
            "function_name": "change_minimum_validator_deposit",
            "args": "{
                \"value\": \"6000000000000000000000\"
            }"
        }))
        .gas(200_000_000_000_000)
        .transact()
        .await
        .expect("Failed to call function 'call_anchor_function'.");
    println!("Result of calling 'call_anchor_function': {:?}", result);
    let new_protocol_settings = anchor
        .call("get_protocol_settings")
        .view()
        .await
        .expect("Failed in calling 'get_registry_settings'")
        .json::<ProtocolSettings>()
        .unwrap();
    assert!(new_protocol_settings.minimum_validator_deposit.0 == 6000000000000000000000 as u128);
    //
    Ok(())
}
