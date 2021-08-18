use near_sdk::json_types::U64;
use near_sdk::Gas;

use crate::types::AppchainId;

use crate::*;

const GAS_FOR_RESOLVE_ADDCHAIN_ANCHOR_DELETION: Gas = 2_500_000_000_000;

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
    /// Start auditing of an appchain
    fn start_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Reject an appchain
    fn reject_appchain(&mut self, appchain_id: AppchainId, refund_percent: U64);
    /// Count voting score of appchains
    fn count_voting_score(&mut self);
    /// Conclude voting score of appchains
    fn conclude_voting_score(&mut self, vote_result_reduction_percent: U64);
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

    fn reject_appchain(&mut self, appchain_id: AppchainId, refund_percent: U64) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Auditing);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::Dead);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        let refund_amount = appchain_basedata.register_deposit() * refund_percent.0 as u128 / 100;
        if refund_amount > 0 {
            ext_fungible_token::ft_transfer(
                appchain_basedata.owner().clone(),
                appchain_basedata.register_deposit().into(),
                None,
                &self.oct_token,
                1,
                GAS_FOR_FT_TRANSFER_CALL,
            )
            .then(ext_self::resolve_appchain_refunding(
                appchain_id,
                refund_amount,
                &env::current_account_id(),
                NO_DEPOSIT,
                env::prepaid_gas() / 2,
            ));
        }
    }

    fn count_voting_score(&mut self) {
        self.assert_owner();
        let ids = self.appchain_basedatas.keys().collect::<Vec<String>>();
        for id in ids {
            let mut appchain_basedata = self.get_appchain_basedata(&id);
            if appchain_basedata.state().eq(&AppchainState::InQueue) {
                appchain_basedata.count_voting_score();
                self.set_appchain_basedata(appchain_basedata.id(), &appchain_basedata);
                let top_appchain_basedata =
                    self.get_appchain_basedata(&self.top_appchain_id_in_queue);
                if appchain_basedata.voting_score() > top_appchain_basedata.voting_score() {
                    self.top_appchain_id_in_queue.clear();
                    self.top_appchain_id_in_queue.push_str(&id);
                }
            }
        }
    }

    fn conclude_voting_score(&mut self, voting_result_reduction_percent: U64) {
        self.assert_owner();
        assert!(
            !self.top_appchain_id_in_queue.is_empty(),
            "There is no appchain on the top of queue yet."
        );
        // Set the appchain with the largest voting score to go `staging`
        let mut top_appchain_basedata = self.get_appchain_basedata(&self.top_appchain_id_in_queue);
        top_appchain_basedata.change_state(AppchainState::Staging);
        self.set_appchain_basedata(top_appchain_basedata.id(), &top_appchain_basedata);
        // Reduce the voting score of all appchains in queue by the given percent
        let ids = self.appchain_basedatas.keys().collect::<Vec<String>>();
        for id in ids {
            let mut appchain_basedata = self.get_appchain_basedata(&id);
            if appchain_basedata.state().eq(&AppchainState::InQueue) {
                appchain_basedata.reduce_voting_score_by_percent(voting_result_reduction_percent.0);
                self.set_appchain_basedata(appchain_basedata.id(), &appchain_basedata);
            }
        }
        // Deploy contract of anchor of the appchain with the largest voting score, and initialize it.
        let sub_account_id = format!(
            "{}.{}",
            self.top_appchain_id_in_queue,
            env::current_account_id()
        );
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
            Promise::new(anchor_account_id)
                .delete_account(env::current_account_id())
                .then(ext_self::resolve_appchain_anchor_deletion(
                    appchain_id.clone(),
                    &env::current_account_id(),
                    0,
                    GAS_FOR_RESOLVE_ADDCHAIN_ANCHOR_DELETION,
                ));
        }
        self.appchain_basedatas.remove(&appchain_id);
        env::log(format!("Appchain '{}' is removed from registry.", &appchain_id).as_bytes())
    }
}

#[near_bindgen]
impl AppchainRegistry {
    ///
    pub fn resolve_appchain_anchor_deletion(&mut self, appchain_id: AppchainId) {
        assert_self();
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => env::log(
                format!("Anchor contract of appchain '{}' is deleted.", &appchain_id).as_bytes(),
            ),
            PromiseResult::Failed => env::log(
                format!(
                    "Failed to delete anchor contract of appchain '{}'.",
                    &appchain_id
                )
                .as_bytes(),
            ),
        }
    }
    ///
    pub fn resolve_appchain_refunding(&mut self, appchain_id: AppchainId, amount: Balance) {
        assert_self();
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => env::log(
                format!(
                    "Appchain '{}' is rejected, and '{}' OCT token returned.",
                    &appchain_id, &amount
                )
                .as_bytes(),
            ),
            PromiseResult::Failed => {}
        }
    }
}
