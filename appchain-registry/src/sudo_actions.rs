use std::convert::TryFrom;

use near_sdk::json_types::Base58PublicKey;

use crate::{interfaces::SudoActions, *};

#[near_bindgen]
impl SudoActions for AppchainRegistry {
    //
    fn set_owner_pk(&mut self, public_key: String) {
        self.assert_owner();
        let parse_result = Base58PublicKey::try_from(public_key);
        assert!(parse_result.is_ok(), "Invalid public key.");
        self.owner_pk = parse_result.unwrap().0;
    }
    //
    fn create_anchor_account(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        let sub_account_id = format!("{}.{}", &appchain_id, env::current_account_id());
        Promise::new(sub_account_id)
            .create_account()
            .transfer(APPCHAIN_ANCHOR_INIT_BALANCE)
            .add_full_access_key(self.owner_pk.clone());
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
