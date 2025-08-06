# https://docs.uniswap.org/contracts/v3/reference/governance/overview

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/governance/overview#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
          * [Overview](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
    * [Permit2](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * Governance
  * [Overview](https://docs.uniswap.org/contracts/v3/reference/governance/overview)


On this page
# Overview
> The updated reference for the newly deployed Governor Bravo is available via [Etherscan](https://etherscan.io/address/0x408ED6354d4973f66138C91495F2f2FCbd8724C3), some of the reference material below may be out of date.
The Uniswap protocol is governed and upgraded by UNI token holders, using three distinct components; the UNI token, governance module, and Timelock. Together, these contracts allow the community to propose, vote, and implement changes to the uniswap protocol.
Any addresses with more than 2.5M UNI (0.25% of total supply) delegated to it may propose governance actions, which contain finished, executable code. When a proposal is created, the community can cast their votes during a 7 day voting period. If there are more 'For' votes than 'Against' (i.e. a simple majority), and the number of 'For' votes >40M (meeting the quorum), it is queued in the Timelock, and may be executed in a minimum of 2 days.
## Timelock[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#timelock "Direct link to Timelock")
The Timelock contract can modify system parameters, logic, and contracts in a 'time-delayed, opt-out' upgrade pattern. Timelock has a hard-coded minimum delay of 2 days, which is the least amount of notice possible for a governance action. Each proposed action will be published at a minimum of 2 days in the future from the time of announcement. Major upgrades, such as changing the risk system, may have up to a 30 day delay. Timelock is controlled by the governance module; pending and completed governance actions can be monitored on the Timelock Dashboard.
![](https://docs.uniswap.org/assets/images/gov_diagram-1-9bc9f7797121de9e8c8210d39b1c0dc3.png)
# Key Events
## DelegateChanged[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#delegatechanged "Direct link to DelegateChanged")
```
DelegateChanged(addressindexed delegator,addressindexed fromDelegate,addressindexed toDelegate)
```

Emitted when an account changes its delegate.
## DelegateVotesChanged[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#delegatevoteschanged "Direct link to DelegateVotesChanged")
```
DelegateVotesChanged(addressindexed delegate,uint previousBalance,uint newBalance)
```

Emitted when a delegate account's vote balance changes.
## ProposalCreated[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposalcreated "Direct link to ProposalCreated")
```
ProposalCreated(uint id,address proposer,address[] targets,uint[] values,string[] signatures,bytes[] calldatas,uint startBlock,uint endBlock,string description)
```

Emitted when a new proposal is created.
## VoteCast[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#votecast "Direct link to VoteCast")
```
VoteCast(address voter,uint proposalId,bool support,uint votes)
```

Emitted when a vote has been cast on a proposal.
## ProposalCanceled[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposalcanceled "Direct link to ProposalCanceled")
```
ProposalCanceled(uint id)
```

Emitted when a proposal has been canceled.
## ProposalQueued[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposalqueued "Direct link to ProposalQueued")
```
ProposalQueued(uint id,uint eta)
```

Emitted when a proposal has been queued in the Timelock.
## ProposalExecuted[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposalexecuted "Direct link to ProposalExecuted")
```
ProposalExecuted(uint id)
```

Emitted when a proposal has been executed in the Timelock.
# Read-Only Functions: UNI
## Get Current Votes[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#get-current-votes "Direct link to Get Current Votes")
```
functiongetCurrentVotes(address account)returns(uint96)
```

Returns the balance of votes for an account as of the current block.
Name| Type|   
---|---|---  
account| `address`| Address of the account of which to retrieve the number of votes.  
## Get Prior Votes[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#get-prior-votes "Direct link to Get Prior Votes")
```
functiongetPriorVotes(address account,uint blockNumber)returns(uint96)
```

Returns the prior number of votes for an account at a specific block number. The block number passed must be a finalized block or the function will revert.
Name| Type|   
---|---|---  
account| `address`| Address of the account of which to retrieve the prior number of votes.  
blocknumber| `uint`| The block number at which to retrieve the prior number of votes.  
| |   
unnamed| `uint96`| The number of prior votes  
# State-Changing Functions: UNI
## Delegate[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#delegate "Direct link to Delegate")
```
functiondelegate(address delegatee)
```

Delegate votes from the sender to the delegatee. Users can delegate to 1 address at a time, and the number of votes added to the delegatee’s vote count is equivalent to the balance of UNI in the user’s account. Votes are delegated from the current block and onward, until the sender delegates again, or transfers their UNI.
Name| Type|   
---|---|---  
delegatee| `address`| The address to which msg.sender wishes to delegate their votes to.  
## Delegate By Signature[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#delegate-by-signature "Direct link to Delegate By Signature")
```
functiondelegateBySig(address delegatee,uint nonce,uint expiry,uint8 v,bytes32 r,bytes32 s)
```

Delegate votes from the sender to the delegatee. Users can delegate to 1 address at a time, and the number of votes added to the delegatee’s vote count is equivalent to the balance of UNI in the user’s account. Votes are delegated from the current block and onward, until the sender delegates again, or transfers their UNI.
Name| Type|   
---|---|---  
delegatee| `address`| The address to which msg.sender wishis to delegate their vote to  
nonce| `uint`| The contract state required to match the signature. This can be retrieved from the contract’s public nonces mapping  
expiry| `uint`| The time when the signature expires. A block timestamp in seconds since the unix epoch.  
v| `uint`| The recovery byte of the signature.  
r| `bytes32`| Half of the ECDSA signature pair.  
s| `bytes32`| Half of the ECDSA signature pair.  
# Read-Only Functions: Governor Alpha
## Quorum Votes[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#quorum-votes "Direct link to Quorum Votes")
```
functionquorumVotes()publicpurereturns(uint)
```

Returns the minimum number of votes required for a proposal to succeed.
## Proposal Threshold[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposal-threshold "Direct link to Proposal Threshold")
```
functionproposalThreshold()returns(uint)
```

Returns the minimum number of votes required for an account to create a proposal.
## Proposal Max Operations[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposal-max-operations "Direct link to Proposal Max Operations")
```
functionproposalMaxOperations()returns(uint)
```

Returns the maximum number of actions that can be included in a proposal. Actions are functions calls that will be made when a proposal succeeds and executes.
## Voting Delay[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#voting-delay "Direct link to Voting Delay")
```
functionvotingDelay()returns(uint)
```

Returns the number of blocks to wait before voting on a proposal may begin. This value is added to the current block number when a proposal is created.
## Voting Period[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#voting-period "Direct link to Voting Period")
```
functionvotingPeriod()returns(uint)
```

Returns the duration of voting on a proposal, in blocks.
## Get Actions[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#get-actions "Direct link to Get Actions")
```
functiongetActions(uint proposalId)returns(uint proposalId)publicviewreturns(address[]memory targets,uint[]memory values,string[]memory signatures,bytes[]memory calldatas)
```

Gets the actions of a selected proposal. Pass a proposal ID and get the targets, values, signatures and calldatas of that proposal.
Name| Type|   
---|---|---  
proposalId| `uint`| ID of the proposal  
Returns:
  * Array of addresses of contracts the proposal calls.
  * Array of unsigned integers the proposal uses as values.
  * Array of strings of the proposal’s signatures.
  * Array of calldata bytes of the proposal.


## Get Receipt[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#get-receipt "Direct link to Get Receipt")
```
functiongetReceipt(uint proposalId,address voter)returns(Receipt memory)
```

Returns a proposal ballot receipt of a given voter.
Name| Type|   
---|---|---  
proposalId| `uint`| ID of the proposal in which to get a voter’s ballot receipt.  
voter| `address`| Address of the account of a proposal voter.  
| |   
Receipt| `struct`| A Receipt struct for the ballot of the voter address.  
## State[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#state "Direct link to State")
```
functionstate(uint proposalId)returns(ProposalState)
```

Returns enum of type ProposalState, possible types are:
  * Pending
  * Active
  * Canceled
  * Defeated
  * Succeeded
  * Queued
  * Expired
  * Executed

Name| Type|   
---|---|---  
proposalId| `uint`| ID of the proposal  
# State-Changing Functions: Governor Alpha
## Propose[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#propose "Direct link to Propose")
```
functionpropose(address[]memory targets,uint[]memory values,string[]memory signatures,bytes[]memory calldatas,stringmemory description)returns(uint)
```

Creates a Proposal to change the protocol.
Proposals will be voted on by delegated voters. If there is sufficient support before the voting period ends, the proposal shall be automatically enacted. Enacted proposals are queued and executed in the Timelock contract.
The sender must hold more UNI than the current proposal threshold (proposalThreshold()) as of the immediately previous block. The proposal can have up to 10 actions (based on proposalMaxOperations()).
The proposer cannot create another proposal if they currently have a pending or active proposal. It is not possible to queue two identical actions in the same block (due to a restriction in the Timelock), therefore actions in a single proposal must be unique, and unique proposals that share an identical action must be queued in different blocks.
Name| Type|   
---|---|---  
targets| `address`| The ordered list of target addresses for calls to be made during proposal execution. This array must be the same length as all other array parameters in this function.  
values| `uint`| The ordered list of values (i.e. msg.value) to be passed to the calls made during proposal execution. This array must be the same length as all other array parameters in this function  
signatures| `string`| The ordered list of function signatures to be passed during execution. This array must be the same length as all other array parameters in this function.  
calldatas| `bytes`| The ordered list of data to be passed to each individual function call during proposal execution. This array must be the same length as all other array parameters in this function.  
description| `string`| A human readable description of the proposal and the changes it will enact.  
| |   
Unnamed| `uint`| Returns ID of the new proposal  
## Queue[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#queue "Direct link to Queue")
```
functionqueue(uint proposalId)
```

After a proposal has succeeded, any address can call the queue method to move the proposal into the Timelock queue. A proposal can only be queued if it has succeeded.
Name| Type|   
---|---|---  
proposalId| `uint`| ID of a given successful proposal  
## Execute[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#execute "Direct link to Execute")
```
functionexecute(uint proposalId)payable
```

After the Timelock delay period, any account may invoke the execute method to apply the changes from the proposal to the target contracts. This will invoke each of the actions described in the proposal. This function is payable so the Timelock contract can invoke payable functions that were selected in the proposal.
Name| Type|   
---|---|---  
proposalId| `uint`| ID of a given successful proposal  
## Cancel[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#cancel "Direct link to Cancel")
```
functioncancel(uint proposalId)
```

Cancel a proposal that has not yet been executed. The Guardian is the only one who may execute this unless the proposer does not maintain the delegates required to create a proposal. If the proposer does not have more delegates than the proposal threshold, anyone can cancel the proposal.
Name| Type|   
---|---|---  
proposalId| `uint`| ID of a proposal to cancel  
## Cast Vote[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#cast-vote "Direct link to Cast Vote")
```
functioncastVote(uint proposalId,bool support)
```

Cast a vote on a proposal. The account's voting weight is determined by it's number of delegated votes at the time the proposal becomes active.
Name| Type|   
---|---|---  
proposalId| `uint`| ID of a given successful proposal  
support| `bool`| A boolean of true for 'yes' or false for 'no' on the proposal vote.  
## Cast Vote By Signature[​](https://docs.uniswap.org/contracts/v3/reference/governance/overview#cast-vote-by-signature "Direct link to Cast Vote By Signature")
```
functioncastVoteBySig(uint proposalId,bool support,uint8 v,bytes32 r,bytes32 s)
```

Cast a vote on a proposal. The account's voting weight is determined by its number of delegated votes at the time the proposal became active. This method has the same purpose as Cast Vote, but instead enables offline signatures to participate in governance voting. For more details on how to create an offline signature, review EIP-712.
Name| Type|   
---|---|---  
proposalId| `uint`| ID of a given successful proposal  
support| `bool`| A boolean of true for 'yes' or false for 'no' on the proposal vote.  
expiry| `uint`| The time when the signature expires. A block timestamp in seconds since the unix epoch.  
v| `uint`| The recovery byte of the signature.  
r| `bytes32`| Half of the ECDSA signature pair.  
s| `bytes32`| Half of the ECDSA signature pair.  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/governance/overview.md)
Was this helpful?
[PreviousUnsafeMath](https://docs.uniswap.org/contracts/v3/reference/core/libraries/UnsafeMath)[NextBlockTimestamp](https://docs.uniswap.org/contracts/v3/reference/periphery/base/BlockTimestamp)
On this page
  * [Timelock](https://docs.uniswap.org/contracts/v3/reference/governance/overview#timelock)
  * [DelegateChanged](https://docs.uniswap.org/contracts/v3/reference/governance/overview#delegatechanged)
  * [DelegateVotesChanged](https://docs.uniswap.org/contracts/v3/reference/governance/overview#delegatevoteschanged)
  * [ProposalCreated](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposalcreated)
  * [VoteCast](https://docs.uniswap.org/contracts/v3/reference/governance/overview#votecast)
  * [ProposalCanceled](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposalcanceled)
  * [ProposalQueued](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposalqueued)
  * [ProposalExecuted](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposalexecuted)
  * [Get Current Votes](https://docs.uniswap.org/contracts/v3/reference/governance/overview#get-current-votes)
  * [Get Prior Votes](https://docs.uniswap.org/contracts/v3/reference/governance/overview#get-prior-votes)
  * [Delegate](https://docs.uniswap.org/contracts/v3/reference/governance/overview#delegate)
  * [Delegate By Signature](https://docs.uniswap.org/contracts/v3/reference/governance/overview#delegate-by-signature)
  * [Quorum Votes](https://docs.uniswap.org/contracts/v3/reference/governance/overview#quorum-votes)
  * [Proposal Threshold](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposal-threshold)
  * [Proposal Max Operations](https://docs.uniswap.org/contracts/v3/reference/governance/overview#proposal-max-operations)
  * [Voting Delay](https://docs.uniswap.org/contracts/v3/reference/governance/overview#voting-delay)
  * [Voting Period](https://docs.uniswap.org/contracts/v3/reference/governance/overview#voting-period)
  * [Get Actions](https://docs.uniswap.org/contracts/v3/reference/governance/overview#get-actions)
  * [Get Receipt](https://docs.uniswap.org/contracts/v3/reference/governance/overview#get-receipt)
  * [State](https://docs.uniswap.org/contracts/v3/reference/governance/overview#state)
  * [Propose](https://docs.uniswap.org/contracts/v3/reference/governance/overview#propose)
  * [Queue](https://docs.uniswap.org/contracts/v3/reference/governance/overview#queue)
  * [Execute](https://docs.uniswap.org/contracts/v3/reference/governance/overview#execute)
  * [Cancel](https://docs.uniswap.org/contracts/v3/reference/governance/overview#cancel)
  * [Cast Vote](https://docs.uniswap.org/contracts/v3/reference/governance/overview#cast-vote)
  * [Cast Vote By Signature](https://docs.uniswap.org/contracts/v3/reference/governance/overview#cast-vote-by-signature)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/governance/overview.md)
## Footer
[Uniswap Labs](https://docs.uniswap.org/)
### Developers
  * [Dev Chat](https://discord.com/invite/uniswap)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)
  * [Whitepaper](https://app.uniswap.org/whitepaper-v4.pdf)


### Ecosystem
  * [Uniswap App](https://app.uniswap.org/)
  * [Governance](https://www.uniswapfoundation.org/governance)
  * [Blog](https://blog.uniswap.org/)


### Company
  * [Careers](https://boards.greenhouse.io/uniswaplabs)
  * [Brand Assets](https://github.com/Uniswap/brand-assets/raw/main/Uniswap%20Brand%20Assets.zip)
  * [Terms of Service](https://support.uniswap.org/hc/en-us/articles/30935100859661-Uniswap-Labs-Terms-of-Service)
  * [Privacy Policy](https://support.uniswap.org/hc/en-us/articles/30934457771405-Uniswap-Labs-Privacy-Policy)
  * [Trademark Policy](https://support.uniswap.org/hc/en-us/articles/30934762216973-Uniswap-Labs-Trademark-Guidelines)


### Need Help?
  * [Help Center](https://support.uniswap.org/)
  * [Contact Us](https://support.uniswap.org/hc/en-us/requests/new)


@2025 Uniswap Labs
[](https://github.com/uniswap/uniswap-docs)[](https://twitter.com/Uniswap)[](https://discord.com/invite/uniswap)
