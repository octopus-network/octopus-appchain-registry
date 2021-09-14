use crate::*;

pub trait SudoActions {
    /// Change account id of OCT token
    fn change_oct_token(&mut self, oct_token: AccountId);
    /// Delete an appchain whatever its state is
    fn delete_appchain(&mut self, appchain_id: AppchainId);
    /// Go booting an appchain
    fn go_booting(&mut self, appchain_id: AppchainId);
    /// Add an appchain id to set appchain_ids
    fn add_appchain_id(&mut self, appchain_id: AppchainId);
}

#[near_bindgen]
impl SudoActions for AppchainRegistry {
    //
    fn change_oct_token(&mut self, oct_token: AccountId) {
        self.oct_token = oct_token;
    }
    //
    fn delete_appchain(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        env::storage_remove(&StorageKey::AppchainVotingScore(appchain_id.clone()).into_bytes());
        self.appchain_ids.remove(&appchain_id);
        self.appchain_basedatas.remove(&appchain_id);
        env::log(format!("Appchain '{}' is removed from registry.", &appchain_id).as_bytes())
    }
    //
    fn go_booting(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.assert_appchain_state(&appchain_id, AppchainState::Staging);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_state(AppchainState::Booting);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        env::log(format!("Appchain '{}' is 'booting'.", appchain_basedata.id()).as_bytes())
    }
    //
    fn add_appchain_id(&mut self, appchain_id: AppchainId) {
        self.assert_owner();
        self.appchain_ids.insert(&appchain_id);
    }
}
