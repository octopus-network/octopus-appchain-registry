use std::convert::TryInto;

use near_sdk::collections::LazyOption;
use near_sdk::Timestamp;

use crate::types::{AppchainMetadata, AppchainState, AppchainStatus};
use crate::upgradable::{OldAppchainBasedata, OldAppchainMetadata};
use crate::*;

/// Appchain basedata
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AppchainBasedata {
    appchain_id: AppchainId,
    appchain_metadata: LazyOption<AppchainMetadata>,
    appchain_anchor: AccountId,
    appchain_owner: AccountId,
    register_deposit: Balance,
    appchain_state: AppchainState,
    upvote_deposit: Balance,
    downvote_deposit: Balance,
    registered_time: Timestamp,
    go_live_time: Timestamp,
    validator_count: u32,
    total_stake: Balance,
}

impl AppchainMetadata {
    ///
    /// Construct a new instance from an old version of this struct
    pub fn from_old_version(old_version: &OldAppchainMetadata) -> Self {
        Self {
            website_url: old_version.website_url.clone(),
            github_address: old_version.github_address.clone(),
            github_release: old_version.github_release.clone(),
            commit_id: old_version.commit_id.clone(),
            contact_email: old_version.contact_email.clone(),
            premined_wrapped_appchain_token: old_version.preminted_wrapped_appchain_token,
            ido_amount_of_wrapped_appchain_token: old_version.ido_amount_of_wrapped_appchain_token,
            initial_era_reward: old_version.initial_era_reward,
            custom_metadata: old_version.custom_metadata.clone(),
        }
    }
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
            appchain_id: appchain_id.clone(),
            appchain_metadata: LazyOption::new(
                StorageKey::AppchainMetadata(appchain_id.clone()).into_bytes(),
                Some(&appchain_metadata),
            ),
            appchain_anchor: String::new(),
            appchain_owner,
            register_deposit,
            appchain_state: AppchainState::Registered,
            upvote_deposit: 0,
            downvote_deposit: 0,
            registered_time: env::block_timestamp(),
            go_live_time: 0,
            validator_count: 0,
            total_stake: 0,
        }
    }
    /// Get appchain id
    pub fn id(&self) -> &AppchainId {
        &self.appchain_id
    }
    /// Get mutable metadata
    pub fn metadata(&mut self) -> AppchainMetadata {
        self.appchain_metadata.get().unwrap()
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
        if let Some(bytes) = env::storage_read(
            &StorageKey::AppchainVotingScore(self.appchain_id.clone()).into_bytes(),
        ) {
            i128::from_be_bytes(bytes.try_into().expect(&format!(
                "Invalid storage data for voting score of appchain {}",
                self.appchain_id
            )))
        } else {
            0
        }
    }
    /// Get full status
    pub fn status(&self) -> AppchainStatus {
        AppchainStatus {
            appchain_id: self.appchain_id.clone(),
            appchain_metadata: self.appchain_metadata.get().unwrap(),
            appchain_anchor: self.appchain_anchor.clone(),
            appchain_owner: self.appchain_owner.clone(),
            register_deposit: self.register_deposit.into(),
            appchain_state: self.appchain_state.clone(),
            upvote_deposit: self.upvote_deposit.into(),
            downvote_deposit: self.downvote_deposit.into(),
            voting_score: self.voting_score().into(),
            registered_time: self.registered_time.into(),
            go_live_time: self.go_live_time.into(),
            validator_count: self.validator_count,
            total_stake: self.total_stake.into(),
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
    /// Set metadata
    pub fn set_metadata(&mut self, metadata: &AppchainMetadata) {
        self.appchain_metadata.set(metadata);
    }
    /// Set initial deposit
    pub fn set_initial_deposit(&mut self, deposit: Balance) {
        self.register_deposit = deposit;
    }
    /// Set anchor account
    pub fn set_anchor_account(&mut self, anchor_account: &AccountId) {
        self.appchain_anchor.clear();
        self.appchain_anchor.push_str(anchor_account);
    }
    /// Set total stake
    pub fn set_total_stake(&mut self, total_stake: Balance) {
        self.total_stake = total_stake;
    }
    /// Change state
    pub fn change_state(&mut self, new_state: AppchainState) {
        assert_ne!(self.appchain_state, new_state, "The state not changed.");
        self.appchain_state = new_state;
    }
    /// Sync staking status
    pub fn sync_staking_status(&mut self, validator_count: u32, total_stake: Balance) {
        self.validator_count = validator_count;
        self.total_stake = total_stake;
    }
    /// Increase upvote deposit
    pub fn increase_upvote_deposit(&mut self, value: Balance) {
        self.upvote_deposit += value;
    }
    /// Decrease upvote deposit
    pub fn decrease_upvote_deposit(&mut self, value: Balance) {
        self.upvote_deposit = self
            .upvote_deposit
            .checked_sub(value)
            .expect("Upvote deposit is not big enough to decrease.");
    }
    /// Increase upvote deposit
    pub fn increase_downvote_deposit(&mut self, value: Balance) {
        self.downvote_deposit += value;
    }
    /// Decrease upvote deposit
    pub fn decrease_downvote_deposit(&mut self, value: Balance) {
        self.downvote_deposit = self
            .downvote_deposit
            .checked_sub(value)
            .expect("Downvote deposit is not big enough to decrease.");
    }
    /// Count voting score
    pub fn count_voting_score(&self) {
        let voting_score =
            self.voting_score() + self.upvote_deposit as i128 - self.downvote_deposit as i128;
        env::storage_write(
            &StorageKey::AppchainVotingScore(self.appchain_id.clone()).into_bytes(),
            &voting_score.to_be_bytes(),
        );
    }
    /// Reduce voting score by the given percent
    pub fn reduce_voting_score_by_percent(&self, percent: u16) {
        assert!(percent <= 100, "Invalid percent value.");
        let mut voting_score = self.voting_score();
        voting_score -= voting_score * percent as i128 / 100;
        env::storage_write(
            &StorageKey::AppchainVotingScore(self.appchain_id.clone()).into_bytes(),
            &voting_score.to_be_bytes(),
        );
    }
    /// Construct a new instance from an old version of this struct
    pub fn from_old_version(old_version: &OldAppchainBasedata) -> Self {
        Self {
            appchain_id: old_version.appchain_id.clone(),
            appchain_metadata: LazyOption::new(
                StorageKey::AppchainMetadata(old_version.appchain_id.clone()).into_bytes(),
                Some(&AppchainMetadata::from_old_version(
                    &old_version.appchain_metadata.get().unwrap(),
                )),
            ),
            appchain_anchor: old_version.appchain_anchor.clone(),
            appchain_owner: old_version.appchain_owner.clone(),
            register_deposit: old_version.register_deposit,
            appchain_state: old_version.appchain_state.clone(),
            upvote_deposit: old_version.upvote_deposit,
            downvote_deposit: old_version.downvote_deposit,
            registered_time: old_version.registered_time,
            go_live_time: old_version.go_live_time,
            validator_count: old_version.validator_count,
            total_stake: old_version.total_stake,
        }
    }
}
