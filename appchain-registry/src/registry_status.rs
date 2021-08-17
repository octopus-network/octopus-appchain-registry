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

impl RegistryStatus for AppchainRegistry {
    fn get_appchains_with_state_of(
        &self,
        appchain_state: Option<AppchainState>,
    ) -> Vec<AppchainStatus> {
        let mut results: Vec<AppchainStatus> = Vec::new();
        let ids = self.appchain_basedatas.keys().collect::<Vec<String>>();
        for id in ids {
            let appchain_basedata = self.get_appchain_basedata(&id);
            match appchain_state {
                Some(ref state) => if appchain_basedata.state().eq(state) {
                    results.push(appchain_basedata.status());
                }
                None => results.push(appchain_basedata.status()),
            }
        }
        results
    }

    fn get_appchain_status_of(&self, appchain_id: AppchainId) -> AppchainStatus {
        let appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.status()
    }

    fn get_upvote_deposit_for(&self, appchain_id: AppchainId) -> Balance {
        match self
            .upvote_deposits
            .get(&(appchain_id, env::predecessor_account_id()))
        {
            Some(value) => value,
            None => 0,
        }
    }

    fn get_downvote_deposit_for(&self, appchain_id: AppchainId) -> Balance {
        match self
            .downvote_deposits
            .get(&(appchain_id, env::predecessor_account_id()))
        {
            Some(value) => value,
            None => 0,
        }
    }
}
