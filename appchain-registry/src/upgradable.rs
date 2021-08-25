use crate::*;
use near_contract_standards::upgrade::Upgradable;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
use near_sdk::json_types::WrappedDuration;
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, Promise, PublicKey, Timestamp};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainRegistry {
    pub owner: AccountId,
    pub owner_pk: PublicKey,
    pub contract_code_staging_timestamp: Timestamp,
    pub contract_code_staging_duration: Duration,
    pub oct_token: AccountId,
    pub minimum_register_deposit: Balance,
    pub appchain_basedatas: UnorderedMap<AppchainId, LazyOption<AppchainBasedata>>,
    pub upvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    pub downvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    pub top_appchain_id_in_queue: AppchainId,
    pub total_stake: Balance,
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

        // Create the new contract using the data from the old contract.
        AppchainRegistry {
            owner: old_contract.owner,
            owner_pk: old_contract.owner_pk,
            contract_code_staging_timestamp: old_contract.contract_code_staging_timestamp,
            contract_code_staging_duration: old_contract.contract_code_staging_duration,
            oct_token: old_contract.oct_token,
            minimum_register_deposit: old_contract.minimum_register_deposit,
            voting_result_reduction_percent: DEFAULT_VOTING_RESULT_REDUCTION_PERCENT,
            appchain_basedatas: old_contract.appchain_basedatas,
            upvote_deposits: old_contract.upvote_deposits,
            downvote_deposits: old_contract.downvote_deposits,
            top_appchain_id_in_queue: old_contract.top_appchain_id_in_queue,
            total_stake: old_contract.total_stake,
        }
    }
}
