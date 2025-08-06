# https://docs.uniswap.org/concepts/protocol/concentrated-liquidity

[Skip to main content](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)
      * [Hooks](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Swaps](https://docs.uniswap.org/concepts/protocol/swaps)
      * [Fees](https://docs.uniswap.org/concepts/protocol/fees)
      * [Oracle](https://docs.uniswap.org/concepts/protocol/oracle)
      * [Range Orders](https://docs.uniswap.org/concepts/protocol/range-orders)
      * [Concentrated Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)
      * [Token Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)
    * [Governance](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)
    * [Research](https://docs.uniswap.org/concepts/research)
    * [Resources](https://docs.uniswap.org/concepts/resources)
    * [Glossary](https://docs.uniswap.org/concepts/glossary)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * Protocol Concepts
  * [Concentrated Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)


On this page
# Concentrated Liquidity
Your browser does not support the video tag.
note
Concentrated liquidity, first introduced as a native feature in Uniswap v3, maintains the same core implementation in v4, ensuring consistency in how liquidity providers can focus their capital.
## Introduction[​](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#introduction "Direct link to Introduction")
The defining idea of Uniswap v3 is concentrated liquidity: liquidity that is allocated within a custom price range. In earlier versions, liquidity was distributed uniformly along the price curve between 0 and infinity.
The previously uniform distribution allowed trading across the entire price interval (0, ∞) without any loss of liquidity. However, in many pools, the majority of the liquidity was never used.
Consider stablecoin pairs, where the relative price of the two assets stays relatively constant. The liquidity outside the typical price range of a stablecoin pair is rarely touched. For example, the v2 DAI/USDC pair utilizes ~0.50% of the total available capital for trading between $0.99 and $1.01, the price range in which LPs would expect to see the most volume - and consequently earn the most fees.
With v3, liquidity providers may concentrate their capital to smaller price intervals than (0, ∞). In a stablecoin/stablecoin pair, for example, an LP may choose to allocate capital solely to the 0.99 - 1.01 range. As a result, traders are offered deeper liquidity around the mid-price, and LPs earn more trading fees with their capital. We call liquidity concentrated to a finite interval a position. LPs may have many different positions per pool, creating individualized price curves that reflect the preferences of each LP.
## Active Liquidity[​](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#active-liquidity "Direct link to Active Liquidity")
As the price of an asset rises or falls, it may exit the price bounds that LPs have set in a position. When the price exits a position's interval, the position's liquidity is no longer active and no longer earns fees.
As price moves in one direction, LPs gain more of the one asset as swappers demand the other, until their entire liquidity consists of only one asset. (In v2, we don't typically see this behavior because LPs rarely reach the upper or lower bound of the price of two assets, i.e., 0 and ∞). If the price ever reenters the interval, the liquidity becomes active again, and in-range LPs begin earning fees once more.
Importantly, LPs are free to create as many positions as they see fit, each with its own price interval. Concentrated liquidity serves as a mechanism to let the market decide what a sensible distribution of liquidity is, as rational LPs are incentivized to concentrate their liquidity while ensuring that their liquidity remains active.
## Ticks[​](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#ticks "Direct link to Ticks")
To achieve concentrated liquidity, the once continuous spectrum of price space has been partitioned with ticks.
Ticks are the boundaries between discrete areas in price space. Ticks are spaced such that an increase or decrease of 1 tick represents a 0.01% increase or decrease in price at any point in price space.
Ticks function as boundaries for liquidity positions. When a position is created, the provider must choose the lower and upper tick that will represent their position's borders.
As the spot price changes during swapping, the pool contract will continuously exchange the outbound asset for the inbound, progressively using all the liquidity available within the current tick interval[1](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#user-content-fn-1) until the next tick is reached. At this point, the contract switches to a new tick and activates any dormant liquidity within a position that has a boundary at the newly active tick.
While each pool has the same number of underlying ticks, in practice only a portion of them are able to serve as active ticks. Due to the nature of the v3 smart contracts, tick spacing is directly correlated to the swap fee. Lower fee tiers allow closer potentially active ticks, and higher fees allow a relatively wider spacing of potential active ticks.
While inactive ticks have no impact on transaction cost during swaps, crossing an active tick does increase the cost of the transaction in which it is crossed, as the tick crossing will activate the liquidity within any new positions using the given tick as a border.
In areas where capital efficiency is paramount, such as stable coin pairs, narrower tick spacing increases the granularity of liquidity provisioning and will likely lower price impact when swapping - the result being significantly improved prices for stable coin swaps.
For more information on fee levels and their correlation to tick spacing, see the [whitepaper](https://uniswap.org/whitepaper-v3.pdf).
## Footnotes[​](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#footnote-label "Direct link to Footnotes")
  1. Tick interval refers to the area of price space between two nearest active ticks [↩](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#user-content-fnref-1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/concentrated-liquidity.md)
Was this helpful?
[PreviousRange Orders](https://docs.uniswap.org/concepts/protocol/range-orders)[NextToken Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)
On this page
  * [Introduction](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#introduction)
  * [Active Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#active-liquidity)
  * [Ticks](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#ticks)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/concentrated-liquidity.md)
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
