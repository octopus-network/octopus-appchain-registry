use std::convert::TryFrom;

use near_sdk::json_types::Base58PublicKey;

use crate::*;

pub trait SudoActions {
    /// Change account id of OCT token
    fn change_oct_token(&mut self, oct_token: AccountId);
    /// Set public key of owner
    fn set_owner_pk(&mut self, public_key: String);
}

#[near_bindgen]
impl SudoActions for AppchainRegistry {
    //
    fn change_oct_token(&mut self, oct_token: AccountId) {
        self.assert_owner();
        self.oct_token = oct_token;
    }
    //
    fn set_owner_pk(&mut self, public_key: String) {
        self.assert_owner();
        let parse_result = Base58PublicKey::try_from(public_key);
        assert!(parse_result.is_ok(), "Invalid public key.");
        self.owner_pk = parse_result.unwrap().0;
    }
}
