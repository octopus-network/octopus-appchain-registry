use appchain_registry::types::{AppchainState, AppchainStatus};

use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk_sim::UserAccount;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ParamOfGetAppchainsWithStateOf {
    appchain_state: Option<AppchainState>,
}

pub fn get_minimum_register_deposit(caller: &UserAccount, registry: &UserAccount) -> U128 {
    let view_result = caller.view(
        registry.account_id(),
        "get_minimum_register_deposit",
        &json!({}).to_string().into_bytes(),
    );
    assert!(view_result.is_ok());
    view_result.unwrap_json::<U128>()
}

pub fn print_appchains(
    caller: &UserAccount,
    registry: &UserAccount,
    appchain_state: Option<AppchainState>,
) -> usize {
    let view_result = caller.view(
        registry.account_id(),
        "get_appchains_with_state_of",
        &serde_json::to_string(&ParamOfGetAppchainsWithStateOf { appchain_state })
            .unwrap()
            .into_bytes(),
    );
    assert!(view_result.is_ok());
    println!("{}", String::from_utf8(view_result.unwrap()).unwrap());
    let appchains: Vec<AppchainStatus> = view_result.unwrap_json();
    appchains.len()
}

pub fn get_appchain_status(
    caller: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
) -> AppchainStatus {
    let view_result = caller.view(
        registry.account_id(),
        "get_appchain_status_of",
        &json!({ "appchain_id": &appchain_id })
            .to_string()
            .into_bytes(),
    );
    assert!(view_result.is_ok());
    println!("{}", String::from_utf8(view_result.unwrap()).unwrap());
    let appchain_status: AppchainStatus = view_result.unwrap_json();
    appchain_status
}

pub fn get_upvote_deposit_of(
    caller: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    account: &UserAccount,
) -> U128 {
    let view_result = caller.view(
        registry.account_id(),
        "get_upvote_deposit_for",
        &json!({
            "appchain_id": &appchain_id,
            "account_id": &account.valid_account_id(),
        })
        .to_string()
        .into_bytes(),
    );
    view_result.unwrap_json::<U128>()
}

pub fn get_downvote_deposit_of(
    caller: &UserAccount,
    registry: &UserAccount,
    appchain_id: &String,
    account: &UserAccount,
) -> U128 {
    let view_result = caller.view(
        registry.account_id(),
        "get_downvote_deposit_for",
        &json!({
            "appchain_id": &appchain_id,
            "account_id": &account.valid_account_id(),
        })
        .to_string()
        .into_bytes(),
    );
    view_result.unwrap_json::<U128>()
}
