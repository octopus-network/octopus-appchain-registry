use near_sdk::Gas;

use crate::types::AppchainId;

use crate::*;

const GAS_FOR_RESOLVE_ADDCHAIN_ANCHOR_DELETION: Gas = 2_500_000_000_000;

#[ext_contract(ext_self)]
trait AppchainAnchorDeletionResolver {
    fn resolve_appchain_anchor_deletion(&mut self, appchain_id: AppchainId);
}

/// The actions which the owner of appchain registry can perform
pub trait RegistryOwnerAction {
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId, code: Vec<u8>);
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
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId, code: Vec<u8>) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Auditing);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::InQueue);
        appchain_basedata.set_anchor_code(code);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'inQueue'.", appchain_basedata.id()).as_bytes())
    }

    fn reject_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Auditing);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::Dead);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'dead'.", appchain_basedata.id()).as_bytes())
    }

    fn count_voting_score(&mut self) {
        todo!()
    }

    fn conclude_voting_score(&mut self) {
        todo!()
    }

    fn remove_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Dead);
        let anchor_account_id = format!("{}.{}", &appchain_id, env::current_account_id());
        Promise::new(anchor_account_id)
            .delete_account(env::current_account_id())
            .then(ext_self::resolve_appchain_anchor_deletion(
                appchain_id,
                &env::current_account_id(),
                0,
                GAS_FOR_RESOLVE_ADDCHAIN_ANCHOR_DELETION,
            ));
    }
}

#[near_bindgen]
impl AppchainRegistry {
    pub fn resolve_appchain_anchor_deletion(&mut self, appchain_id: AppchainId) {
        assert_self();
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                self.appchain_basedatas.remove(&appchain_id);
                env::log(
                    format!("Appchain '{}' is removed from registry.", &appchain_id).as_bytes(),
                )
            }
            PromiseResult::Failed => {}
        }
    }
}
