# https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
          * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
          * [Mint Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position)
          * [Increase Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity)
          * [Decrease Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/decrease-liquidity)
          * [Collect Fees](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/collect)
          * [Burn Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
          * [Batch Modify](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
    * [Permit2](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Quickstart
  * Manage Liquidity
  * [Burn Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)


On this page
# Burn Position
### Context[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#context "Direct link to Context")
To liquidate a position, the _burn_ functionality can be invoked. The funds in the position will be withdrawn and all the information of the underlying token will be cleared. Burning the position is a cost effective way to exit as a liquidity provider.
### Setup[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#setup "Direct link to Setup")
See the [setup guide](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
# Guide
Below is a step-by-step guide to burn a position.
### 1. Import and define `IPositionManager`[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#1-import-and-define-ipositionmanager "Direct link to 1-import-and-define-ipositionmanager")
```
import{IPositionManager}from"v4-periphery/src/interfaces/IPositionManager.sol";// inside a contract, test, or foundry script:IPositionManager posm =IPositionManager(<address>);
```

### 2. Encode Actions[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#2-encode-actions "Direct link to 2. Encode Actions")
To burn a position, two actions are required:
  * burn operation - clears position entirely, withdrawing funds
  * take pair - sends withdrawn funds to the recipient


```
import{Actions}from"v4-periphery/src/libraries/Actions.sol";bytesmemory actions = abi.encodePacked(uint8(Actions.BURN_POSITION),uint8(Actions.TAKE_PAIR));
```

### 3. Encode Parameters[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#3-encode-parameters "Direct link to 3. Encode Parameters")
```
bytes[]memory params =newbytes[](2);
```

The `BURN_POSITION` action requires the following parameters:
Parameter| Type| Description  
---|---|---  
`tokenId`|  _uint256_|  position identifier  
`amount0Min`|  _uint128_|  the minimum amount of currency0 liquidity msg.sender is expecting to get back  
`amount1Min`|  _uint128_|  the minimum amount of currency1 liquidity msg.sender is expecting to get back  
`hookData`|  _bytes_|  arbitrary data that will be forwarded to hook functions  
```
params[0]= abi.encode(tokenId, amount0Min, amount1Min, hookData);
```

The `TAKE_PAIR` action requires the following parameters:
Parameter| Type| Description  
---|---|---  
`currency0`|  _Currency_|  first token currency  
`currency1`|  _Currency_|  second token currency  
`recipient`|  _address_|  address that will receive the tokens  
```
params[1]= abi.encode(currency0, currency1, recipient);
```

### 4. Submit Call[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#4-submit-call "Direct link to 4. Submit Call")
The entrypoint for all liquidity operations is `modifyLiquidities()`
```
uint256 deadline = block.timestamp +60;posm.modifyLiquidities(  abi.encode(actions, params),  deadline);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/05-burn-liquidity.mdx)
Was this helpful?
[PreviousCollect Fees](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/collect)[NextBatch Modify](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
On this page
  * [Context](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#context)
  * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#setup)
  * [1. Import and define `IPositionManager`](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#1-import-and-define-ipositionmanager)
  * [2. Encode Actions](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#2-encode-actions)
  * [3. Encode Parameters](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#3-encode-parameters)
  * [4. Submit Call](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity#4-submit-call)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/05-burn-liquidity.mdx)
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
