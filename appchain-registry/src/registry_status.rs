use near_sdk::json_types::U64;

use crate::{
    interfaces::RegistryStatus,
    types::{AppchainSortingField, AppchainState, AppchainStatus, SortingOrder},
    *,
};

#[near_bindgen]
impl RegistryStatus for AppchainRegistry {
    //
    fn get_owner_pk(&self) -> String {
        format!("{:?}", self.owner_pk)
    }
    //
    fn get_oct_token(&self) -> AccountId {
        self.oct_token.clone()
    }
    //
    fn get_registry_settings(&self) -> RegistrySettings {
        self.registry_settings.get().unwrap()
    }
    //
    fn get_registry_roles(&self) -> RegistryRoles {
        self.registry_roles.get().unwrap()
    }
    //
    fn get_total_stake(&self) -> U128 {
        let mut total_stake: u128 = 0;
        self.appchain_ids.to_vec().iter().for_each(|appchain_id| {
            if let Some(appchain_basedata) = self.appchain_basedatas.get(appchain_id) {
                total_stake += appchain_basedata.status().total_stake.0;
            }
        });
        U128::from(total_stake)
    }
    //
    fn get_appchain_ids(&self) -> Vec<String> {
        self.appchain_ids.to_vec()
    }
    //
    fn get_appchains_with_state_of(
        &self,
        appchain_state: Option<Vec<AppchainState>>,
        page_number: u16,
        page_size: u16,
        sorting_field: AppchainSortingField,
        sorting_order: SortingOrder,
    ) -> Vec<AppchainStatus> {
        assert!(page_number > 0, "Invalid page number.");
        assert!(page_size >= 5 && page_size <= 50, "Invalid page size.");
        let mut results: Vec<AppchainStatus> = Vec::new();
        for id in self.appchain_ids.to_vec() {
            let appchain_basedata = self.get_appchain_basedata(&id);
            match appchain_state {
                Some(ref states) => {
                    for state in states {
                        if appchain_basedata.state().eq(state) {
                            results.push(appchain_basedata.status());
                            break;
                        }
                    }
                }
                None => results.push(appchain_basedata.status()),
            }
        }
        if results.len() > 0 {
            match sorting_field {
                AppchainSortingField::AppchainId => results.sort_by(|a, b| match sorting_order {
                    SortingOrder::Ascending => a.appchain_id.cmp(&b.appchain_id),
                    SortingOrder::Descending => b.appchain_id.cmp(&a.appchain_id),
                }),
                AppchainSortingField::VotingScore => results.sort_by(|a, b| match sorting_order {
                    SortingOrder::Ascending => a.voting_score.0.cmp(&b.voting_score.0),
                    SortingOrder::Descending => b.voting_score.0.cmp(&a.voting_score.0),
                }),
                AppchainSortingField::RegisteredTime => {
                    results.sort_by(|a, b| match sorting_order {
                        SortingOrder::Ascending => a.registered_time.0.cmp(&b.registered_time.0),
                        SortingOrder::Descending => b.registered_time.0.cmp(&a.registered_time.0),
                    })
                }
            }
            let (_, tail) = results.split_at(((page_number - 1) * page_size).into());
            if tail.len() > page_size.into() {
                let (page, _) = tail.split_at(page_size.into());
                page.to_vec()
            } else {
                tail.to_vec()
            }
        } else {
            results
        }
    }
    //
    fn get_appchains_count_of(&self, appchain_state: Option<AppchainState>) -> U64 {
        let mut count: u64 = 0;
        for id in self.appchain_ids.to_vec() {
            let appchain_basedata = self.get_appchain_basedata(&id);
            match appchain_state {
                Some(ref state) => {
                    if appchain_basedata.state().eq(state) {
                        count += 1;
                    }
                }
                None => count += 1,
            }
        }
        count.into()
    }
    //
    fn get_appchain_status_of(&self, appchain_id: AppchainId) -> AppchainStatus {
        let appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.status()
    }
    //
    fn get_upvote_deposit_for(&self, appchain_id: AppchainId, account_id: AccountId) -> U128 {
        match self.upvote_deposits.get(&(appchain_id, account_id)) {
            Some(value) => value.into(),
            None => 0.into(),
        }
    }
    //
    fn get_downvote_deposit_for(&self, appchain_id: AppchainId, account_id: AccountId) -> U128 {
        match self.downvote_deposits.get(&(appchain_id, account_id)) {
            Some(value) => value.into(),
            None => 0.into(),
        }
    }
}
