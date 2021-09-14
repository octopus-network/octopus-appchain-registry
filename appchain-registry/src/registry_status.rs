use near_sdk::json_types::U64;

use crate::types::{AppchainSortingField, AppchainState, AppchainStatus, SortingOrder};
use crate::*;

/// The interface for querying status of appchain registry
pub trait RegistryStatus {
    /// Get account id of OCT token
    fn get_oct_token(&self) -> AccountId;
    /// Get minimum register deposit
    fn get_minimum_register_deposit(&self) -> U128;
    /// Get the value of reduction percent for voting result of all appchains still in queue
    fn get_voting_result_reduction_percent(&self) -> U64;
    /// Get the counting interval for voting score
    fn get_counting_interval_in_seconds(&self) -> U64;
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

#[near_bindgen]
impl RegistryStatus for AppchainRegistry {
    fn get_oct_token(&self) -> AccountId {
        self.oct_token.clone()
    }

    fn get_minimum_register_deposit(&self) -> U128 {
        self.minimum_register_deposit.into()
    }

    fn get_voting_result_reduction_percent(&self) -> U64 {
        U64::from(self.voting_result_reduction_percent as u64)
    }

    fn get_counting_interval_in_seconds(&self) -> U64 {
        U64::from(self.counting_interval_in_seconds)
    }

    fn get_total_stake(&self) -> U128 {
        self.total_stake.into()
    }

    fn get_appchain_ids(&self) -> Vec<String> {
        self.appchain_ids.to_vec()
    }

    fn get_appchains_with_state_of(
        &self,
        appchain_state: Option<Vec<AppchainState>>,
        page_number: u16,
        page_size: u16,
        sorting_field: AppchainSortingField,
        sorting_order: SortingOrder,
    ) -> Vec<AppchainStatus> {
        assert!(page_number > 0, "Invalid page number.");
        assert!(page_size >= 5 && page_size <= 50, "Invalid page size.");
        let mut results: Vec<AppchainStatus> = Vec::new();
        for id in self.appchain_ids.to_vec() {
            let appchain_basedata = self.get_appchain_basedata(&id);
            match appchain_state {
                Some(ref states) => {
                    for state in states {
                        if appchain_basedata.state().eq(state) {
                            results.push(appchain_basedata.status());
                            break;
                        }
                    }
                }
                None => results.push(appchain_basedata.status()),
            }
        }
        if results.len() > 0 {
            match sorting_field {
                AppchainSortingField::AppchainId => results.sort_by(|a, b| match sorting_order {
                    SortingOrder::Ascending => a.appchain_id.cmp(&b.appchain_id),
                    SortingOrder::Descending => b.appchain_id.cmp(&a.appchain_id),
                }),
                AppchainSortingField::VotingScore => results.sort_by(|a, b| match sorting_order {
                    SortingOrder::Ascending => a.voting_score.0.cmp(&b.voting_score.0),
                    SortingOrder::Descending => b.voting_score.0.cmp(&a.voting_score.0),
                }),
                AppchainSortingField::RegisteredTime => {
                    results.sort_by(|a, b| match sorting_order {
                        SortingOrder::Ascending => a.registered_time.0.cmp(&b.registered_time.0),
                        SortingOrder::Descending => b.registered_time.0.cmp(&a.registered_time.0),
                    })
                }
            }
            let (_, tail) = results.split_at(((page_number - 1) * page_size).into());
            if tail.len() > page_size.into() {
                let (page, _) = tail.split_at(page_size.into());
                page.to_vec()
            } else {
                tail.to_vec()
            }
        } else {
            results
        }
    }

    fn get_appchains_count_of(&self, appchain_state: Option<AppchainState>) -> U64 {
        let mut count: u64 = 0;
        for id in self.appchain_ids.to_vec() {
            let appchain_basedata = self.get_appchain_basedata(&id);
            match appchain_state {
                Some(ref state) => {
                    if appchain_basedata.state().eq(state) {
                        count += 1;
                    }
                }
                None => count += 1,
            }
        }
        count.into()
    }

    fn get_appchain_status_of(&self, appchain_id: AppchainId) -> AppchainStatus {
        let appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.status()
    }

    fn get_upvote_deposit_for(&self, appchain_id: AppchainId, account_id: AccountId) -> U128 {
        match self.upvote_deposits.get(&(appchain_id, account_id)) {
            Some(value) => value.into(),
            None => 0.into(),
        }
    }

    fn get_downvote_deposit_for(&self, appchain_id: AppchainId, account_id: AccountId) -> U128 {
        match self.downvote_deposits.get(&(appchain_id, account_id)) {
            Some(value) => value.into(),
            None => 0.into(),
        }
    }
}
