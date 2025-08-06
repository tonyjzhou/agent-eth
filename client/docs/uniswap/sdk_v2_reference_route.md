# https://docs.uniswap.org/sdk/v2/reference/route

[Skip to main content](https://docs.uniswap.org/sdk/v2/reference/route#__docusaurus_skipToContent_fallback)
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
  * Technical Reference
  * [Route](https://docs.uniswap.org/sdk/v2/reference/route)


On this page
```
constructor(pairs: Pair[], input: Token, output: Token)
```

The Route entity represents one or more ordered Uniswap pairs with a fully specified path from input token to output token.
# Example
```
import{ ChainId, Token, CurrencyAmount }from'@uniswap/sdk-core'import{ Pair, Route }from'@uniswap/v2-sdk'constHOT=newToken(ChainId.MAINNET,'0xc0FFee0000000000000000000000000000000000',18,'HOT','Caffeine')constNOT=newToken(ChainId.MAINNET,'0xDeCAf00000000000000000000000000000000000',18,'NOT','Caffeine')constHOT_NOT=newPair(CurrencyAmount.fromRawAmount(HOT,'2000000000000000000'), CurrencyAmount.fromRawAmount(NOT,'1000000000000000000'))const route =newRoute([HOT_NOT],NOT,HOT)
```

# Properties
## pairs[​](https://docs.uniswap.org/sdk/v2/reference/route#pairs "Direct link to pairs")
```
pairs: Pair[]
```

The ordered pairs that the route is comprised of.
## path[​](https://docs.uniswap.org/sdk/v2/reference/route#path "Direct link to path")
```
path: Token[]
```

The full path from input token to output token.
## input[​](https://docs.uniswap.org/sdk/v2/reference/route#input "Direct link to input")
```
input: Token
```

The input token.
## output[​](https://docs.uniswap.org/sdk/v2/reference/route#output "Direct link to output")
```
output: Token
```

The output token.
## midPrice[​](https://docs.uniswap.org/sdk/v2/reference/route#midprice "Direct link to midPrice")
```
midPrice: Price
```

Returns the current mid price along the route.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/reference/03-route.md)
Was this helpful?
[PreviousPair](https://docs.uniswap.org/sdk/v2/reference/pair)[NextTrade](https://docs.uniswap.org/sdk/v2/reference/trade)
On this page
  * [pairs](https://docs.uniswap.org/sdk/v2/reference/route#pairs)
  * [path](https://docs.uniswap.org/sdk/v2/reference/route#path)
  * [input](https://docs.uniswap.org/sdk/v2/reference/route#input)
  * [output](https://docs.uniswap.org/sdk/v2/reference/route#output)
  * [midPrice](https://docs.uniswap.org/sdk/v2/reference/route#midprice)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/reference/03-route.md)
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
