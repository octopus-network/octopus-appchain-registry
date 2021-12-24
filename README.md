# octopus-appchain-registry

This contract provides a registry for appchains of [Octopus Network](https://oct.network). It contains the metadata of the appchains and manage their lifecycle in Octopus Network.

Contents:

* [Terminology](#terminology)
* [Function specification](#function-specification)
  * [Manage registry settings](#manage-registry-settings)
  * [Manage registry roles](#manage-registry-roles)
  * [Register appchain](#register-appchain)
  * [Transfer ownership of an appchain](#transfer-ownership-of-an-appchain)
  * [Manage the lifecycle of appchains](#manage-the-lifecycle-of-appchains)
  * [Count voting score of appchains](#count-voting-score-of-appchains)
  * [Upvote or downvote for an appchain](#upvote-or-downvote-for-an-appchain)
  * [Withdraw upvote or downvote deposit](#withdraw-upvote-or-downvote-deposit)
  * [Pause or resume asset transfer](#pause-or-resume-asset-transfer)
  * [View functions](#view-functions)
* [Registry roles](#registry-roles)
* [Build and test](#build-and-test)

## Terminology

* `owner`: The owner of this contract, which is the Octopus DAO.
* `appchain anchor`: A NEAR contract which is deployed in a subaccount of the account of this contract. It is in charge of managing the necessary data of an appchain on NEAR protocol, providing security and interoperability for the appchain. The anchor contracts are controlled by the `owner` (Octopus DAO) too, and the [octopus-appchain-anchor](https://github.com/octopus-network/octopus-appchain-anchor) is the standard implementation provided by Octopus Core Team.
* `appchain owner`: The owner of an appchain, usually the developer or someone who represent the developer team.
* `appchain state`: The state of an appchain, which is one of the following:
  * `registered`: The initial state of an appchain, after it is successfully registered.
  * `auditing`: The state while the appchain is under auditing.
  * `inQueue`: The state while `voter` can upvote or downvote an appchain.
  * `staging`: The state while `validator` and `delegator` can deposit OCT tokens to this contract to indicate their willing of staking for an appchain. This state is managed by `appchain anchor`.
  * `booting`: The state while an appchain is booting. This state is managed by `appchain anchor`.
  * `active`: The state while an appchain is active normally. This state is managed by `appchain anchor`.
  * `broken`: The state which an appchain is broken for some technical or governance reasons. This state is managed by `appchain anchor`.
  * `dead`: The state which the lifecycle of an appchain is end.
* `register deposit`: To prevent abuse of audit services, an appchain has to deposit a small amount of OCT token when register.
* `upvote deposit`: The total amount of OCT token which the `voter` (s) deposited to this contract for upvoting an appchain.
* `downvote deposit`: The total amount of OCT token which the `voter` (s) deposited to this contract for downvoting an appchain.
* `voting score`: A value representing the result of appchain voting. It is calculated by the total upvote and downvote deposit for an appchain.
* `registry settings`: A set of settings for this contract, which contains the following fields:
  * `minimum register deposit`: The minimum amount of `register deposit` which is specified by Octopus DAO.
  * `voting result reduction percent`: The value of reduction percent for voting result of all appchains still in queue, after an appchain is selected for `staging`.
  * `counting interval in seconds`: The time interval of the frequency of action `count voting score` of appchains `inQueue`.
* `registry roles`: A set of roles for this contract, which contains the following fields:
  * `registry settings manager`: The account id that can perform actions to change `registry settings`.
  * `appchain lifecycle manager`: The account id that can manage the lifecycle of appchains in registry.
  * `operator of counting voting score`: The account id that can perform action `count voting score`.

## Function specification

### Manage registry settings

This contract has a set of functions to manage `registry settings`. Each of these functions is for changing one field of the settings. Only the account that is set to role `registry settings manager` can call these functions. (Refer to [Registry roles](#registry-roles).)

### Manage registry roles

This contract has a set of functions to manage `registry roles`. Each of these functions is for changing the account that acts as a certain role. (Refer to [Registry roles](#registry-roles).)

### Register appchain

Anyone can register appchain in this contract by providing necessary information for the appchain with a certain amount of OCT token deposited to this contract. The amount should be not less than `minimum register deposit` of `registry settings`.

> The `register deposit` will NOT be refunded in any condition. It is considered as auditing fee for registered appchain.

### Transfer ownership of an appchain

The account that successfully registered an appchain in this contract will automatically become `the owner of the appchain`. Only this account can transfer the ownership of the certain appchain to another account.

The initial `appchain state` of a registered appchain is `registered`.

### Manage the lifecycle of appchains

This contract has a set of functions to manage the lifecycle of appchains registered in it. Only the account that is set to role `appchain lifecycle manager` can call these functions. (Refer to [Registry roles](#registry-roles).)

The actions that the `appchain lifecycle manager` can perform are as the following:

* Update the metatdata of a certain appchain.
* Start auditing a certain appchain, can only be performed for the appchain whose `appchain state` is `registered`. This action will change `appchain state` to `auditing`.
* Pass auditing a certain appchain, can only be performed for the appchain whose `appchain state` is `auditing`. This action will change `appchain state` to `inQueue`.
* Reject a certain appchain, can be performed for the appchain whose `appchain state` is `registered`, `auditing` or `inQueue`. This action will change `appchain state` to `dead`.
* Conclude appchain(s) in queue. This action will select the appchain with the biggest `voting score` to become the one that will goes to `staging`, and reduce the `voting score` of all appchains that are still `inQueue` by a certain percentage (that is `voting result reduction percent` of `registry settings`). This action will create a subaccount of registry account for `appchain anchor` contract automatically, and transfer a certain amount of NEAR token to this subaccount as storage deposit.
* Remove a certain appchain, can only be performed for the appchain whose `appchain state` is `dead`. This action will remove the appchain from registry contract permanently. Before all upvoter/downvoter of the appchain withdraw their voting deposit, this action can NOT be successfully applied.

### Count voting score of appchains

This contract has a function to count voting score of appchains whose `appchain state` is `inQueue`. Only the account that is set to role `operator of counting voting score` can call this function. (Refer to [Registry roles](#registry-roles).)

This function should be called by a standalone service or by person manually, and it can only be performed once in each period of `counting interval in seconds` of `registry settings`.

This function will calculate `voting score` of each appchain in all appchains `inQueue` by:

```js
voting_score_of_an_appchain += sum(upvote_amount_from_a_voter_of_the_appchain) - sum(downvote_amount_from_a_voter_of_the_appchain);
```

### Upvote or downvote for an appchain

Anyone can upvote or downvote for an appchain by depositing a certain amount of OCT token into this contract. The upvote or downvote amount is equal to the amount of OCT token deposited.

### Withdraw upvote or downvote deposit

Anyone who upvoted or downvoted for an appchain can withdraw any amount of OCT tokens for a certain appchain (not exceeding the OCT tokens deposited for the appchain) at any time. This action is not restricted by `appchain state` of an appchain.

### Pause or resume asset transfer

The owner account of this contract can pause or resume asset transfer in this contract. The actions that will be limited should be:

* Transfer OCT token into this contract by function `ft_transfer_call` of OCT token contract, with a certain message attached (to register an appchain or upvote/downvote for an appchain).
* Withdraw upvote/downvote deposit from this contract.

### View functions

This contract has a set of view functions for anyone to get the status detail of this contract.

## Registry roles

This contract has different roles to restrict access to certain functions.

Contract action | Contract owner | Registry settings manager | Appchain lifecycle manager | Operator of counting voting score
---|---|---|---|---
change_appchain_lifecycle_manager | allowed |  | allowed |
change_registry_settings_manager | allowed | allowed |  |
change_operator_of_counting_voting_score | allowed |  |  |
change_minimum_register_deposit |  | allowed |  |
change_voting_result_reduction_percent |  | allowed |  |
change_counting_interval_in_seconds |  | allowed |  |
update_appchain_metadata |  |  | allowed |
start_auditing_appchain |  |  | allowed |
pass_auditing_appchain |  |  | allowed |
reject_appchain |  |  | allowed |
count_voting_score |  |  |  | allowed
conclude_voting_score |  |  | allowed |
remove_appchain |  |  | allowed |
pause_asset_transfer | allowed |  |  |
resume_asset_transfer | allowed |  |  |

> An account can NOT has different roles at the same time.

## Build and test

Simply run `.build.sh` to build the project. The script will create folder `out` and `res`.

Run `./build.sh test` to build and run all test code.
