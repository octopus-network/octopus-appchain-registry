use crate::{common, contract_interfaces::registry_viewer};
use appchain_registry::types::{
    AppchainSortingField, RegistryRoles, RegistrySettings, SortingOrder,
};
use near_sdk::serde_json::{self, json};

const TOTAL_SUPPLY: u128 = 100_000_000;

#[test]
fn test_case9() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply, true);
    let outcome = common::ft_transfer_call_oct_token(
        &users[0],
        &registry.user_account,
        common::to_oct_amount(1000),
        json!({
            "RegisterAppchain":{
                "appchain_id": "appchain1",
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
        &oct_token,
    );
    outcome.assert_success();
    //
    // perform migration
    //
    common::deploy_new_registry_contract(&registry);
    let result = common::migrate_state(&root, &registry);
    result.assert_success();
    //
    let registry_settings = registry_viewer::get_registry_settings(&registry);
    println!(
        "Registry settings: {}",
        serde_json::to_string::<RegistrySettings>(&registry_settings).unwrap()
    );
    let registry_roles = registry_viewer::get_registry_roles(&registry);
    println!(
        "Registry roles: {}",
        serde_json::to_string::<RegistryRoles>(&registry_roles).unwrap()
    );
    assert_eq!(
        registry_viewer::print_appchains(
            &registry,
            Option::None,
            1,
            5,
            AppchainSortingField::RegisteredTime,
            SortingOrder::Descending
        ),
        1
    );
}
