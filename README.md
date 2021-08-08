# octopus-appchain-registry

This contract provides a registry for appchains of [Octopus Network](https://oct.network). It contains the metadata of the appchains and manage their lifecycle in Octopus Network.

## Terminology

* `owner`: The owner of this contract, which is the Octopus Foundation.
* `appchain anchor`: A NEAR contract which is deployed in a subaccount of the account of this contract. It is in charge of managing the necessary data of an appchain on NEAR protocol, and providing the security and governance ability for the appchain. The anchor contracts are controlled by the `owner` (Octopus Foundation) too, and the [octopus-appchain-anchor](https://github.com/octopus-network/octopus-appchain-anchor) is the standard implementation provided by Octopus Network.
* `octopus relayer`: A standalone service which will monitor the state change of the validators of an appchain and facts happened on an appchain. It is controlled by the Octopus Foundation, and will relay messages between an appchain and its `appchain anchor`.
* `appchain owner`: The owner of an appchain.
* `initial deposit`: An appchain has to deposit a certain amount of OCT token to this contract for going live in Octopus Network.
* `appchain state`: The state of an appchain, which is one of the following:
  * `registered`: The initial state of an appchain, after it is successfully registered.
  * `auditing`: The state while the appchain is under auditing by Octopus Foundation.
  * `inQueue`: The state while `voter` can upvote or downvote an appchain.
  * `staging`: The state while `validator` and `delegator` can deposit OCT tokens to this contract to indicate their willing of staking for an appchain. This state is managed by `appchain anchor`.
  * `booting`: The state while an appchain is booting. This state is managed by `appchain anchor`.
  * `active`: The state while an appchain is active normally. This state is managed by `appchain anchor`.
  * `broken`: The state which an appchain is broken for some technical or governance reasons. This state is managed by `appchain anchor`.
  * `dead`: The state which the lifecycle of an appchain is end.
* `voter`: Who can `upvote` or `downvote` an appchain when its `appchain state` is `inQueue`.
* `voting result`: A value representing the result of appchain voting. It is calculated by the total upvote and downvote amount for an appchain.
* `validator`: Who can deposit an amount of OCT token for an appchain when its `appchain state` is `staging`, to indicate that he/she wants to be the validator of an appchain after the appchain goes `booting` state.
* `delegator`: Who can deposit an amount of OCT token for an appchain when its `appchain state` is `staging`, to indicate that he/she wants to delegate his/her voting rights to an validator of an appchain after the appchain goes `booting` state.
* `sender`: A NEAR transaction sender, that is the account which perform actions (call functions) on this contract.

## Implementation details

### Register an appchain

This action needs the following parameters:

* `appchain_id`: The unique identity in Octopus Network. It cannot be duplicated with any other registered appchain.
* `website_url`: The url of the official website of the appchain.
* `github_address`: The address of the github repository of the appchain, if it is an open-source project.
* `github_release`: The release vesion of the github repository of the appchain, if it is an open-source project.
* `commit_id`: The commit id of source code of the github repository of the appchain, if it is an open-source project.
* `contact_email`: The email of the contact of the appchain project, which is used for Octopus Foundation to communidate with the appchain team.

If the parameters are all valid, the appchain will be registered to this contract. These data will be saved to the metadata of the new appchain.

* The `appchain state` of the new appchain is set to `registered`.
* The `sender` will be registered as the owner of the appchain.
* Generate log: `Appchain <appchain_id> is registered by <account>.`

### Change value of initial deposit

This action needs the following parameters:

* `value`: The new value of `initial deposit`.

This action will change the value of `initial deposit` which is stored in this contract.

* The `sender` must be the `owner`.
* The default value of `initial deposit` is **100 OCT**.

### Confirm and record OCT token deposit

This contract has a callback interface `FungibleTokenReceiver::ft_on_transfer` for contract `fungible_token` of `near-contract-standards`. It will confirm and record the transfer of OCT token from other accounts to this contract.

The callback function `ft_on_transfer` needs the following parameters:

* `sender_id`: The account id of sender of the transfer.
* `amount`: The amount of the transfer.
* `msg`: The message attached to the transfer, which indicates the purpose of the deposit.

The param `msg` can be one of the following patterns:

* `initial deposit for appchain <appchain_id>`:
  * The `appchain state` of the appchain corresponding to `appchain_id` must be `registered`. Otherwise, the deposit will be considered as `invalid deposit`.
  * The amount of deposit must not be less than `initial deposit`.
  * The state of the given appchain changes to `auditing`.
  * Generate log: `Received initial deposit <amount> for appchain <appchain_id> from <sender>.`
* `upvote for appchain <appchain_id>`:
  * The `appchain state` of the appchain corresponding to `appchain_id` must be `inQueue`. Otherwise, the deposit will be considered as `invalid deposit`.
  * The amount of deposit will be added to `upvote balance` of the appchain corresponding to `appchain_id`.
  * Generate log: `Received upvote <amount> for appchain <appchain_id> from <sender>.`
* `downvote for appchain <appchain_id>`:
  * The `appchain state` of the appchain corresponding to `appchain_id` must be `inQueue`. Otherwise, the deposit will be considered as `invalid deposit`.
  * The amount of deposit will be added to `downvote balance` of the appchain corresponding to `appchain_id`.
  * Generate log: `Received downvote <amount> for appchain <appchain_id> from <sender>.`
* other cases:
  * The deposit will be considered as `invalid deposit`.

For `invalid deposit` case, this contract will save the amount of the deposit to `invalid deposit` of the sender. The sender can withdraw the deposit at anytime.

* Generate log: `Received invalid deposit <amount> from <sender>.`

### Withdraw OCT token deposit

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `deposit_purpose`: The purpose of OCT token deposit recorded in this contract, of which will be withdrawed.

The param `deposit_purpose` can be one of the following values:

* `upvote`:
  * Get the total deposit of `sender` for upvoting the appchain corresponding to `appchain_id`. If the total deposit is 0, throws an error.
  * Send total upvote deposit of the `sender` for the appchain back to `sender`.
  * Generate log: `Upvote deposit <amount> for appchain <appchain_id> is withdrawed by <account>.`
* `downvote`:
  * Get the total deposit of `sender` for downvoting the appchain corresponding to `appchain_id`. If the total deposit is 0, throws an error.
  * Send total downvote deposit of the `sender` for the appchain back to `sender`.
  * Generate log: `Downvote deposit <amount> for appchain <appchain_id> is withdrawed by <account>.`
* `invalid`:
  * Ignore param `appchain_id`.
  * Send total `invalid deposit` of the `sender` back to `sender`.
  * Generate log: `Invalid deposit <amount> is withdrawed by <account>.`

### Transfer the ownership of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `account_id`: The account id of new owner of the given appchain.

This action can transfer the ownership of an appchain to another account.

* The `sender` must be current `appchain owner` of the appchain corresponding to `appchain_id`.
* Set `account_id` to be the new `appchain owner` of the appchain corresponding to `appchain_id`.

### Pass auditing of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `code`: The wasm code of `appchain anthor` of the given appchain.

This action can pass auditing of an appchain, and stage the code of its `appchain anchor` in this contract.

* The `sender` must be the `owner`.
* The `appchain state` of the appchain corresponding to `appchain_id` must be `auditing`.
* The `appchain state` of the appchain corresponding to `appchain_id` will be changed to `inQueue`.
* Stage value of param `code` to the metadata of the appchain corresponding to `appchain_id` in this contract.
* Generate log: `Appchain <appchain_id> is audited by Octopus Foundation.`

> The auditing of appchain code is an off-line action which will be completed by Octopus Foundation.

### Reject an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

This action can refuse an appchain to join Octopus Network.

* The `sender` must be the `owner`.
* The `appchain state` of the appchain corresponding to `appchain_id` must be `auditing`.
* The `appchain state` of the appchain corresponding to `appchain_id` will be changed to `dead`.
* Generate log: `Appchain <appchain_id> is rejected by Octopus Foundation.`

### Cancel an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Only the `appchain owner` can cancel an appchain, only if its `appchain state` is `registered` or `auditing`.

* The `sender` must be current `appchain owner` of the appchain corresponding to `appchain_id`.
* The `appchain state` of the appchain corresponding to `appchain_id` must be `registered` or `auditing`.
* The `appchain state` of the appchain corresponding to `appchain_id` will be changed to `dead`.

### Vote for appchains

Anyone can transfer a certain amount of OCT token to this contract, with the message `upvote for appchain <appchain_id>` attached to the transfer call, to upvote an appchain.

Anyone can transfer a certain amount of OCT token to this contract, with the message `downvote for appchain <appchain_id>` attached to the transfer call, to downvote an appchain.

> Refer to [Confirm and record OCT token deposit](#confirm-and-record-oct-token-deposit)

### Count daily voting result

This action will count daily `voting result` of all appchains whose `appchain state` is `inQueue`, and store the results in this contract.

* The `sender` must be the `owner`.
* The `voting result` of an appchain is calculated by:

```js
voting_result_of_an_appchain += sum(upvote_amount_from_a_voter_of_the_appchain) - sum(downvote_amount_from_a_voter_of_the_appchain);
```

* This action should be performed every day by an standalone service or manually.

### Conclude voting result

This action needs the following parameters:

* `duration_of_next_period`: Count of days which the next appchain selection period will last.
* `vote_result_reduction_percent`: The percent which all appchains' voting result will be reduced in the next voting period.

This action will conclude the `voting result` of all appchains whose `appchain state` is `inQueue`.

* The `sender` must be the `owner`.
* Calculate `voting result` of all appchains whose `appchain state` is `inQueue`.
* The `appchain state` of appchain with the largest `voting result` will become `staging`.
  * Deploy the code of `appchain anchor` of the appchain to the subaccount `<appchain_id>` of the account of this contract.
  * Initialize new `appchain anchor` by the metadata of the appchain.
  * Add a new full access-key to the new `appchain anchor` for the `owner`.
  * Add a new access-key to this contract for the new `appchain anchor`, to allow it syncing its state to this contract.
  * Save the account of new `appchain anchor` to the metadata of the appchain.
* The `voting result` of all appchains with state `inQueue` will be reduced by value of param `vote_result_reduction_percent`.
* Generate log: `Appchain <appchain_id> goes staging at <account>.`
* This action should be performed when the period of appchain selection for `staging` ends. The duration of next period is specified by param `duration_of_next_period`. Normally, the duration is 14 days.

### Sync state of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `state`: The new state of the given appchain.

This action will sync `appchain state` of an appchain to this contract.

* The `sender` must be the account which the `appchain anchor` corresponding to `appchain_id` is deployed.
* The value of param `state` can be one of `staging`, `booting`, `active`, `broken` and `dead`, which are managed by `appchain anchor`.
* Generate log: `The state of appchain <appchain_id> changes to <new state>.`

### Remove an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

This action will remove an appchain from this contract, and also delete the account of its `appchain anchor`.

* The `sender` must be the `owner`.
* The `appchain state` of the appchain corresponding to `appchain_id` must be `dead`.
* Generate log: `Appchain <appchain_id> and its anchor is removed.`
