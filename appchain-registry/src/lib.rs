mod appchain_anchor_callback;
mod appchain_basedata;
mod appchain_owner_actions;
mod registry_owner_actions;
mod registry_settings_actions;
mod registry_status;
mod storage_key;
mod sudo_actions;
pub mod types;
mod upgradable;
mod voter_actions;
use std::collections::HashMap;

pub use appchain_anchor_callback::AppchainAnchorCallback;
pub use appchain_basedata::AppchainBasedata;
pub use appchain_owner_actions::AppchainOwnerActions;
use near_contract_standards::upgrade::Ownable;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_self, env, ext_contract, log, near_bindgen, AccountId, Balance, Duration, Promise,
    PromiseOrValue, PromiseResult, PublicKey, Timestamp,
};
pub use registry_owner_actions::RegistryOwnerActions;
pub use registry_status::RegistryStatus;
pub use storage_key::StorageKey;
pub use sudo_actions::SudoActions;
use types::{AppchainId, AppchainMetadata, AppchainState};
pub use voter_actions::VoterActions;

const NO_DEPOSIT: Balance = 0;
/// Initial balance for the AppchainAnchor contract to cover storage and related.
const APPCHAIN_ANCHOR_INIT_BALANCE: Balance = 3_000_000_000_000_000_000_000_000; // 3e24yN, 3N
const T_GAS: u64 = 1_000_000_000_000;
const GAS_FOR_FT_TRANSFER_CALL: u64 = 35 * T_GAS;
const OCT_DECIMALS_BASE: Balance = 1000_000_000_000_000_000;
/// Default register deposit amount
const DEFAULT_REGISTER_DEPOSIT: Balance = 1000;
/// Multiple of nano seconds for a second
const NANO_SECONDS_MULTIPLE: u64 = 1_000_000_000;
/// Seconds of a day
const SECONDS_OF_A_DAY: u64 = 86400;
/// Default staging duration of contract code for upgrade
const DEFAULT_CONTRACT_CODE_STAGING_DURATION: u64 = 3600 * 24;
/// Default value of voting_result_reduction_percent
const DEFAULT_VOTING_RESULT_REDUCTION_PERCENT: u16 = 50;

const APPCHAIN_NOT_FOUND: &'static str = "Appchain not found.";

near_sdk::setup_alloc!();

#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_self)]
pub trait CrossContractResultResolver {
    /// Resolver for refunding to the owner of an appchain when reject it to go-live
    fn resolve_appchain_refunding(&mut self, appchain_id: AppchainId, amount: Balance);
    /// Resolver for deletion of an appchain anchor
    fn resolve_appchain_anchor_deletion(&mut self, appchain_id: AppchainId);
    /// Resolver for withdrawing the upvote deposit of a voter
    fn resolve_withdraw_upvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: Balance,
    );
    /// Resolver for withdrawing the downvote deposit of a voter
    fn resolve_withdraw_downvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: Balance,
    );
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AppchainRegistry {
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
    /// The minimum deposit amount for registering an appchain
    minimum_register_deposit: Balance,
    /// The reduction percent of voting score of all appchain `inQueue` after each time
    /// the owner conclude the voting score
    voting_result_reduction_percent: u16,
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
    /// The interval for calling function `count_voting_score`,
    /// in the interval this function can only be called once.
    counting_interval_in_seconds: u64,
    /// The only account that can call function `count_voting_score`
    operator_of_counting_voting_score: AccountId,
}

impl Default for AppchainRegistry {
    fn default() -> Self {
        env::panic(b"The contract needs be initialized before use.")
    }
}

#[near_bindgen]
impl AppchainRegistry {
    #[init]
    pub fn new(oct_token: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");
        Self {
            owner: env::signer_account_id(),
            owner_pk: env::signer_account_pk(),
            contract_code_staging_timestamp: u64::MAX,
            contract_code_staging_duration: DEFAULT_CONTRACT_CODE_STAGING_DURATION
                * NANO_SECONDS_MULTIPLE,
            oct_token,
            minimum_register_deposit: DEFAULT_REGISTER_DEPOSIT * OCT_DECIMALS_BASE,
            voting_result_reduction_percent: DEFAULT_VOTING_RESULT_REDUCTION_PERCENT,
            appchain_ids: UnorderedSet::new(StorageKey::AppchainIds.into_bytes()),
            appchain_basedatas: LookupMap::new(StorageKey::AppchainBasedatas.into_bytes()),
            upvote_deposits: LookupMap::new(StorageKey::UpvoteDeposits.into_bytes()),
            downvote_deposits: LookupMap::new(StorageKey::DownvoteDeposits.into_bytes()),
            top_appchain_id_in_queue: String::new(),
            total_stake: 0,
            time_of_last_count_voting_score: 0,
            counting_interval_in_seconds: SECONDS_OF_A_DAY,
            operator_of_counting_voting_score: env::signer_account_id(),
        }
    }
    // Assert that the contract called by the owner.
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Function can only be called by owner."
        );
    }
    // Assert that the contract called by the owner of the given appchain.
    fn assert_appchain_owner(&self, appchain_id: &AppchainId) {
        let appchain_basedata = self.get_appchain_basedata(appchain_id);
        assert_eq!(
            env::signer_account_id(),
            appchain_basedata.owner().clone(),
            "Function can only be called by appchain owner."
        );
    }
    // Assert that the state of the given appchain is equal to the given AppchainState.
    fn assert_appchain_state(&self, appchain_id: &AppchainId, appchain_state: AppchainState) {
        let appchain_basedata = self.get_appchain_basedata(appchain_id);
        assert_eq!(
            appchain_basedata.state().clone(),
            appchain_state,
            "Appchain state must be '{}'.",
            appchain_state,
        );
    }
    // Get AppchainBasedata from storage
    fn get_appchain_basedata(&self, appchain_id: &AppchainId) -> AppchainBasedata {
        self.appchain_basedatas
            .get(appchain_id)
            .expect(APPCHAIN_NOT_FOUND)
    }
    /// Callback function for `ft_transfer_call` of NEP-141 compatible contracts
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        log!(
            "Deposit {} from @{} received. msg: {}",
            amount.0,
            &sender_id,
            msg
        );
        assert_eq!(
            &env::predecessor_account_id(),
            &self.oct_token,
            "Invalid deposit '{}' of unknown NEP-141 asset from '{}' received. Return deposit.",
            amount.0,
            sender_id,
        );

        let msg_vec: Vec<String> = msg.split(",").map(|s| s.to_string()).collect();

        match msg_vec.get(0).unwrap().as_str() {
            "register_appchain" => {
                assert_eq!(
                    msg_vec.len(),
                    7,
                    "Invalid params for `register_appchain`. Return deposit."
                );
                self.register_appchain(
                    sender_id,
                    msg_vec.get(1).unwrap().to_string(),
                    amount.0,
                    msg_vec.get(2).unwrap().to_string(),
                    msg_vec.get(3).unwrap().to_string(),
                    msg_vec.get(4).unwrap().to_string(),
                    msg_vec.get(5).unwrap().to_string(),
                    msg_vec.get(6).unwrap().to_string(),
                );
                PromiseOrValue::Value(0.into())
            }
            "upvote_appchain" => {
                assert_eq!(
                    msg_vec.len(),
                    2,
                    "Invalid params for `upvote_appchain`. Return deposit."
                );
                let appchain_id = msg_vec.get(1).unwrap().to_string();
                let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
                assert_eq!(
                    &appchain_basedata.state(),
                    &AppchainState::InQueue,
                    "Voting appchain must be 'inQueue'."
                );
                let voter_upvote = self
                    .upvote_deposits
                    .get(&(appchain_id.clone(), sender_id.clone()))
                    .unwrap_or_default();
                appchain_basedata.increase_upvote_deposit(amount.0);
                self.appchain_basedatas
                    .insert(&appchain_id, &appchain_basedata);
                self.upvote_deposits
                    .insert(&(appchain_id, sender_id), &(voter_upvote + amount.0));
                PromiseOrValue::Value(0.into())
            }
            "downvote_appchain" => {
                assert_eq!(
                    msg_vec.len(),
                    2,
                    "Invalid params for `downvote_appchain`. Return deposit."
                );
                let appchain_id = msg_vec.get(1).unwrap().to_string();
                let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
                assert_eq!(
                    &appchain_basedata.state(),
                    &AppchainState::InQueue,
                    "Downvoting appchain must be 'inQueue'."
                );
                let voter_downvote = self
                    .downvote_deposits
                    .get(&(appchain_id.clone(), sender_id.clone()))
                    .unwrap_or_default();
                appchain_basedata.increase_downvote_deposit(amount.0);
                self.appchain_basedatas
                    .insert(&appchain_id, &appchain_basedata);
                self.downvote_deposits
                    .insert(&(appchain_id, sender_id), &(voter_downvote + amount.0));
                PromiseOrValue::Value(0.into())
            }
            _ => {
                log!(
                    "Invalid msg '{}' attached in `ft_transfer_call`. Return deposit.",
                    msg
                );
                PromiseOrValue::Value(amount)
            }
        }
    }
    //
    fn register_appchain(
        &mut self,
        sender_id: AccountId,
        appchain_id: AppchainId,
        register_deposit: Balance,
        website_url: String,
        github_address: String,
        github_release: String,
        commit_id: String,
        contact_email: String,
    ) {
        assert!(
            self.appchain_basedatas.get(&appchain_id).is_none(),
            "Appchain already registered."
        );
        assert!(
            register_deposit.eq(&self.minimum_register_deposit),
            "Invalid register deposit."
        );
        let appchain_basedata = AppchainBasedata::new(
            appchain_id.clone(),
            AppchainMetadata {
                website_url,
                github_address,
                github_release,
                commit_id,
                contact_email,
                custom_metadata: HashMap::new(),
            },
            sender_id,
            register_deposit,
        );
        self.appchain_ids.insert(&appchain_id);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        env::log(
            format!(
                "Appchain '{}' is registered by '{}'.",
                appchain_basedata.id(),
                appchain_basedata.owner()
            )
            .as_bytes(),
        )
    }
}

#[near_bindgen]
impl Ownable for AppchainRegistry {
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }
}
