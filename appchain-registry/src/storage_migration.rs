use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, PublicKey, Timestamp};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainBasedata {
    appchain_id: AppchainId,
    appchain_metadata: LazyOption<AppchainMetadata>,
    appchain_anchor: AccountId,
    appchain_owner: AccountId,
    register_deposit: Balance,
    appchain_state: AppchainState,
    upvote_deposit: Balance,
    downvote_deposit: Balance,
    registered_time: Timestamp,
    go_live_time: Timestamp,
    validator_count: u32,
    total_stake: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OldRegistryRoles {
    /// The account that manages the lifecycle of appchains.
    pub appchain_lifecycle_manager: AccountId,
    /// The account that manages the settings of appchain registry.
    pub registry_settings_manager: AccountId,
    /// The only account that can call function `count_voting_score`.
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
    registry_settings: LazyOption<RegistrySettings>,
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
        let old_registry_roles = old_contract.registry_roles.get().unwrap();
        // Create the new contract using the data from the old contract.
        let mut new_appchain_registry = AppchainRegistry {
            owner: old_contract.owner.clone(),
            owner_pk: old_contract.owner_pk,
            contract_code_staging_timestamp: old_contract.contract_code_staging_timestamp,
            contract_code_staging_duration: old_contract.contract_code_staging_duration,
            oct_token: old_contract.oct_token,
            registry_settings: old_contract.registry_settings,
            appchain_ids: old_contract.appchain_ids,
            appchain_basedatas: LookupMap::new(StorageKey::AppchainBasedatas.into_bytes()),
            upvote_deposits: old_contract.upvote_deposits,
            downvote_deposits: old_contract.downvote_deposits,
            top_appchain_id_in_queue: old_contract.top_appchain_id_in_queue,
            total_stake: old_contract.total_stake,
            time_of_last_count_voting_score: old_contract.time_of_last_count_voting_score,
            registry_roles: LazyOption::new(
                StorageKey::RegistryRoles.into_bytes(),
                Some(&RegistryRoles::from_old_version(old_registry_roles)),
            ),
            asset_transfer_is_paused: old_contract.asset_transfer_is_paused,
        };
        //
        let appchain_ids = new_appchain_registry.appchain_ids.to_vec();
        for appchain_id in appchain_ids {
            let old_appchain_basedata = old_contract.appchain_basedatas.get(&appchain_id).unwrap();
            old_contract.appchain_basedatas.remove(&appchain_id);
            new_appchain_registry.appchain_basedatas.insert(
                &appchain_id,
                &AppchainBasedata::from_old_version(old_appchain_basedata),
            );
        }
        //
        new_appchain_registry
    }
}

impl AppchainBasedata {
    pub fn from_old_version(old_version: OldAppchainBasedata) -> Self {
        Self {
            appchain_id: old_version.appchain_id,
            appchain_metadata: old_version.appchain_metadata,
            appchain_anchor: match old_version.appchain_anchor.is_empty() {
                true => None,
                false => Some(old_version.appchain_anchor),
            },
            appchain_owner: old_version.appchain_owner,
            register_deposit: old_version.register_deposit,
            appchain_state: old_version.appchain_state,
            upvote_deposit: old_version.upvote_deposit,
            downvote_deposit: old_version.downvote_deposit,
            registered_time: old_version.registered_time,
            go_live_time: old_version.go_live_time,
            validator_count: old_version.validator_count,
            total_stake: old_version.total_stake,
        }
    }
}

impl RegistryRoles {
    pub fn from_old_version(old_version: OldRegistryRoles) -> Self {
        Self {
            appchain_lifecycle_manager: old_version.appchain_lifecycle_manager,
            registry_settings_manager: old_version.registry_settings_manager,
            operator_of_counting_voting_score: match old_version
                .operator_of_counting_voting_score
                .is_empty()
            {
                true => None,
                false => Some(old_version.operator_of_counting_voting_score),
            },
        }
    }
}