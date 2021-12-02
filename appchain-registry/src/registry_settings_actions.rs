use core::convert::TryInto;

use near_sdk::json_types::U64;

use crate::{interfaces::RegistrySettingsActions, *};

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

#[near_bindgen]
impl RegistrySettingsActions for AppchainRegistry {
    //
    fn change_minimum_register_deposit(&mut self, value: U128) {
        self.assert_owner();
        assert!(value.0 > 0, "The minimum register deposit should NOT be 0.");
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
        assert!(value.0 > 3600, "Too short interval.");
        let mut registry_settings = self.registry_settings.get().unwrap();
        registry_settings.counting_interval_in_seconds = value;
        self.registry_settings.set(&registry_settings);
    }
    //
    fn change_operator_of_counting_voting_score(&mut self, operator_account: AccountId) {
        self.assert_owner();
        assert_ne!(
            operator_account, self.owner,
            "The account should NOT be the owner."
        );
        let mut registry_settings = self.registry_settings.get().unwrap();
        assert_ne!(
            operator_account, registry_settings.operator_of_counting_voting_score,
            "The account is not changed."
        );
        registry_settings.operator_of_counting_voting_score = operator_account;
        self.registry_settings.set(&registry_settings);
    }
}
