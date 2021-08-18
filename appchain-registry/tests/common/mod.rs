use appchain_registry::types::{AppchainState, AppchainStatus};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};

use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk_sim::{init_simulator, to_yocto, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT};

use num_format::{Locale, ToFormattedString};

const OCT_TOKEN_ID: &str = "mock_oct_token";
const REGISTRY_ID: &str = "appchain_registry";

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct ParamOfGetAppchainsWithStateOf {
    appchain_state: Option<AppchainState>,
}

// Register the given `user` to oct_token
fn register_user(user: &UserAccount) {
    user.call(
        OCT_TOKEN_ID.to_string(),
        "storage_deposit",
        &json!({
            "account_id": user.valid_account_id()
        })
        .to_string()
        .into_bytes(),
        near_sdk_sim::DEFAULT_GAS / 2,
        near_sdk::env::storage_byte_cost() * 125, // attached deposit
    )
    .assert_success();
}

pub fn init(total_supply: u128) -> (UserAccount, UserAccount, UserAccount, Vec<UserAccount>) {
    let root = init_simulator(None);
    let mut users: Vec<UserAccount> = Vec::new();
    // Deploy and initialize contracts
    let oct_token = root.deploy(
        &include_bytes!("../../../res/mock_oct_token.wasm").to_vec(),
        OCT_TOKEN_ID.into(),
        10 * STORAGE_AMOUNT,
    );
    let registry = root.deploy(
        &include_bytes!("../../../res/appchain_registry.wasm").to_vec(),
        REGISTRY_ID.into(),
        10 * STORAGE_AMOUNT,
    );
    oct_token
        .call(
            OCT_TOKEN_ID.into(),
            "new",
            &json!({
                "owner_id": root.valid_account_id(),
                "total_supply": U128::from(total_supply),
                "metadata": FungibleTokenMetadata {
                    spec: FT_METADATA_SPEC.to_string(),
                    name: "OCTToken".to_string(),
                    symbol: "OCT".to_string(),
                    icon: None,
                    reference: None,
                    reference_hash: None,
                    decimals: 24,
                }
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();
    registry
        .call(
            REGISTRY_ID.into(),
            "new",
            &json!({
                "oct_token": oct_token.valid_account_id()
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            0, // attached deposit
        )
        .assert_success();
    register_user(&registry);
    // Create users and transfer a certain amount of OCT token to them
    let alice = root.create_user("alice".to_string(), to_yocto("100"));
    register_user(&alice);
    root.call(
        OCT_TOKEN_ID.into(),
        "ft_transfer",
        &json!({
            "receiver_id": alice.valid_account_id(),
            "amount": U128::from(total_supply / 10),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        1, // attached deposit
    )
    .assert_success();
    users.push(alice);
    let bob = root.create_user("bob".to_string(), to_yocto("100"));
    register_user(&bob);
    root.call(
        OCT_TOKEN_ID.into(),
        "ft_transfer",
        &json!({
            "receiver_id": bob.valid_account_id(),
            "amount": U128::from(total_supply / 10),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        1, // attached deposit
    )
    .assert_success();
    users.push(bob);
    let charlie = root.create_user("charlie".to_string(), to_yocto("100"));
    register_user(&charlie);
    root.call(
        OCT_TOKEN_ID.into(),
        "ft_transfer",
        &json!({
            "receiver_id": charlie.valid_account_id(),
            "amount": U128::from(total_supply / 10),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        1, // attached deposit
    )
    .assert_success();
    users.push(charlie);
    let dave = root.create_user("dave".to_string(), to_yocto("100"));
    register_user(&dave);
    root.call(
        OCT_TOKEN_ID.into(),
        "ft_transfer",
        &json!({
            "receiver_id": dave.valid_account_id(),
            "amount": U128::from(total_supply / 10),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        1, // attached deposit
    )
    .assert_success();
    users.push(dave);
    let eve = root.create_user("eve".to_string(), to_yocto("100"));
    register_user(&eve);
    root.call(
        OCT_TOKEN_ID.into(),
        "ft_transfer",
        &json!({
            "receiver_id": eve.valid_account_id(),
            "amount": U128::from(total_supply / 10),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        1, // attached deposit
    )
    .assert_success();
    users.push(eve);
    // return initialized UserAccounts
    (root, oct_token, registry, users)
}

pub fn init_by_previous(total_supply: u128) -> (UserAccount, UserAccount, UserAccount) {
    let root = init_simulator(None);

    let oct_token = root.deploy(
        &include_bytes!("../../../res/mock_oct_token.wasm").to_vec(),
        OCT_TOKEN_ID.into(),
        10 * STORAGE_AMOUNT,
    );
    let registry = root.deploy(
        &include_bytes!("../../../res/previous_appchain_registry.wasm").to_vec(),
        REGISTRY_ID.into(),
        10 * STORAGE_AMOUNT,
    );

    let mut result = oct_token.call(
        OCT_TOKEN_ID.into(),
        "new",
        &json!({
            "owner_id": root.valid_account_id(),
            "total_supply": U128::from(total_supply),
            "metadata": FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "OCTToken".to_string(),
                symbol: "OCT".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 24,
            }
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    );
    println!(
        "Gas burnt of function 'new' of OCT token contract: {}",
        result.gas_burnt().to_formatted_string(&Locale::en)
    );
    result.assert_success();

    result = registry.call(
        REGISTRY_ID.into(),
        "new",
        &json!({
            "oct_token": oct_token.valid_account_id()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    );
    println!(
        "Gas burnt of function 'new' of registry contract: {}",
        result.gas_burnt().to_formatted_string(&Locale::en)
    );
    result.assert_success();

    register_user(&registry);

    (root, oct_token, registry)
}

pub fn upgrade_contract_code_and_perform_migration(registry: &UserAccount) {
    registry
        .create_transaction(registry.account_id())
        .deploy_contract(include_bytes!("../../../res/appchain_registry.wasm").to_vec())
        .submit()
        .assert_success();
    let result = registry.call(
        REGISTRY_ID.into(),
        "migrate_state",
        &json!({
            "new_note_of_validator": "migrate to new version",
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    );
    result.logs().iter().for_each(|l| println!("{}", l));
    println!(
        "Gas burnt of function 'migrate_state' of registry contract: {}",
        result.gas_burnt().to_formatted_string(&Locale::en)
    );
    result.assert_success();
}

pub fn to_oct_amount(amount: u128) -> u128 {
    let bt_decimals_base = (10 as u128).pow(18);
    amount * bt_decimals_base
}
