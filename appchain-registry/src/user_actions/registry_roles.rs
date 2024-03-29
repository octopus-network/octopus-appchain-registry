use crate::*;

impl Default for RegistryRoles {
    fn default() -> Self {
        Self {
            appchain_lifecycle_manager: env::signer_account_id(),
            registry_settings_manager: env::signer_account_id(),
            octopus_council: None,
        }
    }
}

impl RegistryRoles {
    ///
    pub fn has_role(&self, account: &AccountId) -> bool {
        account.eq(&self.appchain_lifecycle_manager)
            || account.eq(&self.registry_settings_manager)
            || account.eq(&self
                .octopus_council
                .as_ref()
                .unwrap_or(&AccountId::new_unchecked(String::new())))
    }
}

#[near_bindgen]
impl AppchainRegistry {
    //
    pub fn change_appchain_lifecycle_manager(&mut self, account: AccountId) {
        let caller = env::predecessor_account_id();
        let mut registry_roles = self.registry_roles.get().unwrap();
        assert!(
            caller.eq(&registry_roles.appchain_lifecycle_manager) || caller.eq(&self.owner),
            "This function can only be called by appchain lifecycle manager or the contract owner."
        );
        self.assert_account_has_no_role(&account);
        registry_roles.appchain_lifecycle_manager = account;
        self.registry_roles.set(&registry_roles);
    }
    //
    pub fn change_registry_settings_manager(&mut self, account: AccountId) {
        let caller = env::predecessor_account_id();
        let mut registry_roles = self.registry_roles.get().unwrap();
        assert!(
            caller.eq(&registry_roles.registry_settings_manager) || caller.eq(&self.owner),
            "This function can only be called by registry settings manager or the contract owner."
        );
        self.assert_account_has_no_role(&account);
        registry_roles.registry_settings_manager = account;
        self.registry_roles.set(&registry_roles);
    }
    //
    pub fn change_octopus_council(&mut self, account: AccountId) {
        self.assert_owner();
        self.assert_account_has_no_role(&account);
        let mut registry_roles = self.registry_roles.get().unwrap();
        registry_roles.octopus_council = Some(account);
        self.registry_roles.set(&registry_roles);
    }
}
