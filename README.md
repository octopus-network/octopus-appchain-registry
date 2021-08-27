# octopus-appchain-registry

This contract provides a registry for appchains of [Octopus Network](https://oct.network). It contains the metadata of the appchains and manage their lifecycle in Octopus Network.

Contents:

* [Terminology](#terminology)
* [Implementation details](#implementation-details)
  * [Initialization](#initialization)
  * [Change value of minimum register deposit](#change-value-of-minimum-register-deposit)
  * [Update metadata of an appchain](#update-metadata-of-an-appchain)
  * [Update custom metadata of an appchain](#update-custom-metadata-of-an-appchain)
  * [Approve an appchain to start auditing](#approve-an-appchain-to-start-auditing)
  * [Register an appchain](#register-an-appchain)
  * [Upvote for an appchain](#upvote-for-an-appchain)
  * [Downvote for an appchain](#downvote-for-an-appchain)
  * [Callback function of token transfer](#callback-function-of-token-transfer)
  * [Withdraw a certain amount of upvote deposit](#withdraw-a-certain-amount-of-upvote-deposit)
  * [Withdraw a certain amount of downvote deposit](#withdraw-a-certain-amount-of-downvote-deposit)
  * [Transfer the ownership of an appchain](#transfer-the-ownership-of-an-appchain)
  * [Pass auditing of an appchain](#pass-auditing-of-an-appchain)
  * [Change code of an appchain anchor](#change-code-of-an-appchain-anchor)
  * [Reject an appchain](#reject-an-appchain)
  * [Count voting score](#count-voting-score)
  * [Change reduction percent of voting result](#change-reduction-percent-of-voting-result)
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
  * [Ownable](#ownable)
  * [Upgradable](#upgradable)

## Terminology

* `owner`: The owner of this contract, which is the Octopus DAO.
* `appchain anchor`: A NEAR contract which is deployed in a subaccount of the account of this contract. It is in charge of managing the necessary data of an appchain on NEAR protocol, providing security and interoperability for the appchain. The anchor contracts are controlled by the `owner` (Octopus DAO) too, and the [octopus-appchain-anchor](https://github.com/octopus-network/octopus-appchain-anchor) is the standard implementation provided by Octopus Core Team.
* `appchain basedata`: The basedata of an appchain, which contains the following fields:
  * `appchain owner`: The owner of an appchain, usually the developer or someone who represent the developer team.
  * `appchain metadata`: The metadata of an appchain. Refer to [Custom types](#custom-types).
  * `appchain anchor code`: The WASM code of the `appchain anchor` of an appchain.
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
* `minimum register deposit`: The minimum amount of `register deposit` which is specified by Octopus DAO.
* `voting result reduction percent`: The value of reduction percent for voting result of all appchains still in queue, after an appchain is selected for `staging`.
* `voter`: Who can `upvote` or `downvote` an appchain when its `appchain state` is `inQueue`.
* `validator`: Who can deposit a certain amount of OCT token for an appchain when its `appchain state` is `staging`, to indicate that he/she wants to be the validator of an appchain after the appchain goes `booting` state.
* `delegator`: Who can deposit a certain amount of OCT token for an appchain when its `appchain state` is `staging`, to indicate that he/she wants to delegate his/her voting rights to an validator of an appchain after the appchain goes `booting` state.
* `sender`: A NEAR transaction sender, that is the account which perform actions (call functions) on this contract.

## Implementation details

### Initialization

This contract has to be initialized with the following parameters:

* `oct_token_contract`: The account id of OCT token contract.

The `oct_token_contract` should be stored in this contract for using in [Callback function of token transfer](#callback-function-of-token-transfer).

### Change value of minimum register deposit

This action needs the following parameters:

* `value`: The value of `minimum register deposit`.

Qualification of this action:

* The `sender` must be the `owner`.

The `minimum register deposit` is set to `value`.

### Update metadata of an appchain

This action needs the following parameters:

* `appchain_id`: The unique identity in Octopus Network. It cannot be duplicated with any other registered appchain.
* `website_url`: The url of the official website of the appchain.
* `github_address`: The address of the github repository of the appchain.
* `github_release`: The release vesion of the github repository of the appchain.
* `commit_id`: The commit id of source code of the github repository of the appchain.
* `contact_email`: The email of the contact of the appchain project, which is used for communidating with the appchain team.
* `custom_metadata`: The extra custom metadata organized by a key-value map.

Qualification of this action:

* The `sender` must be the `owner`.

The metadata will be updated to `appchain basedata` corresponding to `appchain_id`.

Generate log: `The metadata of appchain <appchain_id> is updated by manager.`

### Update custom metadata of an appchain

This action needs the following parameters:

* `custom_metadata`: The extra custom metadata organized by a key-value map.

Qualification of this action:

* The `sender` must be current `appchain owner` of `appchain basedata` corresponding to `appchain_id`.

The custom metadata will be updated to `appchain metadata` of `appchain basedata` corresponding to `appchain_id`.

Generate log: `The custom metadata of appchain <appchain_id> is updated by <sender>.`

### Approve an appchain to start auditing

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `registered`.

The `appchain state` of `appchain basedata` corresponding to `appchain_id` is set to `auditing`.

Generate log: `Appchain <appchain_id> starts auditing.`

### Register an appchain

The `appchain owner` can transfer a certain amount (which must be equal to `minimum register deposit`) of OCT token to this contract by calling function `ft_transfer_call` of `oct_token_contract`. And the calling param `msg` MUST be `register_appchain,<appchain_id>,<website_url>,<github_address>,<github_release>,<commit_id>,<contact_email>`.

### Upvote for an appchain

Any `voter` can transfer a certain amount of OCT token to this contract by calling function `ft_transfer_call` of `oct_token_contract`. And the calling param `msg` MUST be `upvote_appchain,<appchain_id>`.

### Downvote for an appchain

Any `voter` can transfer a certain amount of OCT token to this contract by calling function `ft_transfer_call` of `oct_token_contract`. And the calling param `msg` MUST be `downvote_appchain,<appchain_id>`.

### Callback function of token transfer

This contract has a callback interface `FungibleTokenReceiver::ft_on_transfer` for contract `fungible_token` of `near-contract-standards`.

The callback function `ft_on_transfer` needs the following parameters:

* `sender_id`: The account id of sender of the transfer.
* `amount`: The amount of the transfer.
* `msg`: The message attached to the transfer, which indicates the purpose of the deposit.

If the caller of this callback (`env::predecessor_account_id()`) is `oct_token_contract` which is initialized at construction time of this contract, parse `msg` with the following patterns:

* `register_appchain,<appchain_id>,<website_url>,<github_address>,<github_release>,<commit_id>,<contact_email>`:
  * Parse the fields of `appchain metadata` from `msg`. If missing one or more, the deposit will be considered as `invalid deposit`.
  * The `appchain_id` must NOT be registered in this contract. Otherwise, the deposit will be considered as `invalid deposit`.
  * The amount of deposit must be equal to `minimum register deposit`. Otherwise, the deposit will be considered as `invalid deposit`.
  * The `register deposit` of `appchain basedata` must be 0. Otherwise, the deposit will be considered as `invalid deposit`.
  * The `register deposit` of `appchain basedata` is set to `amount`.
  * The `appchain state` of `appchain basedata` is set to `registered`.
  * The `sender_id` will be registered as the owner of the appchain.
  * Generate log: `Appchain <appchain_id> is registered by <sender_id> with <amount> OCT token deposited.`
  * Return 0.
* `upvote_appchain,<appchain_id>`:
  * The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `inQueue`. Otherwise, the deposit will be considered as `invalid deposit`.
  * Add `amount` to `upvote deposit` of `sender_id` for `appchain_id`.
  * Add `amount` to `upvote deposit` of `appchain basedata` for `appchain_id`.
  * Generate log: `Received upvote <amount> for appchain <appchain_id> from <sender_id>.`
  * Return 0.
* `downvote_appchain,<appchain_id>`:
  * The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `inQueue`. Otherwise, the deposit will be considered as `invalid deposit`.
  * Add `amount` to `downvote deposit` of `sender_id` for `appchain_id`.
  * Add `amount` to `downvote deposit` of `appchain basedata` for `appchain_id`.
  * Generate log: `Received downvote <amount> for appchain <appchain_id> from <sender_id>.`
  * Return 0.
* other cases:
  * The deposit will be considered as `invalid deposit`.

For `invalid deposit` case, throws an error: `Invalid deposit <amount> of OCT token from <sender_id>.`.

If the caller of this callback (`env::predecessor_account_id()`) is NOT `oct_token_contract`, throws an error: `Invalid deposit <amount> of unknown NEP-141 asset from <sender_id>.`.

### Withdraw a certain amount of upvote deposit

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `amount`: The amount which the sender wants to withdraw

Qualification of this action:

* The `amount` must not be larger than `upvote deposit` of `sender` for `appchain_id`.

Reduce `amount` from `upvote deposit` of `sender` for `appchain_id`, reduce `amount` from `upvote deposit` of `appchain basedata` for `appchain_id`, and send `amount` of OCT token back to `sender`.

Generate log: `Upvote deposit <amount> for appchain <appchain_id> is withdrawed by <sender>.`

### Withdraw a certain amount of downvote deposit

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `amount`: The amount which the sender wants to withdraw

Qualification of this action:

* The `amount` must not be larger than `downvote deposit` of `sender` for `appchain_id`.

Reduce `amount` from `downvote deposit` of `sender` for `appchain_id`, reduce `amount` from `downvote deposit` of `appchain basedata` for `appchain_id`, and send `amount` of OCT token back to `sender`.

Generate log: `Downvote deposit <amount> for appchain <appchain_id> is withdrawed by <sender>.`

### Transfer the ownership of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `account_id`: The account id of new owner of the given appchain.

Qualification of this action:

* The `sender` must be current `appchain owner` of `appchain basedata` corresponding to `appchain_id`.

The `appchain owner` of `appchain basedata` corresponding to `appchain_id` is set to `account_id`.

Generate log: `The owner of appchain <appchain_id> is set to <account_id>.`

### Pass auditing of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `appchain_anchor_code`: The wasm code of `appchain anthor` of the given appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `auditing`.

The `appchain state` of `appchain basedata` corresponding to `appchain_id` is set to `inQueue`. The value of `appchain_anchor_code` is staged to `appchain basedata` corresponding to `appchain_id` in this contract.

Generate log: `Appchain <appchain_id> is in queue.`

> The auditing of appchain code is an offchain action which will be completed by the task force assigned by Octopus DAO.

### Change code of an appchain anchor

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `appchain_anchor_code`: The wasm code of `appchain anthor` of the given appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `inQueue`.

The value of `appchain_anchor_code` is staged to `appchain basedata` corresponding to `appchain_id` in this contract.

### Reject an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `refund_percent`: The percent of `register deposit` for refunding for the rejection. This should be an unsigned integer not bigger than 100.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `registered`, `auditing` or `inQueue`.

The `appchain state` of `appchain basedata` corresponding to `appchain_id` is set to `dead`. And send a certain amount of OCT token back to the `appchain owner`. The amount is calculated by:

```js
refund_amount = register_deposit_of_the_appchain * refund_percent / 100
```

Generate log: `Appchain <appchain_id> is rejected, and <refund_amount> OCT token returned.`

### Count voting score

Qualification of this action:

* The `sender` must be the `owner`.
* The action can only be called once a day.

This action will count `voting score` of all appchains whose `appchain state` is `inQueue`, and store the results in `appchain basedata` in this contract.

The `voting score` of an appchain is calculated by:

```js
voting_score_of_an_appchain += sum(upvote_amount_from_a_voter_of_the_appchain) - sum(downvote_amount_from_a_voter_of_the_appchain);
```

> This action should be performed every day by an offchain daemon or an operator.

### Change reduction percent of voting result

This action needs the following parameters:

* `value`: The percent (unsigned integer not bigger than 100) which all appchains' voting score will be reduced in the next voting period.

Qualification of this action:

* The `sender` must be the `owner`.
* The `value` must be not smaller than 0 and not bigger than 100.

The `voting result reduction percent` is set to `value`.

### Conclude voting score

This action needs the following parameters:

* `vote_result_reduction_percent`: The percent (unsigned integer not bigger than 100) which all appchains' voting score will be reduced in the next voting period.

Qualification of this action:

* The `sender` must be the `owner`.

The `appchain state` of appchain with the largest `voting score` will become `staging`. Then:

* Create subaccount `<appchain_id>.<account id of this contract>`.
* Transfer a certain amount of NEAR token to account `<appchain_id>.<account id of this contract>` for storage deposit.
* Add a new full access key to the new `appchain anchor` for the `owner`.
* Deploy the code of `appchain anchor` of the appchain to the account `<appchain_id>.<account id of this contract>`.
* Initialize new `appchain anchor` by the metadata of the appchain.
* Store the account of new `appchain anchor` for the appchain in this contract.

The `voting score` of all appchains with state `inQueue` will be reduced by value of `vote_result_reduction_percent`.

Generate log: `Appchain <appchain_id> goes staging at <account>.`

> This action should be performed when the period of appchain selection for `staging` ends.

### Sync state of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `state`: The new state of the given appchain.

Qualification of this action:

* The `sender` must be the account which the `appchain anchor` corresponding to `appchain_id` is deployed.
* The value of `state` must be one of `staging`, `booting`, `active`, `broken` and `dead`, which are managed by `appchain anchor`.

The `appchain state` of `appchain basedata` corresponding to `appchain_id` is set to `state`.

Generate log: `The state of appchain <appchain_id> changes to <new state>.`

### Remove an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `dead`.

This action will remove the appchain corresponding to `appchain_id` from this contract, and delete the account of its `appchain anchor`.

Generate log: `Appchain <appchain_id> and its anchor is removed.`

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
    pub custom_metadata: HashMap<String, String>,
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
#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AppchainStatus {
    pub appchain_id: AppchainId,
    pub appchain_metadata: AppchainMetadata,
    pub appchain_anchor: AccountId,
    pub appchain_owner: AccountId,
    pub register_deposit: U128,
    pub appchain_state: AppchainState,
    pub upvote_deposit: U128,
    pub downvote_deposit: U128,
    pub voting_score: I128,
    pub registered_time: U64,
    pub go_live_time: U64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum AppchainSortingField {
    AppchainId,
    VotingScore,
    RegisteredTime,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum SortingOrder {
    Ascending,
    Descending,
}
```

### Registry status

```rust
/// The interface for querying status of appchain registry
pub trait RegistryStatus {
    /// Get minimum register deposit
    fn get_minimum_register_deposit(&self) -> U128;
    /// Get the value of reduction percent for voting result of all appchains still in queue
    fn get_voting_result_reduction_percent(&self) -> U64;
    /// Get total stake of all appchains in 'staging', 'booting' and 'active' state
    fn get_total_stake(&self) -> U128;
    /// Get appchains whose state is equal to the given AppchainState
    ///
    /// If param `appchain_state` is `Option::None`, return all appchains in registry
    fn get_appchains_with_state_of(
        &self,
        appchain_state: Option<Vec<AppchainState>>,
        page_number: u16,
        page_size: u16,
        sorting_field: AppchainSortingField,
        sorting_order: SortingOrder,
    ) -> Vec<AppchainStatus>;
    /// Get appchains count whose state is equal to the given AppchainState
    ///
    /// If param `appchain_state` is `Option::None`, return count of all appchains in registry
    fn get_appchains_count_of(&self, appchain_state: Option<AppchainState>) -> U64;
    /// Get status of an appchain
    fn get_appchain_status_of(&self, appchain_id: AppchainId) -> AppchainStatus;
    /// Get upvote deposit of a given account id for a certain appchain
    fn get_upvote_deposit_for(&self, appchain_id: AppchainId, account_id: AccountId) -> U128;
    /// Get downvote deposit of a given account id for a certain appchain
    fn get_downvote_deposit_for(&self, appchain_id: AppchainId, account_id: AccountId) -> U128;
}
```

### Registry owner action

```rust
/// The actions which the owner of appchain registry can perform
pub trait RegistryOwnerAction {
    /// Update metadata of an appchain
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        website_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        commit_id: Option<String>,
        contact_email: Option<String>,
        custom_metadata: Option<HashMap<String, String>>,
    );
    /// Change the value of minimum register deposit
    fn change_minimum_register_deposit(&mut self, value: U128);
    /// Change the value of reduction percent for voting result of all appchains still in queue
    fn change_voting_result_reduction_percent(&mut self, value: U64);
    /// Start auditing of an appchain
    fn start_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId, appchain_anchor_code: Vec<u8>);
    /// Change the code of an appchain anchor
    fn change_appchain_anchor_code(
        &mut self,
        appchain_id: AppchainId,
        appchain_anchor_code: Vec<u8>,
    );
    /// Reject an appchain
    fn reject_appchain(&mut self, appchain_id: AppchainId, refund_percent: U64);
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
    /// Update custom metadata of an appchain
    fn update_appchain_custom_metadata(
        &mut self,
        appchain_id: AppchainId,
        custom_metadata: HashMap<String, String>,
    );
    /// Transfer ownership of an appchain to another account
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId);
}
```

### Voter action

```rust
/// The actions which the voter can perform
pub trait VoterAction {
    /// Withdraw a certain amount of upvote deposit for an appchain
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128);
    /// Withdraw a certain amount of downvote deposit for an appchain
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128);
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

### Ownable

```rust
pub trait Ownable {
    fn assert_owner(&self) {
        require!(env::predecessor_account_id() == self.get_owner(), "Owner must be predecessor");
    }
    fn get_owner(&self) -> AccountId;
    fn set_owner(&mut self, owner: AccountId);
}
```

### Upgradable

```rust
pub trait Upgradable {
    fn get_staging_duration(&self) -> WrappedDuration;
    fn stage_code(&mut self, code: Vec<u8>, timestamp: Timestamp);
    fn deploy_code(&mut self) -> Promise;

    /// Implement migration for the next version.
    /// Should be `unimplemented` for a new contract.
    /// TODO: consider adding version of the contract stored in the storage?
    fn migrate(&mut self) {
        unimplemented!();
    }
}
```
