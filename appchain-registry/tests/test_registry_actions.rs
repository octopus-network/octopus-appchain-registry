use appchain_registry::types::AppchainState;

mod appchain_owner_action;
mod common;
mod oct_token_viewer;
mod registry_owner_action;
mod registry_viewer;
mod voter_action;

const TOTAL_SUPPLY: u128 = 100_000_000;

#[test]
fn test_registry_actions() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (_, oct_token, registry, users) = common::init(total_supply);
    //
    assert_eq!(
        registry_viewer::get_minimum_register_deposit(&users[0], &registry).0,
        common::to_oct_amount(100)
    );
    let amount = common::to_oct_amount(120);
    let outcome =
        registry_owner_action::change_minimum_register_deposit(&users[0], &registry, amount);
    assert!(!outcome.is_ok());
    let outcome =
        registry_owner_action::change_minimum_register_deposit(&registry, &registry, amount);
    outcome.assert_success();
    assert_eq!(
        registry_viewer::get_minimum_register_deposit(&users[0], &registry).0,
        common::to_oct_amount(120)
    );
    //
    assert_eq!(
        oct_token_viewer::get_ft_balance_of(&registry, &oct_token).0,
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
        registry_viewer::print_appchains(&users[0], &registry, Option::None),
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
        oct_token_viewer::get_ft_balance_of(&registry, &oct_token).0,
        common::to_oct_amount(120)
    );
    let appchain = registry_viewer::get_appchain_status(&users[3], &registry, &appchain_id);
    assert_eq!(&appchain.appchain_state, &AppchainState::Registered);
    //
}
