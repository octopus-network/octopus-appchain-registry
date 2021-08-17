use crate::types::{AppchainId, AppchainState};
use crate::*;

/// The callback interface for appchain anchor
pub trait AppchainAnchorCallback {
    /// Sync state of an appchain to registry
    fn sync_state_of(&mut self, appchain_id: AppchainId, appchain_state: AppchainState);
}

#[near_bindgen]
impl AppchainAnchorCallback for AppchainRegistry {
    fn sync_state_of(&mut self, appchain_id: AppchainId, appchain_state: AppchainState) {
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert_eq!(
            env::predecessor_account_id(),
            appchain_basedata.anchor().clone(),
            "Only appchain anchor can call this function."
        );
        assert!(
            appchain_state.is_managed_by_anchor(),
            "Invalid state to sync."
        );
        appchain_basedata.change_state(appchain_state);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
    }
}
