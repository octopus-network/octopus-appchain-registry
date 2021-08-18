use appchain_registry::types::{AppchainState, AppchainStatus};
use appchain_registry::AppchainRegistryContract;

use near_sdk::json_types::U128;
use near_sdk_sim::{view, ContractAccount, UserAccount};

pub fn get_minimum_register_deposit(registry: &ContractAccount<AppchainRegistryContract>) -> U128 {
    let view_result = view!(registry.get_minimum_register_deposit());
    assert!(view_result.is_ok());
    view_result.unwrap_json::<U128>()
}

pub fn print_appchains(
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_state: Option<AppchainState>,
) -> usize {
    let view_result = view!(registry.get_appchains_with_state_of(appchain_state));
    assert!(view_result.is_ok());
    println!("{}", String::from_utf8(view_result.unwrap()).unwrap());
    let appchains: Vec<AppchainStatus> = view_result.unwrap_json();
    appchains.len()
}

pub fn get_appchain_status(
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
) -> AppchainStatus {
    let view_result = view!(registry.get_appchain_status_of(appchain_id.clone()));
    assert!(view_result.is_ok());
    println!("{}", String::from_utf8(view_result.unwrap()).unwrap());
    let appchain_status: AppchainStatus = view_result.unwrap_json();
    appchain_status
}

pub fn get_upvote_deposit_of(
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    user: &UserAccount,
) -> U128 {
    let view_result =
        view!(registry.get_upvote_deposit_for(appchain_id.clone(), user.account_id()));
    view_result.unwrap_json::<U128>()
}

pub fn get_downvote_deposit_of(
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    user: &UserAccount,
) -> U128 {
    let view_result =
        view!(registry.get_downvote_deposit_for(appchain_id.clone(), user.account_id()));
    view_result.unwrap_json::<U128>()
}
