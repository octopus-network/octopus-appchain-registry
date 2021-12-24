use crate::{interfaces::AppchainLifecycleManager, types::AppchainId, *};

#[near_bindgen]
impl AppchainLifecycleManager for AppchainRegistry {
    //
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        website_url: Option<String>,
        function_spec_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        contact_email: Option<String>,
        premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
        premined_wrapped_appchain_token: Option<U128>,
        ido_amount_of_wrapped_appchain_token: Option<U128>,
        initial_era_reward: Option<U128>,
        fungible_token_metadata: Option<FungibleTokenMetadata>,
        custom_metadata: Option<HashMap<String, String>>,
    ) {
        self.assert_appchain_lifecycle_manager();
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        let mut metadata = appchain_basedata.metadata();
        if let Some(website_url) = website_url {
            metadata.website_url = website_url;
        }
        if let Some(function_spec_url) = function_spec_url {
            metadata.function_spec_url = function_spec_url;
        }
        if let Some(github_address) = github_address {
            metadata.github_address = github_address;
        }
        if let Some(github_release) = github_release {
            metadata.github_release = github_release;
        }
        if let Some(contact_email) = contact_email {
            metadata.contact_email = contact_email;
        }
        if let Some(premined_wrapped_appchain_token_beneficiary) =
            premined_wrapped_appchain_token_beneficiary
        {
            metadata.premined_wrapped_appchain_token_beneficiary =
                premined_wrapped_appchain_token_beneficiary;
        }
        if let Some(premined_wrapped_appchain_token) = premined_wrapped_appchain_token {
            metadata.premined_wrapped_appchain_token = premined_wrapped_appchain_token;
        }
        if let Some(ido_amount_of_wrapped_appchain_token) = ido_amount_of_wrapped_appchain_token {
            metadata.ido_amount_of_wrapped_appchain_token = ido_amount_of_wrapped_appchain_token;
        }
        if let Some(initial_era_reward) = initial_era_reward {
            metadata.initial_era_reward = initial_era_reward;
        }
        if let Some(fungible_token_metadata) = fungible_token_metadata {
            metadata.fungible_token_metadata = fungible_token_metadata;
        }
        if let Some(custom_metadata) = custom_metadata {
            metadata.custom_metadata = custom_metadata;
        }
        appchain_basedata.set_metadata(&metadata);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        env::log(
            format!(
                "The metadata of appchain '{}' is updated by '{}'.",
                appchain_basedata.id(),
                env::predecessor_account_id()
            )
            .as_bytes(),
        )
    }
    //
    fn start_auditing_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(&appchain_id, AppchainState::Registered);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.set_state(AppchainState::Auditing);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'auditing'.", appchain_basedata.id()).as_bytes())
    }
    //
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_lifecycle_manager();
        self.assert_appchain_state(&appchain_id, AppchainState::Auditing);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.set_state(AppchainState::InQueue);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'inQueue'.", appchain_basedata.id()).as_bytes())
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
        let sub_account_id = format!(
            "{}.{}",
            &self.top_appchain_id_in_queue,
            env::current_account_id()
        );
        let mut top_appchain_basedata = self.get_appchain_basedata(&self.top_appchain_id_in_queue);
        top_appchain_basedata.set_state(AppchainState::Staging);
        top_appchain_basedata.set_anchor_account(&sub_account_id);
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
        Promise::new(sub_account_id)
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
        if !appchain_basedata.anchor().trim().is_empty() {
            let anchor_account_id = format!("{}.{}", &appchain_id, env::current_account_id());
            env::log(
                format!(
                    "The anchor contract '{}' of appchain '{}' needs to be removed manually.",
                    &anchor_account_id, &appchain_id
                )
                .as_bytes(),
            );
        }
        self.internal_remove_appchain(&appchain_id);
        env::log(format!("Appchain '{}' is removed from registry.", &appchain_id).as_bytes())
    }
}

#[near_bindgen]
impl AppchainRegistry {
    //
    pub fn count_voting_score(&mut self) {
        let registry_settings = self.registry_settings.get().unwrap();
        let registry_roles = self.registry_roles.get().unwrap();
        assert_eq!(
            env::predecessor_account_id(),
            registry_roles.operator_of_counting_voting_score,
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
