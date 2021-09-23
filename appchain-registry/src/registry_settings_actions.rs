use std::convert::TryInto;

use near_sdk::json_types::U64;

use crate::*;

/// The actions related to registry settings
pub trait RegistrySettingsActions {
    /// Change the value of minimum register deposit
    fn change_minimum_register_deposit(&mut self, value: U128);
    /// Change the value of reduction percent for voting result of all appchains still in queue
    fn change_voting_result_reduction_percent(&mut self, value: U64);
    /// Change the interval for counting voting score of appchains
    fn change_counting_interval_in_seconds(&mut self, value: U64);
    /// Change operator of counting voting score
    fn change_operator_of_counting_voting_score(&mut self, operator_account: AccountId);
}

#[near_bindgen]
impl RegistrySettingsActions for AppchainRegistry {
    fn change_minimum_register_deposit(&mut self, value: U128) {
        self.assert_owner();
        self.minimum_register_deposit = value.0;
    }

    fn change_voting_result_reduction_percent(&mut self, value: U64) {
        self.assert_owner();
        assert!(value.0 <= 100, "Invalid percent value.");
        if let Ok(value) = value.0.try_into() {
            self.voting_result_reduction_percent = value;
        }
    }

    fn change_counting_interval_in_seconds(&mut self, value: U64) {
        self.assert_owner();
        self.counting_interval_in_seconds = value.0;
    }

    fn change_operator_of_counting_voting_score(&mut self, operator_account: AccountId) {
        self.assert_owner();
        self.operator_of_counting_voting_score.clear();
        self.operator_of_counting_voting_score
            .push_str(&operator_account);
    }
}
