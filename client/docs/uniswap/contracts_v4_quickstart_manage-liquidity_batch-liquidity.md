# https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
          * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
          * [Mint Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position)
          * [Increase Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity)
          * [Decrease Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/decrease-liquidity)
          * [Collect Fees](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/collect)
          * [Burn Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
          * [Batch Modify](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
    * [Permit2](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Quickstart
  * Manage Liquidity
  * [Batch Modify](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)


On this page
# Batch Modify
### Context[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#context "Direct link to Context")
As seen in previous guides, `PositionManager` is a command-based contract. This design is conducive to batching complex liquidity operations. For example, developers can encode efficient logic to move liquidity between two positions on entirely different Pools.
### Setup[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#setup "Direct link to Setup")
See the [setup guide](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
# Guide
Below is a general reference guide for batch-operating on multiple liquidity positions, in _solidity_. This guide does _not_ focus on a specific batch sequence, and is intended to be a general guide for `PositionManager`'s command-based interface.
### 1. Encoded Actions[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#1-encoded-actions "Direct link to 1. Encoded Actions")
Actions are divided into two types: _liquidity-operations_ and _delta-resolving_.
  * _liquidity-operations_ - actions which that incur a _balance-change_ , a change in the pool's liquidity
  * _delta-resolving_ - actions which facilitate token transfers, such as _settling_ and _taking_


The _ordering_ of `actions` determines the sequence of operations. The minimum number of actions is roughly two actions; and the maximum is limited by block gas limit. Additionally, _liquidity-operations_ do not have to happen prior to _delta-resolving_ actions. Developers can mix / alternate between the two types of actions.
> **However** is good practice to perform _liquidity-operations_ before _delta-resolving_ actions. Minimizing token transfers and leveraging [_flash accounting_](https://docs.uniswap.org/contracts/v4/concepts/flash-accounting) is more gas efficient
Example: `Action.Y` happens after `Action.X` but before `Action.Z`
```
import{Actions}from"v4-periphery/src/libraries/Actions.sol";bytesmemory actions = abi.encodePacked(uint8(Actions.X),uint8(Actions.Y),uint8(Actions.Z),...);
```

**A Note on Special Actions** :
`PositionManager` supports a few _delta-resolving_ actions beyond the standard `SETTLE` and `TAKE` actions
  * `CLOSE_CURRENCY` - automatically determines if a currency should be settled (paid) or taken. Used for cases where callers may not know the final delta
  * `CLEAR_OR_TAKE`- forfeit tokens if the amount is below a specified threshold, otherwise take the tokens. Used for cases where callers may expect to produce dust
  * `SWEEP` - return any excess token balances to a recipient. Used for cases where callers may conversatively overpay tokens


### 2. Encoded Parameters[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#2-encoded-parameters "Direct link to 2. Encoded Parameters")
Each action has its own parameters to encode. Generally:
  * _liquidity-operations_ - encode tokenIds, liquidity amounts, and slippage
  * _delta-resolving_ - encode currencies, amounts, and recipients


Because actions are ordered, the parameters "zip" with their corresponding actions. The second parameter corresponds to the second action. Every action has its own encoded parameters
```
bytes[]memory params =newbytes[](3);params[0]= abi.encode(...);// parameters for the first actionparams[1]= abi.encode(...);// parameters for the second actionparams[2]= abi.encode(...);// parameters for the third action
```

### 3. Submit Call[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#3-submit-call "Direct link to 3. Submit Call")
The entrypoint for all liquidity operations is `modifyLiquidities()`
```
uint256 deadline = block.timestamp +60;posm.modifyLiquidities(  abi.encode(actions, params),  deadline);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/06-batch-liquidity.mdx)
Was this helpful?
[PreviousBurn Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)[NextSwap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
On this page
  * [Context](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#context)
  * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#setup)
  * [1. Encoded Actions](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#1-encoded-actions)
  * [2. Encoded Parameters](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#2-encoded-parameters)
  * [3. Submit Call](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity#3-submit-call)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/06-batch-liquidity.mdx)
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
