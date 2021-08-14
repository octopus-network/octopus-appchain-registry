# octopus-appchain-registry

This contract provides a registry for appchains of [Octopus Network](https://oct.network). It contains the metadata of the appchains and manage their lifecycle in Octopus Network.

Contents:

* [Teminology](#terminology)
* [Implementation details](#implementation-details)
  * [Initialization](#initialization)
  * [Register an appchain](#register-an-appchain)
  * [Callback function of token transfer](#callback-function-of-token-transfer)
  * [Withdraw a certain amount of upvote deposit](#withdraw-a-certain-amount-of-upvote-deposit)
  * [Withdraw a certain amount of downvote deposit](#withdraw-a-certain-amount-of-downvote-deposit)
  * [Transfer the ownership of an appchain](#transfer-the-ownership-of-an-appchain)
  * [Pass auditing of an appchain](#pass-auditing-of-an-appchain)
  * [Reject an appchain](#reject-an-appchain)
  * [Cancel an appchain](#cancel-an-appchain)
  * [Apply register deposit for an appchain](#apply-register-deposit-for-an-appchain)
  * [Upvote for an appchain](#upvote-for-an-appchain)
  * [Downvote for an appchain](#downvote-for-an-appchain)
  * [Count voting score](#count-voting-score)
  * [Conclude voting score](#conclude-voting-score)
  * [Sync state of an appchain](#sync-state-of-an-appchain)
  * [Remove an appchain](#remove-an-appchain)
* [Interfaces](#interfaces)
  * [Custom types](#custom-types)
  * [Registry status](#registry-status)
  * [Registry owner action](#registry-owner-action)
  * [Appchain owner action](#appchain-owner-action)
  * [Voter action](#voter-action)
  * [Appchain anchor callback](#appchain-anchor-callback)

## Terminology

* `owner`: The owner of this contract, which is the Octopus DAO.
* `appchain anchor`: A NEAR contract which is deployed in a subaccount of the account of this contract. It is in charge of managing the necessary data of an appchain on NEAR protocol, providing security and interoperability for the appchain. The anchor contracts are controlled by the `owner` (Octopus DAO) too, and the [octopus-appchain-anchor](https://github.com/octopus-network/octopus-appchain-anchor) is the standard implementation provided by Octopus Core Team.
* `octopus relayer`: A standalone service which will monitor the state change of the validators of an appchain and facts happened on an appchain. It relays messages between an appchain and corresponding `appchain anchor`.
* `appchain owner`: The owner of an appchain, ususally the developer or someone who represent the developer team.
* `register deposit`: To prevent abuse of audit services, an appchain has to deposit a small amount of OCT token when register.
* `minimum register deposit`: The minimum amount of `register deposit` which is specified by Octopus DAO.
* `appchain state`: The state of an appchain, which is one of the following:
  * `registered`: The initial state of an appchain, after it is successfully registered.
  * `auditing`: The state while the appchain is under auditing.
  * `inQueue`: The state while `voter` can upvote or downvote an appchain.
  * `staging`: The state while `validator` and `delegator` can deposit OCT tokens to this contract to indicate their willing of staking for an appchain. This state is managed by `appchain anchor`.
  * `booting`: The state while an appchain is booting. This state is managed by `appchain anchor`.
  * `active`: The state while an appchain is active normally. This state is managed by `appchain anchor`.
  * `broken`: The state which an appchain is broken for some technical or governance reasons. This state is managed by `appchain anchor`.
  * `dead`: The state which the lifecycle of an appchain is end.
* `voter`: Who can `upvote` or `downvote` an appchain when its `appchain state` is `inQueue`.
* `upvote deposit`: The total amount of OCT token which a `voter` deposited to this contract for upvoting an appchain.
* `downvote deposit`: The total amount of OCT token which a `voter` deposited to this contract for downvoting an appchain.
* `voting score`: A value representing the result of appchain voting. It is calculated by the total upvote and downvote amount for an appchain.
* `validator`: Who can deposit a certain amount of OCT token for an appchain when its `appchain state` is `staging`, to indicate that he/she wants to be the validator of an appchain after the appchain goes `booting` state.
* `delegator`: Who can deposit a certain amount of OCT token for an appchain when its `appchain state` is `staging`, to indicate that he/she wants to delegate his/her voting rights to an validator of an appchain after the appchain goes `booting` state.
* `sender`: A NEAR transaction sender, that is the account which perform actions (call functions) on this contract.

## Implementation details

### Initialization

This contract has to be initialized with the following parameters:

* `oct_token_contract`: The account id of OCT token contract.

The `oct_token_contract` should be stored in this contract for using in [Callback function 'ft_on_transfer'](#callback-function-ft_on_transfer).

### Register an appchain

This action needs the following parameters:

* `appchain_id`: The unique identity in Octopus Network. It cannot be duplicated with any other registered appchain.
* `website_url`: The url of the official website of the appchain.
* `github_address`: The address of the github repository of the appchain, if it is an open-source project.
* `github_release`: The release vesion of the github repository of the appchain, if it is an open-source project.
* `commit_id`: The commit id of source code of the github repository of the appchain, if it is an open-source project.
* `contact_email`: The email of the contact of the appchain project, which is used for communidating with the appchain team.

If the parameters are all valid, the appchain will be registered to this contract. These data will be saved to the metadata of the new appchain.

* The `appchain state` of the new appchain is set to `registered`.
* The `sender` will be registered as the owner of the appchain.

This action should generate log: `Appchain <appchain_id> is registered by <sender>.`

### Callback function of token transfer

This contract has a callback interface `FungibleTokenReceiver::ft_on_transfer` for contract `fungible_token` of `near-contract-standards`.

The callback function `ft_on_transfer` needs the following parameters:

* `sender_id`: The account id of sender of the transfer.
* `amount`: The amount of the transfer.
* `msg`: The message attached to the transfer, which indicates the purpose of the deposit.

If the caller of this callback (`env::predecessor_account_id()`) is `oct_token_contract` which is initialized at construction time of this contract, parse `msg` with the following patterns:

* `register deposit for appchain <appchain_id>`:
  * The `appchain state` of the appchain corresponding to `appchain_id` must be `registered`. Otherwise, the deposit will be considered as `invalid deposit`.
  * The amount of deposit must not be less than `minimum register deposit`. Otherwise, the deposit will be considered as `invalid deposit`.
  * The `register deposit` of the appchain must be 0. Otherwise, the deposit will be considered as `invalid deposit`.
  * The `register deposit` of the appchain is set to `amount`.
  * The `appchain state` of the appchain is set to `auditing`.
  * Generate log: `Received register deposit <amount> for appchain <appchain_id> from <sender_id>.`
  * Return 0.
* `upvote for appchain <appchain_id>`:
  * The `appchain state` of the appchain corresponding to `appchain_id` must be `inQueue`. Otherwise, the deposit will be considered as `invalid deposit`.
  * Add `amount` to `upvote balance` of `sender_id` for the appchain corresponding to `appchain_id`.
  * Generate log: `Received upvote <amount> for appchain <appchain_id> from <sender_id>.`
  * Return 0.
* `downvote for appchain <appchain_id>`:
  * The `appchain state` of the appchain corresponding to `appchain_id` must be `inQueue`. Otherwise, the deposit will be considered as `invalid deposit`.
  * Add `amount` to `downvote balance` of `sender_id` for the appchain corresponding to `appchain_id`.
  * Generate log: `Received downvote <amount> for appchain <appchain_id> from <sender_id>.`
  * Return 0.
* other cases:
  * The deposit will be considered as `invalid deposit`.

For `invalid deposit` case, generate log: `Invalid deposit <amount> of OCT token from <sender_id> returned.` and return `amount`.

If the caller of this callback (`env::predecessor_account_id()`) is NOT `oct_token_contract`, generate log: `Invalid deposit <amount> of unknown NEP-141 asset from <sender_id> returned.` and return `amount`.

### Withdraw a certain amount of upvote deposit

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `amount`: The amount which the sender wants to withdraw

Qualification of this action:

* The `amount` must not be larger than `upvote deposit` of `sender` for `appchain_id`.

Reduce `amount` from `upvote deposit` of `sender`, and send `amount` of OCT token back to `sender`.

Generate log: `Upvote deposit <amount> for appchain <appchain_id> is withdrawed by <sender>.`

### Withdraw a certain amount of downvote deposit

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `amount`: The amount which the sender wants to withdraw

Qualification of this action:

* The `amount` must not be larger than `downvote deposit` of `sender` for `appchain_id`.

Reduce `amount` from `downvote deposit` of `sender`, and send `amount` of OCT token back to `sender`.

Generate log: `Downvote deposit <amount> for appchain <appchain_id> is withdrawed by <sender>.`

### Transfer the ownership of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `account_id`: The account id of new owner of the given appchain.

Qualification of this action:

* The `sender` must be current `appchain owner` of the appchain corresponding to `appchain_id`.

The `appchain owner` of the appchain corresponding to `appchain_id` is set to `account_id`.

### Pass auditing of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `code`: The wasm code of `appchain anthor` of the given appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of the appchain corresponding to `appchain_id` must be `auditing`.

The `appchain state` of the appchain corresponding to `appchain_id` is set to `inQueue`. The value of `code` is staged to the metadata of the appchain corresponding to `appchain_id` in this contract.

This action should generate log: `Appchain <appchain_id> is in queue.`

> The auditing of appchain code is an off-line action which will be completed by the task force assigned by Octopus DAO.

### Reject an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of the appchain corresponding to `appchain_id` must be `auditing`.

The `appchain state` of the appchain corresponding to `appchain_id` is set to `dead`.

This action should generate log: `Appchain <appchain_id> is rejected.`

### Cancel an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be current `appchain owner` of the appchain corresponding to `appchain_id`.
* The `appchain state` of the appchain corresponding to `appchain_id` must be `registered` or `auditing`.

The `appchain state` of the appchain corresponding to `appchain_id` is set to `dead`.

### Apply register deposit for an appchain

The `appchain owner` can transfer a certain amount (not less than `minimum register deposit`) of OCT token to this contract by calling function `ft_transfer_call` of `oct_token_contract`. And the calling param `msg` MUST be `register deposit for appchain <appchain_id>`.

### Upvote for an appchain

Any `voter` can transfer a certain amount of OCT token to this contract by calling function `ft_transfer_call` of `oct_token_contract`. And the calling param `msg` MUST be `upvote for appchain <appchain_id>`.

### Downvote for an appchain

Any `voter` can transfer a certain amount of OCT token to this contract by calling function `ft_transfer_call` of `oct_token_contract`. And the calling param `msg` MUST be `downvote for appchain <appchain_id>`.

### Count voting score

Qualification of this action:

* The `sender` must be the `owner`.

This action will count daily `voting score` of all appchains whose `appchain state` is `inQueue`, and store the results in this contract.

The `voting score` of an appchain is calculated by:

```js
voting_score_of_an_appchain += sum(upvote_amount_from_a_voter_of_the_appchain) - sum(downvote_amount_from_a_voter_of_the_appchain);
```

> This action should be performed every day by an offchain daemon or an operatoer.

### Conclude voting score

This action needs the following parameters:

* `vote_result_reduction_percent`: The percent (unsigned integer not bigger than 100) which all appchains' voting score will be reduced in the next voting period.

Qualification of this action:

* The `sender` must be the `owner`.

This action will calculate `voting score` of all appchains whose `appchain state` is `inQueue`.

The `appchain state` of appchain with the largest `voting score` will become `staging`. Then:

* Create subaccount `<appchain_id>.<account id of this contract>`.
* Transfer a certain amount of NEAR token to account `<appchain_id>.<account id of this contract>` for storage deposit.
* Deploy the code of `appchain anchor` of the appchain to the account `<appchain_id>.<account id of this contract>`.
* Initialize new `appchain anchor` by the metadata of the appchain.
* Add a new full access key to the new `appchain anchor` for the `owner`.
* Add a new access key to this contract for the new `appchain anchor`, to allow it syncing its state to this contract.
* Store the account of new `appchain anchor` for the appchain in this contract.

The `voting score` of all appchains with state `inQueue` will be reduced by value of `vote_result_reduction_percent`.

This action should generate log: `Appchain <appchain_id> goes staging at <account>.`

> This action should be performed when the period of appchain selection for `staging` ends.

### Sync state of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `state`: The new state of the given appchain.

Qualification of this action:

* The `sender` must be the account which the `appchain anchor` corresponding to `appchain_id` is deployed.
* The value of `state` must be one of `staging`, `booting`, `active`, `broken` and `dead`, which are managed by `appchain anchor`.

The `appchain state` of the appchain corresponding to `appchain_id` is set to `state`.

This action should generate log: `The state of appchain <appchain_id> changes to <new state>.`

### Remove an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of the appchain corresponding to `appchain_id` must be `dead`.

This action will remove the appchain corresponding to `appchain_id` from this contract, and delete the account of its `appchain anchor`.

This action should generate log: `Appchain <appchain_id> and its anchor is removed.`

## Interfaces

### Custom Types

```rust
/// Appchain metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AppchainMetadata {
    pub website_url: String,
    pub github_address: String,
    pub github_release: String,
    pub commit_id: String,
    pub contact_email: String,
}

/// The state of an appchain
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum AppchainState {
    Registered,
    Auditing,
    InQueue,
    Staging,
    Booting,
    Active,
    Broken,
    Dead,
}

/// Appchain status
///
/// This struct should NOT be used in storage on chain
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AppchainStatus {
    pub appchain_id: AppchainId,
    pub appchain_metadata: AppchainMetadata,
    pub appchain_anchor: AccountId,
    pub appchain_owner: AccountId,
    pub initial_deposit: Balance,
    pub appchain_state: AppchainState,
    pub upvote_deposit: Balance,
    pub downvote_deposit: Balance,
    pub voting_score: u128,
    pub registered_time: Timestamp,
    pub go_live_time: Timestamp,
}
```

### Registry status

```rust
/// The interface for querying status of appchain registry
pub trait RegistryStatus {
    /// Get appchains whose state is equal to the given AppchainState
    ///
    /// If param `appchain_state` is `Option::None`, return all appchains in registry
    fn get_appchains_with_state_of(
        &self,
        appchain_state: Option<AppchainState>,
    ) -> Vec<AppchainStatus>;
    /// Get status of an appchain
    fn get_appchain_status_of(&self, appchain_id: AppchainId) -> AppchainStatus;
    /// Get upvote deposit of the caller for a certain appchain
    fn get_upvote_deposit_for(&self, appchain_id: AppchainId) -> Balance;
    /// Get downvote deposit of the caller for a certain appchain
    fn get_downvote_deposit_for(&self, appchain_id: AppchainId) -> Balance;
}
```

### Registry owner action

```rust
/// The actions which the owner of appchain registry can perform
pub trait RegistryOwnerAction {
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId, code: Vec<u8>);
    /// Reject an appchain
    fn reject_appchain(&mut self, appchain_id: AppchainId);
    /// Count voting score of appchains
    fn count_voting_score(&mut self);
    /// Conclude voting score of appchains
    fn conclude_voting_score(&mut self);
    /// Remove an appchain from registry
    fn remove_appchain(&mut self, appchain_id: AppchainId);
}
```

### Appchain owner action

```rust
/// The actions which the owner of an appchain can perform
pub trait AppchainOwnerAction {
    /// Register an appchain
    fn register_appchain(
        &mut self,
        appchain_id: AppchainId,
        website_url: String,
        github_address: String,
        github_release: String,
        commit_id: String,
        contact_email: String,
    );
    /// Transfer ownership of an appchain to another account
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId);
    /// Cancel an appchain
    fn cancel_appchain(&mut self, appchain_id: AppchainId);
}
```

### Voter action

```rust
/// The actions which the voter can perform
pub trait VoterAction {
    /// Withdraw a certain amount of upvote deposit for an appchain
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance);
    /// Withdraw a certain amount of downvote deposit for an appchain
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: Balance);
}
```

### Appchain anchor callback

```rust
/// The callback interface for appchain anchor
pub trait AppchainAnchorCallback {
    /// Sync state of an appchain to registry
    fn sync_state_of(&mut self, appchain_id: AppchainId, appchain_state: AppchainState);
}
```
