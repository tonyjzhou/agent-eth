# https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
          * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
          * [Mint Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position)
          * [Increase Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity)
          * [Decrease Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/decrease-liquidity)
          * [Collect Fees](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/collect)
          * [Burn Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
          * [Batch Modify](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Quickstart
  * Manage Liquidity
  * [Increase Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity)


On this page
# Increase Liquidity
### Context[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#context "Direct link to Context")
Please note that `PositionManager` is a command-based contract, where integrators will be encoding commands and their corresponding parameters.
Increasing liquidity assumes the position already exists and the user wants to add more tokens to the position.
### Setup[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#setup "Direct link to Setup")
See the [setup guide](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
# Guide
Below is a step-by-step guide for increasing a position's liquidity, in _solidity_.
### 1. Import and define `IPositionManager`[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#1-import-and-define-ipositionmanager "Direct link to 1-import-and-define-ipositionmanager")
```
import{IPositionManager}from"v4-periphery/src/interfaces/IPositionManager.sol";// inside a contract, test, or foundry script:IPositionManager posm =IPositionManager(<address>);
```

### 2. Encode Actions[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#2-encode-actions "Direct link to 2. Encode Actions")
To increase a position's liquidity, the first action must be:
  * _increase_ operation - the addition of liquidity to an existing position.


For _delta resolving_ operations, developers may need to choose between `SETTLE_PAIR`, `CLOSE_CURRENCY`, or `CLEAR_OR_TAKE` actions.
> In Uniswap v4, fee revenue is automatically credited to a position on increasing liquidity
> There are some cases, where the fee revenue can entirely "pay" for a liquidity increase, and remainder tokens need to be collected
If increasing the liquidity requires the transfer of both tokens:
  * _settle pair_ - pays a pair of tokens, to increase liquidity


**If increasing the liquidity for ETH positions, a third action is required:**
  * _sweep_ - to recover excess eth sent to the position manager


Otherwise:
  * _close currency_ - automatically determines if a currency should be settled or taken.
  * OR _clear or take_ - if the token amount to-be-collected is below a threshold, opt to forfeit the dust. Otherwise, claim the tokens


```
import{Actions}from"v4-periphery/src/libraries/Actions.sol";
```

If both tokens need to be sent:
```
bytesmemory actions = abi.encodePacked(uint8(Actions.INCREASE_LIQUIDITY),uint8(Actions.SETTLE_PAIR));
```

If increasing liquidity for ETH positions:
```
bytesmemory actions = abi.encodePacked(uint8(Actions.INCREASE_LIQUIDITY),uint8(Actions.SETTLE_PAIR),uint8(Actions.SWEEP));
```

If converting fees to liquidity, and expect excess fees to be collected
```
bytesmemory actions = abi.encodePacked(uint8(Actions.INCREASE_LIQUIDITY),uint8(Actions.CLOSE_CURRENCY),uint8(Actions.CLOSE_CURRENCY));
```

If converting fees to liquidity, forfeiting dust:
```
bytesmemory actions = abi.encodePacked(uint8(Actions.INCREASE_LIQUIDITY),uint8(Actions.CLEAR_OR_TAKE),uint8(Actions.CLEAR_OR_TAKE));
```

### 3. Encoded Parameters[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#3-encoded-parameters "Direct link to 3. Encoded Parameters")
When settling pair (for non-ETH positions):
```
bytes[]memory params =newbytes[](2);
```

Otherwise:
```
bytes[]memory params =newbytes[](3);
```

The `INCREASE_LIQUIDITY` action requires the following parameters:
Parameter| Type| Description  
---|---|---  
`tokenId`|  _uint256_|  position identifier  
`liquidity`|  _uint256_|  the amount of liquidity to add  
`amount0Max`|  _uint128_|  the maximum amount of currency0 liquidity msg.sender is willing to pay  
`amount1Max`|  _uint128_|  the maximum amount of currency1 liquidity msg.sender is willing to pay  
`hookData`|  _bytes_|  arbitrary data that will be forwarded to hook functions  
```
params[0]= abi.encode(tokenId, liquidity, amount0Max, amount1Max, hookData);
```

The `SETTLE_PAIR` action requires the following parameters:
  * `currency0` - _Currency_ , one of the tokens to be paid by msg.sender
  * `currency1` - _Currency_ , the other token to be paid by msg.sender


In the above case, the parameter encoding is:
```
Currency currency0 = Currency.wrap(<tokenAddress1>);// tokenAddress1 = 0 for native ETHCurrency currency1 = Currency.wrap(<tokenAddress2>);params[1]= abi.encode(currency0, currency1);
```

The `SWEEP` action requires the following parameters:
  * `currency` - _Currency_ , token to sweep - most commonly native Ether: `CurrencyLibrary.ADDRESS_ZERO`
  * `recipient` - _address_ , where to send excess tokens


In this case, the parameter encoding is:
```
params[2]= abi.encode(currency, recipient);
```

The `CLOSE_CURRENCY` action requires only one `currency` parameter and the encoding is:
```
params[1]= abi.encode(currency0)params[2]= abi.encode(currency1)
```

The `CLEAR_OR_TAKE` action requires one `currency` and:
  * `amountMax` - _uint256_ , the maximum threshold to concede dust, otherwise taking the dust.


In this case, the parameter encoding is:
```
params[1]= abi.encode(currency0, amount0Max);params[2]= abi.encode(currency1, amount1Max);
```

### 4. Submit Call[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#4-submit-call "Direct link to 4. Submit Call")
The entrypoint for all liquidity operations is `modifyLiquidities()`.
```
uint256 deadline = block.timestamp +60;uint256 valueToPass = currency0.isAddressZero()? amount0Max :0;posm.modifyLiquidities{value: valueToPass}(  abi.encode(actions, params),  deadline);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/02-increase-liquidity.mdx)
Was this helpful?
[PreviousMint Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position)[NextDecrease Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/decrease-liquidity)
On this page
  * [Context](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#context)
  * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#setup)
  * [1. Import and define `IPositionManager`](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#1-import-and-define-ipositionmanager)
  * [2. Encode Actions](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#2-encode-actions)
  * [3. Encoded Parameters](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#3-encoded-parameters)
  * [4. Submit Call](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity#4-submit-call)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/02-increase-liquidity.mdx)
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
