use near_sdk::{env, near_bindgen, AccountId};

use crate::*;

/// The actions which the owner of an appchain can perform
pub trait AppchainOwnerActions {
    /// Transfer ownership of an appchain to another account
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId);
}

#[near_bindgen]
impl AppchainOwnerActions for AppchainRegistry {
    //
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId) {
        self.assert_appchain_owner(&appchain_id);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_owner(&new_owner);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
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
