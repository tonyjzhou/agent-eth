# https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#__docusaurus_skipToContent_fallback)
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
  * [Mint Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position)


On this page
# Mint Position
Similar to Uniswap v3, liquidity positions are minted as ERC-721 tokens and depend on a _periphery_ contract. v4's `PositionManager` contract will facilitate liquidity management
### Context[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#context "Direct link to Context")
Please note that `PositionManager` is a command-based contract, where integrators will be encoding commands and their corresponding parameters.
### Setup[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#setup "Direct link to Setup")
See the [setup guide](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
# Guide
Below is a step-by-step guide for minting a v4 liquidity position, in _solidity_
### 1. Import and define `IPositionManager`[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#1-import-and-define-ipositionmanager "Direct link to 1-import-and-define-ipositionmanager")
```
import{IPositionManager}from"v4-periphery/src/interfaces/IPositionManager.sol";// inside a contract, test, or foundry script:IPositionManager posm =IPositionManager(<address>);
```

### 2. Encode Actions[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#2-encode-actions "Direct link to 2. Encode Actions")
To mint a position, two actions are required:
  * mint operation - the creation of the liquidity position
  * settle pair - the two tokens to be paid by msg.sender


**If providing ETH liquidity, a third action is required:**
  * sweep - to recover excess eth sent to the position manager


```
import{Actions}from"v4-periphery/src/libraries/Actions.sol";bytesmemory actions = abi.encodePacked(uint8(Actions.MINT_POSITION),uint8(Actions.SETTLE_PAIR));// For ETH liquidity positionsbytesmemory actions = abi.encodePacked(uint8(Actions.MINT_POSITION),uint8(Actions.SETTLE_PAIR),uint8(Actions.SWEEP));
```

### 3. Encode Parameters[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#3-encode-parameters "Direct link to 3. Encode Parameters")
```
bytes[]memory params =newbytes[](2);// new bytes[](3) for ETH liquidity positions
```

The `MINT_POSITION` action requires the following parameters:
Parameter| Type| Description  
---|---|---  
`poolKey`|  _PoolKey_|  where the liquidity will be added to  
`tickLower`|  _int24_|  the lower tick boundary of the position  
`tickUpper`|  _int24_|  the upper tick boundary of the position  
`liquidity`|  _uint256_|  the amount of liquidity units to mint  
`amount0Max`|  _uint128_|  the maximum amount of currency0 msg.sender is willing to pay  
`amount1Max`|  _uint128_|  the maximum amount of currency1 msg.sender is willing to pay  
`recipient`|  _address_|  the address that will receive the liquidity position (ERC-721)  
`hookData`| _bytes_|  arbitrary data that will be forwarded to hook functions  
```
Currency currency0 = Currency.wrap(<tokenAddress1>);// tokenAddress1 = 0 for native ETHCurrency currency1 = Currency.wrap(<tokenAddress2>);PoolKey poolKey =PoolKey(currency0, currency1,3000,60,IHooks(hook));params[0]= abi.encode(poolKey, tickLower, tickUpper, liquidity, amount0Max, amount1Max, recipient, hookData);
```

The `SETTLE_PAIR` action requires the following parameters:
  * `currency0` - _Currency_ , one of the tokens to be paid by msg.sender
  * `currency1` - _Currency_ , the other token to be paid by msg.sender


```
params[1]= abi.encode(currency0, currency1);
```

The `SWEEP` action requires the following parameters:
  * `currency` - _Currency_ , token to sweep - most commonly native Ether: `CurrencyLibrary.ADDRESS_ZERO`
  * `recipient` - _address_ , where to send excess tokens


```
params[2]= abi.encode(currency, recipient);
```

### 4. Submit Call[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#4-submit-call "Direct link to 4. Submit Call")
The entrypoint for all liquidity operations is `modifyLiquidities()`
```
uint256 deadline = block.timestamp +60;uint256 valueToPass = currency0.isAddressZero()? amount0Max :0;posm.modifyLiquidities{value: valueToPass}(  abi.encode(actions, params),  deadline);
```

## Additional notes:[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#additional-notes "Direct link to Additional notes:")
  * To obtain balance changes, callers should read token balances before and after the `.modifyLiquidities()` call


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/01-mint-position.mdx)
Was this helpful?
[PreviousSetup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)[NextIncrease Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity)
On this page
  * [Context](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#context)
  * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#setup)
  * [1. Import and define `IPositionManager`](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#1-import-and-define-ipositionmanager)
  * [2. Encode Actions](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#2-encode-actions)
  * [3. Encode Parameters](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#3-encode-parameters)
  * [4. Submit Call](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#4-submit-call)
  * [Additional notes:](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position#additional-notes)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/01-mint-position.mdx)
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
