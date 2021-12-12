use std::collections::HashMap;

use appchain_registry::types::{AppchainSortingField, AppchainState, SortingOrder};
use near_sdk::json_types::U128;

mod appchain_owner_action;
mod common;
mod oct_token_viewer;
mod registry_owner_action;
mod registry_settings;
mod registry_viewer;
mod voter_action;

const TOTAL_SUPPLY: u128 = 100_000_000;

/// Test 'register', 'update metadata', 'start auditing', 'reject' and 'remove' actions.
#[test]
fn test_case1() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply, false);
    //
    assert_eq!(
        registry_viewer::get_registry_settings(&registry)
            .minimum_register_deposit
            .0,
        common::to_oct_amount(1000)
    );
    let amount = common::to_oct_amount(1200);
    let outcome = registry_settings::change_minimum_register_deposit(&users[0], &registry, amount);
    assert!(!outcome.is_ok());
    let outcome = registry_settings::change_minimum_register_deposit(&root, &registry, amount);
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_registry_settings(&registry)
            .minimum_register_deposit
            .0,
        common::to_oct_amount(1200)
    );
    //
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        0
    );
    //
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[0], &oct_token).0,
        total_supply / 10
    );
    let appchain_id = String::from("test_appchain");
    let amount = common::to_oct_amount(1000);
    let outcome = appchain_owner_action::register_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
        amount,
    );
    outcome.assert_success();
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::None,
            1,
            5,
            AppchainSortingField::AppchainId,
            SortingOrder::Ascending
        ),
        0
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[0], &oct_token).0,
        total_supply / 10
    );
    let amount = common::to_oct_amount(1200);
    let outcome = appchain_owner_action::register_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id,
        amount,
    );
    outcome.assert_success();
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[0], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1200)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        common::to_oct_amount(1200)
    );
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id);
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    let outcome = appchain_owner_action::transfer_appchain_ownership(
        &users[1],
        &registry,
        &appchain_id,
        &users[1],
    );
    assert!(!outcome.is_ok());
    let outcome = appchain_owner_action::transfer_appchain_ownership(
        &users[0],
        &registry,
        &appchain_id,
        &users[1],
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id);
    assert_eq!(&appchain.appchain_owner, &users[1].account_id());
    //
    let mut custom_metadata: HashMap<String, String> = HashMap::new();
    custom_metadata.insert("key1".to_string(), "value1".to_string());
    custom_metadata.insert("key2".to_string(), "value2".to_string());
    //
    custom_metadata.clear();
    custom_metadata.insert("key3".to_string(), "value3".to_string());
    let outcome = registry_owner_action::update_appchain_metadata(
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
        Option::from(U128::from(1_000_000_000_000_000_000_000_000)),
        Option::from(U128::from(100_000_000_000_000_000_000)),
        Option::None,
        Option::from(custom_metadata.clone()),
    );
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::update_appchain_metadata(
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
        Option::from(U128::from(1_000_000_000_000_000_000_000_000)),
        Option::from(U128::from(100_000_000_000_000_000_000)),
        Option::None,
        Option::from(custom_metadata.clone()),
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id);
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
    let outcome =
        registry_owner_action::start_auditing_appchain(&users[1], &registry, &appchain_id);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::start_auditing_appchain(&root, &registry, &appchain_id);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id);
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    let outcome = voter_action::upvote_appchain(
        &users[2],
        &oct_token,
        &registry,
        &appchain_id,
        common::to_oct_amount(300),
    );
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_upvote_deposit_of(&registry, &appchain_id, &users[2]).0,
        0
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[2], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        common::to_oct_amount(1200)
    );
    let outcome = voter_action::downvote_appchain(
        &users[3],
        &oct_token,
        &registry,
        &appchain_id,
        common::to_oct_amount(200),
    );
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_downvote_deposit_of(&registry, &appchain_id, &users[3]).0,
        0
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[3], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        common::to_oct_amount(1200)
    );
    //
    let outcome = registry_owner_action::reject_appchain(&users[4], &registry, &appchain_id);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::reject_appchain(&root, &registry, &appchain_id);
    outcome.assert_success();
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[1], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        common::to_oct_amount(1200)
    );
    //
    let outcome = registry_owner_action::remove_appchain(&users[2], &registry, &appchain_id);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::remove_appchain(&root, &registry, &appchain_id);
    outcome.assert_success();
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::None,
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Descending
        ),
        0
    );
}
