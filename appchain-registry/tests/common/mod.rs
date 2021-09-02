use appchain_registry::AppchainRegistryContract;
use mock_oct_token::MockOctTokenContract;
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};

use near_sdk::json_types::U128;
use near_sdk_sim::{
    call, deploy, init_simulator, lazy_static_include,
    runtime::{init_runtime, GenesisConfig},
    to_yocto, ContractAccount, ExecutionResult, UserAccount,
};

use num_format::{Locale, ToFormattedString};

lazy_static_include::lazy_static_include_bytes! {
    TOKEN_WASM_BYTES => "../res/mock_oct_token.wasm",
    REGISTRY_WASM_BYTES => "../res/appchain_registry.wasm",
    PREVIOUS_REGISTRY_WASM_BYTES => "../res/previous_appchain_registry.wasm",
    ANCHOR_WASM_BYTES => "../res/mock_appchain_anchor.wasm",
}

// Register the given `user` to oct_token
fn register_user_to_oct_token(
    account: &UserAccount,
    contract: &ContractAccount<MockOctTokenContract>,
) {
    let outcome = call!(
        account,
        contract.storage_deposit(Option::from(account.valid_account_id()), Option::None),
        near_sdk::env::storage_byte_cost() * 125,
        near_sdk_sim::DEFAULT_GAS / 2
    );
    outcome.assert_success();
}

pub fn ft_transfer_oct_token(
    sender: &UserAccount,
    receiver: &UserAccount,
    amount: u128,
    oct_token: &ContractAccount<MockOctTokenContract>,
) {
    let outcome = call!(
        sender,
        oct_token.ft_transfer(
            receiver.valid_account_id(),
            U128::from(amount),
            Option::None
        ),
        1,
        near_sdk_sim::DEFAULT_GAS
    );
    print_outcome_result("ft_transfer", &outcome);
    outcome.assert_success();
}

pub fn ft_transfer_call_oct_token(
    sender: &UserAccount,
    receiver: &UserAccount,
    amount: u128,
    msg: String,
    oct_token: &ContractAccount<MockOctTokenContract>,
) -> ExecutionResult {
    let outcome = call!(
        sender,
        oct_token.ft_transfer_call(
            receiver.valid_account_id(),
            U128::from(amount),
            Option::None,
            msg.clone()
        ),
        1,
        near_sdk_sim::DEFAULT_GAS
    );
    print_outcome_result("ft_transfer_call", &outcome);
    outcome.assert_success();
    outcome
}

fn get_genesis_config() -> GenesisConfig {
    let mut genesis_config = GenesisConfig::default();
    genesis_config.block_prod_time = 86400 * 1_000_000_000;
    genesis_config
}

pub fn init(
    total_supply: u128,
) -> (
    UserAccount,
    ContractAccount<MockOctTokenContract>,
    ContractAccount<AppchainRegistryContract>,
    Vec<UserAccount>,
) {
    let root = init_simulator(Some(get_genesis_config()));
    let mut users: Vec<UserAccount> = Vec::new();
    // Deploy and initialize contracts
    let ft_metadata = FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "OCTToken".to_string(),
        symbol: "OCT".to_string(),
        icon: None,
        reference: None,
        reference_hash: None,
        decimals: 18,
    };
    let oct_token = deploy! {
        contract: MockOctTokenContract,
        contract_id: "oct_token",
        bytes: &TOKEN_WASM_BYTES,
        signer_account: root,
        init_method: new(root.valid_account_id(), U128::from(total_supply), ft_metadata)
    };
    let registry = deploy! {
        contract: AppchainRegistryContract,
        contract_id: "registry",
        bytes: &REGISTRY_WASM_BYTES,
        signer_account: root,
        init_method: new(oct_token.valid_account_id().to_string())
    };
    register_user_to_oct_token(&registry.user_account, &oct_token);
    // Create users and transfer a certain amount of OCT token to them
    let alice = root.create_user("alice".to_string(), to_yocto("100"));
    register_user_to_oct_token(&alice, &oct_token);
    ft_transfer_oct_token(&root, &alice, total_supply / 10, &oct_token);
    users.push(alice);
    let bob = root.create_user("bob".to_string(), to_yocto("100"));
    register_user_to_oct_token(&bob, &oct_token);
    ft_transfer_oct_token(&root, &bob, total_supply / 10, &oct_token);
    users.push(bob);
    let charlie = root.create_user("charlie".to_string(), to_yocto("100"));
    register_user_to_oct_token(&charlie, &oct_token);
    ft_transfer_oct_token(&root, &charlie, total_supply / 10, &oct_token);
    users.push(charlie);
    let dave = root.create_user("dave".to_string(), to_yocto("100"));
    register_user_to_oct_token(&dave, &oct_token);
    ft_transfer_oct_token(&root, &dave, total_supply / 10, &oct_token);
    users.push(dave);
    let eve = root.create_user("eve".to_string(), to_yocto("100"));
    register_user_to_oct_token(&eve, &oct_token);
    ft_transfer_oct_token(&root, &eve, total_supply / 10, &oct_token);
    users.push(eve);
    // return initialized UserAccounts
    (root, oct_token, registry, users)
}

pub fn init_by_previous(
    total_supply: u128,
) -> (
    UserAccount,
    ContractAccount<MockOctTokenContract>,
    ContractAccount<AppchainRegistryContract>,
) {
    let root = init_simulator(None);
    // Deploy and initialize contracts
    let ft_metadata = FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "OCTToken".to_string(),
        symbol: "OCT".to_string(),
        icon: None,
        reference: None,
        reference_hash: None,
        decimals: 24,
    };
    let oct_token = deploy! {
        contract: MockOctTokenContract,
        contract_id: "oct_token",
        bytes: &TOKEN_WASM_BYTES,
        signer_account: root,
        init_method: new(root.valid_account_id(), U128::from(total_supply), ft_metadata)
    };
    let registry = deploy! {
        contract: AppchainRegistryContract,
        contract_id: "registry",
        bytes: &PREVIOUS_REGISTRY_WASM_BYTES,
        signer_account: root,
        init_method: new(oct_token.valid_account_id().to_string())
    };
    register_user_to_oct_token(&registry.user_account, &oct_token);

    (root, oct_token, registry)
}

pub fn upgrade_contract_code_and_perform_migration(root: &UserAccount, registry: &UserAccount) {
    let outcome = root
        .create_transaction(registry.account_id())
        .deploy_contract(PREVIOUS_REGISTRY_WASM_BYTES.to_vec())
        .submit();
    print_outcome_result("deploy_registry_contract", &outcome);
    //
    todo!("call function for storage migration");
}

pub fn to_oct_amount(amount: u128) -> u128 {
    let bt_decimals_base = (10 as u128).pow(18);
    amount * bt_decimals_base
}

pub fn print_outcome_result(function_name: &str, outcome: &ExecutionResult) {
    println!(
        "Gas burnt of function '{}': {}",
        function_name,
        outcome.gas_burnt().to_formatted_string(&Locale::en)
    );
    let results = outcome.promise_results();
    for result in results {
        let logs = result.as_ref().unwrap().logs();
        if logs.len() > 0 {
            println!("{:#?}", logs);
        }
    }
}
