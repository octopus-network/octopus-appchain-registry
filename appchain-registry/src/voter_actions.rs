use std::ops::{Div, Mul};

use near_sdk::{env, Gas};

use crate::{interfaces::VoterActions, types::AppchainId, *};

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
#[near_bindgen]
impl VoterActions for AppchainRegistry {
    //
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128) {
        self.assert_asset_transfer_is_not_paused();
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
            self.oct_token.clone(),
            1,
            Gas::ONE_TERA.mul(T_GAS_FOR_FT_TRANSFER_CALL),
        )
        .then(ext_self::resolve_withdraw_upvote_deposit(
            appchain_id.clone(),
            voter.clone(),
            amount,
            env::current_account_id(),
            NO_DEPOSIT,
            env::prepaid_gas().div(2),
        ));
    }
    //
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128) {
        self.assert_asset_transfer_is_not_paused();
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
            self.oct_token.clone(),
            1,
            Gas::ONE_TERA.mul(T_GAS_FOR_FT_TRANSFER_CALL),
        )
        .then(ext_self::resolve_withdraw_downvote_deposit(
            appchain_id.clone(),
            voter.clone(),
            amount,
            env::current_account_id(),
            NO_DEPOSIT,
            env::prepaid_gas().div(2),
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
            PromiseResult::Successful(_) => log!(
                "Upvote for appchain '{}' withdrawed by '{}'. Amount: '{}'",
                &appchain_id,
                &account_id,
                &amount.0
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
            PromiseResult::Successful(_) => log!(
                "Downvote for appchain '{}' withdrawed by '{}'. Amount: '{}'",
                &appchain_id,
                &account_id,
                &amount.0
            ),
            PromiseResult::Failed => {}
        }
    }
}
