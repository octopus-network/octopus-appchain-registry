use crate::{interfaces::AppchainLifecycleManager, types::AppchainId, *};
use near_sdk::AccountId;

#[near_bindgen]
impl AppchainLifecycleManager for AppchainRegistry {
    //
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        description: Option<String>,
        template_type: Option<AppchainTemplateType>,
        evm_chain_id: Option<U64>,
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
    fn start_auditing_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(&appchain_id, AppchainState::Registered);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.set_state(AppchainState::Auditing);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log!("Appchain '{}' is 'auditing'.", appchain_basedata.id());
    }
    //
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(&appchain_id, AppchainState::Auditing);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.set_state(AppchainState::InQueue);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log!("Appchain '{}' is 'inQueue'.", appchain_basedata.id());
    }
    //
    fn reject_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert!(
            appchain_basedata.state().eq(&AppchainState::Registered)
                || appchain_basedata.state().eq(&AppchainState::Auditing)
                || appchain_basedata.state().eq(&AppchainState::InQueue),
            "Appchain state must be 'registered', 'auditing' or 'inQueue'."
        );
        appchain_basedata.set_state(AppchainState::Dead);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
    }
    //
    fn conclude_voting_score(&mut self) {
        self.assert_appchain_lifecycle_manager();
        assert!(
            !self.top_appchain_id_in_queue.is_empty(),
            "There is no appchain on the top of queue yet."
        );
        // Set the appchain with the largest voting score to go `staging`
        let sub_account_id = AccountId::try_from(format!(
            "{}.{}",
            &self.top_appchain_id_in_queue,
            env::current_account_id()
        ));
        assert!(
            sub_account_id.is_ok(),
            "Invalid sub account id of target appchain '{}'.",
            self.top_appchain_id_in_queue
        );
        let sub_account_id = sub_account_id.unwrap();
        let mut top_appchain_basedata = self.get_appchain_basedata(&self.top_appchain_id_in_queue);
        top_appchain_basedata.set_state(AppchainState::Staging);
        top_appchain_basedata.set_anchor_account(sub_account_id.clone());
        self.appchain_basedatas
            .insert(top_appchain_basedata.id(), &top_appchain_basedata);
        let registry_settings = self.registry_settings.get().unwrap();
        // Reduce the voting score of all appchains in queue by the given percent
        for id in self.appchain_ids.to_vec() {
            let mut appchain_basedata = self.get_appchain_basedata(&id);
            if appchain_basedata.state().eq(&AppchainState::InQueue) {
                if appchain_basedata.voting_score() < 0 {
                    appchain_basedata.set_state(AppchainState::Dead);
                    self.appchain_basedatas
                        .insert(appchain_basedata.id(), &appchain_basedata);
                } else {
                    appchain_basedata.reduce_voting_score_by_percent(
                        registry_settings.voting_result_reduction_percent,
                    );
                }
            }
        }
        self.top_appchain_id_in_queue.clear();
        Promise::new(sub_account_id.clone())
            .create_account()
            .transfer(APPCHAIN_ANCHOR_INIT_BALANCE)
            .add_full_access_key(self.owner_pk.clone());
    }
    //
    fn remove_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(&appchain_id, AppchainState::Dead);
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

#[near_bindgen]
impl AppchainRegistry {
    //
    pub fn count_voting_score(&mut self) {
        let registry_settings = self.registry_settings.get().unwrap();
        let registry_roles = self.registry_roles.get().unwrap();
        assert!(
            registry_roles.operator_of_counting_voting_score.is_some(),
            "Operator for counting voting score is not set."
        );
        assert_eq!(
            env::predecessor_account_id(),
            registry_roles.operator_of_counting_voting_score.unwrap(),
            "Only certain operator can call this function."
        );
        assert!(
            env::block_timestamp() - self.time_of_last_count_voting_score
                > registry_settings.counting_interval_in_seconds.0 * NANO_SECONDS_MULTIPLE,
            "Count voting score can only be performed once in every {} seconds.",
            registry_settings.counting_interval_in_seconds.0
        );
        assert!(
            self.appchain_ids.len() > 0,
            "There is no appchain to count."
        );
        let mut top_appchain_id = self.top_appchain_id_in_queue.clone();
        for id in self.appchain_ids.to_vec() {
            let appchain_basedata = self.get_appchain_basedata(&id);
            if appchain_basedata.state().eq(&AppchainState::InQueue) {
                appchain_basedata.count_voting_score();
                if let Some(top_appchain_basedata) = self.appchain_basedatas.get(&top_appchain_id) {
                    if appchain_basedata.voting_score() > top_appchain_basedata.voting_score() {
                        top_appchain_id.clear();
                        top_appchain_id.push_str(&id);
                    }
                } else {
                    top_appchain_id.clear();
                    top_appchain_id.push_str(&id);
                }
            }
        }
        self.top_appchain_id_in_queue.clear();
        self.top_appchain_id_in_queue.push_str(&top_appchain_id);
        self.time_of_last_count_voting_score = env::block_timestamp()
            - (env::block_timestamp()
                % (registry_settings.counting_interval_in_seconds.0 * NANO_SECONDS_MULTIPLE));
    }
}
