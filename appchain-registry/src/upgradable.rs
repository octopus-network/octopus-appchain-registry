use crate::*;
use near_contract_standards::upgrade::Upgradable;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
use near_sdk::json_types::WrappedDuration;
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, Promise, PublicKey, Timestamp};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainBasedata {
    pub appchain_id: AppchainId,
    pub appchain_metadata: AppchainMetadata,
    pub appchain_anchor: AccountId,
    pub appchain_owner: AccountId,
    pub register_deposit: Balance,
    pub appchain_state: AppchainState,
    pub upvote_deposit: Balance,
    pub downvote_deposit: Balance,
    pub registered_time: Timestamp,
    pub go_live_time: Timestamp,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainRegistry {
    owner: AccountId,
    owner_pk: PublicKey,
    contract_code_staging_timestamp: Timestamp,
    contract_code_staging_duration: Duration,
    oct_token: AccountId,
    minimum_register_deposit: Balance,
    voting_result_reduction_percent: u16,
    appchain_basedatas: UnorderedMap<AppchainId, LazyOption<OldAppchainBasedata>>,
    upvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    downvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    top_appchain_id_in_queue: AppchainId,
    total_stake: Balance,
    time_of_last_count_voting_score: Timestamp,
    counting_interval_in_seconds: u64,
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
        let mut old_contract: OldAppchainRegistry =
            env::state_read().expect("Old state doesn't exist");
        // Verify that the migration can only be done by the owner.
        // This is not necessary, if the upgrade is done internally.
        assert_eq!(
            &env::predecessor_account_id(),
            &old_contract.owner,
            "Can only be called by the owner"
        );

        let appchain_ids = old_contract.appchain_basedatas.to_vec();

        // Create the new contract using the data from the old contract.
        let mut new_appchain_registry = AppchainRegistry {
            owner: old_contract.owner.clone(),
            owner_pk: old_contract.owner_pk,
            contract_code_staging_timestamp: old_contract.contract_code_staging_timestamp,
            contract_code_staging_duration: old_contract.contract_code_staging_duration,
            oct_token: old_contract.oct_token,
            minimum_register_deposit: old_contract.minimum_register_deposit,
            voting_result_reduction_percent: old_contract.voting_result_reduction_percent,
            appchain_ids: UnorderedSet::new(StorageKey::AppchainIds.into_bytes()),
            appchain_basedatas: LookupMap::new(StorageKey::AppchainBasedatas.into_bytes()),
            upvote_deposits: old_contract.upvote_deposits,
            downvote_deposits: old_contract.downvote_deposits,
            top_appchain_id_in_queue: old_contract.top_appchain_id_in_queue,
            total_stake: old_contract.total_stake,
            time_of_last_count_voting_score: old_contract.time_of_last_count_voting_score,
            counting_interval_in_seconds: old_contract.counting_interval_in_seconds,
            operator_of_counting_voting_score: old_contract.owner.clone(),
        };

        for appchain_id in appchain_ids {
            if let Some(old_appchain_basedata) = old_contract.appchain_basedatas.get(&appchain_id.0)
            {
                if let Some(old_appchain_basedata) = old_appchain_basedata.get() {
                    old_contract
                        .appchain_basedatas
                        .remove(&old_appchain_basedata.appchain_id);
                    new_appchain_registry.add_appchain_basedata(
                        AppchainBasedata::from_old_version(&old_appchain_basedata),
                    );
                }
            }
        }

        new_appchain_registry
    }
}

impl AppchainRegistry {
    //
    fn add_appchain_basedata(&mut self, appchain_basedata: AppchainBasedata) {
        self.appchain_basedatas
            .insert(&appchain_basedata.id(), &appchain_basedata);
    }
}
