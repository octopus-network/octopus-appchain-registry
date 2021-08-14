mod appchain_anchor_callback;
mod appchain_basedata;
mod appchain_owner_action;
mod registry_owner_action;
mod registry_status;
mod storage_key;
pub mod types;
mod voter_action;
use crate::storage_key::StorageKey;
use appchain_basedata::AppchainBasedata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_self, env, ext_contract, log, near_bindgen, AccountId, Balance, BlockHeight, Promise,
    PromiseOrValue, PromiseResult,
};
use types::{AppchainId, AppchainState};

const OCT_DECIMALS_BASE: Balance = 1000_000_000_000_000_000_000_000;

const APPCHAIN_NOT_FOUND: &'static str = "Appchain not found.";

near_sdk::setup_alloc!();

#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AppchainRegistry {
    pub owner: AccountId,
    pub oct_token: AccountId,
    pub appchain_basedatas: UnorderedMap<AppchainId, LazyOption<AppchainBasedata>>,
}

impl Default for AppchainRegistry {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage.")
    }
}

#[near_bindgen]
impl AppchainRegistry {
    #[init]
    pub fn new(oct_token: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");
        assert_self();
        Self {
            owner: env::current_account_id(),
            oct_token,
            appchain_basedatas: UnorderedMap::new(StorageKey::AppchainBasedatas.into_bytes()),
        }
    }
    // Assert that the contract called by the owner.
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Function can only be called by owner."
        );
    }
    // Assert that the contract called by the owner of the given appchain.
    fn assert_appchain_owner(&self, appchain_id: &AppchainId) {
        let appchain_basedata = self.get_appchain_basedata(appchain_id);
        assert_eq!(
            env::predecessor_account_id(),
            appchain_basedata.owner().clone(),
            "Function can only be called by appchain owner."
        );
    }
    // Assert that the state of the given appchain is equal to the given AppchainState.
    fn assert_appchain_state(&self, appchain_id: &AppchainId, appchain_state: AppchainState) {
        let appchain_basedata = self.get_appchain_basedata(appchain_id);
        assert_eq!(
            appchain_basedata.state().clone(),
            appchain_state,
            "Appchain state must be '{}'.",
            appchain_state,
        );
    }
    // Get AppchainBasedata from storage
    fn get_appchain_basedata(&self, appchain_id: &AppchainId) -> AppchainBasedata {
        self.appchain_basedatas
            .get(appchain_id)
            .expect(APPCHAIN_NOT_FOUND)
            .get()
            .expect(APPCHAIN_NOT_FOUND)
    }
    // Save AppchainBasedata to storage
    fn set_appchain_basedata(
        &mut self,
        appchain_id: &AppchainId,
        appchain_basedata: &AppchainBasedata,
    ) {
        self.appchain_basedatas
            .get(appchain_id)
            .expect(APPCHAIN_NOT_FOUND)
            .set(appchain_basedata);
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
