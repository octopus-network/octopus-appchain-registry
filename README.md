# octopus-appchain-registry

This contract provides a registry for appchains of [Octopus Network](https://oct.network). It contains the metadata of the appchains and manage their lifecycle in Octopus Network.

Contents:

* [Terminology](#terminology)
* [Contract data design](#contract-data-design)
* [Custom types](#custom-types)
* [Initialization](#initialization)
* [Registry settings actions](#registry-settings-actions)
  * [Change value of minimum register deposit](#change-value-of-minimum-register-deposit)
  * [Change reduction percent of voting result](#change-reduction-percent-of-voting-result)
  * [Change the interval of counting voting score](#change-the-interval-of-counting-voting-score)
  * [Change operator account of counting voting score](#change-operator-account-of-counting-voting-score)
* [Appchain owner actions](#appchain-owner-actions)
  * [Register an appchain](#register-an-appchain)
  * [Update custom metadata of an appchain](#update-custom-metadata-of-an-appchain)
  * [Transfer the ownership of an appchain](#transfer-the-ownership-of-an-appchain)
* [Registry owner actions](#registry-owner-actions)
  * [Update metadata of an appchain](#update-metadata-of-an-appchain)
  * [Start auditing of an appchain](#start-auditing-of-an-appchain)
  * [Pass auditing of an appchain](#pass-auditing-of-an-appchain)
  * [Reject an appchain](#reject-an-appchain)
  * [Count voting score](#count-voting-score)
  * [Conclude voting score](#conclude-voting-score)
  * [Remove an appchain](#remove-an-appchain)
* [Voter actions](#voter-actions)
  * [Upvote for an appchain](#upvote-for-an-appchain)
  * [Downvote for an appchain](#downvote-for-an-appchain)
  * [Withdraw a certain amount of upvote deposit](#withdraw-a-certain-amount-of-upvote-deposit)
  * [Withdraw a certain amount of downvote deposit](#withdraw-a-certain-amount-of-downvote-deposit)
* [Callback function of token transfer](#callback-function-of-token-transfer)
* [Appchain anchor callback](#appchain-anchor-callback)
  * [Sync state of an appchain](#sync-state-of-an-appchain)
* [Ownable interface](#ownable-interface)
* [Upgradable interface](#upgradable-interface)
* [Registry status](#registry-status)
* [Change notes](#change-notes)
* [Build and test](#build-and-test)

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
* `registry settings`: A set of settings for this contract, which contains the following fields:
  * `minimum register deposit`: The minimum amount of `register deposit` which is specified by Octopus DAO.
  * `voting result reduction percent`: The value of reduction percent for voting result of all appchains still in queue, after an appchain is selected for `staging`.
  * `counting_interval_in_seconds`: The time interval of the frequency of action `count voting score` of appchains `inQueue`.
  * `operator_of_counting_voting_score`: The account id that can perform action `count voting score`.
* `wrapped appchain token`: The wrapped token of the appchain native token, which is managed by a contract in NEAR protocol.
* `era`: A certain period in the corresponding appchain that the reward distribution and validator set switching need to be performed.
* `era reward`: The total reward (in unit of `wrapped appchain token`) of an ended era.
* `voter`: Who can `upvote` or `downvote` an appchain when its `appchain state` is `inQueue`.
* `validator`: Who can deposit a certain amount of OCT token for an appchain when its `appchain state` is `staging`, to indicate that he/she wants to be the validator of an appchain after the appchain goes `booting` state.
* `delegator`: Who can deposit a certain amount of OCT token for an appchain when its `appchain state` is `staging`, to indicate that he/she wants to delegate his/her voting rights to an validator of an appchain after the appchain goes `booting` state.
* `sender`: A NEAR transaction sender, that is the account which perform actions (call functions) on this contract.

## Contract data design

```rust
pub struct AppchainRegistry {
    /// The account of the owner of this contract
    owner: AccountId,
    /// The public key of owner account
    owner_pk: PublicKey,
    /// The earliest time that the staged code can be deployed
    contract_code_staging_timestamp: Timestamp,
    /// The shortest time range between code staging and code deployment
    contract_code_staging_duration: Duration,
    /// The account of OCT token contract
    oct_token: AccountId,
    /// The settings of appchain registry
    registry_settings: LazyOption<RegistrySettings>,
    /// The set of all appchain ids
    appchain_ids: UnorderedSet<AppchainId>,
    /// The map from appchain id to their basedata
    appchain_basedatas: LookupMap<AppchainId, AppchainBasedata>,
    /// The map from pair (appchain id, account id) to their upvote deposit
    upvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    /// The map from pair (appchain id, account id) to their downvote deposit
    downvote_deposits: LookupMap<(AppchainId, AccountId), Balance>,
    /// The appchain id with the highest voting score at a certain time
    top_appchain_id_in_queue: AppchainId,
    /// The total stake of OCT token in all appchains
    total_stake: Balance,
    /// The time of the last calling of function `count_voting_score`
    time_of_last_count_voting_score: Timestamp,
}
```

## Custom Types

```rust
pub struct RegistrySettings {
    /// The minimum deposit amount for registering an appchain
    pub minimum_register_deposit: U128,
    /// The reduction percent of voting score of all appchain `inQueue` after each time
    /// the owner conclude the voting score
    pub voting_result_reduction_percent: u16,
    /// The interval for calling function `count_voting_score`,
    /// in the interval this function can only be called once.
    pub counting_interval_in_seconds: U64,
    /// The only account that can call function `count_voting_score`
    pub operator_of_counting_voting_score: AccountId,
}

/// Appchain metadata
pub struct AppchainMetadata {
    pub website_url: String,
    pub github_address: String,
    pub github_release: String,
    pub commit_id: String,
    pub contact_email: String,
    pub preminted_wrapped_appchain_token: U128,
    pub ido_amount_of_wrapped_appchain_token: U128,
    pub initial_era_reward: U128,
    pub custom_metadata: HashMap<String, String>,
}

/// The state of an appchain
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
    pub validator_count: u32,
    pub total_stake: U128,
}

pub enum AppchainSortingField {
    AppchainId,
    VotingScore,
    RegisteredTime,
}

pub enum SortingOrder {
    Ascending,
    Descending,
}
```

## Initialization

This contract has to be initialized with the following parameters:

* `oct_token_contract`: The account id of OCT token contract.

The `oct_token_contract` should be stored in this contract for using in [Callback function of token transfer](#callback-function-of-token-transfer).

## Registry settings actions

The interface of registry settings actions is defined as:

```rust
/// The actions related to registry settings
pub trait RegistrySettingsActions {
    /// Change the value of minimum register deposit
    fn change_minimum_register_deposit(&mut self, value: U128);
    /// Change the value of reduction percent for voting result of all appchains still in queue
    fn change_voting_result_reduction_percent(&mut self, value: U64);
    /// Change the interval for counting voting score of appchains
    fn change_counting_interval_in_seconds(&mut self, value: U64);
    /// Change operator of counting voting score
    fn change_operator_of_counting_voting_score(&mut self, operator_account: AccountId);
}
```

### Change value of minimum register deposit

This action needs the following parameters:

* `value`: The value of `minimum register deposit`.

Qualification of this action:

* The `sender` must be the `owner`.

The `registry_settings.minimum_register_deposit` is set to `value`.

### Change reduction percent of voting result

This action needs the following parameters:

* `value`: The percent (unsigned integer not bigger than 100) which all appchains' voting score will be reduced in the next voting period.

Qualification of this action:

* The `sender` must be the `owner`.
* The `value` must be not smaller than 0 and not bigger than 100.

The `registry_settings.voting_result_reduction_percent` is set to `value`.

### Change the interval of counting voting score

This action needs the following parameters:

* `value`: The time interval in seconds.

Qualification of this action:

* The `sender` must be the `owner`.

The `registry_settings.counting_interval_in_seconds` is set to `value`.

### Change operator account of counting voting score

This action needs the following parameters:

* `operator_account`: A valid account id on NEAR protocol.

Qualification of this action:

* The `sender` must be the `owner`.

The `registry_settings.operator_of_counting_voting_score` is set to `operator_account`.

## Appchain owner actions

The interface of appchain owner actions is defined as:

```rust
/// The actions which the owner of an appchain can perform
pub trait AppchainOwnerActions {
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

### Register an appchain

This action can only be performed in [Callback function of token transfer](#callback-function-of-token-transfer).

This action needs the following parameters:

* `appchain_id`: The unique identity in Octopus Network. It cannot be duplicated with any other registered appchain.
* `website_url`: The url of the official website of the appchain.
* `github_address`: The address of the github repository of the appchain.
* `github_release`: The release vesion of the github repository of the appchain.
* `commit_id`: The commit id of source code of the github repository of the appchain.
* `contact_email`: The email of the contact of the appchain project, which is used for communidating with the appchain team.
* `preminted_wrapped_appchain_token`: The pre-minted amount of `wrapped appchain token`.
* `ido_amount_of_wrapped_appchain_token`: The IDO amount of `wrapped appchain token`.
* `initial_era_reward`: The initial `era reward` when the appchain go live.
* `custom_metadata`: The extra custom metadata organized by a key-value map.

Qualification of this action:

* The `appchain_id` must NOT be registered in this contract.
* The amount of deposit must be equal to `registry_settings.minimum_register_deposit`.

Processing steps:

* The `register deposit` of `appchain basedata` is set to `amount`.
* The `appchain state` of `appchain basedata` is set to `registered`.
* The `sender_id` will be registered as the owner of the appchain.
* Generate log: `Appchain <appchain_id> is registered by <sender_id> with <amount> OCT token deposited.`
* Return 0.

### Update custom metadata of an appchain

This action needs the following parameters:

* `custom_metadata`: The extra custom metadata organized by a key-value map.

Qualification of this action:

* The `sender` must be current `appchain owner` of `appchain basedata` corresponding to `appchain_id`.

The custom metadata will be updated to `appchain metadata` of `appchain basedata` corresponding to `appchain_id`.

Generate log: `The custom metadata of appchain <appchain_id> is updated by <sender>.`

### Transfer the ownership of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `account_id`: The account id of new owner of the given appchain.

Qualification of this action:

* The `sender` must be current `appchain owner` of `appchain basedata` corresponding to `appchain_id`.

The `appchain owner` of `appchain basedata` corresponding to `appchain_id` is set to `account_id`.

Generate log: `The owner of appchain <appchain_id> is set to <account_id>.`

## Registry owner actions

The interface of registry owner actions is defined as:

```rust
/// The actions which the owner of appchain registry can perform
pub trait RegistryOwnerActions {
    /// Update metadata of an appchain
    fn update_appchain_metadata(
        &mut self,
        appchain_id: AppchainId,
        website_url: Option<String>,
        github_address: Option<String>,
        github_release: Option<String>,
        commit_id: Option<String>,
        contact_email: Option<String>,
        preminted_wrapped_appchain_token: Option<U128>,
        ido_amount_of_wrapped_appchain_token: Option<U128>,
        initial_era_reward: Option<U128>,
        custom_metadata: Option<HashMap<String, String>>,
    );
    /// Start auditing of an appchain
    fn start_auditing_appchain(&mut self, appchain_id: AppchainId);
    /// Pass auditing of an appchain
    fn pass_auditing_appchain(&mut self, appchain_id: AppchainId);
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

### Update metadata of an appchain

This action needs the following parameters:

* `appchain_id`: The unique identity in Octopus Network. It cannot be duplicated with any other registered appchain.
* `website_url`: The url of the official website of the appchain.
* `github_address`: The address of the github repository of the appchain.
* `github_release`: The release vesion of the github repository of the appchain.
* `commit_id`: The commit id of source code of the github repository of the appchain.
* `contact_email`: The email of the contact of the appchain project, which is used for communidating with the appchain team.
* `preminted_wrapped_appchain_token`: The pre-minted amount of `wrapped appchain token`.
* `ido_amount_of_wrapped_appchain_token`: The IDO amount of `wrapped appchain token`.
* `initial_era_reward`: The initial `era reward` when the appchain go live.
* `custom_metadata`: The extra custom metadata organized by a key-value map.

Qualification of this action:

* The `sender` must be the `owner`.

The metadata will be updated to `appchain basedata` corresponding to `appchain_id`.

Generate log: `The metadata of appchain <appchain_id> is updated by manager.`

### Start auditing of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `registered`.

The `appchain state` of `appchain basedata` corresponding to `appchain_id` is set to `auditing`.

Generate log: `Appchain <appchain_id> starts auditing.`

### Pass auditing of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `auditing`.

The `appchain state` of `appchain basedata` corresponding to `appchain_id` is set to `inQueue`.

Generate log: `Appchain <appchain_id> is in queue.`

> The auditing of appchain code is an offchain action which will be completed by the task force assigned by Octopus DAO.

### Reject an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `registered`, `auditing` or `inQueue`.

The `appchain state` of `appchain basedata` corresponding to `appchain_id` is set to `dead`.

Generate log: `Appchain <appchain_id> is rejected.`

### Count voting score

This action will calculate the `voting score` of all appchains `inQueue`. This action can only be performed once in each period of `registry_settings.counting_interval_in_seconds`.

Qualification of this action:

* The `sender` must be `registry_settings.operator_of_counting_voting_score`.
* The value of `env::block_timestamp() - self.time_of_last_count_voting_score` must be bigger than `registry_settings.counting_interval_in_seconds * NANO_SECONDS_MULTIPLE`.

Processing steps:

* Count `voting score` of all appchains whose `appchain state` is `inQueue`, and store the results to `self.appchain_basedatas`. The `voting score` of an appchain is calculated by:

```js
voting_score_of_an_appchain += sum(upvote_amount_from_a_voter_of_the_appchain) - sum(downvote_amount_from_a_voter_of_the_appchain);
```

* The `self.time_of_last_count_voting_score` is set to `env::block_timestamp() - (env::block_timestamp() % (registry_settings.counting_interval_in_seconds * NANO_SECONDS_MULTIPLE)`.

### Conclude voting score

This action will select the appchain with the biggest `voting score` to become the one that will goes to `staging`, and reduce the `voting score` of all appchains that are still `inQueue` by a certain percentage.

Qualification of this action:

* The `sender` must be the `owner`.

Processing steps:

* The `appchain state` of appchain with the largest `voting score` will become `staging`. Then:
  * Create subaccount `<appchain_id>.<account id of this contract>`.
  * Transfer a certain amount of NEAR token to account `<appchain_id>.<account id of this contract>` for storage deposit.
  * Add a new full access key to the new `appchain anchor` for the `owner`.
  * Store the account of new `appchain anchor` for the appchain in this contract.
* The `voting score` of all appchains with state `inQueue` will be reduced by value of `registry_settings.voting_result_reduction_percent`.
* If the `voting score` of an appchain goes to negative number, the state of the appchain will be set to `dead`.
* Generate log: `Appchain <appchain_id> goes staging at <account>.`

### Remove an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.

Qualification of this action:

* The `sender` must be the `owner`.
* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `dead`.
* The `upvote deposit` and `downvote deposit` of the appchain must be both `0`.

Processing steps:

* Remove the appchain corresponding to `appchain_id` from this contract.
* Generate log: `Appchain <appchain_id> and its anchor is removed.`

## Voter actions

The interface for voter actions is defined as:

```rust
/// The actions which the voter can perform
pub trait VoterActions {
    /// Withdraw a certain amount of upvote deposit for an appchain
    fn withdraw_upvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128);
    /// Withdraw a certain amount of downvote deposit for an appchain
    fn withdraw_downvote_deposit_of(&mut self, appchain_id: AppchainId, amount: U128);
}
```

### Upvote for an appchain

This action can only be performed in [Callback function of token transfer](#callback-function-of-token-transfer).

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `amount`: The amount which the sender wants to upvote for the appchain.

Qualification of this action:

* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `inQueue`.

Processing steps:

* Add `amount` to `upvote deposit` of `sender_id` for `appchain_id`. And store the `upvote deposit` to `self.upvote_deposits` by key `(appchain_id, sender_id)`.
* Add `amount` to `upvote deposit` of `appchain basedata` for `appchain_id`.
* Generate log: `Received upvote <amount> for appchain <appchain_id> from <sender_id>.`
* Return 0.

### Downvote for an appchain

This action can only be performed in [Callback function of token transfer](#callback-function-of-token-transfer).

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `amount`: The amount which the sender wants to upvote for the appchain.

Qualification of this action:

* The `appchain state` of `appchain basedata` corresponding to `appchain_id` must be `inQueue`.

Processing steps:

* Add `amount` to `downvote deposit` of `sender_id` for `appchain_id`. And store the `downvote deposit` to `self.downvote_deposits` by key `(appchain_id, sender_id)`.
* Add `amount` to `downvote deposit` of `appchain basedata` for `appchain_id`.
* Generate log: `Received downvote <amount> for appchain <appchain_id> from <sender_id>.`
* Return 0.

### Withdraw a certain amount of upvote deposit

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `amount`: The amount which the sender wants to withdraw

Qualification of this action:

* The `amount` must not be larger than `upvote deposit` of `sender` for `appchain_id`.

Processing steps:

* Reduce `amount` from `upvote deposit` of `sender` for `appchain_id`. If the `upvote deposit` of `sender` goes to `0`, remove the pair `(appchain_id, sender)` from `self.upvote_deposits`.
* Reduce `amount` from `upvote deposit` of `appchain basedata` for `appchain_id`.
* Send `amount` of OCT token back to `sender`.

Generate log: `Upvote deposit <amount> for appchain <appchain_id> is withdrawed by <sender>.`

### Withdraw a certain amount of downvote deposit

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `amount`: The amount which the sender wants to withdraw

Qualification of this action:

* The `amount` must not be larger than `downvote deposit` of `sender` for `appchain_id`.

Processing steps:

* Reduce `amount` from `downvote deposit` of `sender` for `appchain_id`. If the `downvote deposit` of `sender` goes to `0`, remove the pair `(appchain_id, sender)` from `self.downvote_deposits`.
* Reduce `amount` from `downvote deposit` of `appchain basedata` for `appchain_id`.
* Send `amount` of OCT token back to `sender`.

Generate log: `Downvote deposit <amount> for appchain <appchain_id> is withdrawed by <sender>.`

## Callback function of token transfer

This contract has a callback interface `FungibleTokenReceiver::ft_on_transfer` for contract `fungible_token` of `near-contract-standards`.

The callback function `ft_on_transfer` needs the following parameters:

* `sender_id`: The account id of sender of the transfer.
* `amount`: The amount of the transfer.
* `msg`: The message attached to the transfer, which indicates the purpose of the deposit.

If the caller of this callback (`env::predecessor_account_id()`) is `oct_token_contract` which is initialized at construction time of this contract, parse `msg` with the following patterns:

* `register_appchain,<appchain_id>,<website_url>,<github_address>,<github_release>,<commit_id>,<contact_email>,<preminted_wrapped_appchain_token>,<ido_amount_of_wrapped_appchain_token>,<initial_era_reward>`: Perform [Register an appchain](#register-an-appchain).
* `upvote_appchain,<appchain_id>`: Perform [Upvote for an appchain](#upvote-for-an-appchain).
* `downvote_appchain,<appchain_id>`: Perform [Downvote for an appchain](#downvote-for-an-appchain).
* other cases: Throws an error: `Invalid deposit <amount> of OCT token from <sender_id>.`.

If the caller of this callback (`env::predecessor_account_id()`) is NOT `oct_token_contract`, throws an error: `Invalid deposit <amount> of unknown NEP-141 asset from <sender_id>.`.

## Appchain anchor callback

The interface for appchain anchor callback is defined as:

```rust
/// The callback interface for appchain anchor
pub trait AppchainAnchorCallback {
    /// Sync state of an appchain to registry
    fn sync_state_of(
        &mut self,
        appchain_id: AppchainId,
        appchain_state: AppchainState,
        validator_count: u32,
        total_stake: Balance,
    );
}
```

### Sync state of an appchain

This action needs the following parameters:

* `appchain_id`: The id of an appchain.
* `state`: The new state of the given appchain.

Qualification of this action:

* The `sender` must be the account which the `appchain anchor` corresponding to `appchain_id` is deployed.
* The value of `state` must be one of `staging`, `booting`, `active`, `broken` and `dead`, which are managed by `appchain anchor`.

Processing steps:

* The `appchain state` of `appchain basedata` corresponding to `appchain_id` is set to `state`.
* Generate log: `The state of appchain <appchain_id> changes to <new state>.`

## Ownable interface

The interface is from `near_contract_standards::upgrade`.

```rust
pub trait Ownable {
    fn assert_owner(&self) {
        require!(env::predecessor_account_id() == self.get_owner(), "Owner must be predecessor");
    }
    fn get_owner(&self) -> AccountId;
    fn set_owner(&mut self, owner: AccountId);
}
```

## Upgradable interface

The interface is from `near_contract_standards::upgrade`.

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

## Registry status

The interface of registry status is defined as:

```rust
/// The interface for querying status of appchain registry
pub trait RegistryStatus {
    /// Get account id of OCT token
    fn get_oct_token(&self) -> AccountId;
    /// Get registry settings
    fn get_registry_settings(&self) -> RegistrySettings;
    /// Get total stake of all appchains in 'staging', 'booting' and 'active' state
    fn get_total_stake(&self) -> U128;
    /// Get appchain ids
    fn get_appchain_ids(&self) -> Vec<String>;
    /// Get appchains whose state is equal to the given AppchainState
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

## Change notes

Refer to [change notes](https://github.com/octopus-network/octopus-appchain-registry/blob/main/change_notes.md).

## Build and test

Simply run `.build.sh` to build the project. The script will create folder `out` and `res`.

Run `./build.sh test` to build and run all test code.

> The `test_case9` in `./appchain-registry/tests/test_registry_actions.rs` will fail because of the limitation of NEAR transaction. So the upgrade of this contract has to be performed manually (by cli or api.js).
