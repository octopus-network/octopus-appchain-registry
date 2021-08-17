use near_sdk::{env, near_bindgen, AccountId};

use crate::*;

/// The actions which the owner of an appchain can perform
pub trait AppchainOwnerAction {
    /// Update metadata of an appchain
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        website_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        commit_id: Option<String>,
        contact_email: Option<String>,
    );
    /// Transfer ownership of an appchain to another account
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId);
}

#[near_bindgen]
impl AppchainOwnerAction for AppchainRegistry {
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        website_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        commit_id: Option<String>,
        contact_email: Option<String>,
    ) {
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
}
