use near_sdk::Gas;

use crate::types::AppchainId;

use crate::*;

const GAS_FOR_RESOLVE_ADDCHAIN_ANCHOR_DELETION: Gas = 2_500_000_000_000;

/// The actions which the owner of appchain registry can perform
pub trait RegistryOwnerAction {
    /// Change the value of minimum register deposit
    fn change_minimum_register_deposit(&mut self, value: Balance);
    /// Start auditing of an appchain
    fn start_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId, appchain_anthor_code: Vec<u8>);
    /// Reject an appchain
    fn reject_appchain(&mut self, appchain_id: AppchainId, refund_percent: u8);
    /// Count voting score of appchains
    fn count_voting_score(&mut self);
    /// Conclude voting score of appchains
    fn conclude_voting_score(&mut self);
    /// Remove an appchain from registry
    fn remove_appchain(&mut self, appchain_id: AppchainId);
}

#[near_bindgen]
impl RegistryOwnerAction for AppchainRegistry {
    fn change_minimum_register_deposit(&mut self, value: Balance) {
        self.assert_owner();
        self.minimum_register_deposit = value;
    }

    fn start_auditing_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Registered);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::Auditing);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'auditing'.", appchain_basedata.id()).as_bytes())
    }

    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId, appchain_anthor_code: Vec<u8>) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Auditing);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::InQueue);
        appchain_basedata.set_anchor_code(appchain_anthor_code);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'inQueue'.", appchain_basedata.id()).as_bytes())
    }

    fn reject_appchain(&mut self, appchain_id: AppchainId, refund_percent: u8) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Auditing);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::Dead);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        let refund_amount = appchain_basedata.register_deposit() * refund_percent as u128 / 100;
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
    ///
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
