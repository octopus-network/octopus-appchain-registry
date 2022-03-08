use std::collections::HashMap;

use appchain_registry::AppchainRegistryContract;
use mock_oct_token::MockOctTokenContract;
use near_sdk::{
    serde::{Deserialize, Serialize},
    serde_json::json,
};
use near_sdk_sim::{call, ContractAccount, ExecutionResult, UserAccount};

use crate::common;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ParamOfUpdateAppchainCustomMetadata {
    appchain_id: String,
    custom_metadata: HashMap<String, String>,
}

pub fn register_appchain(
    signer: &UserAccount,
    oct_token: &ContractAccount<MockOctTokenContract>,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    amount: u128,
) -> ExecutionResult {
    let outcome = common::ft_transfer_call_oct_token(
        signer,
        &registry.user_account,
        amount,
        json!({
            "RegisterAppchain":{
                "appchain_id": appchain_id,
                "website_url":"http://ddfs.dsdfs",
                "function_spec_url":"https://testchain.org/function_spec",
                "github_address":"https://jldfs.yoasdfasd",
                "github_release":"v1.0.0",
                "commit_id":"commit_id",
                "contact_email":"joe@lksdf.com",
                "premined_wrapped_appchain_token_beneficiary":"bob",
                "premined_wrapped_appchain_token":"10000000",
                "initial_supply_of_wrapped_appchain_token":"10000000",
                "ido_amount_of_wrapped_appchain_token":"1000000",
                "initial_era_reward":"100",
                "fungible_token_metadata":{
                    "spec":"ft-1.0.0",
                    "name":"joeToken",
                    "symbol":"JOT",
                    "icon":null,
                    "reference":null,
                    "reference_hash":null,
                    "decimals":18
                },
                "custom_metadata":{
                    "key1":"value1"
                }
            }
        })
        .to_string(),
        oct_token,
    );
    common::print_outcome_result("ft_transfer_call_oct_token", &outcome);
    outcome
}

pub fn transfer_appchain_ownership(
    signer: &UserAccount,
    registry: &ContractAccount<AppchainRegistryContract>,
    appchain_id: &String,
    new_owner: &UserAccount,
) -> ExecutionResult {
    let outcome = call!(
        signer,
        registry.transfer_appchain_ownership(appchain_id.clone(), new_owner.account_id())
    );
    common::print_outcome_result("transfer_appchain_ownership", &outcome);
    outcome
}
