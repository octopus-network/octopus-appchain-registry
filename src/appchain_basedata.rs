use near_sdk::Timestamp;

use crate::types::{AppchainMetadata, AppchainState, AppchainStatus};
use crate::*;

/// Appchain basedata
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AppchainBasedata {
    appchain_id: AppchainId,
    appchain_metadata: AppchainMetadata,
    appchain_anchor: AccountId,
    appchain_anchor_code: Vec<u8>,
    appchain_owner: AccountId,
    initial_deposit: Balance,
    appchain_state: AppchainState,
    upvote_deposit: Balance,
    downvote_deposit: Balance,
    voting_score: u128,
    registered_time: Timestamp,
    go_live_time: Timestamp,
}

impl AppchainBasedata {
    /// Return a new instance of AppchainBasedata with the given parameters
    pub fn new(
        appchain_id: AppchainId,
        appchain_metadata: AppchainMetadata,
        appchain_owner: AccountId,
    ) -> Self {
        Self {
            appchain_id,
            appchain_metadata,
            appchain_anchor: String::new(),
            appchain_anchor_code: Vec::<u8>::new(),
            appchain_owner,
            initial_deposit: 0,
            appchain_state: AppchainState::Registered,
            upvote_deposit: 0,
            downvote_deposit: 0,
            voting_score: 0,
            registered_time: env::block_timestamp(),
            go_live_time: 0,
        }
    }
    /// Get appchain id
    pub fn id(&self) -> &AppchainId {
        &self.appchain_id
    }
    /// Get mutable metadata
    pub fn metadata(&mut self) -> &mut AppchainMetadata {
        &mut self.appchain_metadata
    }
    /// Get acount id of anchor
    pub fn anchor(&self) -> &AccountId {
        &self.appchain_anchor
    }
    /// Get account id of owner
    pub fn owner(&self) -> &AccountId {
        &self.appchain_owner
    }
    /// Get initial deposit
    pub fn initial_deposit(&self) -> Balance {
        self.initial_deposit
    }
    /// Get state
    pub fn state(&self) -> AppchainState {
        self.appchain_state.clone()
    }
    /// Get full status
    pub fn status(&self) -> AppchainStatus {
        AppchainStatus {
            appchain_id: self.appchain_id.clone(),
            appchain_metadata: self.appchain_metadata.clone(),
            appchain_anchor: self.appchain_anchor.clone(),
            appchain_owner: self.appchain_owner.clone(),
            initial_deposit: self.initial_deposit,
            appchain_state: self.appchain_state.clone(),
            upvote_deposit: self.upvote_deposit,
            downvote_deposit: self.downvote_deposit,
            voting_score: self.voting_score,
            registered_time: self.registered_time,
            go_live_time: self.go_live_time,
        }
    }
    /// Change owner
    pub fn change_owner(&mut self, new_owner: &AccountId) {
        assert_ne!(
            self.appchain_owner,
            new_owner.clone(),
            "The owner not changed."
        );
        self.appchain_owner.clear();
        self.appchain_owner.push_str(new_owner);
    }
    /// Set initial deposit
    pub fn set_initial_deposit(&mut self, deposit: Balance) {
        self.initial_deposit = deposit;
    }
    /// Set code of anchor
    pub fn set_anchor_code(&mut self, mut code: Vec<u8>) {
        self.appchain_anchor_code.clear();
        self.appchain_anchor_code.append(&mut code);
    }
    /// Change state
    pub fn change_state(&mut self, new_state: AppchainState) {
        assert_ne!(self.appchain_state, new_state, "The state not changed.");
        self.appchain_state = new_state;
    }
    /// Deploy anchor
    pub fn deploy_anchor(&mut self) {}
}
