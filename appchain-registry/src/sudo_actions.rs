use crate::{interfaces::SudoActions, *};
use std::{convert::TryFrom, str::FromStr};

#[near_bindgen]
impl SudoActions for AppchainRegistry {
    //
    fn set_owner_pk(&mut self, public_key: String) {
        self.assert_owner();
        let parse_result = PublicKey::from_str(public_key.as_str());
        assert!(parse_result.is_ok(), "Invalid public key.");
        self.owner_pk = parse_result.unwrap();
    }
    //
    fn create_anchor_account(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        let sub_account_id =
            AccountId::try_from(format!("{}.{}", &appchain_id, env::current_account_id()));
        assert!(
            sub_account_id.is_ok(),
            "Invalid sub account id for appchain '{}'.",
            appchain_id
        );
        Promise::new(sub_account_id.unwrap())
            .create_account()
            .transfer(APPCHAIN_ANCHOR_INIT_BALANCE)
            .add_full_access_key(self.owner_pk.clone());
    }
    //
    fn force_change_appchain_state(&mut self, appchain_id: AppchainId, new_state: AppchainState) {
        self.assert_owner();
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        assert!(
            !appchain_basedata.state().eq(&new_state),
            "Appchain is already in state '{}'",
            &new_state
        );
        appchain_basedata.set_state(new_state);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
    }
    //
    fn pause_asset_transfer(&mut self) {
        self.assert_owner();
        self.asset_transfer_is_paused = true;
    }
    //
    fn resume_asset_transfer(&mut self) {
        self.assert_owner();
        self.asset_transfer_is_paused = false;
    }
    //
    fn set_evm_chain_id_of_appchain(&mut self, appchain_id: String, evm_chain_id: U64) {
        self.assert_owner();
        if let Some(mut appchain_basedata) = self.appchain_basedatas.get(&appchain_id) {
            appchain_basedata.evm_chain_id = Some(evm_chain_id);
            self.appchain_basedatas
                .insert(&appchain_id, &appchain_basedata);
        } else {
            panic!("Appchain id is not existed.");
        }
    }
    fn force_remove_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Dead);
        let appchain_basedata = self.get_appchain_basedata(&appchain_id);
        if !appchain_basedata.anchor().is_none() {
            let anchor_account_id = format!("{}.{}", &appchain_id, env::current_account_id());
            log!(
                "The anchor contract '{}' of appchain '{}' needs to be removed manually.",
                &anchor_account_id,
                &appchain_id
            );
        }
        self.internal_remove_appchain(&appchain_id);
        log!("Appchain '{}' is removed from registry.", &appchain_id);
    }
}
