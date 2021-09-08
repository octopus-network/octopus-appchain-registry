use crate::*;

pub trait SudoActions {
    /// Change account id of OCT token
    fn change_oct_token(&mut self, oct_token: AccountId);
    /// Go booting an appchain
    fn go_booting(&mut self, appchain_id: AppchainId);
}

#[near_bindgen]
impl SudoActions for AppchainRegistry {
    //
    fn change_oct_token(&mut self, oct_token: AccountId) {
        self.oct_token = oct_token;
    }
    //
    fn go_booting(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Staging);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::Booting);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'booting'.", appchain_basedata.id()).as_bytes())
    }
}
