use near_sdk::json_types::U64;

use crate::{
    types::{AppchainSortingField, AppchainStatus, SortingOrder},
    *,
};

/// The callback interface for appchain anchor
pub trait AppchainAnchorCallback {
    /// Sync state of an appchain to registry
    fn sync_state_of(
        &mut self,
        appchain_id: AppchainId,
        appchain_state: AppchainState,
        validator_count: u32,
        total_stake: U128,
    );
}

/// The actions which the owner of an appchain can perform
pub trait AppchainOwnerActions {
    /// Transfer ownership of an appchain to another account
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId);
}

pub trait AppchainLifecycleManager {
    /// Update metadata of an appchain
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        website_url: Option<String>,
        function_spec_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        contact_email: Option<String>,
        premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
        premined_wrapped_appchain_token: Option<U128>,
        initial_supply_of_wrapped_appchain_token: Option<U128>,
        ido_amount_of_wrapped_appchain_token: Option<U128>,
        initial_era_reward: Option<U128>,
        fungible_token_metadata: Option<FungibleTokenMetadata>,
        custom_metadata: Option<HashMap<String, String>>,
    );
    /// Start auditing of an appchain
    fn start_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Reject an appchain
    fn reject_appchain(&mut self, appchain_id: AppchainId);
    /// Conclude voting score of appchains
    fn conclude_voting_score(&mut self);
    /// Remove an appchain from registry
    fn remove_appchain(&mut self, appchain_id: AppchainId);
}

pub trait RegistrySettingsManager {
    /// Change the value of minimum register deposit
    fn change_minimum_register_deposit(&mut self, value: U128);
    /// Change the value of reduction percent for voting result of all appchains still in queue
    fn change_voting_result_reduction_percent(&mut self, value: U64);
    /// Change the interval for counting voting score of appchains
    fn change_counting_interval_in_seconds(&mut self, value: U64);
}

/// The interface for querying status of appchain registry
pub trait RegistryStatus {
    /// Get the public key of current owner
    fn get_owner_pk(&self) -> String;
    /// Get account id of OCT token
    fn get_oct_token(&self) -> AccountId;
    /// Get registry settings
    fn get_registry_settings(&self) -> RegistrySettings;
    /// Get registry roles
    fn get_registry_roles(&self) -> RegistryRoles;
    /// Get total stake of all appchains in 'staging', 'booting' and 'active' state
    fn get_total_stake(&self) -> U128;
    /// Get appchain ids
    fn get_appchain_ids(&self) -> Vec<String>;
    /// Get appchains whose state is equal to the given AppchainState
    /// If param `appchain_state` is `Option::None`, return all appchains in registry
    fn get_appchains_with_state_of(
        &self,
        appchain_state: Option<Vec<AppchainState>>,
        page_number: u16,
        page_size: u16,
        sorting_field: AppchainSortingField,
        sorting_order: SortingOrder,
    ) -> Vec<AppchainStatus>;
    /// Get appchains count whose state is equal to the given AppchainState
    ///
    /// If param `appchain_state` is `Option::None`, return count of all appchains in registry
    fn get_appchains_count_of(&self, appchain_state: Option<AppchainState>) -> U64;
    /// Get status of an appchain
    fn get_appchain_status_of(&self, appchain_id: AppchainId) -> AppchainStatus;
    /// Get upvote deposit of a given account id for a certain appchain
    fn get_upvote_deposit_for(&self, appchain_id: AppchainId, account_id: AccountId) -> U128;
    /// Get downvote deposit of a given account id for a certain appchain
    fn get_downvote_deposit_for(&self, appchain_id: AppchainId, account_id: AccountId) -> U128;
}

pub trait SudoActions {
    /// Set public key of owner.
    fn set_owner_pk(&mut self, public_key: String);
    /// Create subaccount for a specific appchain.
    fn create_anchor_account(&mut self, appchain_id: AppchainId);
    /// Force change state of an appchain
    fn force_change_appchain_state(&mut self, appchain_id: AppchainId, state: AppchainState);
    /// Pause asset transfer in this contract.
    fn pause_asset_transfer(&mut self);
    /// Resume asset transfer in this contract.
    fn resume_asset_transfer(&mut self);
}

pub trait VoterActions {
    /// Withdraw a certain amount of upvote deposit for an appchain
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128);
    /// Withdraw a certain amount of downvote deposit for an appchain
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128);
}
