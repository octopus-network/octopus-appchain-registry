use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::UserAccount;

pub fn get_ft_balance_of(caller: &UserAccount, oct_token: &UserAccount) -> U128 {
    let view_result = caller.view(
        oct_token.account_id(),
        "ft_balance_of",
        &json!({ "account_id": &caller.valid_account_id() })
            .to_string()
            .into_bytes(),
    );
    assert!(view_result.is_ok());
    view_result.unwrap_json::<U128>()
}
