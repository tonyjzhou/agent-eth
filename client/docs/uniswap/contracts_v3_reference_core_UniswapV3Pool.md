# https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
          * [UniswapV3Factory](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
          * [UniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
          * [UniswapV3PoolDeployer](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
          * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
          * [Libraries](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
    * [Permit2](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * Core
  * [UniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)


On this page
# UniswapV3Pool
## Functions[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#functions "Direct link to Functions")
### _blockTimestamp[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#_blocktimestamp "Direct link to _blockTimestamp")
```
function_blockTimestamp()internalview virtual returns(uint32)
```

Returns the block timestamp truncated to 32 bits, i.e. mod 2**32. This method is overridden in tests.
### snapshotCumulativesInside[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#snapshotcumulativesinside "Direct link to snapshotCumulativesInside")
```
functionsnapshotCumulativesInside(int24 tickLower,int24 tickUpper)externalview override noDelegateCall returns(int56 tickCumulativeInside,uint160 secondsPerLiquidityInsideX128,uint32 secondsInside)
```

Returns a snapshot of the tick cumulative, seconds per liquidity and seconds inside a tick range
Snapshots must only be compared to other snapshots, taken over a period for which a position existed. I.e., snapshots cannot be compared if a position is not held for the entire period between when the first snapshot is taken and the second snapshot is taken.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tickLower`| int24| The lower tick of the range  
`tickUpper`| int24| The upper tick of the range  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#return-values "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`tickCumulativeInside`| int56| The snapshot of the tick accumulator for the range  
`secondsPerLiquidityInsideX128`| uint160| The snapshot of seconds per liquidity for the range  
`secondsInside`| uint32| The snapshot of seconds per liquidity for the range  
### observe[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#observe "Direct link to observe")
```
functionobserve(uint32[] secondsAgos)externalview override noDelegateCall returns(int56[] tickCumulatives,uint160[] secondsPerLiquidityCumulativeX128s)
```

Returns the cumulative tick and liquidity as of each timestamp `secondsAgo` from the current block timestamp
To get a time weighted average tick or liquidity-in-range, you must call this with two values, one representing the beginning of the period and another for the end of the period. E.g., to get the last hour time-weighted average tick, you must call it with secondsAgos = [3600, 0]. The time weighted average tick represents the geometric time weighted average price of the pool, in log base sqrt(1.0001) of token1 / token0. The TickMath library can be used to go from a tick value to a ratio.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-1 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`secondsAgos`| uint32[]| From how long ago each cumulative tick and liquidity value should be returned  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#return-values-1 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`tickCumulatives`| int56[]| Cumulative tick values as of each `secondsAgos` from the current block timestamp  
`secondsPerLiquidityCumulativeX128s`| uint160[]| Cumulative seconds per liquidity-in-range value as of each `secondsAgos` from the current block  
timestamp
### increaseObservationCardinalityNext[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#increaseobservationcardinalitynext "Direct link to increaseObservationCardinalityNext")
```
functionincreaseObservationCardinalityNext(uint16 observationCardinalityNext)external override lock noDelegateCall
```

Increase the maximum number of price and liquidity observations that this pool will store
This method is no-op if the pool already has an observationCardinalityNext greater than or equal to the input observationCardinalityNext.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-2 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`observationCardinalityNext`| uint16| The desired minimum number of observations for the pool to store  
### initialize[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#initialize "Direct link to initialize")
```
functioninitialize(uint160 sqrtPriceX96)external override
```

Sets the initial price for the pool
not locked because it initializes unlocked
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-3 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`sqrtPriceX96`| uint160| the initial sqrt price of the pool as a Q64.96  
### mint[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#mint "Direct link to mint")
```
functionmint(address recipient,int24 tickLower,int24 tickUpper,uint128 amount,bytes data)external override lock returns(uint256 amount0,uint256 amount1)
```

Adds liquidity for the given recipient/tickLower/tickUpper position
noDelegateCall is applied indirectly via _modifyPosition
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-4 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`recipient`| address| The address for which the liquidity will be created  
`tickLower`| int24| The lower tick of the position in which to add liquidity  
`tickUpper`| int24| The upper tick of the position in which to add liquidity  
`amount`| uint128| The amount of liquidity to mint  
`data`| bytes| Any data that should be passed through to the callback  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#return-values-2 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amount0`| uint256| The amount of token0 that was paid to mint the given amount of liquidity. Matches the value in the callback  
`amount1`| uint256| The amount of token1 that was paid to mint the given amount of liquidity. Matches the value in the callback  
### collect[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#collect "Direct link to collect")
```
functioncollect(address recipient,int24 tickLower,int24 tickUpper,uint128 amount0Requested,uint128 amount1Requested)external override lock returns(uint128 amount0,uint128 amount1)
```

Collects tokens owed to a position
Does not recompute fees earned, which must be done either via mint or burn of any amount of liquidity. Collect must be called by the position owner. To withdraw only token0 or only token1, amount0Requested or amount1Requested may be set to zero. To withdraw all tokens owed, caller may pass any value greater than the actual tokens owed, e.g. type(uint128).max. Tokens owed may be from accumulated swap fees or burned liquidity.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-5 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`recipient`| address| The address which should receive the fees collected  
`tickLower`| int24| The lower tick of the position for which to collect fees  
`tickUpper`| int24| The upper tick of the position for which to collect fees  
`amount0Requested`| uint128| How much token0 should be withdrawn from the fees owed  
`amount1Requested`| uint128| How much token1 should be withdrawn from the fees owed  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#return-values-3 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amount0`| uint128| The amount of fees collected in token0  
`amount1`| uint128| The amount of fees collected in token1  
### burn[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#burn "Direct link to burn")
```
functionburn(int24 tickLower,int24 tickUpper,uint128 amount)external override lock returns(uint256 amount0,uint256 amount1)
```

Burn liquidity from the sender and account tokens owed for the liquidity to the position
noDelegateCall is applied indirectly via _modifyPosition
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-6 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tickLower`| int24| The lower tick of the position for which to burn liquidity  
`tickUpper`| int24| The upper tick of the position for which to burn liquidity  
`amount`| uint128| How much liquidity to burn  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#return-values-4 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amount0`| uint256| The amount of token0 sent to the recipient  
`amount1`| uint256| The amount of token1 sent to the recipient  
### swap[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#swap "Direct link to swap")
```
functionswap(address recipient,bool zeroForOne,int256 amountSpecified,uint160 sqrtPriceLimitX96,bytes data)external override noDelegateCall returns(int256 amount0,int256 amount1)
```

Swap token0 for token1, or token1 for token0
The caller of this method receives a callback in the form of IUniswapV3SwapCallback#uniswapV3SwapCallback
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-7 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`recipient`| address| The address to receive the output of the swap  
`zeroForOne`| bool| The direction of the swap, true for token0 to token1, false for token1 to token0  
`amountSpecified`| int256| The amount of the swap, which implicitly configures the swap as exact input (positive), or exact output (negative)  
`sqrtPriceLimitX96`| uint160| The Q64.96 sqrt price limit. If zero for one, the price cannot be less than this value after the swap. If one for zero, the price cannot be greater than this value after the swap  
`data`| bytes| Any data to be passed through to the callback  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#return-values-5 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amount0`| int256| The delta of the balance of token0 of the pool, exact when negative, minimum when positive  
`amount1`| int256| The delta of the balance of token1 of the pool, exact when negative, minimum when positive  
### flash[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#flash "Direct link to flash")
```
functionflash(address recipient,uint256 amount0,uint256 amount1,bytes data)external override lock noDelegateCall
```

Receive token0 and/or token1 and pay it back, plus a fee, in the callback
The caller of this method receives a callback in the form of IUniswapV3FlashCallback#uniswapV3FlashCallback Can be used to donate underlying tokens pro-rata to currently in-range liquidity providers by calling with 0 amount1 and sending the donation amount(s) from the callback
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-8 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`recipient`| address| The address which will receive the token0 and token1 amounts  
`amount0`| uint256| The amount of token0 to send  
`amount1`| uint256| The amount of token1 to send  
`data`| bytes| Any data to be passed through to the callback  
### setFeeProtocol[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#setfeeprotocol "Direct link to setFeeProtocol")
```
functionsetFeeProtocol(uint8 feeProtocol0,uint8 feeProtocol1)external override lock onlyFactoryOwner
```

Set the denominator of the protocol's % share of the fees
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-9 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`feeProtocol0`| uint8| new protocol fee for token0 of the pool  
`feeProtocol1`| uint8| new protocol fee for token1 of the pool  
### collectProtocol[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#collectprotocol "Direct link to collectProtocol")
```
functioncollectProtocol(address recipient,uint128 amount0Requested,uint128 amount1Requested)external override lock onlyFactoryOwner returns(uint128 amount0,uint128 amount1)
```

Collect the protocol fee accrued to the pool
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#parameters-10 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`recipient`| address| The address to which collected protocol fees should be sent  
`amount0Requested`| uint128| The maximum amount of token0 to send, can be 0 to collect fees in only token1  
`amount1Requested`| uint128| The maximum amount of token1 to send, can be 0 to collect fees in only token0  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#return-values-6 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amount0`| uint128| The protocol fee collected in token0  
`amount1`| uint128| The protocol fee collected in token1  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/core/UniswapV3Pool.md)
Was this helpful?
[PreviousUniswapV3Factory](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)[NextUniswapV3PoolDeployer](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#functions)
    * [_blockTimestamp](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#_blocktimestamp)
    * [snapshotCumulativesInside](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#snapshotcumulativesinside)
    * [observe](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#observe)
    * [increaseObservationCardinalityNext](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#increaseobservationcardinalitynext)
    * [initialize](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#initialize)
    * [mint](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#mint)
    * [collect](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#collect)
    * [burn](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#burn)
    * [swap](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#swap)
    * [flash](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#flash)
    * [setFeeProtocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#setfeeprotocol)
    * [collectProtocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#collectprotocol)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/core/UniswapV3Pool.md)
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
