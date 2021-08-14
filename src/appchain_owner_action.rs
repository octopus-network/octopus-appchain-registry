use near_sdk::{env, near_bindgen, AccountId};

use crate::types::{AppchainMetadata, AppchainState};
use crate::*;

/// The actions which the owner of an appchain can perform
pub trait AppchainOwnerAction {
    /// Register an appchain
    fn register_appchain(
        &mut self,
        appchain_id: AppchainId,
        website_url: String,
        github_address: String,
        github_release: String,
        commit_id: String,
        contact_email: String,
    );
    /// Transfer ownership of an appchain to another account
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId);
    /// Cancel an appchain
    fn cancel_appchain(&mut self, appchain_id: AppchainId);
}

#[near_bindgen]
impl AppchainOwnerAction for AppchainRegistry {
    fn register_appchain(
        &mut self,
        appchain_id: AppchainId,
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
        let appchain_basedata = AppchainBasedata::new(
            appchain_id.clone(),
            AppchainMetadata {
                website_url,
                github_address,
                github_release,
                commit_id,
                contact_email,
            },
            env::predecessor_account_id(),
        );
        self.appchain_basedatas.insert(
            &appchain_id,
            &LazyOption::new(
                StorageKey::AppchainBasedata(appchain_id.clone()).into_bytes(),
                Option::from(&appchain_basedata),
            ),
        );
        env::log(
            format!(
                "Appchain '{}' is registered by '{}'.",
                appchain_basedata.id(),
                appchain_basedata.owner()
            )
            .as_bytes(),
        )
    }

    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId) {
        self.assert_appchain_owner(&appchain_id);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_owner(&new_owner);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(
            format!(
                "The ownership of appchain '{}' is transfered to '{}'.",
                appchain_basedata.id(),
                appchain_basedata.owner()
            )
            .as_bytes(),
        )
    }

    fn cancel_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_owner(&appchain_id);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert!(
            appchain_basedata.state() == AppchainState::Registered
                || appchain_basedata.state() == AppchainState::Auditing,
            "Can not be cancelled now"
        );
        appchain_basedata.change_state(AppchainState::Dead);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(
            format!(
                "Appchain '{}' is cancelled by '{}'.",
                appchain_basedata.id(),
                appchain_basedata.owner()
            )
            .as_bytes(),
        )
    }
}
