use std::convert::TryInto;

use crate::types::{AppchainId, AppchainState};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::{env, AccountId, Balance, Timestamp};

/// Appchain state of an appchain of Octopus Network
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AppchainManager {
    pub appchain_id: AppchainId,
    pub appchain_anchor: AccountId,
    pub appchain_owner: AccountId,
    pub initial_deposit: Balance,
    pub appchain_state: AppchainState,
}

impl AppchainManager {
    /// Return a new instance of AppchainState with the given `AppchainId`
    pub fn new(
        appchain_id: AppchainId,
        appchain_anchor: AccountId,
        appchain_owner: AccountId,
        initial_deposit: Balance,
        appchain_state: AppchainState,
    ) -> Self {
        Self {
            appchain_id,
            appchain_anchor,
            appchain_owner,
            initial_deposit,
            appchain_state,
        }
    }
}
