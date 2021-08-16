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
    register_deposit: Balance,
    appchain_state: AppchainState,
    upvote_deposit: Balance,
    downvote_deposit: Balance,
    voting_score: i128,
    registered_time: Timestamp,
    go_live_time: Timestamp,
}

impl AppchainBasedata {
    /// Return a new instance of AppchainBasedata with the given parameters
    pub fn new(
        appchain_id: AppchainId,
        appchain_metadata: AppchainMetadata,
        appchain_owner: AccountId,
        register_deposit: Balance,
    ) -> Self {
        Self {
            appchain_id,
            appchain_metadata,
            appchain_anchor: String::new(),
            appchain_anchor_code: Vec::<u8>::new(),
            appchain_owner,
            register_deposit,
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
    /// Get code of anchor
    pub fn anchor_code(&self) -> Vec<u8> {
        self.appchain_anchor_code.iter().map(|f| f.clone()).collect()
    }
    /// Get account id of owner
    pub fn owner(&self) -> &AccountId {
        &self.appchain_owner
    }
    /// Get initial deposit
    pub fn register_deposit(&self) -> Balance {
        self.register_deposit
    }
    /// Get state
    pub fn state(&self) -> AppchainState {
        self.appchain_state.clone()
    }
    /// Get upvote deposit
    pub fn upvote_deposit(&self) -> Balance {
        self.upvote_deposit
    }
    /// Get downvote deposit
    pub fn downvote_deposit(&self) -> Balance {
        self.downvote_deposit
    }
    /// Get voting score
    pub fn voting_score(&self) -> i128 {
        self.voting_score
    }
    /// Get full status
    pub fn status(&self) -> AppchainStatus {
        AppchainStatus {
            appchain_id: self.appchain_id.clone(),
            appchain_metadata: self.appchain_metadata.clone(),
            appchain_anchor: self.appchain_anchor.clone(),
            appchain_owner: self.appchain_owner.clone(),
            register_deposit: self.register_deposit,
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
        self.register_deposit = deposit;
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
    /// Increase upvote deposit
    pub fn increase_upvote_deposit(&mut self, value: Balance) {
        self.upvote_deposit += value;
    }
    /// Decrease upvote deposit
    pub fn decrease_upvote_deposit(&mut self, value: Balance) {
        self.upvote_deposit
            .checked_sub(value)
            .expect("Upvote deposit is not big enough to decrease.");
    }
    /// Increase upvote deposit
    pub fn increase_downvote_deposit(&mut self, value: Balance) {
        self.downvote_deposit += value;
    }
    /// Decrease upvote deposit
    pub fn decrease_downvote_deposit(&mut self, value: Balance) {
        self.downvote_deposit
            .checked_sub(value)
            .expect("Downvote deposit is not big enough to decrease.");
    }
    /// Count voting score
    pub fn count_voting_score(&mut self) {
        self.voting_score += self.upvote_deposit as i128 - self.downvote_deposit as i128;
    }
    /// Reduce voting score by the given percent
    pub fn reduce_voting_score_by_percent(&mut self, percent: u8) {
        assert!(percent <= 100, "Invalid value of percent.");
        self.voting_score -= self.voting_score * percent as i128 / 100;
    }
    /// Deploy anchor
    pub fn deploy_anchor(&mut self) {}
}
