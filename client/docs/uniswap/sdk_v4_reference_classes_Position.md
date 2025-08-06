# https://docs.uniswap.org/sdk/v4/reference/classes/Position

[Skip to main content](https://docs.uniswap.org/sdk/v4/reference/classes/Position#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/overview)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [Overview](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
          * [Hook](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
          * [Pool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
          * [Position](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
          * [Route](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
          * [Trade](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
          * [V4BaseActionsParser](https://docs.uniswap.org/sdk/v4/reference/classes/V4BaseActionsParser)
          * [V4Planner](https://docs.uniswap.org/sdk/v4/reference/classes/V4Planner)
          * [V4PositionManager](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
          * [V4PositionPlanner](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionPlanner)
        * [enumerations](https://docs.uniswap.org/sdk/v4/reference/enumerations/Actions)
        * [interfaces](https://docs.uniswap.org/sdk/v4/reference/interfaces/AllowanceTransferPermitBatch)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/overview)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Technical Reference
  * classes
  * [Position](https://docs.uniswap.org/sdk/v4/reference/classes/Position)


On this page
# Position
[@uniswap/v4-sdk](https://docs.uniswap.org/sdk/v4/reference/overview) / Position
Defined in: [entities/position.ts:23](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L23)
Represents a position on a Uniswap V4 Pool
## Dev[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#dev "Direct link to Dev")
Similar to the V3 implementation
  * using Currency instead of Token
  * keep in mind that Pool and liquidity must be fetched from the pool manager


## Constructors[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#constructors "Direct link to Constructors")
### new Position()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#new-position "Direct link to new Position\(\)")
> **new Position**(`__namedParameters`): [`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
Defined in: [entities/position.ts:41](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L41)
Constructs a position for a given pool with the given liquidity
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#parameters "Direct link to Parameters")
Parameter| Type  
---|---  
`__namedParameters`| `PositionConstructorArgs`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns "Direct link to Returns")
[`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
## Properties[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#properties "Direct link to Properties")
### liquidity[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#liquidity "Direct link to liquidity")
> `readonly` **liquidity** : `JSBI`
Defined in: [entities/position.ts:27](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L27)
### pool[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#pool "Direct link to pool")
> `readonly` **pool** : [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
Defined in: [entities/position.ts:24](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L24)
### tickLower[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#ticklower "Direct link to tickLower")
> `readonly` **tickLower** : `number`
Defined in: [entities/position.ts:25](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L25)
### tickUpper[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#tickupper "Direct link to tickUpper")
> `readonly` **tickUpper** : `number`
Defined in: [entities/position.ts:26](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L26)
## Accessors[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#accessors "Direct link to Accessors")
### amount0[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#amount0 "Direct link to amount0")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#get-signature "Direct link to Get Signature")
> **get** **amount0**(): `CurrencyAmount`<`Currency`>
Defined in: [entities/position.ts:69](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L69)
Returns the amount of token0 that this position's liquidity could be burned for at the current pool price
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-1 "Direct link to Returns")
`CurrencyAmount`<`Currency`>
### amount1[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#amount1 "Direct link to amount1")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#get-signature-1 "Direct link to Get Signature")
> **get** **amount1**(): `CurrencyAmount`<`Currency`>
Defined in: [entities/position.ts:101](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L101)
Returns the amount of token1 that this position's liquidity could be burned for at the current pool price
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-2 "Direct link to Returns")
`CurrencyAmount`<`Currency`>
### mintAmounts[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#mintamounts "Direct link to mintAmounts")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#get-signature-2 "Direct link to Get Signature")
> **get** **mintAmounts**(): `Readonly`<{ `amount0`: `JSBI`; `amount1`: `JSBI`; }>
Defined in: [entities/position.ts:272](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L272)
Returns the minimum amounts that must be sent in order to mint the amount of liquidity held by the position at the current price for the pool
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-3 "Direct link to Returns")
`Readonly`<{ `amount0`: `JSBI`; `amount1`: `JSBI`; }>
### token0PriceLower[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#token0pricelower "Direct link to token0PriceLower")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#get-signature-3 "Direct link to Get Signature")
> **get** **token0PriceLower**(): `Price`<`Currency`, `Currency`>
Defined in: [entities/position.ts:55](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L55)
Returns the price of token0 at the lower tick
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-4 "Direct link to Returns")
`Price`<`Currency`, `Currency`>
### token0PriceUpper[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#token0priceupper "Direct link to token0PriceUpper")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#get-signature-4 "Direct link to Get Signature")
> **get** **token0PriceUpper**(): `Price`<`Currency`, `Currency`>
Defined in: [entities/position.ts:62](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L62)
Returns the price of token0 at the upper tick
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-5 "Direct link to Returns")
`Price`<`Currency`, `Currency`>
## Methods[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#methods "Direct link to Methods")
### burnAmountsWithSlippage()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#burnamountswithslippage "Direct link to burnAmountsWithSlippage\(\)")
> **burnAmountsWithSlippage**(`slippageTolerance`): `Readonly`<{ `amount0`: `JSBI`; `amount1`: `JSBI`; }>
Defined in: [entities/position.ts:223](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L223)
Returns the minimum amounts that should be requested in order to safely burn the amount of liquidity held by the position with the given slippage tolerance
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#parameters-1 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| tolerance of unfavorable slippage from the current price  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-6 "Direct link to Returns")
`Readonly`<{ `amount0`: `JSBI`; `amount1`: `JSBI`; }>
The amounts, with slippage
### fromAmount0()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#fromamount0 "Direct link to fromAmount0\(\)")
> `static` **fromAmount0**(`__namedParameters`): [`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
Defined in: [entities/position.ts:402](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L402)
Computes a position with the maximum amount of liquidity received for a given amount of token0, assuming an unlimited amount of token1
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#parameters-2 "Direct link to Parameters")
Parameter| Type  
---|---  
`__namedParameters`| { `amount0`: `BigintIsh`; `pool`: [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool); `tickLower`: `number`; `tickUpper`: `number`; `useFullPrecision`: `boolean`; }  
`__namedParameters.amount0`| `BigintIsh`  
`__namedParameters.pool`| [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)  
`__namedParameters.tickLower`| `number`  
`__namedParameters.tickUpper`| `number`  
`__namedParameters.useFullPrecision`| `boolean`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-7 "Direct link to Returns")
[`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
The position
### fromAmount1()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#fromamount1 "Direct link to fromAmount1\(\)")
> `static` **fromAmount1**(`__namedParameters`): [`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
Defined in: [entities/position.ts:426](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L426)
Computes a position with the maximum amount of liquidity received for a given amount of token1, assuming an unlimited amount of token0
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#parameters-3 "Direct link to Parameters")
Parameter| Type  
---|---  
`__namedParameters`| { `amount1`: `BigintIsh`; `pool`: [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool); `tickLower`: `number`; `tickUpper`: `number`; }  
`__namedParameters.amount1`| `BigintIsh`  
`__namedParameters.pool`| [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)  
`__namedParameters.tickLower`| `number`  
`__namedParameters.tickUpper`| `number`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-8 "Direct link to Returns")
[`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
The position
### fromAmounts()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#fromamounts "Direct link to fromAmounts\(\)")
> `static` **fromAmounts**(`__namedParameters`): [`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
Defined in: [entities/position.ts:360](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L360)
Computes the maximum amount of liquidity received for a given amount of token0, token1, and the prices at the tick boundaries.
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#parameters-4 "Direct link to Parameters")
Parameter| Type  
---|---  
`__namedParameters`| { `amount0`: `BigintIsh`; `amount1`: `BigintIsh`; `pool`: [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool); `tickLower`: `number`; `tickUpper`: `number`; `useFullPrecision`: `boolean`; }  
`__namedParameters.amount0`| `BigintIsh`  
`__namedParameters.amount1`| `BigintIsh`  
`__namedParameters.pool`| [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)  
`__namedParameters.tickLower`| `number`  
`__namedParameters.tickUpper`| `number`  
`__namedParameters.useFullPrecision`| `boolean`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-9 "Direct link to Returns")
[`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
The amount of liquidity for the position
### mintAmountsWithSlippage()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#mintamountswithslippage "Direct link to mintAmountsWithSlippage\(\)")
> **mintAmountsWithSlippage**(`slippageTolerance`): `Readonly`<{ `amount0`: `JSBI`; `amount1`: `JSBI`; }>
Defined in: [entities/position.ts:159](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L159)
Returns the maximum amount of token0 and token1 that must be sent in order to safely mint the amount of liquidity held by the position with the given slippage tolerance
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#parameters-5 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| Tolerance of unfavorable slippage from the current price  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-10 "Direct link to Returns")
`Readonly`<{ `amount0`: `JSBI`; `amount1`: `JSBI`; }>
The amounts, with slippage
#### Dev[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#dev-1 "Direct link to Dev")
In v4, minting and increasing is protected by maximum amounts of token0 and token1.
### permitBatchData()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#permitbatchdata "Direct link to permitBatchData\(\)")
> **permitBatchData**(`slippageTolerance`, `spender`, `nonce`, `deadline`): [`AllowanceTransferPermitBatch`](https://docs.uniswap.org/sdk/v4/reference/interfaces/AllowanceTransferPermitBatch)
Defined in: [entities/position.ts:321](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/position.ts#L321)
Returns the AllowanceTransferPermitBatch for adding liquidity to a position
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#parameters-6 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| The amount by which the price can 'slip' before the transaction will revert  
`spender`| `string`| The spender of the permit (should usually be the PositionManager)  
`nonce`| `BigintIsh`| A valid permit2 nonce  
`deadline`| `BigintIsh`| The deadline for the permit  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Position#returns-11 "Direct link to Returns")
[`AllowanceTransferPermitBatch`](https://docs.uniswap.org/sdk/v4/reference/interfaces/AllowanceTransferPermitBatch)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Position.md)
Was this helpful?
[PreviousPool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)[NextRoute](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
On this page
  * [Dev](https://docs.uniswap.org/sdk/v4/reference/classes/Position#dev)
  * [Constructors](https://docs.uniswap.org/sdk/v4/reference/classes/Position#constructors)
    * [new Position()](https://docs.uniswap.org/sdk/v4/reference/classes/Position#new-position)
  * [Properties](https://docs.uniswap.org/sdk/v4/reference/classes/Position#properties)
    * [liquidity](https://docs.uniswap.org/sdk/v4/reference/classes/Position#liquidity)
    * [pool](https://docs.uniswap.org/sdk/v4/reference/classes/Position#pool)
    * [tickLower](https://docs.uniswap.org/sdk/v4/reference/classes/Position#ticklower)
    * [tickUpper](https://docs.uniswap.org/sdk/v4/reference/classes/Position#tickupper)
  * [Accessors](https://docs.uniswap.org/sdk/v4/reference/classes/Position#accessors)
    * [amount0](https://docs.uniswap.org/sdk/v4/reference/classes/Position#amount0)
    * [amount1](https://docs.uniswap.org/sdk/v4/reference/classes/Position#amount1)
    * [mintAmounts](https://docs.uniswap.org/sdk/v4/reference/classes/Position#mintamounts)
    * [token0PriceLower](https://docs.uniswap.org/sdk/v4/reference/classes/Position#token0pricelower)
    * [token0PriceUpper](https://docs.uniswap.org/sdk/v4/reference/classes/Position#token0priceupper)
  * [Methods](https://docs.uniswap.org/sdk/v4/reference/classes/Position#methods)
    * [burnAmountsWithSlippage()](https://docs.uniswap.org/sdk/v4/reference/classes/Position#burnamountswithslippage)
    * [fromAmount0()](https://docs.uniswap.org/sdk/v4/reference/classes/Position#fromamount0)
    * [fromAmount1()](https://docs.uniswap.org/sdk/v4/reference/classes/Position#fromamount1)
    * [fromAmounts()](https://docs.uniswap.org/sdk/v4/reference/classes/Position#fromamounts)
    * [mintAmountsWithSlippage()](https://docs.uniswap.org/sdk/v4/reference/classes/Position#mintamountswithslippage)
    * [permitBatchData()](https://docs.uniswap.org/sdk/v4/reference/classes/Position#permitbatchdata)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Position.md)
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
