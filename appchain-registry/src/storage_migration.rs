use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U64;
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, PublicKey, Timestamp};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OldRegistrySettings {
    /// The minimum deposit amount for registering an appchain.
    pub minimum_register_deposit: U128,
    /// The reduction percent of voting score of all appchain `inQueue` after each time
    /// the owner conclude the voting score.
    pub voting_result_reduction_percent: u16,
    /// The interval for calling function `count_voting_score`,
    /// in the interval this function can only be called once.
    pub counting_interval_in_seconds: U64,
    ///
    pub latest_evm_chain_id: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OldRegistryRoles {
    /// The account that manages the lifecycle of appchains.
    pub appchain_lifecycle_manager: AccountId,
    /// The account that manages the settings of appchain registry.
    pub registry_settings_manager: AccountId,
    /// The only account that can call function `count_voting_score`.
    pub operator_of_counting_voting_score: Option<AccountId>,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum OldAppchainState {
    Registered,
    Audited,
    Voting,
    Staging,
    Booting,
    Active,
    Broken,
    Dead,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainBasedata {
    pub appchain_id: AppchainId,
    pub evm_chain_id: Option<U64>,
    pub appchain_metadata: LazyOption<AppchainMetadata>,
    pub appchain_anchor: Option<AccountId>,
    pub appchain_owner: AccountId,
    pub register_deposit: Balance,
    pub appchain_state: OldAppchainState,
    pub upvote_deposit: Balance,
    pub downvote_deposit: Balance,
    pub registered_time: Timestamp,
    pub go_live_time: Timestamp,
    pub validator_count: u32,
    pub total_stake: Balance,
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
    appchain_basedatas: LookupMap<AppchainId, OldAppchainBasedata>,
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
    /// The roles of appchain registry
    registry_roles: LazyOption<OldRegistryRoles>,
    /// Whether the asset transfer is paused
    asset_transfer_is_paused: bool,
}

#[near_bindgen]
impl AppchainRegistry {
    #[init(ignore_state)]
    pub fn migrate_state() -> Self {
        // Deserialize the state using the old contract structure.
        let old_contract: OldAppchainRegistry = env::state_read().expect("Old state doesn't exist");
        //
        assert_self();
        //
        let old_registry_settings = old_contract.registry_settings.get().unwrap();
        let old_registry_roles = old_contract.registry_roles.get().unwrap();
        // Create the new contract using the data from the old contract.
        let new_appchain_registry = AppchainRegistry {
            owner: old_contract.owner.clone(),
            owner_pk: old_contract.owner_pk,
            contract_code_staging_timestamp: old_contract.contract_code_staging_timestamp,
            contract_code_staging_duration: old_contract.contract_code_staging_duration,
            oct_token: old_contract.oct_token,
            registry_settings: LazyOption::new(
                StorageKey::RegistrySettings.into_bytes(),
                Some(&RegistrySettings::from_old_version(old_registry_settings)),
            ),
            appchain_ids: old_contract.appchain_ids,
            appchain_basedatas: LookupMap::new(StorageKey::AppchainBasedatas.into_bytes()),
            upvote_deposits: old_contract.upvote_deposits,
            downvote_deposits: old_contract.downvote_deposits,
            total_stake: old_contract.total_stake,
            registry_roles: LazyOption::new(
                StorageKey::RegistryRoles.into_bytes(),
                Some(&RegistryRoles::from_old_version(old_registry_roles)),
            ),
            asset_transfer_is_paused: old_contract.asset_transfer_is_paused,
        };
        //
        let appchain_ids = new_appchain_registry.appchain_ids.to_vec();
        for appchain_id in appchain_ids {
            if let Some(old_data) = env::storage_read(&get_storage_key_in_lookup_array(
                &StorageKey::AppchainBasedatas,
                &appchain_id,
            )) {
                if let Ok(old_version) = OldAppchainBasedata::try_from_slice(&old_data) {
                    env::storage_write(
                        &get_storage_key_in_lookup_array(
                            &StorageKey::AppchainBasedatas,
                            &appchain_id,
                        ),
                        &AppchainBasedata::from_old_version(old_version)
                            .try_to_vec()
                            .unwrap(),
                    );
                }
            }
        }
        //
        new_appchain_registry
    }
}

fn get_storage_key_in_lookup_array<T: BorshSerialize>(prefix: &StorageKey, index: &T) -> Vec<u8> {
    [prefix.into_bytes(), index.try_to_vec().unwrap()].concat()
}

impl RegistrySettings {
    pub fn from_old_version(old_version: OldRegistrySettings) -> Self {
        Self {
            minimum_register_deposit: old_version.minimum_register_deposit,
        }
    }
}

impl RegistryRoles {
    pub fn from_old_version(old_version: OldRegistryRoles) -> Self {
        Self {
            appchain_lifecycle_manager: old_version.appchain_lifecycle_manager,
            registry_settings_manager: old_version.registry_settings_manager,
            octopus_council: None,
        }
    }
}

impl AppchainBasedata {
    pub fn from_old_version(old_version: OldAppchainBasedata) -> Self {
        Self {
            appchain_id: old_version.appchain_id,
            evm_chain_id: old_version.evm_chain_id,
            appchain_metadata: old_version.appchain_metadata,
            appchain_anchor: old_version.appchain_anchor,
            appchain_owner: old_version.appchain_owner,
            register_deposit: old_version.register_deposit,
            appchain_state: AppchainState::from_old_version(old_version.appchain_state),
            upvote_deposit: old_version.upvote_deposit,
            downvote_deposit: old_version.downvote_deposit,
            registered_time: old_version.registered_time,
            go_live_time: old_version.go_live_time,
            validator_count: old_version.validator_count,
            total_stake: old_version.total_stake,
            dao_proposal_url: None,
        }
    }
}

impl AppchainState {
    pub fn from_old_version(old_version: OldAppchainState) -> Self {
        match old_version {
            OldAppchainState::Registered => AppchainState::Registered,
            OldAppchainState::Audited => AppchainState::Audited,
            OldAppchainState::Voting => AppchainState::Voting,
            OldAppchainState::Staging => AppchainState::Booting,
            OldAppchainState::Booting => AppchainState::Booting,
            OldAppchainState::Active => AppchainState::Active,
            OldAppchainState::Broken => AppchainState::Closing,
            OldAppchainState::Dead => AppchainState::Closed,
        }
    }
}
