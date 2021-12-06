use near_sdk_sim::lazy_static_include;

mod appchain_owner_action;
mod common;
mod oct_token_viewer;
mod registry_owner_action;
mod registry_viewer;
mod voter_action;

lazy_static_include::lazy_static_include_bytes! {
    TOKEN_WASM_BYTES => "../res/mock_oct_token.wasm",
    REGISTRY_WASM_BYTES => "../res/appchain_registry.wasm",
    PREVIOUS_REGISTRY_WASM_BYTES => "../res/previous_appchain_registry.wasm",
    ANCHOR_WASM_BYTES => "../res/mock_appchain_anchor.wasm",
}

const TOTAL_SUPPLY: u128 = 100_000_000;

#[test]
fn test_case9() {
    let total_supply = common::to_oct_amount(TOTAL_SUPPLY);
    let (root, oct_token, registry, users) = common::init(total_supply);

    let staging_timestamp =
        root.borrow_runtime().current_block().block_timestamp + 1000000000 + 86500;
    println!("staging timestamp {}", staging_timestamp);

    let outcome = registry_owner_action::stage_code(
        &root,
        &registry,
        REGISTRY_WASM_BYTES.to_vec(),
        staging_timestamp,
    );
    outcome.assert_success();
}
