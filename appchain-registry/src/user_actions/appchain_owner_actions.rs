use crate::*;
use near_sdk::{near_bindgen, AccountId};

/// The actions which the owner of an appchain can perform
pub trait AppchainOwnerActions {
    /// Transfer ownership of an appchain to another account.
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId);
    /// Withdraw the go live request of an appchain.
    /// Can be called by the appchain owner while the appchain state is 'registered' or 'audited'.
    /// After the withdrawal, the appchain's state will change to 'Closed'.
    fn withdraw_appchain(&mut self, appchain_id: AppchainId);
}

#[near_bindgen]
impl AppchainOwnerActions for AppchainRegistry {
    //
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId) {
        self.assert_appchain_owner(&appchain_id);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_owner(new_owner);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log!(
            "The ownership of appchain '{}' is transfered to '{}'.",
            appchain_basedata.id(),
            appchain_basedata.owner()
        );
    }
    //
    fn withdraw_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_appchain_owner(&appchain_id);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert!(
            appchain_basedata.state().eq(&AppchainState::Registered)
                || appchain_basedata.state().eq(&AppchainState::Audited),
            "Appchain state must be '{}' or '{}'.",
            AppchainState::Registered,
            AppchainState::Audited,
        );
        appchain_basedata.appchain_state = AppchainState::Closed;
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        log!(
            "Go live request of appchain '{}' is withdrawn by '{}'.",
            appchain_basedata.id(),
            appchain_basedata.owner()
        );
    }
}
