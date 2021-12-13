use appchain_registry::types::{RegistryRoles, RegistrySettings};
use near_sdk::serde_json;

mod appchain_lifecycle_manager;
mod appchain_owner_action;
mod common;
mod oct_token_viewer;
mod registry_viewer;
mod voter_action;

const TOTAL_SUPPLY: u128 = 100_000_000;

#[test]
fn test_case9() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply, true);
    common::deploy_new_registry_contract(&registry);
    let result = common::migrate_state(&root, &registry);
    result.assert_success();
    //
    let registry_settings = registry_viewer::get_registry_settings(&registry);
    println!(
        "Anchor status: {}",
        serde_json::to_string::<RegistrySettings>(&registry_settings).unwrap()
    );
    let registry_roles = registry_viewer::get_registry_roles(&registry);
    println!(
        "Anchor status: {}",
        serde_json::to_string::<RegistryRoles>(&registry_roles).unwrap()
    );
}
