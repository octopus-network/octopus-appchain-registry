use std::{convert::TryFrom, str::FromStr};

use crate::{interfaces::SudoActions, *};

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
}
