use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{assert_self, env, near_bindgen, AccountId, PanicOnDefault};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct MockAppchainAnchor {
    appchain_id: String,
    owner: AccountId,
    oct_token: AccountId,
}

#[near_bindgen]
impl MockAppchainAnchor {
    #[init]
    pub fn new(appchain_id: String, oct_token: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert_self();
        Self {
            appchain_id,
            owner: env::signer_account_id(),
            oct_token,
        }
    }
}
