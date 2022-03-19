use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, PublicKey, Timestamp};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OldAppchainMetadata {
    pub website_url: String,
    pub function_spec_url: String,
    pub github_address: String,
    pub github_release: String,
    pub contact_email: String,
    pub premined_wrapped_appchain_token_beneficiary: AccountId,
    pub premined_wrapped_appchain_token: U128,
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
        //
        // Create the new contract using the data from the old contract.
        let new_appchain_registry = AppchainRegistry {
            owner: old_contract.owner.clone(),
            owner_pk: old_contract.owner_pk,
            contract_code_staging_timestamp: old_contract.contract_code_staging_timestamp,
            contract_code_staging_duration: old_contract.contract_code_staging_duration,
            oct_token: old_contract.oct_token,
            registry_settings: old_contract.registry_settings,
            appchain_ids: old_contract.appchain_ids,
            appchain_basedatas: old_contract.appchain_basedatas,
            upvote_deposits: old_contract.upvote_deposits,
            downvote_deposits: old_contract.downvote_deposits,
            top_appchain_id_in_queue: old_contract.top_appchain_id_in_queue,
            total_stake: old_contract.total_stake,
            time_of_last_count_voting_score: old_contract.time_of_last_count_voting_score,
            registry_roles: old_contract.registry_roles,
            asset_transfer_is_paused: old_contract.asset_transfer_is_paused,
        };
        //
        let appchain_ids = new_appchain_registry.appchain_ids.to_vec();
        appchain_ids.iter().for_each(|appchain_id| {
            let old_metadata = OldAppchainMetadata::try_from_slice(
                &env::storage_read(&StorageKey::AppchainMetadata(appchain_id.clone()).into_bytes())
                    .unwrap(),
            )
            .unwrap();
            env::storage_write(
                &StorageKey::AppchainMetadata(appchain_id.clone()).into_bytes(),
                &AppchainMetadata::from_old_version(old_metadata)
                    .try_to_vec()
                    .unwrap(),
            );
        });
        //
        new_appchain_registry
    }
}

impl AppchainMetadata {
    pub fn from_old_version(old_version: OldAppchainMetadata) -> Self {
        Self {
            website_url: old_version.website_url,
            function_spec_url: old_version.function_spec_url,
            github_address: old_version.github_address,
            github_release: old_version.github_release,
            contact_email: old_version.contact_email,
            premined_wrapped_appchain_token_beneficiary: old_version
                .premined_wrapped_appchain_token_beneficiary,
            premined_wrapped_appchain_token: old_version.premined_wrapped_appchain_token,
            initial_supply_of_wrapped_appchain_token: old_version.premined_wrapped_appchain_token,
            ido_amount_of_wrapped_appchain_token: old_version.ido_amount_of_wrapped_appchain_token,
            initial_era_reward: old_version.initial_era_reward,
            fungible_token_metadata: old_version.fungible_token_metadata,
            custom_metadata: old_version.custom_metadata,
        }
    }
}
