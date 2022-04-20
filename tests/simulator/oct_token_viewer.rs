use mock_oct_token::MockOctTokenContract;
use near_sdk::json_types::U128;
use near_sdk_sim::{view, ContractAccount, UserAccount};

pub fn get_ft_balance_of(
    user: &UserAccount,
    oct_token: &ContractAccount<MockOctTokenContract>,
) -> U128 {
    let view_result = view!(oct_token.ft_balance_of(user.account_id()));
    assert!(view_result.is_ok());
    view_result.unwrap_json::<U128>()
}
