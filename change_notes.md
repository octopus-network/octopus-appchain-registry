# Change notes

## 20210909

* Add function `change_counting_interval_in_seconds` with param `value`.
* Add view function `get_counting_interval_in_seconds`.
* Add sudo function `delete_appchain` with param `appchain_id`.
* Add sudo function `go_booting` with param `appchain_id`.
* Function `count_voting_score` will fail if there is no appchain `inQueue`.
* Function `conclude_voting_score` will clear `top_appchain_id_in_queue` after changes its state to `staging`.

## 20210915

* Add function `change_operator_of_counting_voting_score` with param `operator_account`. The default account which is changed by this function is `owner` of this contract.
* The function `count_voting_score` can only be called by the account which is speicifed by function `change_operator_of_counting_voting_score`.
* Add field `validator_count` and `total_stake` to `AppchainStatus`.
* Add param `validator_count` and `total_stake` to function `sync_state_of`
* Optimize storage of this contract to reduce general gas consumption of function calls.

## 20210922

* The state of appchain(s) with negative or zero `voting score` will be set to `dead` while running the function `conclude_voting_score`.
* The function `remove_appchain` will also check the `upvote deposit` and `downvote deposit` of the given appchain now. The given appchain will be removed only if the deposits are all `0` (which means, all voters have already withdrawed their deposits for the given appchain).

## 20211005

* Add fields `preminted_wrapped_appchain_token`, `ido_amount_of_wrapped_appchain_token` and `initial_era_reward` to `AppchainMetadata`.
* Add params `preminted_wrapped_appchain_token`, `ido_amount_of_wrapped_appchain_token` and `initial_era_reward` to function `update_appchain_metadata` of `RegistryOwnerActions`.
