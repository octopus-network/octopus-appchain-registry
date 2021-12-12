use crate::*;
use near_contract_standards::upgrade::Upgradable;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::{WrappedDuration, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, Promise, PublicKey, Timestamp};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldRegistrySettings {
    /// The minimum deposit amount for registering an appchain
    pub minimum_register_deposit: U128,
    /// The reduction percent of voting score of all appchain `inQueue` after each time
    /// the owner conclude the voting score
    pub voting_result_reduction_percent: u16,
    /// The interval for calling function `count_voting_score`,
    /// in the interval this function can only be called once.
    pub counting_interval_in_seconds: U64,
    /// The only account that can call function `count_voting_score`
    pub operator_of_counting_voting_score: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainRegistry {
    /// The account of the owner of this contract
    owner: AccountId,
    /// The public key of owner account
    owner_pk: PublicKey,
    /// The earliest time that the staged code can be deployed
    contract_code_staging_timestamp: Timestamp,
    /// The shortest time range between code staging and code deployment
    contract_code_staging_duration: Duration,
    /// The account of OCT token contract
    oct_token: AccountId,
    /// The settings of appchain registry
    registry_settings: LazyOption<OldRegistrySettings>,
    /// The set of all appchain ids
    appchain_ids: UnorderedSet<AppchainId>,
    /// The map from appchain id to their basedata
    appchain_basedatas: LookupMap<AppchainId, AppchainBasedata>,
    /// The map from pair (appchain id, account id) to their upvote deposit
    upvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    /// The map from pair (appchain id, account id) to their downvote deposit
    downvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    /// The appchain id with the highest voting score at a certain time
    top_appchain_id_in_queue: AppchainId,
    /// The total stake of OCT token in all appchains
    total_stake: Balance,
    /// The time of the last calling of function `count_voting_score`
    time_of_last_count_voting_score: Timestamp,
}

#[near_bindgen]
impl Upgradable for AppchainRegistry {
    //
    fn get_staging_duration(&self) -> WrappedDuration {
        self.contract_code_staging_duration.into()
    }
    //
    fn stage_code(&mut self, code: Vec<u8>, timestamp: Timestamp) {
        self.assert_owner();
        assert!(
            env::block_timestamp() + self.contract_code_staging_duration < timestamp,
            "Timestamp {} must be later than staging duration {}",
            timestamp,
            env::block_timestamp() + self.contract_code_staging_duration
        );
        // Writes directly into storage to avoid serialization penalty by using default struct.
        env::storage_write(&StorageKey::ContractCode.into_bytes(), &code);
        self.contract_code_staging_timestamp = timestamp;
    }
    //
    fn deploy_code(&mut self) -> Promise {
        self.assert_owner();
        assert!(
            self.contract_code_staging_timestamp < env::block_timestamp(),
            "Deploy code too early: staging ends on {}",
            self.contract_code_staging_timestamp
        );
        let code = env::storage_read(&StorageKey::ContractCode.into_bytes())
            .unwrap_or_else(|| panic!("No upgrade code available"));
        env::storage_remove(&StorageKey::ContractCode.into_bytes());
        Promise::new(env::current_account_id()).deploy_contract(code)
    }
}

#[near_bindgen]
impl AppchainRegistry {
    #[init(ignore_state)]
    pub fn migrate_state() -> Self {
        // Deserialize the state using the old contract structure.
        let mut old_contract: OldAppchainRegistry =
            env::state_read().expect("Old state doesn't exist");
        // Verify that the migration can only be done by the owner.
        // This is not necessary, if the upgrade is done internally.
        assert_eq!(
            &env::predecessor_account_id(),
            &old_contract.owner,
            "Can only be called by the owner"
        );
        //
        let old_registry_settings = old_contract.registry_settings.get().unwrap();
        // Create the new contract using the data from the old contract.
        let new_appchain_registry = AppchainRegistry {
            owner: old_contract.owner.clone(),
            owner_pk: old_contract.owner_pk,
            contract_code_staging_timestamp: old_contract.contract_code_staging_timestamp,
            contract_code_staging_duration: old_contract.contract_code_staging_duration,
            oct_token: old_contract.oct_token,
            registry_settings: LazyOption::new(
                StorageKey::RegistrySettings.into_bytes(),
                Some(&RegistrySettings {
                    minimum_register_deposit: old_registry_settings.minimum_register_deposit,
                    voting_result_reduction_percent: old_registry_settings
                        .voting_result_reduction_percent,
                    counting_interval_in_seconds: old_registry_settings
                        .counting_interval_in_seconds,
                }),
            ),
            appchain_ids: old_contract.appchain_ids,
            appchain_basedatas: old_contract.appchain_basedatas,
            upvote_deposits: old_contract.upvote_deposits,
            downvote_deposits: old_contract.downvote_deposits,
            top_appchain_id_in_queue: old_contract.top_appchain_id_in_queue,
            total_stake: old_contract.total_stake,
            time_of_last_count_voting_score: old_contract.time_of_last_count_voting_score,
            registry_roles: LazyOption::new(
                StorageKey::RegistryRoles.into_bytes(),
                Some(&RegistryRoles {
                    appchain_lifecycle_manager: old_contract.owner.clone(),
                    registry_settings_manager: old_contract.owner.clone(),
                    operator_of_counting_voting_score: old_registry_settings
                        .operator_of_counting_voting_score,
                }),
            ),
            asset_transfer_is_paused: false,
        };
        //
        old_contract.registry_settings.remove();
        //
        new_appchain_registry
    }
}
