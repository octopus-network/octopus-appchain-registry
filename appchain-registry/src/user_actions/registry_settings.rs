use crate::*;
use near_sdk::json_types::U64;

pub trait RegistrySettingsManager {
    /// Change the value of minimum register deposit
    fn change_minimum_register_deposit(&mut self, value: U128);
}

impl Default for RegistrySettings {
    fn default() -> Self {
        Self {
            minimum_register_deposit: U128::from(DEFAULT_REGISTER_DEPOSIT * OCT_DECIMALS_BASE),
            voting_result_reduction_percent: DEFAULT_VOTING_RESULT_REDUCTION_PERCENT,
            counting_interval_in_seconds: U64::from(SECONDS_OF_A_DAY),
        }
    }
}

#[near_bindgen]
impl RegistrySettingsManager for AppchainRegistry {
    //
    fn change_minimum_register_deposit(&mut self, value: U128) {
        self.assert_registry_settings_manager();
        assert!(value.0 > 0, "The minimum register deposit should NOT be 0.");
        let mut registry_settings = self.registry_settings.get().unwrap();
        registry_settings.minimum_register_deposit = value;
        self.registry_settings.set(&registry_settings);
    }
}
