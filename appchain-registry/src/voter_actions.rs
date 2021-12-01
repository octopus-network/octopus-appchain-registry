use near_sdk::env;

use crate::*;
use crate::{types::AppchainId, AppchainRegistry};

pub trait VoterActionsResultResolver {
    /// Resolver for withdrawing the upvote deposit of a voter
    fn resolve_withdraw_upvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: U128,
    );
    /// Resolver for withdrawing the downvote deposit of a voter
    fn resolve_withdraw_downvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: U128,
    );
}

/// The actions which the voter can perform
pub trait VoterActions {
    /// Withdraw a certain amount of upvote deposit for an appchain
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128);
    /// Withdraw a certain amount of downvote deposit for an appchain
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128);
}

#[near_bindgen]
impl VoterActions for AppchainRegistry {
    //
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128) {
        assert!(amount.0 > 0, "Withdraw amount is zero.");
        let voter = env::predecessor_account_id();
        let voter_upvote = self
            .upvote_deposits
            .get(&(appchain_id.clone(), voter.clone()))
            .unwrap_or_default();
        assert!(
            voter_upvote >= amount.0,
            "Not enough upvote deposit to withdraw."
        );
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.decrease_upvote_deposit(amount.0);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        if amount.0 == voter_upvote {
            self.upvote_deposits
                .remove(&(appchain_id.clone(), voter.clone()));
        } else {
            self.upvote_deposits.insert(
                &(appchain_id.clone(), voter.clone()),
                &(voter_upvote - amount.0),
            );
        }
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
    //
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128) {
        assert!(amount.0 > 0, "Withdraw amount is zero.");
        let voter = env::predecessor_account_id();
        let voter_downvote = self
            .downvote_deposits
            .get(&(appchain_id.clone(), voter.clone()))
            .unwrap_or_default();
        assert!(
            voter_downvote >= amount.0,
            "Not enough downvote deposit to withdraw."
        );
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.decrease_downvote_deposit(amount.0);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        if amount.0 == voter_downvote {
            self.downvote_deposits
                .remove(&(appchain_id.clone(), voter.clone()));
        } else {
            self.downvote_deposits.insert(
                &(appchain_id.clone(), voter.clone()),
                &(voter_downvote - amount.0),
            );
        }
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

#[near_bindgen]
impl VoterActionsResultResolver for AppchainRegistry {
    //
    fn resolve_withdraw_upvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: U128,
    ) {
        assert_self();
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => env::log(
                format!(
                    "Upvote for appchain '{}' withdrawed by '{}'. Amount: '{}'",
                    &appchain_id, &account_id, &amount.0
                )
                .as_bytes(),
            ),
            PromiseResult::Failed => {}
        }
    }
    //
    fn resolve_withdraw_downvote_deposit(
        &mut self,
        appchain_id: AppchainId,
        account_id: AccountId,
        amount: U128,
    ) {
        assert_self();
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => env::log(
                format!(
                    "Downvote for appchain '{}' withdrawed by '{}'. Amount: '{}'",
                    &appchain_id, &account_id, &amount.0
                )
                .as_bytes(),
            ),
            PromiseResult::Failed => {}
        }
    }
}
