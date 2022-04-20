use appchain_registry::types::{AppchainSortingField, AppchainState, SortingOrder};

use crate::appchain_lifecycle_manager;
use crate::appchain_owner_actions;
use crate::common;
use crate::oct_token_viewer;
use crate::registry_roles;
use crate::registry_settings;
use crate::registry_viewer;
use crate::sudo_actions;
use crate::voter_actions;

const TOTAL_SUPPLY: u128 = 100_000_000;

/// Test 'pass auditing', 'upvote', 'downvote', 'withdraw upvote' and 'withdraw downvote' actions.
#[test]
fn test_case2() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply, false);
    //
    let appchain_id1 = "test_appchain1".to_string();
    let amount = common::to_oct_amount(1000);
    let outcome = appchain_owner_actions::register_appchain(
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
    let amount = common::to_oct_amount(1000);
    let outcome = appchain_owner_actions::register_appchain(
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
    let amount = common::to_oct_amount(1000);
    let outcome = appchain_owner_actions::register_appchain(
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
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[2], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&users[3], &oct_token).0,
        common::to_oct_amount(TOTAL_SUPPLY / 10 - 1000)
    );
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry.user_account, &oct_token).0,
        common::to_oct_amount(3000)
    );
    //
    let outcome =
        appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id1);
    assert!(!outcome.is_ok());
    let outcome =
        appchain_lifecycle_manager::pass_auditing_appchain(&users[0], &registry, &appchain_id1);
    assert!(!outcome.is_ok());
    let outcome =
        appchain_lifecycle_manager::pass_auditing_appchain(&users[1], &registry, &appchain_id1);
    assert!(!outcome.is_ok());
    //
    let outcome =
        appchain_lifecycle_manager::start_auditing_appchain(&root, &registry, &appchain_id1);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    let outcome =
        appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id1);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
    //
    let outcome =
        appchain_lifecycle_manager::start_auditing_appchain(&root, &registry, &appchain_id2);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    let outcome =
        appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id2);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
    //
    let outcome =
        appchain_lifecycle_manager::start_auditing_appchain(&root, &registry, &appchain_id3);
    outcome.assert_success();
    let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id3);
    assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
    let outcome =
        appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id3);
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
    let result = sudo_actions::pause_asset_transfer(&root, &registry);
    result.assert_success();
    let outcome = voter_actions::upvote_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id1,
        common::to_oct_amount(1000),
    );
    outcome.assert_success();
    let outcome = voter_actions::downvote_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id2,
        common::to_oct_amount(1500),
    );
    outcome.assert_success();
    let appchain1 = registry_viewer::get_appchain_status(&registry, &appchain_id1);
    let appchain2 = registry_viewer::get_appchain_status(&registry, &appchain_id2);
    assert_eq!(appchain1.upvote_deposit.0, 0);
    assert_eq!(appchain2.downvote_deposit.0, 0);
    let result = sudo_actions::resume_asset_transfer(&root, &registry);
    result.assert_success();
    //
    let outcome = voter_actions::upvote_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id1,
        common::to_oct_amount(1000),
    );
    outcome.assert_success();
    let outcome = voter_actions::downvote_appchain(
        &users[0],
        &oct_token,
        &registry,
        &appchain_id2,
        common::to_oct_amount(1500),
    );
    outcome.assert_success();
    let outcome = voter_actions::upvote_appchain(
        &users[4],
        &oct_token,
        &registry,
        &appchain_id2,
        common::to_oct_amount(2000),
    );
    outcome.assert_success();
    let outcome = voter_actions::downvote_appchain(
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
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[1], &registry);
    assert!(!outcome.is_ok());
    let outcome = appchain_lifecycle_manager::count_voting_score(&root, &registry);
    assert!(!outcome.is_ok());
    let outcome =
        registry_roles::change_operator_of_counting_voting_score(&root, &registry, &users[4]);
    outcome.assert_success();
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[4], &registry);
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
    let outcome = voter_actions::withdraw_upvote_deposit_of(
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
    let outcome = voter_actions::withdraw_downvote_deposit_of(
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
    let result = sudo_actions::pause_asset_transfer(&root, &registry);
    result.assert_success();
    let outcome = voter_actions::withdraw_upvote_deposit_of(
        &users[0],
        &registry,
        &appchain_id1,
        common::to_oct_amount(550),
    );
    assert!(!outcome.is_ok());
    let outcome = voter_actions::withdraw_downvote_deposit_of(
        &users[4],
        &registry,
        &appchain_id3,
        common::to_oct_amount(450),
    );
    assert!(!outcome.is_ok());
    let result = sudo_actions::resume_asset_transfer(&root, &registry);
    result.assert_success();
    //
    let outcome = voter_actions::withdraw_upvote_deposit_of(
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
    let outcome = voter_actions::withdraw_downvote_deposit_of(
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
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[2], &registry);
    assert!(!outcome.is_ok());
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[4], &registry);
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
    let outcome = voter_actions::withdraw_downvote_deposit_of(
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
    let outcome = voter_actions::withdraw_upvote_deposit_of(
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
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[3], &registry);
    assert!(!outcome.is_ok());
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[4], &registry);
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
        registry_settings::change_voting_result_reduction_percent(&users[4], &registry, 60);
    assert!(!outcome.is_ok());
    let outcome = registry_settings::change_voting_result_reduction_percent(&root, &registry, 101);
    assert!(!outcome.is_ok());
    let outcome = registry_settings::change_voting_result_reduction_percent(&root, &registry, 60);
    outcome.assert_success();
    //
    let outcome = appchain_lifecycle_manager::conclude_voting_score(&users[0], &registry);
    assert!(!outcome.is_ok());
    let outcome = appchain_lifecycle_manager::conclude_voting_score(&root, &registry);
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
    assert_eq!(appchain3.appchain_state, AppchainState::Dead);
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::Some([AppchainState::InQueue, AppchainState::Staging].to_vec()),
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Ascending
        ),
        2
    );
}
