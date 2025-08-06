# https://docs.uniswap.org/sdk/v4/reference/classes/Trade

[Skip to main content](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
        * [Swaps](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
        * [Position Management](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
        * [Advanced](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
        * [Overview](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
          * [Hook](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
          * [Pool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
          * [Position](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
          * [Route](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
          * [Trade](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
          * [V4BaseActionsParser](https://docs.uniswap.org/sdk/v4/reference/classes/V4BaseActionsParser)
          * [V4Planner](https://docs.uniswap.org/sdk/v4/reference/classes/V4Planner)
          * [V4PositionManager](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
          * [V4PositionPlanner](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionPlanner)
        * [enumerations](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
        * [interfaces](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
    * [web3-react](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Technical Reference
  * classes
  * [Trade](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)


On this page
# Trade
[@uniswap/v4-sdk](https://docs.uniswap.org/sdk/v4/reference/overview) / Trade
Defined in: [entities/trade.ts:66](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L66)
Represents a trade executed against a set of routes where some percentage of the input is split across each route.
Each route has its own set of pools. Pools can not be re-used across routes.
Does not account for slippage, i.e., changes in price environment that can occur between the time the trade is submitted and when it is executed.
## Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters "Direct link to Type Parameters")
Type Parameter| Description  
---|---  
`TInput` _extends_ `Currency`| The input currency, either Ether or an ERC-20  
`TOutput` _extends_ `Currency`| The output currency, either Ether or an ERC-20  
`TTradeType` _extends_ `TradeType`| The trade type, either exact input or exact output  
## Properties[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#properties "Direct link to Properties")
### swaps[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#swaps "Direct link to swaps")
> `readonly` **swaps** : `object`[]
Defined in: [entities/trade.ts:83](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L83)
The swaps of the trade, i.e. which routes and how much is swapped in each that make up the trade.
#### inputAmount[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#inputamount "Direct link to inputAmount")
> **inputAmount** : `CurrencyAmount`<`TInput`>
#### outputAmount[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#outputamount "Direct link to outputAmount")
> **outputAmount** : `CurrencyAmount`<`TOutput`>
#### route[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#route "Direct link to route")
> **route** : [`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>
### tradeType[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#tradetype "Direct link to tradeType")
> `readonly` **tradeType** : `TTradeType`
Defined in: [entities/trade.ts:92](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L92)
The type of the trade, either exact in or exact out.
## Accessors[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#accessors "Direct link to Accessors")
### executionPrice[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#executionprice "Direct link to executionPrice")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#get-signature "Direct link to Get Signature")
> **get** **executionPrice**(): `Price`<`TInput`, `TOutput`>
Defined in: [entities/trade.ts:149](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L149)
The price expressed in terms of output amount/input amount.
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns "Direct link to Returns")
`Price`<`TInput`, `TOutput`>
### inputAmount[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#inputamount-1 "Direct link to inputAmount")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#get-signature-1 "Direct link to Get Signature")
> **get** **inputAmount**(): `CurrencyAmount`<`TInput`>
Defined in: [entities/trade.ts:103](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L103)
The input amount for the trade assuming no slippage.
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-1 "Direct link to Returns")
`CurrencyAmount`<`TInput`>
### outputAmount[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#outputamount-1 "Direct link to outputAmount")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#get-signature-2 "Direct link to Get Signature")
> **get** **outputAmount**(): `CurrencyAmount`<`TOutput`>
Defined in: [entities/trade.ts:126](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L126)
The output amount for the trade assuming no slippage.
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-2 "Direct link to Returns")
`CurrencyAmount`<`TOutput`>
### priceImpact[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#priceimpact "Direct link to priceImpact")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#get-signature-3 "Direct link to Get Signature")
> **get** **priceImpact**(): `Percent`
Defined in: [entities/trade.ts:170](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L170)
Returns the percent difference between the route's mid price and the price impact
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-3 "Direct link to Returns")
`Percent`
### route[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#route-1 "Direct link to route")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#get-signature-4 "Direct link to Get Signature")
> **get** **route**(): [`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>
Defined in: [entities/trade.ts:74](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L74)
##### Deprecated[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#deprecated "Direct link to Deprecated")
Deprecated in favor of 'swaps' property. If the trade consists of multiple routes this will return an error.
When the trade consists of just a single route, this returns the route of the trade, i.e. which pools the trade goes through.
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-4 "Direct link to Returns")
[`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>
## Methods[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#methods "Direct link to Methods")
### bestTradeExactIn()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#besttradeexactin "Direct link to bestTradeExactIn\(\)")
> `static` **bestTradeExactIn** <`TInput`, `TOutput`>(`pools`, `currencyAmountIn`, `currencyOut`, `__namedParameters`, `currentPools`, `nextAmountIn`, `bestTrades`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>[]>
Defined in: [entities/trade.ts:454](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L454)
Given a list of pools, and a fixed amount in, returns the top `maxNumResults` trades that go from an input currency amount to an output currency, making at most `maxHops` hops. Note this does not consider aggregation, as routes are linear. It's possible a better route exists by splitting the amount in among multiple routes.
#### Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters-1 "Direct link to Type Parameters")
Type Parameter  
---  
`TInput` _extends_ `Currency`  
`TOutput` _extends_ `Currency`  
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters "Direct link to Parameters")
Parameter| Type| Default value| Description  
---|---|---|---  
`pools`| [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)[]| `undefined`| the pools to consider in finding the best trade  
`currencyAmountIn`| `CurrencyAmount`<`TInput`>| `undefined`| used in recursion; the original value of the currencyAmountIn parameter  
`currencyOut`| `TOutput`| `undefined`| the desired currency out  
`__namedParameters`| [`BestTradeOptions`](https://docs.uniswap.org/sdk/v4/reference/interfaces/BestTradeOptions)| `{}`| -  
`currentPools`| [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)[]| `[]`| used in recursion; the current list of pools  
`nextAmountIn`| `CurrencyAmount`<`Currency`>| `currencyAmountIn`| exact amount of input currency to spend  
`bestTrades`| [`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>[]| `[]`| used in recursion; the current list of best trades  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-5 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>[]>
The exact in trade
### bestTradeExactOut()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#besttradeexactout "Direct link to bestTradeExactOut\(\)")
> `static` **bestTradeExactOut** <`TInput`, `TOutput`>(`pools`, `currencyIn`, `currencyAmountOut`, `__namedParameters`, `currentPools`, `nextAmountOut`, `bestTrades`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>[]>
Defined in: [entities/trade.ts:533](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L533)
similar to the above method but instead targets a fixed output amount given a list of pools, and a fixed amount out, returns the top `maxNumResults` trades that go from an input currency to an output currency amount, making at most `maxHops` hops note this does not consider aggregation, as routes are linear. it's possible a better route exists by splitting the amount in among multiple routes.
#### Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters-2 "Direct link to Type Parameters")
Type Parameter  
---  
`TInput` _extends_ `Currency`  
`TOutput` _extends_ `Currency`  
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-1 "Direct link to Parameters")
Parameter| Type| Default value| Description  
---|---|---|---  
`pools`| [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)[]| `undefined`| the pools to consider in finding the best trade  
`currencyIn`| `TInput`| `undefined`| the currency to spend  
`currencyAmountOut`| `CurrencyAmount`<`TOutput`>| `undefined`| the desired currency amount out  
`__namedParameters`| [`BestTradeOptions`](https://docs.uniswap.org/sdk/v4/reference/interfaces/BestTradeOptions)| `{}`| -  
`currentPools`| [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)[]| `[]`| used in recursion; the current list of pools  
`nextAmountOut`| `CurrencyAmount`<`Currency`>| `currencyAmountOut`| the exact amount of currency out  
`bestTrades`| [`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>[]| `[]`| used in recursion; the current list of best trades  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-6 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>[]>
The exact out trade
### createUncheckedTrade()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#createuncheckedtrade "Direct link to createUncheckedTrade\(\)")
> `static` **createUncheckedTrade** <`TInput`, `TOutput`, `TTradeType`>(`constructorArguments`): [`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>
Defined in: [entities/trade.ts:305](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L305)
Creates a trade without computing the result of swapping through the route. Useful when you have simulated the trade elsewhere and do not have any tick data
#### Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters-3 "Direct link to Type Parameters")
Type Parameter| Description  
---|---  
`TInput` _extends_ `Currency`| The input currency, either Ether or an ERC-20  
`TOutput` _extends_ `Currency`| The output currency, either Ether or an ERC-20  
`TTradeType` _extends_ `TradeType`| The type of the trade, either exact in or exact out  
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-2 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`constructorArguments`| { `inputAmount`: `CurrencyAmount`<`TInput`>; `outputAmount`: `CurrencyAmount`<`TOutput`>; `route`: [`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>; `tradeType`: `TTradeType`; }| The arguments passed to the trade constructor  
`constructorArguments.inputAmount`| `CurrencyAmount`<`TInput`>| -  
`constructorArguments.outputAmount`| `CurrencyAmount`<`TOutput`>| -  
`constructorArguments.route`| [`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>| -  
`constructorArguments.tradeType`| `TTradeType`| -  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-7 "Direct link to Returns")
[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>
The unchecked trade
### createUncheckedTradeWithMultipleRoutes()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#createuncheckedtradewithmultipleroutes "Direct link to createUncheckedTradeWithMultipleRoutes\(\)")
> `static` **createUncheckedTradeWithMultipleRoutes** <`TInput`, `TOutput`, `TTradeType`>(`constructorArguments`): [`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>
Defined in: [entities/trade.ts:336](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L336)
Creates a trade without computing the result of swapping through the routes. Useful when you have simulated the trade elsewhere and do not have any tick data
#### Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters-4 "Direct link to Type Parameters")
Type Parameter| Description  
---|---  
`TInput` _extends_ `Currency`| The input currency, either Ether or an ERC-20  
`TOutput` _extends_ `Currency`| The output currency, either Ether or an ERC-20  
`TTradeType` _extends_ `TradeType`| The type of the trade, either exact in or exact out  
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-3 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`constructorArguments`| { `routes`: `object`[]; `tradeType`: `TTradeType`; }| The arguments passed to the trade constructor  
`constructorArguments.routes`| `object`[]| -  
`constructorArguments.tradeType`| `TTradeType`| -  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-8 "Direct link to Returns")
[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>
The unchecked trade
### exactIn()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#exactin "Direct link to exactIn\(\)")
> `static` **exactIn** <`TInput`, `TOutput`>(`route`, `amountIn`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>>
Defined in: [entities/trade.ts:195](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L195)
Constructs an exact in trade with the given amount in and route
#### Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters-5 "Direct link to Type Parameters")
Type Parameter| Description  
---|---  
`TInput` _extends_ `Currency`| The input currency, either Ether or an ERC-20  
`TOutput` _extends_ `Currency`| The output currency, either Ether or an ERC-20  
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-4 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`route`| [`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>| The route of the exact in trade  
`amountIn`| `CurrencyAmount`<`TInput`>| The amount being passed in  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-9 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_INPUT`>>
The exact in trade
### exactOut()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#exactout "Direct link to exactOut\(\)")
> `static` **exactOut** <`TInput`, `TOutput`>(`route`, `amountOut`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>>
Defined in: [entities/trade.ts:210](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L210)
Constructs an exact out trade with the given amount out and route
#### Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters-6 "Direct link to Type Parameters")
Type Parameter| Description  
---|---  
`TInput` _extends_ `Currency`| The input currency, either Ether or an ERC-20  
`TOutput` _extends_ `Currency`| The output currency, either Ether or an ERC-20  
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-5 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`route`| [`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>| The route of the exact out trade  
`amountOut`| `CurrencyAmount`<`TOutput`>| The amount returned by the trade  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-10 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `EXACT_OUTPUT`>>
The exact out trade
### fromRoute()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#fromroute "Direct link to fromRoute\(\)")
> `static` **fromRoute** <`TInput`, `TOutput`, `TTradeType`>(`route`, `amount`, `tradeType`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>>
Defined in: [entities/trade.ts:227](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L227)
Constructs a trade by simulating swaps through the given route
#### Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters-7 "Direct link to Type Parameters")
Type Parameter| Description  
---|---  
`TInput` _extends_ `Currency`| The input currency, either Ether or an ERC-20.  
`TOutput` _extends_ `Currency`| The output currency, either Ether or an ERC-20.  
`TTradeType` _extends_ `TradeType`| The type of the trade, either exact in or exact out.  
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-6 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`route`| [`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>| route to swap through  
`amount`| `TTradeType` _extends_ `EXACT_INPUT` ? `CurrencyAmount`<`TInput`> : `CurrencyAmount`<`TOutput`>| the amount specified, either input or output, depending on tradeType  
`tradeType`| `TTradeType`| whether the trade is an exact input or exact output swap  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-11 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>>
The route
### fromRoutes()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#fromroutes "Direct link to fromRoutes\(\)")
> `static` **fromRoutes** <`TInput`, `TOutput`, `TTradeType`>(`routes`, `tradeType`): `Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>>
Defined in: [entities/trade.ts:272](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L272)
Constructs a trade from routes by simulating swaps
#### Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters-8 "Direct link to Type Parameters")
Type Parameter| Description  
---|---  
`TInput` _extends_ `Currency`| The input currency, either Ether or an ERC-20.  
`TOutput` _extends_ `Currency`| The output currency, either Ether or an ERC-20.  
`TTradeType` _extends_ `TradeType`| The type of the trade, either exact in or exact out.  
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-7 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`routes`| `object`[]| the routes to swap through and how much of the amount should be routed through each  
`tradeType`| `TTradeType`| whether the trade is an exact input or exact output swap  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-12 "Direct link to Returns")
`Promise`<[`Trade`](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)<`TInput`, `TOutput`, `TTradeType`>>
The trade
### maximumAmountIn()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#maximumamountin "Direct link to maximumAmountIn\(\)")
> **maximumAmountIn**(`slippageTolerance`, `amountIn`): `CurrencyAmount`<`TInput`>
Defined in: [entities/trade.ts:415](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L415)
Get the maximum amount in that can be spent via this trade for the given slippage tolerance
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-8 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| The tolerance of unfavorable slippage from the execution price of this trade  
`amountIn`| `CurrencyAmount`<`TInput`>| -  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-13 "Direct link to Returns")
`CurrencyAmount`<`TInput`>
The amount in
### minimumAmountOut()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#minimumamountout "Direct link to minimumAmountOut\(\)")
> **minimumAmountOut**(`slippageTolerance`, `amountOut`): `CurrencyAmount`<`TOutput`>
Defined in: [entities/trade.ts:397](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L397)
Get the minimum amount that must be received from this trade for the given slippage tolerance
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-9 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| The tolerance of unfavorable slippage from the execution price of this trade  
`amountOut`| `CurrencyAmount`<`TOutput`>| -  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-14 "Direct link to Returns")
`CurrencyAmount`<`TOutput`>
The amount out
### worstExecutionPrice()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#worstexecutionprice "Direct link to worstExecutionPrice\(\)")
> **worstExecutionPrice**(`slippageTolerance`): `Price`<`TInput`, `TOutput`>
Defined in: [entities/trade.ts:430](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/trade.ts#L430)
Return the execution price after accounting for slippage tolerance
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#parameters-10 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`slippageTolerance`| `Percent`| the allowed tolerated slippage  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#returns-15 "Direct link to Returns")
`Price`<`TInput`, `TOutput`>
The execution price
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Trade.md)
Was this helpful?
[PreviousRoute](https://docs.uniswap.org/sdk/v4/reference/classes/Route)[NextV4BaseActionsParser](https://docs.uniswap.org/sdk/v4/reference/classes/V4BaseActionsParser)
On this page
  * [Type Parameters](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#type-parameters)
  * [Properties](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#properties)
    * [swaps](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#swaps)
    * [tradeType](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#tradetype)
  * [Accessors](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#accessors)
    * [executionPrice](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#executionprice)
    * [inputAmount](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#inputamount-1)
    * [outputAmount](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#outputamount-1)
    * [priceImpact](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#priceimpact)
    * [route](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#route-1)
  * [Methods](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#methods)
    * [bestTradeExactIn()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#besttradeexactin)
    * [bestTradeExactOut()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#besttradeexactout)
    * [createUncheckedTrade()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#createuncheckedtrade)
    * [createUncheckedTradeWithMultipleRoutes()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#createuncheckedtradewithmultipleroutes)
    * [exactIn()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#exactin)
    * [exactOut()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#exactout)
    * [fromRoute()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#fromroute)
    * [fromRoutes()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#fromroutes)
    * [maximumAmountIn()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#maximumamountin)
    * [minimumAmountOut()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#minimumamountout)
    * [worstExecutionPrice()](https://docs.uniswap.org/sdk/v4/reference/classes/Trade#worstexecutionprice)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Trade.md)
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
