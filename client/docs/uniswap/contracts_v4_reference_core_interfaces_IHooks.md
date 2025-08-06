# https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
            * [IERC20Minimal](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
            * [IERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims)
            * [IExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload)
            * [IExttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload)
            * [IHooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
            * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
            * [IProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
            * [IUnlockCallback](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IUnlockCallback)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * interfaces
  * [IHooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)


On this page
# IHooks
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/interfaces/IHooks.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
V4 decides whether to invoke specific hooks by inspecting the least significant bits of the address that the hooks contract is deployed to. For example, a hooks contract deployed to address: 0x0000000000000000000000000000000000002400 has the lowest bits '10 0100 0000 0000' which would cause the 'before initialize' and 'after add liquidity' hooks to be used. See the Hooks library for the full spec.
_Should only be callable by the v4 PoolManager._
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#functions "Direct link to Functions")
### beforeInitialize[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeinitialize "Direct link to beforeInitialize")
The hook called before the state of a pool is initialized
```
functionbeforeInitialize(address sender, PoolKey calldata key,uint160 sqrtPriceX96)externalreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the initialize call  
`key`| `PoolKey`| The key for the pool being initialized  
`sqrtPriceX96`| `uint160`| The sqrt(price) of the pool as a Q64.96  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### afterInitialize[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afterinitialize "Direct link to afterInitialize")
The hook called after the state of a pool is initialized
```
functionafterInitialize(address sender, PoolKey calldata key,uint160 sqrtPriceX96,int24 tick)externalreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the initialize call  
`key`| `PoolKey`| The key for the pool being initialized  
`sqrtPriceX96`| `uint160`| The sqrt(price) of the pool as a Q64.96  
`tick`| `int24`| The current tick after the state of a pool is initialized  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### beforeAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeaddliquidity "Direct link to beforeAddLiquidity")
The hook called before liquidity is added
```
functionbeforeAddLiquidity(address sender,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,bytescalldata hookData)externalreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the add liquidity call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for adding liquidity  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### afterAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afteraddliquidity "Direct link to afterAddLiquidity")
The hook called after liquidity is added
```
functionafterAddLiquidity(address sender,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,  BalanceDelta delta,  BalanceDelta feesAccrued,bytescalldata hookData)externalreturns(bytes4, BalanceDelta);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the add liquidity call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for adding liquidity  
`delta`| `BalanceDelta`| The caller's balance delta after adding liquidity; the sum of principal delta, fees accrued, and hook delta  
`feesAccrued`| `BalanceDelta`| The fees accrued since the last time fees were collected from this position  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
`<none>`| `BalanceDelta`| BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency  
### beforeRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeremoveliquidity "Direct link to beforeRemoveLiquidity")
The hook called before liquidity is removed
```
functionbeforeRemoveLiquidity(address sender,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,bytescalldata hookData)externalreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the remove liquidity call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for removing liquidity  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### afterRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afterremoveliquidity "Direct link to afterRemoveLiquidity")
The hook called after liquidity is removed
```
functionafterRemoveLiquidity(address sender,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,  BalanceDelta delta,  BalanceDelta feesAccrued,bytescalldata hookData)externalreturns(bytes4, BalanceDelta);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the remove liquidity call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for removing liquidity  
`delta`| `BalanceDelta`| The caller's balance delta after removing liquidity; the sum of principal delta, fees accrued, and hook delta  
`feesAccrued`| `BalanceDelta`| The fees accrued since the last time fees were collected from this position  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
`<none>`| `BalanceDelta`| BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency  
### beforeSwap[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeswap "Direct link to beforeSwap")
The hook called before a swap
```
functionbeforeSwap(address sender,  PoolKey calldata key,  IPoolManager.SwapParams calldata params,bytescalldata hookData)externalreturns(bytes4, BeforeSwapDelta,uint24);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the swap call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.SwapParams`| The parameters for the swap  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
`<none>`| `BeforeSwapDelta`| BeforeSwapDelta The hook's delta in specified and unspecified currencies. Positive: the hook is owed/took currency, negative: the hook owes/sent currency  
`<none>`| `uint24`| uint24 Optionally override the lp fee, only used if three conditions are met: 1. the Pool has a dynamic fee, 2. the value's 2nd highest bit is set (23rd bit, 0x400000), and 3. the value is less than or equal to the maximum fee (1 million)  
### afterSwap[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afterswap "Direct link to afterSwap")
The hook called after a swap
```
functionafterSwap(address sender,  PoolKey calldata key,  IPoolManager.SwapParams calldata params,  BalanceDelta delta,bytescalldata hookData)externalreturns(bytes4,int128);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the swap call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.SwapParams`| The parameters for the swap  
`delta`| `BalanceDelta`| The amount owed to the caller (positive) or owed to the pool (negative)  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
`<none>`| `int128`| int128 The hook's delta in unspecified currency. Positive: the hook is owed/took currency, negative: the hook owes/sent currency  
### beforeDonate[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforedonate "Direct link to beforeDonate")
The hook called before donate
```
functionbeforeDonate(address sender, PoolKey calldata key,uint256 amount0,uint256 amount1,bytescalldata hookData)externalreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the donate call  
`key`| `PoolKey`| The key for the pool  
`amount0`| `uint256`| The amount of token0 being donated  
`amount1`| `uint256`| The amount of token1 being donated  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### afterDonate[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afterdonate "Direct link to afterDonate")
The hook called after donate
```
functionafterDonate(address sender, PoolKey calldata key,uint256 amount0,uint256 amount1,bytescalldata hookData)externalreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the donate call  
`key`| `PoolKey`| The key for the pool  
`amount0`| `uint256`| The amount of token0 being donated  
`amount1`| `uint256`| The amount of token1 being donated  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IHooks.md)
Was this helpful?
[PreviousIExttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload)[NextIPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#functions)
    * [beforeInitialize](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeinitialize)
    * [afterInitialize](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afterinitialize)
    * [beforeAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeaddliquidity)
    * [afterAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afteraddliquidity)
    * [beforeRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeremoveliquidity)
    * [afterRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afterremoveliquidity)
    * [beforeSwap](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeswap)
    * [afterSwap](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afterswap)
    * [beforeDonate](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforedonate)
    * [afterDonate](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#afterdonate)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IHooks.md)
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
