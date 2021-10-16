use appchain_registry::types::{
    AppchainSortingField, AppchainState, AppchainStatus, RegistrySettings, SortingOrder,
};
use appchain_registry::AppchainRegistryContract;

use near_sdk::json_types::U128;
use near_sdk_sim::{view, ContractAccount, UserAccount};

pub fn get_registry_settings(
    registry: &ContractAccount<AppchainRegistryContract>,
) -> RegistrySettings {
    let view_result = view!(registry.get_registry_settings());
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<RegistrySettings>()
}

pub fn print_appchains(
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_state: Option<Vec<AppchainState>>,
    page_number: u16,
    page_size: u16,
    sorting_field: AppchainSortingField,
    sorting_order: SortingOrder,
) -> usize {
    let view_result = view!(registry.get_appchains_with_state_of(
        appchain_state,
        page_number,
        page_size,
        sorting_field,
        sorting_order
    ));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
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
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
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
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<U128>()
}

pub fn get_downvote_deposit_of(
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    user: &UserAccount,
) -> U128 {
    let view_result =
        view!(registry.get_downvote_deposit_for(appchain_id.clone(), user.account_id()));
    if view_result.is_err() {
        println!("{:#?}", view_result);
    }
    assert!(view_result.is_ok());
    view_result.unwrap_json::<U128>()
}
