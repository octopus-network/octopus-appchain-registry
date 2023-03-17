mod appchain_basedata;
mod registry_status;
mod storage_key;
pub mod storage_migration;
pub mod types;
mod upgrade;
mod user_actions;

use core::convert::TryFrom;
use std::collections::HashMap;

use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_self, env, ext_contract, log, near_bindgen, serde_json, AccountId, Balance, Duration,
    Gas, PanicOnDefault, Promise, PromiseOrValue, PromiseResult, PublicKey, Timestamp,
};

use appchain_basedata::AppchainBasedata;
use storage_key::StorageKey;
use types::{
    AppchainId, AppchainMetadata, AppchainState, AppchainTemplateType, RegistryRoles,
    RegistrySettings,
};

const VERSION: &str = "v3.1.1";
/// Initial balance for the AppchainAnchor contract to cover storage and related.
const APPCHAIN_ANCHOR_INIT_BALANCE: Balance = 26_000_000_000_000_000_000_000_000;
const T_GAS_FOR_RESOLVER_FUNCTION: u64 = 10;
const T_GAS_FOR_FT_TRANSFER: u64 = 20;
const T_GAS_FOR_CALLING_ANCHOR_FUNCTION: u64 = 150;
const OCT_DECIMALS_BASE: u128 = 1000_000_000_000_000_000;
/// Default register deposit amount
const DEFAULT_REGISTER_DEPOSIT: u128 = 1000;
/// Multiple of nano seconds for a second
const NANO_SECONDS_MULTIPLE: u64 = 1_000_000_000;
/// Default staging duration of contract code for upgrade
const DEFAULT_CONTRACT_CODE_STAGING_DURATION: u64 = 3600 * 24;

const APPCHAIN_NOT_FOUND: &'static str = "Appchain not found.";

#[ext_contract(ext_self)]
pub trait SelfCallback {
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

/// The callback interface for appchain anchor
pub trait AppchainAnchorCallback {
    /// Sync state of an appchain to registry
    fn sync_state_of(
        &mut self,
        appchain_id: AppchainId,
        appchain_state: AppchainState,
        validator_count: u32,
        total_stake: U128,
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
    /// The total stake of OCT token in all appchains
    total_stake: Balance,
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
        description: String,
        template_type: AppchainTemplateType,
        evm_chain_id: Option<U64>,
        website_url: String,
        github_address: String,
        contact_email: String,
        premined_wrapped_appchain_token_beneficiary: AccountId,
        premined_wrapped_appchain_token: U128,
        initial_supply_of_wrapped_appchain_token: U128,
        ido_amount_of_wrapped_appchain_token: U128,
        initial_era_reward: U128,
        fungible_token_metadata: FungibleTokenMetadata,
        custom_metadata: HashMap<String, String>,
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
            total_stake: 0,
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
    //
    fn assert_octopus_council(&self) {
        let registry_roles = self.registry_roles.get().unwrap();
        assert!(
            registry_roles
                .octopus_council
                .expect("Octopus council account is not setup.")
                .eq(&env::predecessor_account_id()),
            "Only octopus council account can call this function."
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
    // Assert that the state of the given appchain is one of the given `AppchainState`s.
    fn assert_appchain_state(&self, appchain_id: &AppchainId, appchain_states: Vec<AppchainState>) {
        let appchain_basedata = self.get_appchain_basedata(appchain_id);
        assert!(
            appchain_states.contains(&appchain_basedata.state()),
            "Appchain state can NOT be '{}'.",
            appchain_basedata.state(),
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
                description,
                template_type,
                evm_chain_id,
                website_url,
                github_address,
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
                    description,
                    template_type,
                    evm_chain_id,
                    amount.0,
                    website_url,
                    github_address,
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
        }
    }
    //
    fn register_appchain(
        &mut self,
        sender_id: AccountId,
        appchain_id: AppchainId,
        description: String,
        template_type: AppchainTemplateType,
        evm_chain_id: Option<U64>,
        register_deposit: Balance,
        website_url: String,
        github_address: String,
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
            appchain_id.len() <= 20,
            "Appchain id is too long (max length is 20)."
        );
        assert!(
            AccountId::try_from(format!("{}.{}", appchain_id, env::current_account_id())).is_ok(),
            "Invalid 'appchain_id'."
        );
        assert!(
            !website_url.trim().is_empty(),
            "Missing necessary field 'website_url'."
        );
        assert!(
            !github_address.trim().is_empty(),
            "Missing necessary field 'github_address'."
        );
        assert!(
            !contact_email.trim().is_empty(),
            "Missing necessary field 'contact_email'."
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
        //
        let appchain_basedata = AppchainBasedata::new(
            appchain_id.clone(),
            evm_chain_id,
            AppchainMetadata {
                description,
                template_type,
                website_url,
                function_spec_url: String::new(),
                github_address,
                github_release: String::new(),
                contact_email,
                premined_wrapped_appchain_token_beneficiary: Some(
                    premined_wrapped_appchain_token_beneficiary,
                ),
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
        log!(
            "Appchain '{}' is registered by '{}'.",
            appchain_basedata.id(),
            appchain_basedata.owner()
        );
    }
    //
    pub fn call_anchor_function(
        &mut self,
        appchain_id: String,
        function_name: String,
        args: String,
    ) {
        self.assert_octopus_council();
        self.assert_appchain_state(
            &appchain_id,
            [AppchainState::Booting, AppchainState::Active].to_vec(),
        );
        //
        let anchor_account_id =
            AccountId::try_from(format!("{}.{}", &appchain_id, env::current_account_id())).unwrap();
        Promise::new(anchor_account_id).function_call(
            function_name,
            args.into_bytes(),
            0,
            Gas::ONE_TERA * T_GAS_FOR_CALLING_ANCHOR_FUNCTION,
        );
    }
}

#[near_bindgen]
impl AppchainRegistry {
    //
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }
    //
    pub fn set_owner(&mut self, owner: AccountId) {
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

#[near_bindgen]
impl AppchainAnchorCallback for AppchainRegistry {
    //
    fn sync_state_of(
        &mut self,
        appchain_id: AppchainId,
        appchain_state: AppchainState,
        validator_count: u32,
        total_stake: U128,
    ) {
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert!(
            appchain_basedata.anchor().is_some(),
            "Anchor of appchain {} is not set.",
            appchain_id
        );
        assert_eq!(
            env::predecessor_account_id(),
            appchain_basedata
                .anchor()
                .unwrap_or(AccountId::new_unchecked(String::new())),
            "Only appchain anchor can call this function."
        );
        assert!(
            appchain_state.is_managed_by_anchor(),
            "Invalid state to sync."
        );
        appchain_basedata.set_state(appchain_state);
        appchain_basedata.sync_staking_status(validator_count, total_stake.0);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
    }
}
