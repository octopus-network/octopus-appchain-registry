use crate::{interfaces::VoterActions, types::AppchainId, *};
use near_contract_standards::fungible_token::core::ext_ft_core;
use near_sdk::{env, Gas};
use std::ops::Mul;

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
        ext_ft_core::ext(self.oct_token.clone())
            .with_attached_deposit(1)
            .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_FT_TRANSFER))
            .with_unused_gas_weight(0)
            .ft_transfer(voter.clone(), amount.into(), None)
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_RESOLVER_FUNCTION))
                    .with_unused_gas_weight(0)
                    .resolve_withdraw_upvote_deposit(appchain_id.clone(), voter.clone(), amount),
            );
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
        ext_ft_core::ext(self.oct_token.clone())
            .with_attached_deposit(1)
            .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_FT_TRANSFER))
            .with_unused_gas_weight(0)
            .ft_transfer(voter.clone(), amount.into(), None)
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_RESOLVER_FUNCTION))
                    .with_unused_gas_weight(0)
                    .resolve_withdraw_upvote_deposit(appchain_id.clone(), voter.clone(), amount),
            );
    }
}

#[near_bindgen]
impl SelfCallback for AppchainRegistry {
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
