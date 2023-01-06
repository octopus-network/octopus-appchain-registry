use crate::{types::AppchainId, *};
use near_sdk::AccountId;

pub trait AppchainLifecycleManager {
    /// Update metadata of an appchain
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        description: Option<String>,
        template_type: Option<AppchainTemplateType>,
        evm_chain_id: Option<U64>,
        dao_proposal_url: Option<String>,
        website_url: Option<String>,
        function_spec_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        contact_email: Option<String>,
        premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
        premined_wrapped_appchain_token: Option<U128>,
        initial_supply_of_wrapped_appchain_token: Option<U128>,
        ido_amount_of_wrapped_appchain_token: Option<U128>,
        initial_era_reward: Option<U128>,
        fungible_token_metadata: Option<FungibleTokenMetadata>,
        custom_metadata: Option<HashMap<String, String>>,
    );
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Reject an appchain
    fn reject_appchain(&mut self, appchain_id: AppchainId);
    /// Start voting of an appchain
    fn start_voting_appchain(&mut self, appchain_id: AppchainId, dao_proposal_url: String);
    /// Change the state of a given appchain to 'booting',
    /// create sub-account for the appchain and transfer the initial deposit
    fn start_booting_appchain(&mut self, appchain_id: AppchainId);
    /// Remove an appchain from registry
    fn remove_appchain(&mut self, appchain_id: AppchainId);
}

#[near_bindgen]
impl AppchainLifecycleManager for AppchainRegistry {
    //
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        description: Option<String>,
        template_type: Option<AppchainTemplateType>,
        evm_chain_id: Option<U64>,
        dao_proposal_url: Option<String>,
        website_url: Option<String>,
        function_spec_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        contact_email: Option<String>,
        premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
        premined_wrapped_appchain_token: Option<U128>,
        initial_supply_of_wrapped_appchain_token: Option<U128>,
        ido_amount_of_wrapped_appchain_token: Option<U128>,
        initial_era_reward: Option<U128>,
        fungible_token_metadata: Option<FungibleTokenMetadata>,
        custom_metadata: Option<HashMap<String, String>>,
    ) {
        self.assert_appchain_lifecycle_manager();
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        let mut metadata = appchain_basedata.metadata();
        if let Some(description) = description {
            assert!(
                !metadata.description.eq(&description),
                "The description is not changed."
            );
            metadata.description = description;
        }
        if let Some(template_type) = template_type {
            assert!(
                !metadata.template_type.eq(&template_type),
                "The template type is not changed."
            );
            metadata.template_type = template_type;
        }
        if let Some(evm_chain_id) = evm_chain_id {
            assert!(
                !appchain_basedata.evm_chain_id.unwrap_or(U64::from(0)).0 == evm_chain_id.0,
                "The evm chain id is not changed."
            );
            appchain_basedata.evm_chain_id = Some(evm_chain_id);
        }
        if let Some(dao_proposal_url) = dao_proposal_url {
            assert!(
                !appchain_basedata
                    .dao_proposal_url
                    .unwrap_or(String::new())
                    .eq(&dao_proposal_url),
                "The website url is not changed."
            );
            appchain_basedata.dao_proposal_url = Some(dao_proposal_url);
        }
        if let Some(website_url) = website_url {
            assert!(
                !metadata.website_url.eq(&website_url),
                "The website url is not changed."
            );
            metadata.website_url = website_url;
        }
        if let Some(function_spec_url) = function_spec_url {
            assert!(
                !metadata.function_spec_url.eq(&function_spec_url),
                "The function spec url is not changed."
            );
            metadata.function_spec_url = function_spec_url;
        }
        if let Some(github_address) = github_address {
            assert!(
                !metadata.github_address.eq(&github_address),
                "The github address is not changed."
            );
            metadata.github_address = github_address;
        }
        if let Some(github_release) = github_release {
            assert!(
                !metadata.github_release.eq(&github_release),
                "The github release is not changed."
            );
            metadata.github_release = github_release;
        }
        if let Some(contact_email) = contact_email {
            assert!(
                !metadata.contact_email.eq(&contact_email),
                "The contact email is not changed."
            );
            metadata.contact_email = contact_email;
        }
        if let Some(premined_wrapped_appchain_token_beneficiary) =
            premined_wrapped_appchain_token_beneficiary
        {
            assert!(
                !metadata
                    .premined_wrapped_appchain_token_beneficiary
                    .map_or(AccountId::new_unchecked("".to_string()), |f| f)
                    .eq(&premined_wrapped_appchain_token_beneficiary),
                "The premined wrapped appchain token beneficiary is not changed."
            );
            metadata.premined_wrapped_appchain_token_beneficiary =
                Some(premined_wrapped_appchain_token_beneficiary);
        }
        if let Some(premined_wrapped_appchain_token) = premined_wrapped_appchain_token {
            assert!(
                !metadata
                    .premined_wrapped_appchain_token
                    .eq(&premined_wrapped_appchain_token),
                "The premined wrapped appchain token is not changed."
            );
            metadata.premined_wrapped_appchain_token = premined_wrapped_appchain_token;
        }
        if let Some(initial_supply_of_wrapped_appchain_token) =
            initial_supply_of_wrapped_appchain_token
        {
            assert!(
                !metadata
                    .initial_supply_of_wrapped_appchain_token
                    .eq(&initial_supply_of_wrapped_appchain_token),
                "The initial supply of wrapped appchain token is not changed."
            );
            assert!(
                initial_supply_of_wrapped_appchain_token.0 >= metadata.premined_wrapped_appchain_token.0,
                "The initial supply of wrapped appchain token should not be less than the premined amount."
            );
            metadata.initial_supply_of_wrapped_appchain_token =
                initial_supply_of_wrapped_appchain_token;
        }
        if let Some(ido_amount_of_wrapped_appchain_token) = ido_amount_of_wrapped_appchain_token {
            assert!(
                !metadata
                    .ido_amount_of_wrapped_appchain_token
                    .eq(&ido_amount_of_wrapped_appchain_token),
                "The ido amount of wrapped appchain token is not changed."
            );
            metadata.ido_amount_of_wrapped_appchain_token = ido_amount_of_wrapped_appchain_token;
        }
        if let Some(initial_era_reward) = initial_era_reward {
            assert!(
                !metadata.initial_era_reward.eq(&initial_era_reward),
                "The initial era reward is not changed."
            );
            metadata.initial_era_reward = initial_era_reward;
        }
        if let Some(fungible_token_metadata) = fungible_token_metadata {
            metadata.fungible_token_metadata = fungible_token_metadata;
        }
        if let Some(custom_metadata) = custom_metadata {
            metadata.custom_metadata = custom_metadata;
        }
        appchain_basedata.set_metadata(metadata);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log!(
            "The metadata of appchain '{}' is updated by '{}'.",
            appchain_basedata.id(),
            env::predecessor_account_id()
        );
    }
    //
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(&appchain_id, [AppchainState::Registered].to_vec());
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.set_state(AppchainState::Audited);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log_appchain_state(&appchain_basedata);
    }
    //
    fn reject_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(
            &appchain_id,
            [
                AppchainState::Registered,
                AppchainState::Audited,
                AppchainState::Voting,
            ]
            .to_vec(),
        );
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.set_state(AppchainState::Closed);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log_appchain_state(&appchain_basedata);
    }
    //
    fn start_voting_appchain(&mut self, appchain_id: AppchainId, dao_proposal_url: String) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(&appchain_id, [AppchainState::Audited].to_vec());
        assert!(
            !dao_proposal_url.trim().is_empty(),
            "The DAO proposal url can not be blank."
        );
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.set_state(AppchainState::Voting);
        appchain_basedata.dao_proposal_url = Some(dao_proposal_url);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log_appchain_state(&appchain_basedata);
    }
    //
    fn start_booting_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_octopus_council();
        self.assert_appchain_state(&appchain_id, [AppchainState::Voting].to_vec());
        //
        let sub_account_id =
            AccountId::try_from(format!("{}.{}", &appchain_id, env::current_account_id())).unwrap();
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.set_state(AppchainState::Booting);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log_appchain_state(&appchain_basedata);
        //
        Promise::new(sub_account_id.clone())
            .create_account()
            .transfer(APPCHAIN_ANCHOR_INIT_BALANCE)
            .add_full_access_key(self.owner_pk.clone());
    }
    //
    fn remove_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(&appchain_id, [AppchainState::Closed].to_vec());
        let appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert!(
            appchain_basedata.upvote_deposit() == 0,
            "The appchain still has upvote deposit(s)."
        );
        assert!(
            appchain_basedata.downvote_deposit() == 0,
            "The appchain still has downvote deposit(s)."
        );
        if !appchain_basedata.anchor().is_none() {
            let anchor_account_id = format!("{}.{}", &appchain_id, env::current_account_id());
            log!(
                "The anchor contract '{}' of appchain '{}' needs to be removed manually.",
                &anchor_account_id,
                &appchain_id
            );
        }
        self.internal_remove_appchain(&appchain_id);
        log!("Appchain '{}' is removed from registry.", &appchain_id);
    }
}

fn log_appchain_state(appchain_basedata: &AppchainBasedata) {
    log!(
        "Appchain '{}' is '{}'.",
        appchain_basedata.id(),
        appchain_basedata.appchain_state
    );
}
