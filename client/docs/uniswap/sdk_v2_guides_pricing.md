# https://docs.uniswap.org/sdk/v2/guides/pricing

[Skip to main content](https://docs.uniswap.org/sdk/v2/guides/pricing#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v2/guides/pricing)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Swaps](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Position Management](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Advanced](https://docs.uniswap.org/sdk/v2/guides/pricing)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/guides/pricing)
    * [v3 SDK](https://docs.uniswap.org/sdk/v2/guides/pricing)
    * [Swap Widget](https://docs.uniswap.org/sdk/v2/guides/pricing)
    * [web3-react](https://docs.uniswap.org/sdk/v2/guides/pricing)
    * [Core SDK](https://docs.uniswap.org/sdk/v2/guides/pricing)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/guides/pricing)
      * [Overview](https://docs.uniswap.org/sdk/v2/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [SDK Quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [Fetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)
        * [Pricing](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Trading](https://docs.uniswap.org/sdk/v2/guides/trading)
        * [Pair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Getting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Pair](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Route](https://docs.uniswap.org/sdk/v2/reference/route)
        * [Trade](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Other Exports](https://docs.uniswap.org/sdk/v2/reference/other-exports)
    * [v1 SDK](https://docs.uniswap.org/sdk/v2/guides/pricing)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v2 SDK
  * Guides
  * [Pricing](https://docs.uniswap.org/sdk/v2/guides/pricing)


On this page
> Looking for a [quickstart](https://docs.uniswap.org/sdk/v2/guides/quick-start)?
Let's talk pricing. This guide will focus on the two most important Uniswap prices: the **mid price** and the **execution price**.
# Mid Price
The mid price, in the context of Uniswap, is the price that reflects the _ratio of reserves in one or more pairs_. There are three ways we can think about this price. Perhaps most simply, it defines the relative value of one token in terms of the other. It also represents the price at which you could theoretically trade an infinitesimal amount (ε) of one token for the other. Finally, it can be interpreted as the current _market-clearing or fair value price_ of the assets.
Let's consider the mid price for DAI-WETH (that is, the amount of DAI per 1 WETH).
## Direct[​](https://docs.uniswap.org/sdk/v2/guides/pricing#direct "Direct link to Direct")
The simplest way to get the DAI-WETH mid price is to observe the pair directly:
```
import{ ChainId, Token,WETH9}from'@uniswap/sdk-core'import{ Route }from'@uniswap/v2-sdk'constDAI=newToken(ChainId.MAINNET,'0x6B175474E89094C44Da98b954EedeAC495271d0F',18)// To learn how to get Pair data, refer to the previous guide.const pair =awaitcreatePair(DAI,WETH9[ChainId.MAINNET])const route =newRoute([pair],WETH9[DAI.chainId],DAI)console.log(route.midPrice.toSignificant(6))// 1901.08console.log(route.midPrice.invert().toSignificant(6))// 0.000526017
```

You may be wondering why we have to construct a _route_ to get the mid price, as opposed to simply getting it from the pair (which, after all, includes all the necessary data). The reason is simple: a route forces us to be opinionated about the _direction_ of trading. Routes consist of one or more pairs, an input token and an output token (which fully defines a trading path). In this case, we passed WETH as the input token and DAI as the output token, meaning we're interested in a WETH -> DAI trade.
Now we understand that the mid price is going to be defined in terms of DAI/WETH. Not to worry though, if we need the WETH/DAI price, we can easily invert.
Finally, you may have noticed that we're formatting the price to 6 significant digits. This is because internally, prices are stored as exact-precision fractions, which can be converted to other representations on demand. For a full list of options, see [Price](https://docs.uniswap.org/sdk/core/reference/classes/Price).
## Indirect[​](https://docs.uniswap.org/sdk/v2/guides/pricing#indirect "Direct link to Indirect")
For the sake of example, let's imagine a direct pair between DAI and WETH _doesn't exist_. In order to get a DAI-WETH mid price we'll need to pick a valid route. Imagine both DAI and WETH have pairs with a third token, USDC. In that case, we can calculate an indirect mid price through the USDC pairs:
```
import{ ChainId, Token,WETH9}from'@uniswap/sdk-core'import{ Route, Pair }from'@uniswap/v2-sdk'constUSDC=newToken(ChainId.MAINNET,'0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48',6)constDAI=newToken(ChainId.MAINNET,'0x6B175474E89094C44Da98b954EedeAC495271d0F',18)// To learn how to get Pair data, refer to the previous guide.const USDCWETHPair =awaitcreatePair(USDC,WETH9[ChainId.MAINNET])const DAIUSDCPair =awaitcreatePair(DAI,USDC)const route =newRoute([USDCWETHPair, DAIUSDCPair],WETH9[ChainId.MAINNET],DAI)console.log(route.midPrice.toSignificant(6))// 1896.34console.log(route.midPrice.invert().toSignificant(6))// 0.000527331
```

# Execution Price
Mid prices are great representations of the _current_ state of a route, but what about trades? It turns out that it makes sense to define another price, the _execution_ price of a trade, as the ratio of assets sent/received.
Imagine we're interested in trading 1 WETH for DAI:
```
import{ ChainId, Token,WETH9, CurrencyAmount, TradeType }from'@uniswap/sdk-core'import{ Route, Pair, Trade }from'@uniswap/v2-sdk'constDAI=newToken(ChainId.MAINNET,'0x6B175474E89094C44Da98b954EedeAC495271d0F',18)// To learn how to get Pair data, refer to the previous guide.const pair =awaitcreatePair(DAI,WETH9[DAI.chainId])const route =newRoute([pair],WETH9[DAI.chainId],DAI)const trade =newTrade(route, CurrencyAmount.fromRawAmount(WETH9[DAI.chainId],'1000000000000000000'), TradeType.EXACT_INPUT)console.log(trade.executionPrice.toSignificant(6))// 1894.91
```

Notice that we're constructing a trade of 1 WETH for as much DAI as possible, _given the current reserves of the direct pair_. The execution price represents the average DAI/WETH price for this trade. Of course, the reserves of any pair can change every block, which would affect the execution price.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/03-pricing.md)
Was this helpful?
[PreviousFetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)[NextTrading](https://docs.uniswap.org/sdk/v2/guides/trading)
On this page
  * [Direct](https://docs.uniswap.org/sdk/v2/guides/pricing#direct)
  * [Indirect](https://docs.uniswap.org/sdk/v2/guides/pricing#indirect)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/03-pricing.md)
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
