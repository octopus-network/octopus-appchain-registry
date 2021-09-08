use std::convert::TryInto;

use near_sdk::json_types::U64;

use crate::types::AppchainId;

use crate::*;

/// The actions which the owner of appchain registry can perform
pub trait RegistryOwnerAction {
    /// Update metadata of an appchain
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        website_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        commit_id: Option<String>,
        contact_email: Option<String>,
        custom_metadata: Option<HashMap<String, String>>,
    );
    /// Change the value of minimum register deposit
    fn change_minimum_register_deposit(&mut self, value: U128);
    /// Change the value of reduction percent for voting result of all appchains still in queue
    fn change_voting_result_reduction_percent(&mut self, value: U64);
    /// Change the interval for counting voting score of appchains
    fn change_counting_interval_in_seconds(&mut self, value: U64);
    /// Start auditing of an appchain
    fn start_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Reject an appchain
    fn reject_appchain(&mut self, appchain_id: AppchainId);
    /// Count voting score of appchains
    fn count_voting_score(&mut self);
    /// Conclude voting score of appchains
    fn conclude_voting_score(&mut self);
    /// Remove an appchain from registry
    fn remove_appchain(&mut self, appchain_id: AppchainId);
}

#[near_bindgen]
impl RegistryOwnerAction for AppchainRegistry {
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        website_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        commit_id: Option<String>,
        contact_email: Option<String>,
        custom_metadata: Option<HashMap<String, String>>,
    ) {
        self.assert_owner();
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        if let Some(website_url) = website_url {
            appchain_basedata.metadata().website_url.clear();
            appchain_basedata
                .metadata()
                .website_url
                .push_str(&website_url);
        }
        if let Some(github_address) = github_address {
            appchain_basedata.metadata().github_address.clear();
            appchain_basedata
                .metadata()
                .github_address
                .push_str(&github_address);
        }
        if let Some(github_release) = github_release {
            appchain_basedata.metadata().github_release.clear();
            appchain_basedata
                .metadata()
                .github_release
                .push_str(&github_release);
        }
        if let Some(commit_id) = commit_id {
            appchain_basedata.metadata().commit_id.clear();
            appchain_basedata.metadata().commit_id.push_str(&commit_id);
        }
        if let Some(contact_email) = contact_email {
            appchain_basedata.metadata().contact_email.clear();
            appchain_basedata
                .metadata()
                .contact_email
                .push_str(&contact_email);
        }
        if let Some(custom_metadata) = custom_metadata {
            appchain_basedata.metadata().custom_metadata.clear();
            custom_metadata.keys().for_each(|key| {
                appchain_basedata
                    .metadata()
                    .custom_metadata
                    .insert(key.clone(), custom_metadata.get(key).unwrap().clone());
            });
        }
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(
            format!(
                "The metadata of appchain '{}' is updated by '{}'.",
                appchain_basedata.id(),
                env::predecessor_account_id()
            )
            .as_bytes(),
        )
    }

    fn change_minimum_register_deposit(&mut self, value: U128) {
        self.assert_owner();
        self.minimum_register_deposit = value.0;
    }

    fn change_voting_result_reduction_percent(&mut self, value: U64) {
        self.assert_owner();
        assert!(value.0 <= 100, "Invalid percent value.");
        if let Ok(value) = value.0.try_into() {
            self.voting_result_reduction_percent = value;
        }
    }

    fn change_counting_interval_in_seconds(&mut self, value: U64) {
        self.assert_owner();
        self.counting_interval_in_seconds = value.0;
    }

    fn start_auditing_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Registered);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::Auditing);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'auditing'.", appchain_basedata.id()).as_bytes())
    }

    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Auditing);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::InQueue);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'inQueue'.", appchain_basedata.id()).as_bytes())
    }

    fn reject_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert!(
            appchain_basedata.state().eq(&AppchainState::Registered)
                || appchain_basedata.state().eq(&AppchainState::Auditing)
                || appchain_basedata.state().eq(&AppchainState::InQueue),
            "Appchain state must be 'registered', 'auditing' or 'inQueue'."
        );
        appchain_basedata.change_state(AppchainState::Dead);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
    }

    fn count_voting_score(&mut self) {
        self.assert_owner();
        assert!(
            env::block_timestamp() - self.time_of_last_count_voting_score
                > self.counting_interval_in_seconds * NANO_SECONDS_MULTIPLE,
            "Count voting score can only be performed once a day."
        );
        let ids = self.appchain_basedatas.keys().collect::<Vec<String>>();
        assert!(ids.len() > 0, "There is no appchain to count.");
        let mut top_appchain_id = self.top_appchain_id_in_queue.clone();
        for id in ids {
            let appchain_basedata = self.get_appchain_basedata(&id);
            if appchain_basedata.state().eq(&AppchainState::InQueue) {
                appchain_basedata.count_voting_score();
                if let Some(top_appchain_basedata) = self.appchain_basedatas.get(&top_appchain_id) {
                    if let Some(top_appchain_basedata) = top_appchain_basedata.get() {
                        if appchain_basedata.voting_score() > top_appchain_basedata.voting_score() {
                            top_appchain_id.clear();
                            top_appchain_id.push_str(&id);
                        }
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
                % (self.counting_interval_in_seconds * NANO_SECONDS_MULTIPLE));
    }

    fn conclude_voting_score(&mut self) {
        self.assert_owner();
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
        top_appchain_basedata.change_state(AppchainState::Staging);
        top_appchain_basedata.set_anchor_account(&sub_account_id);
        self.set_appchain_basedata(top_appchain_basedata.id(), &top_appchain_basedata);
        // Reduce the voting score of all appchains in queue by the given percent
        let ids = self.appchain_basedatas.keys().collect::<Vec<String>>();
        for id in ids {
            let appchain_basedata = self.get_appchain_basedata(&id);
            if appchain_basedata.state().eq(&AppchainState::InQueue) {
                appchain_basedata
                    .reduce_voting_score_by_percent(self.voting_result_reduction_percent);
            }
        }
        // Deploy contract of anchor of the appchain with the largest voting score, and initialize it.
        env::storage_remove(
            &StorageKey::AppchainAnchorCode(self.top_appchain_id_in_queue.clone()).into_bytes(),
        );
        self.top_appchain_id_in_queue.clear();
        Promise::new(sub_account_id)
            .create_account()
            .transfer(APPCHAIN_ANCHOR_INIT_BALANCE)
            .add_full_access_key(self.owner_pk.clone());
    }

    fn remove_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Dead);
        let appchain_basedata = self.get_appchain_basedata(&appchain_id);
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
        self.appchain_basedatas.remove(&appchain_id);
        env::log(format!("Appchain '{}' is removed from registry.", &appchain_id).as_bytes())
    }
}
