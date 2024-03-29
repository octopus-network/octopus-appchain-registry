use appchain_registry::types::{
    AppchainSortingField, AppchainState, AppchainStatus, RegistryRoles, RegistrySettings,
    SortingOrder,
};
use near_sdk::serde_json::json;
use workspaces::Contract;

pub async fn get_registry_settings(
    registry: &Contract,
) -> Result<RegistrySettings, workspaces::error::Error> {
    registry
        .call("get_registry_settings")
        .view()
        .await
        .expect("Failed in calling 'get_registry_settings'")
        .json::<RegistrySettings>()
}

pub async fn get_registry_roles(
    registry: &Contract,
) -> Result<RegistryRoles, workspaces::error::Error> {
    registry
        .call("get_registry_roles")
        .view()
        .await
        .expect("Failed in calling 'get_registry_roles'")
        .json::<RegistryRoles>()
}

pub async fn print_appchains(
    registry: &Contract,
    appchain_state: Option<Vec<AppchainState>>,
    page_number: u16,
    page_size: u16,
    sorting_field: AppchainSortingField,
    sorting_order: SortingOrder,
) -> anyhow::Result<usize> {
    let result = registry
        .call("get_appchains_with_state_of")
        .args_json(json!({
            "appchain_state": appchain_state,
            "page_number": page_number,
            "page_size": page_size,
            "sorting_field": sorting_field,
            "sorting_order": sorting_order,
        }))
        .view()
        .await
        .expect("Failed in calling 'get_appchains_with_state_of'")
        .json::<Vec<AppchainStatus>>()
        .expect("Failed in calling 'get_appchains_with_state_of'");
    result.iter().for_each(|appchain_status| {
        println!(
            "Appchain: {}",
            near_sdk::serde_json::ser::to_string(appchain_status).unwrap()
        );
    });
    Ok(result.len())
}

pub async fn get_appchain_status_of(
    registry: &Contract,
    appchain_id: &String,
) -> anyhow::Result<AppchainStatus> {
    let result = registry
        .call("get_appchain_status_of")
        .args_json(json!({ "appchain_id": appchain_id }))
        .view()
        .await
        .expect("Failed in calling 'get_appchain_status_of'")
        .json::<AppchainStatus>()
        .expect("Failed in calling 'get_appchain_status_of'");
    println!(
        "AppchainStatus: {}",
        near_sdk::serde_json::ser::to_string(&result).unwrap()
    );
    Ok(result)
}
