# octopus-appchain-registry

This contract provides a registry for appchains of [Octopus Network](https://oct.network). It contains the metadata of the appchains and manage their lifecycle in Octopus Network.

Contents:

* [Terminology](#terminology)
* [Function specification](#function-specification)
  * [Manage registry settings](#manage-registry-settings)
  * [Manage registry roles](#manage-registry-roles)
  * [Register appchain](#register-appchain)
  * [Appchain owner actions](#appchain-owner-actions)
  * [Manage the lifecycle of appchains](#manage-the-lifecycle-of-appchains)
  * [Pause or resume asset transfer](#pause-or-resume-asset-transfer)
  * [View functions](#view-functions)
* [Registry roles](#registry-roles)
* [Auditing](#auditing)
* [Building and Testing](#building-and-testing)

## Terminology

* `owner`: The owner of this contract, which is the Octopus DAO.
* `appchain anchor`: A NEAR contract which is deployed in a subaccount of the account of this contract. It is in charge of managing the necessary data of an appchain on NEAR protocol, providing security and interoperability for the appchain. The anchor contracts are controlled by the `owner` (Octopus DAO) too, and the [octopus-appchain-anchor](https://github.com/octopus-network/octopus-appchain-anchor) is the standard implementation provided by Octopus Core Team.
* `appchain owner`: The owner of an appchain, usually the developer or someone who represent the developer team.
* `Octopus DAO`: The DAO contract for on-chain governance of Octopus Network.
* `Octopus Council`: The council composed of a certain number of the users with the largest staking amount in Octopus Network.
* `appchain state`: The state of an appchain, which is one of the following:
  * `registered`: The initial state of an appchain, after it is successfully registered.
  * `audited`: The state while the appchain had been audited.
  * `voting`: The state while the octopus council members can upvote in octopus DAO.
  * `booting`: The state while an appchain is booting. This state is managed by `appchain anchor`.
  * `active`: The state while an appchain is active normally. This state is managed by `appchain anchor`.
  * `closing`: The state which an appchain is closing for some technical or governance reasons. This state is managed by `appchain anchor`.
  * `closed`: The state which the lifecycle of an appchain is end.
* `register deposit`: To prevent abuse of audit services, an appchain has to deposit a small amount of OCT token when register.
* `registry settings`: A set of settings for this contract, which contains the following fields:
  * `minimum register deposit`: The minimum amount of `register deposit` which is specified by Octopus DAO.
* `registry roles`: A set of roles for this contract, which contains the following fields:
  * `registry settings manager`: The account id that can perform actions to change `registry settings`.
  * `appchain lifecycle manager`: The account id that can manage the lifecycle of appchains in registry.
  * `octopus council`: The account id representing the octopus council (in octopus DAO).

## Function specification

### Manage registry settings

This contract has a set of functions to manage `registry settings`. Each of these functions is for changing one field of the settings. Only the account that is set to role `registry settings manager` can call these functions. (Refer to [Registry roles](#registry-roles).)

### Manage registry roles

This contract has a set of functions to manage `registry roles`. Each of these functions is for changing the account that acts as a certain role. (Refer to [Registry roles](#registry-roles).)

### Register appchain

Anyone can register appchain in this contract by providing necessary information for the appchain with a certain amount of OCT token deposited to this contract. The amount should be not less than `minimum register deposit` of `registry settings`.

> The `register deposit` will NOT be refunded in any condition. It is considered as auditing fee for registered appchain.

### Appchain owner actions

The account that successfully registered an appchain in this contract will automatically become `the owner of the appchain`. This account can perform the following actions:

* Transfer the ownership of the certain appchain to another account.
* Withdraw the registration of the certain appchain.

### Manage the lifecycle of appchains

This contract has a set of functions to manage the lifecycle of appchains registered in it. The general process of appchain lifecycle management are as the following:

![Appchain state transition flow](/images/appchain_state_transition.drawio.png)

Business action | Description | Contract function | Role/Account to perform action in contract | Appchain State after the action
---|---|---|---|---
Register appchain | Refer to [Register appchain](#register-appchain). | ft_on_transfer | any account / manually | Registered
Audit appchain | Octopus network team will check necessary content to confirm whether the appchain can be proposed in octopus DAO to start booting. | pass_auditing_appchain | Appchain lifecycle manager / manually | Audited
Sponsor appchain | Members of Octopus Council can sponsor a certain appchain to create a proposal in Octpus DAO for voting. | start_voting_appchain | Appchain lifecycle manager / manually | Voting
Vote for appchain | Members of Octopus Council can vote for a certain appchain in Octopus DAO. | start_booting_appchain | Octopus DAO account / automatically | Booting
Reject appchain | Octopus Network team can reject an appchain if it didn't pass auditing or it didn't pass voting in Octopus DAO. | reject_appchain | Appchain lifecycle manager / manually | Closed
Boot appchain | Octopus Network team will prepare the necessary infrastructure for the appchain to go live. Refer to [Octopus Appchain Anchor](https://github.com/octopus-network/octopus-appchain-anchor). | N/A | N/A | N/A
Remove appchain | Octopus Network team can remove an appchain from this contract if it is dead. | remove_appchain | Appchain lifecycle manager / manually | N/A

Besides the above actions, the `Appchain lifecycle manager` can also update the metadata of any appchain.

### Pause or resume asset transfer

The owner account of this contract can pause or resume asset transfer in this contract. The actions that will be limited should be:

* Transfer OCT token into this contract by function `ft_transfer_call` of OCT token contract, with a certain message attached (to register an appchain or upvote/downvote for an appchain).
* Withdraw upvote/downvote deposit from this contract.

### View functions

This contract has a set of view functions for anyone to get the status detail of this contract.

## Registry roles

This contract has different roles to restrict access to certain functions.

Contract action | Contract owner | Registry settings manager | Appchain lifecycle manager | Octopus Council
---|---|---|---|---
change_appchain_lifecycle_manager | allowed |  | allowed |
change_registry_settings_manager | allowed | allowed |  |
change_octopus_council | allowed |  |  |
change_minimum_register_deposit |  | allowed |  |
update_appchain_metadata |  |  | allowed |
pass_auditing_appchain |  |  | allowed |
start_voting_appchain |  |  | allowed |
start_booting_appchain |  |  |  | allowed
reject_appchain |  |  | allowed |
remove_appchain |  |  | allowed |
pause_asset_transfer | allowed |  |  |
resume_asset_transfer | allowed |  |  |

> An account can NOT has different roles at the same time.

## Auditing

This contract (`v1.1.0`) had been audited by [Halborn](https://halborn.com). Here is the [report](https://github.com/octopus-network/octopus-appchain-registry/blob/main/Octopus_Network_NEAR_Smart_Contract_Security_Audit_Report_Halborn_Final.pdf).

## Building and testing

It is recommended to use [contract-builder](contract-builder/README.md) to build this contract.

Run `./build.sh test` to build and run all test code.
