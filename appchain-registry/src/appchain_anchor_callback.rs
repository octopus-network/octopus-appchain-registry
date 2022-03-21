use crate::{
    interfaces::AppchainAnchorCallback,
    types::{AppchainId, AppchainState},
    *,
};

#[near_bindgen]
impl AppchainAnchorCallback for AppchainRegistry {
    fn sync_state_of(
        &mut self,
        appchain_id: AppchainId,
        appchain_state: AppchainState,
        validator_count: u32,
        total_stake: U128,
    ) {
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert!(
            appchain_basedata.appchain_anchor.is_some(),
            "Anchor of appchain {} is not set.",
            appchain_id
        );
        assert_eq!(
            env::predecessor_account_id(),
            appchain_basedata.anchor().unwrap(),
            "Only appchain anchor can call this function."
        );
        assert!(
            appchain_state.is_managed_by_anchor(),
            "Invalid state to sync."
        );
        appchain_basedata.set_state(appchain_state);
        appchain_basedata.sync_staking_status(validator_count, total_stake.0);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
    }
}
