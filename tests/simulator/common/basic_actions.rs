use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::{
    json_types::{Base64VecU8, U128},
    serde_json::json,
};
use near_units::parse_near;
use workspaces::{network::Sandbox, Account, Contract, Worker};

pub async fn initialize_contracts_and_users(
    worker: &Worker<Sandbox>,
    total_supply: u128,
    with_old_anchor: bool,
) -> anyhow::Result<(Account, Contract, Contract, Vec<Account>)> {
    let root = worker.root_account().unwrap();
    let mut users: Vec<Account> = Vec::new();
    //
    // deploy OCT token contract
    //
    let oct_ft_metadata = FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "OCTToken".to_string(),
        symbol: "OCT".to_string(),
        icon: Option::<String>::None,
        reference: Option::<String>::None,
        reference_hash: Option::<Base64VecU8>::None,
        decimals: 18,
    };
    let oct_token = root
        .create_subaccount("oct_token")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    let oct_token = oct_token
        .deploy(&std::fs::read(format!("res/mock_oct_token.wasm"))?)
        .await?
        .unwrap();
    assert!(oct_token
        .call("new")
        .args_json(json!({
            "owner_id": root.id(),
            "total_supply": U128::from(total_supply),
            "metadata": oct_ft_metadata
        }))
        .gas(300_000_000_000_000)
        .transact()
        .await
        .expect("Failed to call function 'new' of oct token contract.")
        .is_success());
    //
    // deploy appchain registry contract
    //
    let appchain_registry = root
        .create_subaccount("appchain_registry")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    let appchain_registry = match with_old_anchor {
        true => appchain_registry
            .deploy(&std::fs::read(format!(
                "res/appchain_registry_v2.1.0.wasm"
            ))?)
            .await?
            .unwrap(),
        false => appchain_registry
            .deploy(&std::fs::read(format!("res/appchain_registry.wasm"))?)
            .await?
            .unwrap(),
    };
    assert!(root
        .call(appchain_registry.id(), "new")
        .args_json(json!({
            "oct_token": oct_token.id(),
        }))
        .gas(300_000_000_000_000)
        .transact()
        .await
        .expect("Failed to call function 'new' of registry contract.")
        .is_success());
    //
    // register appchain registry contract to OCT token
    //
    register_user_to_ft_contract(appchain_registry.as_account(), &oct_token).await?;
    // Create users and transfer a certain amount of OCT token to them
    // alice
    let alice = root
        .create_subaccount("alice")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    register_user_to_ft_contract(&alice, &oct_token).await?;
    super::call_ft_transfer(&root, &alice, total_supply / 10, &oct_token)
        .await?
        .unwrap();
    users.push(alice);
    // bob
    let bob = root
        .create_subaccount("bob")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    register_user_to_ft_contract(&bob, &oct_token).await?;
    super::call_ft_transfer(&root, &bob, total_supply / 10, &oct_token)
        .await?
        .unwrap();
    users.push(bob);
    // charlie
    let charlie = root
        .create_subaccount("charlie")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    register_user_to_ft_contract(&charlie, &oct_token).await?;
    super::call_ft_transfer(&root, &charlie, total_supply / 10, &oct_token)
        .await?
        .unwrap();
    users.push(charlie);
    // dave
    let dave = root
        .create_subaccount("dave")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    register_user_to_ft_contract(&dave, &oct_token).await?;
    super::call_ft_transfer(&root, &dave, total_supply / 10, &oct_token)
        .await?
        .unwrap();
    users.push(dave);
    // eve
    let eve = root
        .create_subaccount("eve")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    register_user_to_ft_contract(&eve, &oct_token).await?;
    super::call_ft_transfer(&root, &eve, total_supply / 10, &oct_token)
        .await?
        .unwrap();
    users.push(eve);
    // council
    let council = root
        .create_subaccount("council")
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();
    register_user_to_ft_contract(&council, &oct_token).await?;
    super::call_ft_transfer(&root, &council, total_supply / 10, &oct_token)
        .await?
        .unwrap();
    users.push(council);
    // Return initialized UserAccounts
    Ok((root, oct_token, appchain_registry, users))
}

// Register the given `user` to fungible token contract
pub async fn register_user_to_ft_contract(
    account: &Account,
    ft_token_contract: &Contract,
) -> anyhow::Result<()> {
    assert!(ft_token_contract
        .call("storage_deposit")
        .args_json(json!({
            "account_id": Some(account.id()),
            "registration_only": Option::<bool>::None,
        }))
        .gas(20_000_000_000_000)
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .transact()
        .await
        .expect("Failed to call 'storage_deposit' of token contract.")
        .is_success());
    Ok(())
}
