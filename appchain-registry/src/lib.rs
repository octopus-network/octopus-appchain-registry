mod appchain_anchor_callback;
mod appchain_basedata;
mod appchain_lifecycle;
mod appchain_owner_actions;
pub mod interfaces;
mod registry_roles;
mod registry_settings;
mod registry_status;
mod storage_key;
mod storage_migration;
mod sudo_actions;
pub mod types;
mod voter_actions;

use core::convert::TryFrom;
use std::collections::HashMap;

use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::upgrade::Ownable;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_self, env, ext_contract, log, near_bindgen, serde_json, AccountId, Balance, Duration,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult, PublicKey, Timestamp,
};

use appchain_basedata::AppchainBasedata;
use storage_key::StorageKey;
use types::{AppchainId, AppchainMetadata, AppchainState, RegistryRoles, RegistrySettings};

const NO_DEPOSIT: Balance = 0;
/// Initial balance for the AppchainAnchor contract to cover storage and related.
const APPCHAIN_ANCHOR_INIT_BALANCE: Balance = 23_000_000_000_000_000_000_000_000; // 23e24yN, 23 NEAR
const T_GAS: u64 = 1_000_000_000_000;
const GAS_FOR_FT_TRANSFER_CALL: u64 = 35 * T_GAS;
const OCT_DECIMALS_BASE: u128 = 1000_000_000_000_000_000;
/// Default register deposit amount
const DEFAULT_REGISTER_DEPOSIT: u128 = 1000;
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
pub trait ResolverForSelfCallback {
    /// Resolver for withdrawing the upvote deposit of a voter
    fn resolve_withdraw_upvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: U128,
    );
    /// Resolver for withdrawing the downvote deposit of a voter
    fn resolve_withdraw_downvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: U128,
    );
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
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
    /// The settings of appchain registry
    registry_settings: LazyOption<RegistrySettings>,
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
    /// The roles of appchain registry
    registry_roles: LazyOption<RegistryRoles>,
    /// Whether the asset transfer is paused
    asset_transfer_is_paused: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
enum RegistryDepositMessage {
    RegisterAppchain {
        appchain_id: String,
        website_url: String,
        function_spec_url: String,
        github_address: String,
        github_release: String,
        contact_email: String,
        premined_wrapped_appchain_token_beneficiary: AccountId,
        premined_wrapped_appchain_token: U128,
        initial_supply_of_wrapped_appchain_token: U128,
        ido_amount_of_wrapped_appchain_token: U128,
        initial_era_reward: U128,
        fungible_token_metadata: FungibleTokenMetadata,
        custom_metadata: HashMap<String, String>,
    },
    UpvoteAppchain {
        appchain_id: String,
    },
    DownvoteAppchain {
        appchain_id: String,
    },
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
            registry_settings: LazyOption::new(
                StorageKey::RegistrySettings.into_bytes(),
                Some(&RegistrySettings::default()),
            ),
            appchain_ids: UnorderedSet::new(StorageKey::AppchainIds.into_bytes()),
            appchain_basedatas: LookupMap::new(StorageKey::AppchainBasedatas.into_bytes()),
            upvote_deposits: LookupMap::new(StorageKey::UpvoteDeposits.into_bytes()),
            downvote_deposits: LookupMap::new(StorageKey::DownvoteDeposits.into_bytes()),
            top_appchain_id_in_queue: String::new(),
            total_stake: 0,
            time_of_last_count_voting_score: 0,
            registry_roles: LazyOption::new(
                StorageKey::RegistryRoles.into_bytes(),
                Some(&RegistryRoles::default()),
            ),
            asset_transfer_is_paused: false,
        }
    }
    // Assert the asset transfer is not paused.
    fn assert_asset_transfer_is_not_paused(&self) {
        assert!(
            !self.asset_transfer_is_paused,
            "The asset transfer in this contract has been paused."
        );
    }
    // Assert that the contract called by the owner.
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Function can only be called by owner."
        );
    }
    // Assert that the contract is called by appchain lifecycle manager.
    fn assert_appchain_lifecycle_manager(&self) {
        let registry_roles = self.registry_roles.get().unwrap();
        assert_eq!(
            env::predecessor_account_id(),
            registry_roles.appchain_lifecycle_manager,
            "Function can only be called by appchain lifecycle manager."
        );
    }
    // Assert that the contract is called by registry settings manager.
    fn assert_registry_settings_manager(&self) {
        let registry_roles = self.registry_roles.get().unwrap();
        assert_eq!(
            env::predecessor_account_id(),
            registry_roles.registry_settings_manager,
            "Function can only be called by registry settings manager."
        );
    }
    // Assert that the given account has no role in this contract.
    fn assert_account_has_no_role(&self, account: &AccountId) {
        let registry_roles = self.registry_roles.get().unwrap();
        assert!(
            !registry_roles.has_role(&account) && !account.eq(&self.owner),
            "The account already has role in contract."
        );
    }
    // Assert that the contract is called by the owner of the given appchain.
    fn assert_appchain_owner(&self, appchain_id: &AppchainId) {
        let appchain_basedata = self.get_appchain_basedata(appchain_id);
        assert_eq!(
            env::predecessor_account_id(),
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
}

#[near_bindgen]
impl AppchainRegistry {
    /// Callback function for `ft_transfer_call` of NEP-141 compatible contracts
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.assert_asset_transfer_is_not_paused();
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

        let parse_result = serde_json::from_str(msg.as_str());
        assert!(
            parse_result.is_ok(),
            "Invalid msg '{}' attached in `ft_transfer_call`. Refund deposit.",
            msg
        );
        let deposit_message = parse_result.unwrap();

        match deposit_message {
            RegistryDepositMessage::RegisterAppchain {
                appchain_id,
                website_url,
                function_spec_url,
                github_address,
                github_release,
                contact_email,
                premined_wrapped_appchain_token_beneficiary,
                premined_wrapped_appchain_token,
                initial_supply_of_wrapped_appchain_token,
                ido_amount_of_wrapped_appchain_token,
                initial_era_reward,
                fungible_token_metadata,
                custom_metadata,
            } => {
                self.register_appchain(
                    sender_id,
                    appchain_id,
                    amount.0,
                    website_url,
                    function_spec_url,
                    github_address,
                    github_release,
                    contact_email,
                    premined_wrapped_appchain_token_beneficiary,
                    premined_wrapped_appchain_token,
                    initial_supply_of_wrapped_appchain_token,
                    ido_amount_of_wrapped_appchain_token,
                    initial_era_reward,
                    fungible_token_metadata,
                    custom_metadata,
                );
                PromiseOrValue::Value(0.into())
            }
            RegistryDepositMessage::UpvoteAppchain { appchain_id } => {
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
            RegistryDepositMessage::DownvoteAppchain { appchain_id } => {
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
        }
    }
    //
    fn register_appchain(
        &mut self,
        sender_id: AccountId,
        appchain_id: AppchainId,
        register_deposit: Balance,
        website_url: String,
        function_spec_url: String,
        github_address: String,
        github_release: String,
        contact_email: String,
        premined_wrapped_appchain_token_beneficiary: AccountId,
        premined_wrapped_appchain_token: U128,
        initial_supply_of_wrapped_appchain_token: U128,
        ido_amount_of_wrapped_appchain_token: U128,
        initial_era_reward: U128,
        fungible_token_metadata: FungibleTokenMetadata,
        custom_metadata: HashMap<String, String>,
    ) {
        assert_ne!(
            sender_id, self.owner,
            "The register account should NOT be the contract owner."
        );
        assert!(
            self.appchain_basedatas.get(&appchain_id).is_none(),
            "Appchain already registered."
        );
        assert!(
            register_deposit.eq(&self
                .registry_settings
                .get()
                .unwrap()
                .minimum_register_deposit
                .0),
            "Invalid register deposit."
        );
        assert!(
            !appchain_id.trim().is_empty(),
            "Missing necessary field 'appchain_id'."
        );
        assert!(appchain_id.find(".").is_none(), "Invalid 'appchain_id'.");
        assert!(
            ValidAccountId::try_from(format!("{}.{}", appchain_id, env::current_account_id()))
                .is_ok(),
            "Invalid 'appchain_id'."
        );
        assert!(
            !website_url.trim().is_empty(),
            "Missing necessary field 'website_url'."
        );
        assert!(
            !function_spec_url.trim().is_empty(),
            "Missing necessary field 'function_spec_url'."
        );
        assert!(
            !github_address.trim().is_empty(),
            "Missing necessary field 'github_address'."
        );
        assert!(
            !github_release.trim().is_empty(),
            "Missing necessary field 'github_release'."
        );
        assert!(
            !contact_email.trim().is_empty(),
            "Missing necessary field 'contact_email'."
        );
        assert!(
            !premined_wrapped_appchain_token_beneficiary
                .trim()
                .is_empty(),
            "Missing necessary field 'premined_wrapped_appchain_token_beneficiary'."
        );
        fungible_token_metadata.assert_valid();
        assert!(
            !fungible_token_metadata.name.trim().is_empty(),
            "Missing necessary field 'fungible token name'."
        );
        assert!(
            !fungible_token_metadata.symbol.trim().is_empty(),
            "Missing necessary field 'fungible token symbol'."
        );
        assert!(
            initial_supply_of_wrapped_appchain_token.0 >= premined_wrapped_appchain_token.0,
            "The initial supply of wrapped appchain token should not be less than the premined amount."
        );
        let appchain_basedata = AppchainBasedata::new(
            appchain_id.clone(),
            AppchainMetadata {
                website_url,
                function_spec_url,
                github_address,
                github_release,
                contact_email,
                premined_wrapped_appchain_token_beneficiary,
                premined_wrapped_appchain_token,
                initial_supply_of_wrapped_appchain_token,
                ido_amount_of_wrapped_appchain_token,
                initial_era_reward,
                fungible_token_metadata,
                custom_metadata,
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
    //
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }
    //
    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        assert_ne!(owner, self.owner, "The account is the same.");
        self.owner = owner;
    }
}

impl AppchainRegistry {
    ///
    fn internal_remove_appchain(&mut self, appchain_id: &AppchainId) {
        env::storage_remove(&StorageKey::AppchainAnchorCode(appchain_id.clone()).into_bytes());
        env::storage_remove(&StorageKey::AppchainMetadata(appchain_id.clone()).into_bytes());
        env::storage_remove(&StorageKey::AppchainVotingScore(appchain_id.clone()).into_bytes());
        self.appchain_ids.remove(&appchain_id);
        self.appchain_basedatas.remove(&appchain_id);
    }
}
