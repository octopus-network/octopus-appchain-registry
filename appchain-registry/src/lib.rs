mod appchain_manager;
mod storage_key;
pub mod types;
use crate::storage_key::StorageKey;
use appchain_manager::AppchainManager;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_self, env, ext_contract, log, near_bindgen, AccountId, Balance, BlockHeight, Promise,
    PromiseOrValue, PromiseResult,
};
use types::AppchainId;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AppchainRegistry {
    pub owner: AccountId,
    pub appchain_managers: UnorderedMap<AppchainId, LazyOption<AppchainManager>>,
}

impl Default for AppchainRegistry {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl AppchainRegistry {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        assert_self();
        Self {
            owner: env::current_account_id(),
            appchain_managers: UnorderedMap::new(StorageKey::AppchainManagers.into_bytes()),
        }
    }
}

pub trait Ownable {
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.get_owner(),
            "You are not the contract owner."
        );
    }
    fn get_owner(&self) -> AccountId;
    fn set_owner(&mut self, owner: AccountId);
}

#[near_bindgen]
impl Ownable for AppchainRegistry {
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }
}
