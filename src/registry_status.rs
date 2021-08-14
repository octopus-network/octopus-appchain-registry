use crate::types::{AppchainState, AppchainStatus};
use crate::*;

/// The interface for querying status of appchain registry
pub trait RegistryStatus {
    /// Get appchains whose state is equal to the given AppchainState
    ///
    /// If param `appchain_state` is `Option::None`, return all appchains in registry
    fn get_appchains_with_state_of(
        &self,
        appchain_state: Option<AppchainState>,
    ) -> Vec<AppchainStatus>;
    /// Get status of an appchain
    fn get_appchain_status_of(&self, appchain_id: AppchainId) -> AppchainStatus;
    /// Get upvote deposit of the caller for a certain appchain
    fn get_upvote_deposit_for(&self, appchain_id: AppchainId) -> Balance;
    /// Get downvote deposit of the caller for a certain appchain
    fn get_downvote_deposit_for(&self, appchain_id: AppchainId) -> Balance;
}
