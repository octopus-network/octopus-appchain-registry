use crate::AppchainId;

/// Storage keys for collections of sub-struct in main contract
pub enum StorageKey {
    AppchainBasedatas,
    UpvoteDeposits,
    DownvoteDeposits,
    AppchainBasedata(AppchainId),
}

impl StorageKey {
    pub fn to_string(&self) -> String {
        match self {
            StorageKey::AppchainBasedatas => "a".to_string(),
            StorageKey::UpvoteDeposits => "u".to_string(),
            StorageKey::DownvoteDeposits => "d".to_string(),
            StorageKey::AppchainBasedata(appchain_id) => format!("{}bd", appchain_id),
        }
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
