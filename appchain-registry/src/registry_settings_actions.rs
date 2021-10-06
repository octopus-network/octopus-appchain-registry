use std::convert::TryInto;

use near_sdk::json_types::U64;

use crate::*;

impl Default for RegistrySettings {
    fn default() -> Self {
        Self {
            minimum_register_deposit: U128::from(DEFAULT_REGISTER_DEPOSIT * OCT_DECIMALS_BASE),
            voting_result_reduction_percent: DEFAULT_VOTING_RESULT_REDUCTION_PERCENT,
            counting_interval_in_seconds: U64::from(SECONDS_OF_A_DAY),
            operator_of_counting_voting_score: env::signer_account_id(),
        }
    }
}

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
    //
    fn change_minimum_register_deposit(&mut self, value: U128) {
        self.assert_owner();
        let mut registry_settings = self.registry_settings.get().unwrap();
        registry_settings.minimum_register_deposit = value;
        self.registry_settings.set(&registry_settings);
    }
    //
    fn change_voting_result_reduction_percent(&mut self, value: U64) {
        self.assert_owner();
        assert!(value.0 <= 100, "Invalid percent value.");
        if let Ok(value) = value.0.try_into() {
            let mut registry_settings = self.registry_settings.get().unwrap();
            registry_settings.voting_result_reduction_percent = value;
            self.registry_settings.set(&registry_settings);
        }
    }
    //
    fn change_counting_interval_in_seconds(&mut self, value: U64) {
        self.assert_owner();
        let mut registry_settings = self.registry_settings.get().unwrap();
        registry_settings.counting_interval_in_seconds = value;
        self.registry_settings.set(&registry_settings);
    }
    //
    fn change_operator_of_counting_voting_score(&mut self, operator_account: AccountId) {
        self.assert_owner();
        let mut registry_settings = self.registry_settings.get().unwrap();
        registry_settings.operator_of_counting_voting_score.clear();
        registry_settings
            .operator_of_counting_voting_score
            .push_str(&operator_account);
        self.registry_settings.set(&registry_settings);
    }
}
