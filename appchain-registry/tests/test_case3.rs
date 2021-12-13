use appchain_registry::types::{AppchainSortingField, AppchainState, SortingOrder};

mod appchain_lifecycle_manager;
mod appchain_owner_action;
mod common;
mod oct_token_viewer;
mod registry_roles;
mod registry_viewer;
mod voter_action;

const TOTAL_SUPPLY: u128 = 100_000_000;

#[test]
fn test_case3() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply, false);
    //
    let mut i = 1;
    while i <= 50 {
        let appchain_id = format!("test_appchain{}", i);
        let amount = common::to_oct_amount(1000);
        let outcome = appchain_owner_action::register_appchain(
            &users[1],
            &oct_token,
            &registry,
            &appchain_id,
            amount,
        );
        outcome.assert_success();
        let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id);
        assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
        //
        let outcome =
            appchain_lifecycle_manager::start_auditing_appchain(&root, &registry, &appchain_id);
        outcome.assert_success();
        let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id);
        assert_eq!(&appchain.appchain_state, &AppchainState::Auditing);
        let outcome =
            appchain_lifecycle_manager::pass_auditing_appchain(&root, &registry, &appchain_id);
        outcome.assert_success();
        let appchain = registry_viewer::get_appchain_status(&registry, &appchain_id);
        assert_eq!(&appchain.appchain_state, &AppchainState::InQueue);
        //
        let outcome = voter_action::upvote_appchain(
            &users[0],
            &oct_token,
            &registry,
            &appchain_id,
            common::to_oct_amount(i * 10),
        );
        outcome.assert_success();
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
        ),
        50
    );
    //
    let outcome =
        registry_roles::change_operator_of_counting_voting_score(&root, &registry, &users[3]);
    outcome.assert_success();
    //
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[3], &registry);
    outcome.assert_success();
    //
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[3], &registry);
    outcome.assert_success();
    //
    let outcome = appchain_lifecycle_manager::count_voting_score(&users[3], &registry);
    outcome.assert_success();
    //
    let outcome = appchain_lifecycle_manager::conclude_voting_score(&root, &registry);
    outcome.assert_success();
    //
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::Some([AppchainState::InQueue, AppchainState::Staging].to_vec()),
            1,
            50,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Ascending
        ),
        50
    );
}
