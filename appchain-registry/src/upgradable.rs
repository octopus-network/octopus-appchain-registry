use crate::*;
use near_contract_standards::upgrade::Upgradable;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::WrappedDuration;
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, Promise, PublicKey, Timestamp};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainMetadata {
    pub website_url: String,
    pub function_spec_url: String,
    pub github_address: String,
    pub github_release: String,
    pub commit_id: String,
    pub contact_email: String,
    pub premined_wrapped_appchain_token_beneficiary: AccountId,
    pub premined_wrapped_appchain_token: U128,
    pub ido_amount_of_wrapped_appchain_token: U128,
    pub initial_era_reward: U128,
    pub fungible_token_metadata: FungibleTokenMetadata,
    pub custom_metadata: HashMap<String, String>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainBasedata {
    pub appchain_id: AppchainId,
    pub appchain_metadata: LazyOption<OldAppchainMetadata>,
    pub appchain_anchor: AccountId,
    pub appchain_owner: AccountId,
    pub register_deposit: Balance,
    pub appchain_state: AppchainState,
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
}

#[near_bindgen]
impl Upgradable for AppchainRegistry {
    fn get_staging_duration(&self) -> WrappedDuration {
        self.contract_code_staging_duration.into()
    }

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
        let old_contract: OldAppchainRegistry = env::state_read().expect("Old state doesn't exist");
        // Verify that the migration can only be done by the owner.
        // This is not necessary, if the upgrade is done internally.
        assert_eq!(
            &env::predecessor_account_id(),
            &old_contract.owner,
            "Can only be called by the owner"
        );

        let appchain_ids = old_contract.appchain_ids.to_vec();

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
        };

        for appchain_id in appchain_ids {
            let old_basedata = old_contract.appchain_basedatas.get(&appchain_id).unwrap();
            new_appchain_registry.appchain_basedatas.insert(
                &appchain_id,
                &AppchainBasedata::from_old_version(old_basedata),
            );
        }

        new_appchain_registry
    }
}
