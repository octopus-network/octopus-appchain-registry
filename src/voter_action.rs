use near_sdk::Balance;

use crate::{types::AppchainId, AppchainRegistry};

/// The actions which the voter can perform
pub trait VoterAction {
    /// Withdraw a certain amount of upvote deposit for an appchain
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance);
    /// Withdraw a certain amount of downvote deposit for an appchain
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance);
}

impl VoterAction for AppchainRegistry {
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance) {
        todo!()
    }

    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance) {
        todo!()
    }
}
