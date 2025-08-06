# https://docs.uniswap.org/sdk/v3/reference/classes/Trade

[Skip to main content](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#__docusaurus_skipToContent_fallback)
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
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/overview)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/advanced/introduction)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/overview)
        * [Overview](https://docs.uniswap.org/sdk/v3/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v3/reference/classes/FullMath)
          * [FullMath](https://docs.uniswap.org/sdk/v3/reference/classes/FullMath)
          * [LiquidityMath](https://docs.uniswap.org/sdk/v3/reference/classes/LiquidityMath)
          * [Multicall](https://docs.uniswap.org/sdk/v3/reference/classes/Multicall)
          * [NoTickDataProvider](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
          * [NonfungiblePositionManager](https://docs.uniswap.org/sdk/v3/reference/classes/NonfungiblePositionManager)
          * [Payments](https://docs.uniswap.org/sdk/v3/reference/classes/Payments)
          * [Pool](https://docs.uniswap.org/sdk/v3/reference/classes/Pool)
          * [Position](https://docs.uniswap.org/sdk/v3/reference/classes/Position)
          * [PositionLibrary](https://docs.uniswap.org/sdk/v3/reference/classes/PositionLibrary)
          * [Route](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
          * [SelfPermit](https://docs.uniswap.org/sdk/v3/reference/classes/SelfPermit)
          * [SqrtPriceMath](https://docs.uniswap.org/sdk/v3/reference/classes/SqrtPriceMath)
          * [Staker](https://docs.uniswap.org/sdk/v3/reference/classes/Staker)
          * [SwapMath](https://docs.uniswap.org/sdk/v3/reference/classes/SwapMath)
          * [SwapQuoter](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
          * [SwapRouter](https://docs.uniswap.org/sdk/v3/reference/classes/SwapRouter)
          * [Tick](https://docs.uniswap.org/sdk/v3/reference/classes/Tick)
          * [TickLibrary](https://docs.uniswap.org/sdk/v3/reference/classes/TickLibrary)
          * [TickList](https://docs.uniswap.org/sdk/v3/reference/classes/TickList)
          * [TickListDataProvider](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider)
          * [TickMath](https://docs.uniswap.org/sdk/v3/reference/classes/TickMath)
          * [Trade](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)
        * [enums](https://docs.uniswap.org/sdk/v3/reference/enums/FeeAmount)
        * [interfaces](https://docs.uniswap.org/sdk/v3/reference/interfaces/AllowedPermitArguments)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Technical Reference
  * classes
  * [Trade](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)


On this page
[@uniswap/v3-sdk](https://docs.uniswap.org/sdk/v3/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/v3/reference/modules.md) / Trade
# Class: Trade<TInput, TOutput, TTradeType>
Represents a trade executed against a set of routes where some percentage of the input is split across each route.
Each route has its own set of pools. Pools can not be re-used across routes.
Does not account for slippage, i.e., changes in price environment that can occur between the time the trade is submitted and when it is executed.
## Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token, either Ether or an ERC-20  
`TOutput`| extends `Currency`| The output token, either Ether or an ERC-20  
`TTradeType`| extends `TradeType`| The trade type, either exact input or exact output  
## Table of contents[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#constructor)


### Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#properties "Direct link to Properties")
  * [_executionPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_executionprice)
  * [_inputAmount](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_inputamount)
  * [_outputAmount](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_outputamount)
  * [_priceImpact](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_priceimpact)
  * [swaps](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#swaps)
  * [tradeType](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#tradetype)


### Accessors[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#accessors "Direct link to Accessors")
  * [executionPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#executionprice)
  * [inputAmount](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#inputamount)
  * [outputAmount](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#outputamount)
  * [priceImpact](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#priceimpact)
  * [route](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#route)


### Methods[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#methods "Direct link to Methods")
  * [maximumAmountIn](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#maximumamountin)
  * [minimumAmountOut](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#minimumamountout)
  * [worstExecutionPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#worstexecutionprice)
  * [bestTradeExactIn](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#besttradeexactin)
  * [bestTradeExactOut](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#besttradeexactout)
  * [createUncheckedTrade](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#createuncheckedtrade)
  * [createUncheckedTradeWithMultipleRoutes](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#createuncheckedtradewithmultipleroutes)
  * [exactIn](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#exactin)
  * [exactOut](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#exactout)
  * [fromRoute](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#fromroute)
  * [fromRoutes](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#fromroutes)


## Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#constructor "Direct link to constructor")
• `Private` **new Trade** <`TInput`, `TOutput`, `TTradeType`>(`__namedParameters`)
Construct a trade by passing in the pre-computed property values
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-1 "Direct link to Type parameters")
Name| Type  
---|---  
`TInput`| extends `Currency`  
`TOutput`| extends `Currency`  
`TTradeType`| extends `TradeType`  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters "Direct link to Parameters")
Name| Type  
---|---  
`__namedParameters`| `Object`  
`__namedParameters.routes`| { `inputAmount`: `CurrencyAmount`<`TInput`> ; `outputAmount`: `CurrencyAmount`<`TOutput`> ; `route`: [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`> }[]  
`__namedParameters.tradeType`| `TTradeType`  
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in "Direct link to Defined in")
[entities/trade.ts:397](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L397)
## Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#properties-1 "Direct link to Properties")
### _executionPrice[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_executionprice "Direct link to _executionPrice")
• `Private` **_executionPrice** : `undefined` | `Price`<`TInput`, `TOutput`>
The cached result of the computed execution price
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-1 "Direct link to Defined in")
[entities/trade.ts:143](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L143)
### _inputAmount[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_inputamount "Direct link to _inputAmount")
• `Private` **_inputAmount** : `undefined` | `CurrencyAmount`<`TInput`>
The cached result of the input amount computation
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-2 "Direct link to Defined in")
[entities/trade.ts:97](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L97)
### _outputAmount[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_outputamount "Direct link to _outputAmount")
• `Private` **_outputAmount** : `undefined` | `CurrencyAmount`<`TOutput`>
The cached result of the output amount computation
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-3 "Direct link to Defined in")
[entities/trade.ts:120](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L120)
### _priceImpact[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_priceimpact "Direct link to _priceImpact")
• `Private` **_priceImpact** : `undefined` | `Percent`
The cached result of the price impact computation
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-4 "Direct link to Defined in")
[entities/trade.ts:164](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L164)
### swaps[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#swaps "Direct link to swaps")
• `Readonly` **swaps** : { `inputAmount`: `CurrencyAmount`<`TInput`> ; `outputAmount`: `CurrencyAmount`<`TOutput`> ; `route`: [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`> }[]
The swaps of the trade, i.e. which routes and how much is swapped in each that make up the trade.
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-5 "Direct link to Defined in")
[entities/trade.ts:82](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L82)
### tradeType[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#tradetype "Direct link to tradeType")
• `Readonly` **tradeType** : `TTradeType`
The type of the trade, either exact in or exact out.
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-6 "Direct link to Defined in")
[entities/trade.ts:91](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L91)
## Accessors[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#accessors-1 "Direct link to Accessors")
### executionPrice[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#executionprice "Direct link to executionPrice")
• `get` **executionPrice**(): `Price`<`TInput`, `TOutput`>
The price expressed in terms of output amount/input amount.
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns "Direct link to Returns")
`Price`<`TInput`, `TOutput`>
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-7 "Direct link to Defined in")
[entities/trade.ts:148](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L148)
### inputAmount[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#inputamount "Direct link to inputAmount")
• `get` **inputAmount**(): `CurrencyAmount`<`TInput`>
The input amount for the trade assuming no slippage.
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-1 "Direct link to Returns")
`CurrencyAmount`<`TInput`>
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-8 "Direct link to Defined in")
[entities/trade.ts:102](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L102)
### outputAmount[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#outputamount "Direct link to outputAmount")
• `get` **outputAmount**(): `CurrencyAmount`<`TOutput`>
The output amount for the trade assuming no slippage.
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-2 "Direct link to Returns")
`CurrencyAmount`<`TOutput`>
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-9 "Direct link to Defined in")
[entities/trade.ts:125](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L125)
### priceImpact[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#priceimpact "Direct link to priceImpact")
• `get` **priceImpact**(): `Percent`
Returns the percent difference between the route's mid price and the price impact
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-3 "Direct link to Returns")
`Percent`
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-10 "Direct link to Defined in")
[entities/trade.ts:169](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L169)
### route[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#route "Direct link to route")
• `get` **route**(): [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`>
**`Deprecated`**
Deprecated in favor of 'swaps' property. If the trade consists of multiple routes this will return an error.
When the trade consists of just a single route, this returns the route of the trade, i.e. which pools the trade goes through.
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-4 "Direct link to Returns")
[`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`>
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-11 "Direct link to Defined in")
[entities/trade.ts:73](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L73)
## Methods[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#methods-1 "Direct link to Methods")
### maximumAmountIn[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#maximumamountin "Direct link to maximumAmountIn")
▸ **maximumAmountIn**(`slippageTolerance`, `amountIn?`): `CurrencyAmount`<`TInput`>
Get the maximum amount in that can be spent via this trade for the given slippage tolerance
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-1 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| The tolerance of unfavorable slippage from the execution price of this trade  
`amountIn`| `CurrencyAmount`<`TInput`>| -  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-5 "Direct link to Returns")
`CurrencyAmount`<`TInput`>
The amount in
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-12 "Direct link to Defined in")
[entities/trade.ts:456](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L456)
### minimumAmountOut[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#minimumamountout "Direct link to minimumAmountOut")
▸ **minimumAmountOut**(`slippageTolerance`, `amountOut?`): `CurrencyAmount`<`TOutput`>
Get the minimum amount that must be received from this trade for the given slippage tolerance
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-2 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| The tolerance of unfavorable slippage from the execution price of this trade  
`amountOut`| `CurrencyAmount`<`TOutput`>| -  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-6 "Direct link to Returns")
`CurrencyAmount`<`TOutput`>
The amount out
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-13 "Direct link to Defined in")
[entities/trade.ts:438](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L438)
### worstExecutionPrice[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#worstexecutionprice "Direct link to worstExecutionPrice")
▸ **worstExecutionPrice**(`slippageTolerance`): `Price`<`TInput`, `TOutput`>
Return the execution price after accounting for slippage tolerance
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-3 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| the allowed tolerated slippage  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-7 "Direct link to Returns")
`Price`<`TInput`, `TOutput`>
The execution price
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-14 "Direct link to Defined in")
[entities/trade.ts:471](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L471)
### bestTradeExactIn[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#besttradeexactin "Direct link to bestTradeExactIn")
▸ `Static` **bestTradeExactIn** <`TInput`, `TOutput`>(`pools`, `currencyAmountIn`, `currencyOut`, `__namedParameters?`, `currentPools?`, `nextAmountIn?`, `bestTrades?`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>[]>
Given a list of pools, and a fixed amount in, returns the top `maxNumResults` trades that go from an input token amount to an output token, making at most `maxHops` hops. Note this does not consider aggregation, as routes are linear. It's possible a better route exists by splitting the amount in among multiple routes.
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-2 "Direct link to Type parameters")
Name| Type  
---|---  
`TInput`| extends `Currency`  
`TOutput`| extends `Currency`  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-4 "Direct link to Parameters")
Name| Type| Default value| Description  
---|---|---|---  
`pools`| [`Pool`](https://docs.uniswap.org/sdk/v3/reference/classes/Pool)[]| `undefined`| the pools to consider in finding the best trade  
`currencyAmountIn`| `CurrencyAmount`<`TInput`>| `undefined`| used in recursion; the original value of the currencyAmountIn parameter  
`currencyOut`| `TOutput`| `undefined`| the desired currency out  
`__namedParameters`| [`BestTradeOptions`](https://docs.uniswap.org/sdk/v3/reference/interfaces/BestTradeOptions)| `{}`| -  
`currentPools`| [`Pool`](https://docs.uniswap.org/sdk/v3/reference/classes/Pool)[]| `[]`| used in recursion; the current list of pools  
`nextAmountIn`| `CurrencyAmount`<`Currency`>| `currencyAmountIn`| exact amount of input currency to spend  
`bestTrades`| [`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>[]| `[]`| used in recursion; the current list of best trades  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-8 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>[]>
The exact in trade
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-15 "Direct link to Defined in")
[entities/trade.ts:495](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L495)
### bestTradeExactOut[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#besttradeexactout "Direct link to bestTradeExactOut")
▸ `Static` **bestTradeExactOut** <`TInput`, `TOutput`>(`pools`, `currencyIn`, `currencyAmountOut`, `__namedParameters?`, `currentPools?`, `nextAmountOut?`, `bestTrades?`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>[]>
similar to the above method but instead targets a fixed output amount given a list of pools, and a fixed amount out, returns the top `maxNumResults` trades that go from an input token to an output token amount, making at most `maxHops` hops note this does not consider aggregation, as routes are linear. it's possible a better route exists by splitting the amount in among multiple routes.
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-3 "Direct link to Type parameters")
Name| Type  
---|---  
`TInput`| extends `Currency`  
`TOutput`| extends `Currency`  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-5 "Direct link to Parameters")
Name| Type| Default value| Description  
---|---|---|---  
`pools`| [`Pool`](https://docs.uniswap.org/sdk/v3/reference/classes/Pool)[]| `undefined`| the pools to consider in finding the best trade  
`currencyIn`| `TInput`| `undefined`| the currency to spend  
`currencyAmountOut`| `CurrencyAmount`<`TOutput`>| `undefined`| the desired currency amount out  
`__namedParameters`| [`BestTradeOptions`](https://docs.uniswap.org/sdk/v3/reference/interfaces/BestTradeOptions)| `{}`| -  
`currentPools`| [`Pool`](https://docs.uniswap.org/sdk/v3/reference/classes/Pool)[]| `[]`| used in recursion; the current list of pools  
`nextAmountOut`| `CurrencyAmount`<`Currency`>| `currencyAmountOut`| the exact amount of currency out  
`bestTrades`| [`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>[]| `[]`| used in recursion; the current list of best trades  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-9 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>[]>
The exact out trade
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-16 "Direct link to Defined in")
[entities/trade.ts:576](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L576)
### createUncheckedTrade[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#createuncheckedtrade "Direct link to createUncheckedTrade")
▸ `Static` **createUncheckedTrade** <`TInput`, `TOutput`, `TTradeType`>(`constructorArguments`): [`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>
Creates a trade without computing the result of swapping through the route. Useful when you have simulated the trade elsewhere and do not have any tick data
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-4 "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token, either Ether or an ERC-20  
`TOutput`| extends `Currency`| The output token, either Ether or an ERC-20  
`TTradeType`| extends `TradeType`| The type of the trade, either exact in or exact out  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-6 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`constructorArguments`| `Object`| The arguments passed to the trade constructor  
`constructorArguments.inputAmount`| `CurrencyAmount`<`TInput`>| -  
`constructorArguments.outputAmount`| `CurrencyAmount`<`TOutput`>| -  
`constructorArguments.route`| [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`>| -  
`constructorArguments.tradeType`| `TTradeType`| -  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-10 "Direct link to Returns")
[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>
The unchecked trade
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-17 "Direct link to Defined in")
[entities/trade.ts:346](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L346)
### createUncheckedTradeWithMultipleRoutes[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#createuncheckedtradewithmultipleroutes "Direct link to createUncheckedTradeWithMultipleRoutes")
▸ `Static` **createUncheckedTradeWithMultipleRoutes** <`TInput`, `TOutput`, `TTradeType`>(`constructorArguments`): [`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>
Creates a trade without computing the result of swapping through the routes. Useful when you have simulated the trade elsewhere and do not have any tick data
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-5 "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token, either Ether or an ERC-20  
`TOutput`| extends `Currency`| The output token, either Ether or an ERC-20  
`TTradeType`| extends `TradeType`| The type of the trade, either exact in or exact out  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-7 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`constructorArguments`| `Object`| The arguments passed to the trade constructor  
`constructorArguments.routes`| { `inputAmount`: `CurrencyAmount`<`TInput`> ; `outputAmount`: `CurrencyAmount`<`TOutput`> ; `route`: [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`> }[]| -  
`constructorArguments.tradeType`| `TTradeType`| -  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-11 "Direct link to Returns")
[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>
The unchecked trade
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-18 "Direct link to Defined in")
[entities/trade.ts:377](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L377)
### exactIn[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#exactin "Direct link to exactIn")
▸ `Static` **exactIn** <`TInput`, `TOutput`>(`route`, `amountIn`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>>
Constructs an exact in trade with the given amount in and route
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-6 "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token, either Ether or an ERC-20  
`TOutput`| extends `Currency`| The output token, either Ether or an ERC-20  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-8 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`route`| [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`>| The route of the exact in trade  
`amountIn`| `CurrencyAmount`<`TInput`>| The amount being passed in  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-12 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>>
The exact in trade
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-19 "Direct link to Defined in")
[entities/trade.ts:194](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L194)
### exactOut[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#exactout "Direct link to exactOut")
▸ `Static` **exactOut** <`TInput`, `TOutput`>(`route`, `amountOut`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>>
Constructs an exact out trade with the given amount out and route
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-7 "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token, either Ether or an ERC-20  
`TOutput`| extends `Currency`| The output token, either Ether or an ERC-20  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-9 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`route`| [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`>| The route of the exact out trade  
`amountOut`| `CurrencyAmount`<`TOutput`>| The amount returned by the trade  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-13 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>>
The exact out trade
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-20 "Direct link to Defined in")
[entities/trade.ts:209](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L209)
### fromRoute[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#fromroute "Direct link to fromRoute")
▸ `Static` **fromRoute** <`TInput`, `TOutput`, `TTradeType`>(`route`, `amount`, `tradeType`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>>
Constructs a trade by simulating swaps through the given route
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-8 "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token, either Ether or an ERC-20.  
`TOutput`| extends `Currency`| The output token, either Ether or an ERC-20.  
`TTradeType`| extends `TradeType`| The type of the trade, either exact in or exact out.  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-10 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`route`| [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`>| route to swap through  
`amount`| `TTradeType` extends `EXACT_INPUT` ? `CurrencyAmount`<`TInput`> : `CurrencyAmount`<`TOutput`>| the amount specified, either input or output, depending on tradeType  
`tradeType`| `TTradeType`| whether the trade is an exact input or exact output swap  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-14 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>>
The route
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-21 "Direct link to Defined in")
[entities/trade.ts:226](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L226)
### fromRoutes[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#fromroutes "Direct link to fromRoutes")
▸ `Static` **fromRoutes** <`TInput`, `TOutput`, `TTradeType`>(`routes`, `tradeType`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>>
Constructs a trade from routes by simulating swaps
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters-9 "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token, either Ether or an ERC-20.  
`TOutput`| extends `Currency`| The output token, either Ether or an ERC-20.  
`TTradeType`| extends `TradeType`| The type of the trade, either exact in or exact out.  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#parameters-11 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`routes`| { `amount`: `TTradeType` extends `EXACT_INPUT` ? `CurrencyAmount`<`TInput`> : `CurrencyAmount`<`TOutput`> ; `route`: [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`> }[]| the routes to swap through and how much of the amount should be routed through each  
`tradeType`| `TTradeType`| whether the trade is an exact input or exact output swap  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#returns-15 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>>
The trade
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#defined-in-22 "Direct link to Defined in")
[entities/trade.ts:276](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/trade.ts#L276)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/Trade.md)
Was this helpful?
[PreviousTickMath](https://docs.uniswap.org/sdk/v3/reference/classes/TickMath)[NextFeeAmount](https://docs.uniswap.org/sdk/v3/reference/enums/FeeAmount)
On this page
  * [Type parameters](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#type-parameters)
  * [Table of contents](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#constructors)
    * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#properties)
    * [Accessors](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#accessors)
    * [Methods](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#methods)
  * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#constructor)
  * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#properties-1)
    * [_executionPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_executionprice)
    * [_inputAmount](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_inputamount)
    * [_outputAmount](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_outputamount)
    * [_priceImpact](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#_priceimpact)
    * [swaps](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#swaps)
    * [tradeType](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#tradetype)
  * [Accessors](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#accessors-1)
    * [executionPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#executionprice)
    * [inputAmount](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#inputamount)
    * [outputAmount](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#outputamount)
    * [priceImpact](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#priceimpact)
    * [route](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#route)
  * [Methods](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#methods-1)
    * [maximumAmountIn](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#maximumamountin)
    * [minimumAmountOut](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#minimumamountout)
    * [worstExecutionPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#worstexecutionprice)
    * [bestTradeExactIn](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#besttradeexactin)
    * [bestTradeExactOut](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#besttradeexactout)
    * [createUncheckedTrade](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#createuncheckedtrade)
    * [createUncheckedTradeWithMultipleRoutes](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#createuncheckedtradewithmultipleroutes)
    * [exactIn](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#exactin)
    * [exactOut](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#exactout)
    * [fromRoute](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#fromroute)
    * [fromRoutes](https://docs.uniswap.org/sdk/v3/reference/classes/Trade#fromroutes)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/Trade.md)
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
