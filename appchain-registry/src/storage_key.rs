use near_sdk::AccountId;

use crate::AppchainId;

/// Storage keys for collections of sub-struct in main contract
pub enum StorageKey {
    AppchainManagers,
    AppchainManager(AppchainId),
}

impl StorageKey {
    pub fn to_string(&self) -> String {
        match self {
            StorageKey::AppchainManagers => "a".to_string(),
            StorageKey::AppchainManager(appchain_id) => format!("{}am", appchain_id),
        }
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
