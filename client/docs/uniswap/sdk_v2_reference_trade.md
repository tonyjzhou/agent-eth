# https://docs.uniswap.org/sdk/v2/reference/trade

[Skip to main content](https://docs.uniswap.org/sdk/v2/reference/trade#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v2/reference/trade)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Swaps](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Position Management](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Advanced](https://docs.uniswap.org/sdk/v2/reference/trade)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/reference/trade)
    * [v3 SDK](https://docs.uniswap.org/sdk/v2/reference/trade)
    * [Swap Widget](https://docs.uniswap.org/sdk/v2/reference/trade)
    * [web3-react](https://docs.uniswap.org/sdk/v2/reference/trade)
    * [Core SDK](https://docs.uniswap.org/sdk/v2/reference/trade)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/reference/trade)
      * [Overview](https://docs.uniswap.org/sdk/v2/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [SDK Quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [Fetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)
        * [Pricing](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Trading](https://docs.uniswap.org/sdk/v2/guides/trading)
        * [Pair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Getting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Pair](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Route](https://docs.uniswap.org/sdk/v2/reference/route)
        * [Trade](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Other Exports](https://docs.uniswap.org/sdk/v2/reference/other-exports)
    * [v1 SDK](https://docs.uniswap.org/sdk/v2/reference/trade)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v2 SDK
  * Technical Reference
  * [Trade](https://docs.uniswap.org/sdk/v2/reference/trade)


On this page
```
constructor(route: Route, amount: CurrencyAmount, tradeType: TradeType)
```

The Trade entity represents a fully specified trade along a route. This entity supplies all the information necessary to craft a router transaction.
# Example
```
import{ ChainId, Token, CurrencyAmount, TradeType }from'@uniswap/sdk-core'import{ Pair, Trade, Route }constHOT=newToken(ChainId.MAINNET,'0xc0FFee0000000000000000000000000000000000',18,'HOT','Caffeine')constNOT=newToken(ChainId.MAINNET,'0xDeCAf00000000000000000000000000000000000',18,'NOT','Caffeine')constHOT_NOT=newPair(CurrencyAmount.fromRawAmount(HOT,'2000000000000000000'), CurrencyAmount.fromRawAmount(NOT,'1000000000000000000'))constNOT_TO_HOT=newRoute([HOT_NOT],NOT,HOT)const trade =newTrade(NOT_TO_HOT, CurrencyAmount.fromRawAmount(NOT,'1000000000000000'), TradeType.EXACT_INPUT)
```

# Properties
## route[​](https://docs.uniswap.org/sdk/v2/reference/trade#route "Direct link to route")
```
route: Route
```

The [path](https://docs.uniswap.org/sdk/v2/reference/route#path) property of the route should be passed as the path parameter to router functions.
## tradeType[​](https://docs.uniswap.org/sdk/v2/reference/trade#tradetype "Direct link to tradeType")
```
tradeType: TradeType
```

`TradeType.EXACT_INPUT` corresponds to `swapExact*For*` router functions. `TradeType.EXACT_OUTPUT` corresponds to `swap*ForExact*` router functions.
## inputAmount[​](https://docs.uniswap.org/sdk/v2/reference/trade#inputamount "Direct link to inputAmount")
```
inputAmount: CurrencyAmount
```

For exact input trades, this value should be passed as amountIn to router functions. For exact output trades, this value should be multiplied by a factor >1, representing slippage tolerance, and passed as amountInMax to router functions.
## outputAmount[​](https://docs.uniswap.org/sdk/v2/reference/trade#outputamount "Direct link to outputAmount")
```
outputAmount: CurrencyAmount
```

For exact output trades, this value should be passed as amountOut to router functions. For exact input trades, this value should be multiplied by a factor <1, representing slippage tolerance, and passed as amountOutMin to router functions.
## executionPrice[​](https://docs.uniswap.org/sdk/v2/reference/trade#executionprice "Direct link to executionPrice")
```
executionPrice: Price
```

The average price that the trade would execute at.
## priceImpact[​](https://docs.uniswap.org/sdk/v2/reference/trade#priceimpact "Direct link to priceImpact")
```
priceImpact: Percent
```

The percent difference between the mid price before the trade and the trade execution price.
# Methods
In the context of the following two methods, slippage refers to the percent difference between the actual price and the trade `executionPrice`.
## minimumAmountOut (since 2.0.4)[​](https://docs.uniswap.org/sdk/v2/reference/trade#minimumamountout-since-204 "Direct link to minimumAmountOut \(since 2.0.4\)")
```
minimumAmountOut(slippageTolerance: Percent): CurrencyAmount
```

Returns the minimum amount of the output token that should be received from a trade, given the slippage tolerance.
Useful when constructing a transaction for a trade of type `EXACT_INPUT`.
## maximumAmountIn (since 2.0.4)[​](https://docs.uniswap.org/sdk/v2/reference/trade#maximumamountin-since-204 "Direct link to maximumAmountIn \(since 2.0.4\)")
```
maximumAmountIn(slippageTolerance: Percent): CurrencyAmount
```

Returns the maximum amount of the input token that should be spent on the trade, given the slippage tolerance.
Useful when constructing a transaction for a trade of type `EXACT_OUTPUT`.
## worstExecutionPrice[​](https://docs.uniswap.org/sdk/v2/reference/trade#worstexecutionprice "Direct link to worstExecutionPrice")
Return the execution price after accounting for slippage tolerance
```
worstExecutionPrice(slippageTolerance: Percent): Price
```

# Static methods
These static methods provide ways to construct ideal trades from lists of pairs. Note these methods do not perform any aggregation across routes, as routes are linear. It's possible that a better price can be had by combining multiple trades across different routes.
## exactIn[​](https://docs.uniswap.org/sdk/v2/reference/trade#exactin "Direct link to exactIn")
Constructs an exact in trade with the given amount in and route.
```
Trade.exactIn(route: Route, amountIn: CurrencyAmount): Trade
```

## exactOut[​](https://docs.uniswap.org/sdk/v2/reference/trade#exactout "Direct link to exactOut")
Constructs an exact out trade with the given amount out and route
```
Trade.exactOut(route: Route, amountOut: CurrencyAmount): Trade
```

## bestTradeExactIn[​](https://docs.uniswap.org/sdk/v2/reference/trade#besttradeexactin "Direct link to bestTradeExactIn")
Given a list of pairs, and a fixed amount in, returns the top `maxNumResults` trades that go from an input token amount to an output token, making at most `maxHops` hops. Note this does not consider aggregation, as routes are linear. It's possible a better route exists by splitting the amount in among multiple routes.
```
Trade.bestTradeExactIn(  pairs: Pair[],  nextAmountIn: CurrencyAmount,  currencyOut: Token,{ maxNumResults =3, maxHops =3}: BestTradeOptions ={}): Trade[]
```

## bestTradeExactOut[​](https://docs.uniswap.org/sdk/v2/reference/trade#besttradeexactout "Direct link to bestTradeExactOut")
Similar to the above method but instead targets a fixed output amount given a list of pairs, and a fixed amount out, returns the top `maxNumResults` trades that go from an input token to an output token amount, making at most `maxHops` hops. Note this does not consider aggregation, as routes are linear. It is possible a better route exists by splitting the amountIn among multiple routes.
```
Trade.bestTradeExactOut(  pairs: Pair[],  currencyIn: Token,  nextAmountOut: CurrencyAmount,{ maxNumResults =3, maxHops =3}: BestTradeOptions ={}): Trade[]
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/reference/04-trade.md)
Was this helpful?
[PreviousRoute](https://docs.uniswap.org/sdk/v2/reference/route)[NextOther Exports](https://docs.uniswap.org/sdk/v2/reference/other-exports)
On this page
  * [route](https://docs.uniswap.org/sdk/v2/reference/trade#route)
  * [tradeType](https://docs.uniswap.org/sdk/v2/reference/trade#tradetype)
  * [inputAmount](https://docs.uniswap.org/sdk/v2/reference/trade#inputamount)
  * [outputAmount](https://docs.uniswap.org/sdk/v2/reference/trade#outputamount)
  * [executionPrice](https://docs.uniswap.org/sdk/v2/reference/trade#executionprice)
  * [priceImpact](https://docs.uniswap.org/sdk/v2/reference/trade#priceimpact)
  * [minimumAmountOut (since 2.0.4)](https://docs.uniswap.org/sdk/v2/reference/trade#minimumamountout-since-204)
  * [maximumAmountIn (since 2.0.4)](https://docs.uniswap.org/sdk/v2/reference/trade#maximumamountin-since-204)
  * [worstExecutionPrice](https://docs.uniswap.org/sdk/v2/reference/trade#worstexecutionprice)
  * [exactIn](https://docs.uniswap.org/sdk/v2/reference/trade#exactin)
  * [exactOut](https://docs.uniswap.org/sdk/v2/reference/trade#exactout)
  * [bestTradeExactIn](https://docs.uniswap.org/sdk/v2/reference/trade#besttradeexactin)
  * [bestTradeExactOut](https://docs.uniswap.org/sdk/v2/reference/trade#besttradeexactout)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/reference/04-trade.md)
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
