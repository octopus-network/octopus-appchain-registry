use std::collections::HashMap;

use appchain_registry::types::{AppchainSortingField, AppchainState, SortingOrder};
use near_sdk_sim::lazy_static_include;

mod appchain_owner_action;
mod common;
mod oct_token_viewer;
mod registry_owner_action;
mod registry_viewer;
mod voter_action;

lazy_static_include::lazy_static_include_bytes! {
    TOKEN_WASM_BYTES => "../res/mock_oct_token.wasm",
    REGISTRY_WASM_BYTES => "../res/appchain_registry.wasm",
    PREVIOUS_REGISTRY_WASM_BYTES => "../res/previous_appchain_registry.wasm",
    ANCHOR_WASM_BYTES => "../res/mock_appchain_anchor.wasm",
}

const TOTAL_SUPPLY: u128 = 100_000_000;

/// Test 'register', 'update metadata', 'start auditing', 'reject' and 'remove' actions.
#[test]
fn test_case1() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply);
    //
    assert_eq!(
        registry_viewer::get_minimum_register_deposit(&registry).0,
        common::to_oct_amount(100)
    );
    let amount = common::to_oct_amount(120);
    let outcome =
        registry_owner_action::change_minimum_register_deposit(&users[0], &registry, amount);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::change_minimum_register_deposit(&root, &registry, amount);
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_minimum_register_deposit(&registry).0,
        common::to_oct_amount(120)
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
    let amount = common::to_oct_amount(100);
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
    let amount = common::to_oct_amount(120);
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
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 120)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        common::to_oct_amount(120)
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
    let outcome = appchain_owner_action::update_appchain_custom_metadata(
        &users[0],
        &registry,
        &appchain_id,
        &custom_metadata,
    );
    assert!(!outcome.is_ok());
    let outcome = appchain_owner_action::update_appchain_custom_metadata(
        &users[1],
        &registry,
        &appchain_id,
        &custom_metadata,
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id);
    assert_eq!(appchain.appchain_metadata.custom_metadata.keys().len(), 2);
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
        common::to_oct_amount(120)
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
        common::to_oct_amount(120)
    );
    //
    let outcome = registry_owner_action::reject_appchain(&users[4], &registry, &appchain_id, 100);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::reject_appchain(&root, &registry, &appchain_id, 100);
    outcome.assert_success();
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[1], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 + 120)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        0
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

/// Test 'pass auditing', 'upvote', 'downvote', 'withdraw upvote' and 'withdraw downvote' actions.
#[test]
fn test_case2() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply);
    //
    let appchain_id1 = "test_appchain1".to_string();
    let amount = common::to_oct_amount(100);
    let outcome = appchain_owner_action::register_appchain(
        &users[1],
        &oct_token,
        &registry,
        &appchain_id1,
        amount,
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    let appchain_id2 = "test_appchain2".to_string();
    let amount = common::to_oct_amount(100);
    let outcome = appchain_owner_action::register_appchain(
        &users[2],
        &oct_token,
        &registry,
        &appchain_id2,
        amount,
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    let appchain_id3 = "test_appchain3".to_string();
    let amount = common::to_oct_amount(100);
    let outcome = appchain_owner_action::register_appchain(
        &users[3],
        &oct_token,
        &registry,
        &appchain_id3,
        amount,
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[1], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 100)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[2], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 100)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[3], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 100)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        common::to_oct_amount(300)
    );
    //
    let outcome = registry_owner_action::pass_auditing_appchain(
        &root,
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::pass_auditing_appchain(
        &users[0],
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::pass_auditing_appchain(
        &users[1],
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    assert!(!outcome.is_ok());
    //
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &root,
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &users[0],
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &users[1],
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    assert!(!outcome.is_ok());
    //
    let outcome = registry_owner_action::start_auditing_appchain(&root, &registry, &appchain_id1);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    let outcome = registry_owner_action::pass_auditing_appchain(
        &root,
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    outcome.assert_success();
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &root,
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
    //
    let outcome = registry_owner_action::start_auditing_appchain(&root, &registry, &appchain_id2);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    let outcome = registry_owner_action::pass_auditing_appchain(
        &root,
        &registry,
        &appchain_id2,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    outcome.assert_success();
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &root,
        &registry,
        &appchain_id2,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
    //
    let outcome = registry_owner_action::start_auditing_appchain(&root, &registry, &appchain_id3);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    let outcome = registry_owner_action::pass_auditing_appchain(
        &root,
        &registry,
        &appchain_id3,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    outcome.assert_success();
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &root,
        &registry,
        &appchain_id3,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::None,
            1,
            10,
            AppchainSortingField::AppchainId,
            SortingOrder::Descending
        ),
        3
    );
    //
    let outcome = voter_action::upvote_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id1,
        common::to_oct_amount(1000),
    );
    outcome.assert_success();
    let outcome = voter_action::downvote_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id2,
        common::to_oct_amount(1500),
    );
    outcome.assert_success();
    let outcome = voter_action::upvote_appchain(
        &users[4],
        &oct_token,
        &registry,
        &appchain_id2,
        common::to_oct_amount(2000),
    );
    outcome.assert_success();
    let outcome = voter_action::downvote_appchain(
        &users[4],
        &oct_token,
        &registry,
        &appchain_id3,
        common::to_oct_amount(800),
    );
    outcome.assert_success();
    let appchain1 = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(appchain1.upvote_deposit.0, common::to_oct_amount(1000));
    assert_eq!(appchain1.downvote_deposit.0, 0);
    let appchain2 = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(appchain2.upvote_deposit.0, common::to_oct_amount(2000));
    assert_eq!(appchain2.downvote_deposit.0, common::to_oct_amount(1500));
    let appchain3 = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(appchain3.upvote_deposit.0, 0);
    assert_eq!(appchain3.downvote_deposit.0, common::to_oct_amount(800));
    //
    let outcome = registry_owner_action::count_voting_score(&users[1], &registry);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::count_voting_score(&root, &registry);
    outcome.assert_success();
    let appchain1 = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(
        appchain1.voting_score.0,
        common::to_oct_amount(1000) as i128
    );
    let appchain2 = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(appchain2.voting_score.0, common::to_oct_amount(500) as i128);
    let appchain3 = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(
        appchain3.voting_score.0,
        0 - common::to_oct_amount(800) as i128
    );
    //
    let outcome = voter_action::withdraw_upvote_deposit_of(
        &users[0],
        &registry,
        &appchain_id1,
        common::to_oct_amount(1000) + 1,
    );
    assert!(!outcome.is_ok());
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[0], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 2500)
    );
    let outcome = voter_action::withdraw_downvote_deposit_of(
        &users[4],
        &registry,
        &appchain_id3,
        common::to_oct_amount(800) + 1,
    );
    assert!(!outcome.is_ok());
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[4], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 2800)
    );
    //
    let outcome = voter_action::withdraw_upvote_deposit_of(
        &users[0],
        &registry,
        &appchain_id1,
        common::to_oct_amount(550),
    );
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_upvote_deposit_of(&registry, &appchain_id1, &users[0]).0,
        common::to_oct_amount(450)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[0], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1950)
    );
    let outcome = voter_action::withdraw_downvote_deposit_of(
        &users[4],
        &registry,
        &appchain_id3,
        common::to_oct_amount(450),
    );
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_downvote_deposit_of(&registry, &appchain_id3, &users[4]).0,
        common::to_oct_amount(350)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[4], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 2350)
    );
    //
    let outcome = registry_owner_action::count_voting_score(&users[2], &registry);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::count_voting_score(&root, &registry);
    outcome.assert_success();
    let appchain1 = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(
        appchain1.voting_score.0,
        common::to_oct_amount(1450) as i128
    );
    let appchain2 = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(
        appchain2.voting_score.0,
        common::to_oct_amount(1000) as i128
    );
    let appchain3 = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(
        appchain3.voting_score.0,
        0 - common::to_oct_amount(1150) as i128
    );
    //
    let outcome = voter_action::withdraw_downvote_deposit_of(
        &users[0],
        &registry,
        &appchain_id2,
        common::to_oct_amount(550),
    );
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_downvote_deposit_of(&registry, &appchain_id2, &users[0]).0,
        common::to_oct_amount(950)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[0], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1400)
    );
    let outcome = voter_action::withdraw_upvote_deposit_of(
        &users[4],
        &registry,
        &appchain_id2,
        common::to_oct_amount(50),
    );
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_upvote_deposit_of(&registry, &appchain_id2, &users[4]).0,
        common::to_oct_amount(1950)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[4], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 2300)
    );
    //
    let outcome = registry_owner_action::count_voting_score(&users[3], &registry);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::count_voting_score(&root, &registry);
    outcome.assert_success();
    let appchain1 = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(
        appchain1.voting_score.0,
        common::to_oct_amount(1900) as i128
    );
    let appchain2 = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(
        appchain2.voting_score.0,
        common::to_oct_amount(2000) as i128
    );
    let appchain3 = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(
        appchain3.voting_score.0,
        0 - common::to_oct_amount(1500) as i128
    );
    //
    let outcome =
        registry_owner_action::change_voting_result_reduction_percent(&users[4], &registry, 60);
    assert!(!outcome.is_ok());
    let outcome =
        registry_owner_action::change_voting_result_reduction_percent(&root, &registry, 101);
    assert!(!outcome.is_ok());
    let outcome =
        registry_owner_action::change_voting_result_reduction_percent(&root, &registry, 60);
    outcome.assert_success();
    //
    let outcome = registry_owner_action::conclude_voting_score(&users[0], &registry);
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::conclude_voting_score(&root, &registry);
    outcome.assert_success();
    let appchain1 = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(appchain1.voting_score.0, common::to_oct_amount(760) as i128);
    let appchain2 = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(&appchain2.appchain_state, &AppchainState::Staging);
    assert_eq!(
        appchain2.voting_score.0,
        common::to_oct_amount(2000) as i128
    );
    let appchain3 = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(
        appchain3.voting_score.0,
        0 - common::to_oct_amount(600) as i128
    );
    //
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &root,
        &registry,
        &appchain_id1,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    outcome.assert_success();
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &root,
        &registry,
        &appchain_id2,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    assert!(!outcome.is_ok());
    let outcome = registry_owner_action::change_appchain_anchor_code(
        &root,
        &registry,
        &appchain_id3,
        ANCHOR_WASM_BYTES.to_vec(),
    );
    outcome.assert_success();
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::Some([AppchainState::InQueue, AppchainState::Staging].to_vec()),
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Ascending
        ),
        3
    );
}

#[test]
fn test_case3() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply);

    let staging_timestamp =
        root.borrow_runtime().current_block().block_timestamp + 1000000000 + 86500;
    println!("staging timestamp {}", staging_timestamp);

    let outcome = registry_owner_action::stage_code(
        &root,
        &registry,
        REGISTRY_WASM_BYTES.to_vec(),
        staging_timestamp,
    );
    outcome.assert_success();
}
