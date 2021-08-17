use near_sdk::{env, Balance};

use crate::*;
use crate::{types::AppchainId, AppchainRegistry};

/// The actions which the voter can perform
pub trait VoterAction {
    /// Withdraw a certain amount of upvote deposit for an appchain
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance);
    /// Withdraw a certain amount of downvote deposit for an appchain
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance);
}

#[near_bindgen]
impl VoterAction for AppchainRegistry {
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance) {
        let voter = env::predecessor_account_id();
        let voter_upvote = self
            .upvote_deposits
            .get(&(appchain_id.clone(), voter.clone()))
            .unwrap_or_default();
        assert!(
            voter_upvote >= amount,
            "Not enough upvote deposit to withdraw."
        );
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        let account_id = env::predecessor_account_id();
        let voter_upvote = self
            .upvote_deposits
            .get(&(appchain_id.clone(), account_id.clone()))
            .unwrap_or_default();
        appchain_basedata.decrease_upvote_deposit(amount);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        self.upvote_deposits.insert(
            &(appchain_id.clone(), account_id.clone()),
            &(voter_upvote - amount),
        );
        ext_fungible_token::ft_transfer(
            voter.clone(),
            amount.into(),
            None,
            &self.oct_token,
            1,
            GAS_FOR_FT_TRANSFER_CALL,
        )
        .then(ext_self::resolve_withdraw_upvote_deposit(
            appchain_id.clone(),
            voter.clone(),
            amount,
            &env::current_account_id(),
            NO_DEPOSIT,
            env::prepaid_gas() / 2,
        ));
    }

    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance) {
        let voter = env::predecessor_account_id();
        let voter_downvote = self
            .downvote_deposits
            .get(&(appchain_id.clone(), voter.clone()))
            .unwrap_or_default();
        assert!(
            voter_downvote >= amount,
            "Not enough downvote deposit to withdraw."
        );
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        let account_id = env::predecessor_account_id();
        let voter_downvote = self
            .downvote_deposits
            .get(&(appchain_id.clone(), account_id.clone()))
            .unwrap_or_default();
        appchain_basedata.decrease_downvote_deposit(amount);
        self.set_appchain_basedata(&appchain_id, &appchain_basedata);
        self.downvote_deposits.insert(
            &(appchain_id.clone(), account_id.clone()),
            &(voter_downvote - amount),
        );
        ext_fungible_token::ft_transfer(
            voter.clone(),
            amount.into(),
            None,
            &self.oct_token,
            1,
            GAS_FOR_FT_TRANSFER_CALL,
        )
        .then(ext_self::resolve_withdraw_downvote_deposit(
            appchain_id.clone(),
            voter.clone(),
            amount,
            &env::current_account_id(),
            NO_DEPOSIT,
            env::prepaid_gas() / 2,
        ));
    }
}

impl AppchainRegistry {
    //
    pub fn resolve_withdraw_upvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: Balance,
    ) {
        assert_self();
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => env::log(
                format!(
                    "Upvote for appchain '{}' withdrawed by '{}'. Amount: '{}'",
                    &appchain_id, &account_id, &amount
                )
                .as_bytes(),
            ),
            PromiseResult::Failed => {}
        }
    }
    //
    pub fn resolve_withdraw_downvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: Balance,
    ) {
        assert_self();
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => env::log(
                format!(
                    "Downvote for appchain '{}' withdrawed by '{}'. Amount: '{}'",
                    &appchain_id, &account_id, &amount
                )
                .as_bytes(),
            ),
            PromiseResult::Failed => {}
        }
    }
}
