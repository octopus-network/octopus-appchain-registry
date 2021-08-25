use crate::*;
use near_contract_standards::upgrade::{Ownable, Upgradable};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap};
use near_sdk::json_types::{WrappedDuration, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_self, env, ext_contract, log, near_bindgen, AccountId, Balance, Duration, Promise,
    PromiseOrValue, PromiseResult, PublicKey, Timestamp,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldAppchainRegistry {
    owner: AccountId,
    owner_pk: PublicKey,
    contract_code_staging_timestamp: Timestamp,
    contract_code_staging_duration: Duration,
    oct_token: AccountId,
    minimum_register_deposit: Balance,
    appchain_basedatas: UnorderedMap<AppchainId, LazyOption<AppchainBasedata>>,
    upvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    downvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    top_appchain_id_in_queue: AppchainId,
    total_stake: Balance,
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

    fn migrate(&mut self) {
        todo!("Add implementation");
    }
}
