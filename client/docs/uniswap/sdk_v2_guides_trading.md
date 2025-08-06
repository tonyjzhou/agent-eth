# https://docs.uniswap.org/sdk/v2/guides/trading

[Skip to main content](https://docs.uniswap.org/sdk/v2/guides/trading#__docusaurus_skipToContent_fallback)
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
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
      * [Overview](https://docs.uniswap.org/sdk/v2/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [SDK Quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [Fetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)
        * [Pricing](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Trading](https://docs.uniswap.org/sdk/v2/guides/trading)
        * [Pair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Getting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Pair](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Route](https://docs.uniswap.org/sdk/v2/reference/route)
        * [Trade](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Other Exports](https://docs.uniswap.org/sdk/v2/reference/other-exports)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v2 SDK
  * Guides
  * [Trading](https://docs.uniswap.org/sdk/v2/guides/trading)


> Looking for a [quickstart](https://docs.uniswap.org/sdk/v2/guides/quick-start)?
The SDK _cannot execute trades or send transactions on your behalf_. Rather, it offers utility classes and functions which make it easy to calculate the data required to safely interact with Uniswap. Nearly everything you need to safely transact with Uniswap is provided by the [Trade](https://docs.uniswap.org/sdk/v2/reference/trade) entity. However, it is your responsibility to use this data to send transactions in whatever context makes sense for your application.
This guide will focus exclusively on sending a transaction to the [latest Uniswap V2 router](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
# Sending a Transaction to the Router
Let's say we want to trade 1 WETH for as much DAI as possible:
```
import{ ChainId, Token,WETH9, CurrencyAmount, TradeType }from'@uniswap/sdk-core'import{Trade, Route}from'@uniswap/v2-sdk'constDAI=newToken(ChainId.MAINNET,'0x6B175474E89094C44Da98b954EedeAC495271d0F',18)// See the Fetching Data guide to learn how to get Pair dataconst pair =awaitcreatePair(DAI,WETH9[DAI.chainId])const route =newRoute([pair],WETH9[DAI.chainId],DAI)const amountIn ='1000000000000000000'// 1 WETHconst trade =newTrade(route, CurrencyAmount.fromRawAmount(WETH9[DAI.chainId], amountIn), TradeType.EXACT_INPUT)
```

So, we've constructed a trade entity, but how do we use it to actually send a transaction? There are still a few pieces we need to put in place.
Before going on, we should explore how ETH works in the context of trading. Internally, the SDK uses WETH, as all Uniswap V2 pairs use WETH under the hood. However, it's perfectly possible for you as an end user to use ETH, and rely on the router to handle converting to/from WETH. So, let's use ETH.
The first step is selecting the appropriate router function. The names of router functions are intended to be self-explanatory; in this case we want [swapExactETHForTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexactethfortokens), because we're swapping an exact amount of ETH for tokens.
That Solidity interface for this function is:
```
functionswapExactETHForTokens(uint amountOutMin,address[]calldata path,address to,uint deadline)externalpayablereturns(uint[]memory amounts);
```

Jumping back to our trading code, we can construct all the necessary parameters:
```
import{Percent}from'@uniswap/sdk-core'const slippageTolerance =newPercent('50','10000')// 50 bips, or 0.50%const amountOutMin = trade.minimumAmountOut(slippageTolerance).toExact()// needs to be converted to e.g. decimal stringconst path =[WETH9[DAI.chainId].address,DAI.address]const to =''// should be a checksummed recipient addressconst deadline = Math.floor(Date.now()/1000)+60*20// 20 minutes from the current Unix timeconst value = trade.inputAmount.toExact()// // needs to be converted to e.g. decimal string
```

The slippage tolerance encodes _how large of a price movement we're willing to tolerate before our trade will fail to execute_. Since Ethereum transactions are broadcast and confirmed in an adversarial environment, this tolerance is the best we can do to protect ourselves against price movements. We use this slippage tolerance to calculate the _minumum_ amount of DAI we must receive before our trade reverts, thanks to [minimumAmountOut](https://docs.uniswap.org/sdk/v2/reference/trade#minimumamountout-since-204). Note that this code calculates this worst-case outcome _assuming that the current price, i.e the route's mid price,_ is fair (usually a good assumption because of arbitrage).
The path is simply the ordered list of token addresses we're trading through, in our case WETH and DAI (note that we use the WETH address, even though we're using ETH).
The to address is the address that will receive the DAI.
The deadline is the Unix timestamp after which the transaction will fail, to protect us in the case that our transaction takes a long time to confirm and we wish to rescind our trade.
The value is the amount of ETH that must be included as the `msg.value` in our transaction.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/04-trading.md)
Was this helpful?
[PreviousPricing](https://docs.uniswap.org/sdk/v2/guides/pricing)[NextPair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/04-trading.md)
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
