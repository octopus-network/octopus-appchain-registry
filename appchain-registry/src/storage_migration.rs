use crate::{types::SubstrateTemplateType, *};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, PublicKey, Timestamp};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OldAppchainMetadata {
    pub description: String,
    pub template_type: SubstrateTemplateType,
    pub website_url: String,
    pub function_spec_url: String,
    pub github_address: String,
    pub github_release: String,
    pub contact_email: String,
    pub premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
    pub premined_wrapped_appchain_token: U128,
    pub initial_supply_of_wrapped_appchain_token: U128,
    pub ido_amount_of_wrapped_appchain_token: U128,
    pub initial_era_reward: U128,
    pub fungible_token_metadata: FungibleTokenMetadata,
    pub custom_metadata: HashMap<String, String>,
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

#[near_bindgen]
impl AppchainRegistry {
    #[init(ignore_state)]
    pub fn migrate_state() -> Self {
        // Deserialize the state using the old contract structure.
        let old_contract: OldAppchainRegistry = env::state_read().expect("Old state doesn't exist");
        //
        assert_self();
        //
        // Create the new contract using the data from the old contract.
        let new_appchain_registry = AppchainRegistry {
            owner: old_contract.owner,
            owner_pk: old_contract.owner_pk,
            contract_code_staging_timestamp: old_contract.contract_code_staging_timestamp,
            contract_code_staging_duration: old_contract.contract_code_staging_duration,
            oct_token: old_contract.oct_token,
            registry_settings: old_contract.registry_settings,
            appchain_ids: old_contract.appchain_ids,
            appchain_basedatas: old_contract.appchain_basedatas,
            upvote_deposits: old_contract.upvote_deposits,
            downvote_deposits: old_contract.downvote_deposits,
            total_stake: old_contract.total_stake,
            registry_roles: old_contract.registry_roles,
            asset_transfer_is_paused: old_contract.asset_transfer_is_paused,
        };
        //
        //
        new_appchain_registry
    }
}

pub fn get_storage_key_in_lookup_array<T: BorshSerialize>(
    prefix: &StorageKey,
    index: &T,
) -> Vec<u8> {
    [prefix.into_bytes(), index.try_to_vec().unwrap()].concat()
}
