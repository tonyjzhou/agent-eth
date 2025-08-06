# https://docs.uniswap.org/concepts/protocol/fees

[Skip to main content](https://docs.uniswap.org/concepts/protocol/fees#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Hooks](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Swaps](https://docs.uniswap.org/concepts/protocol/swaps)
      * [Fees](https://docs.uniswap.org/concepts/protocol/fees)
      * [Oracle](https://docs.uniswap.org/concepts/protocol/oracle)
      * [Range Orders](https://docs.uniswap.org/concepts/protocol/range-orders)
      * [Concentrated Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)
      * [Token Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)
    * [Governance](https://docs.uniswap.org/concepts/governance/overview)
    * [Research](https://docs.uniswap.org/concepts/research)
    * [Resources](https://docs.uniswap.org/concepts/resources)
    * [Glossary](https://docs.uniswap.org/concepts/glossary)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * Protocol Concepts
  * [Fees](https://docs.uniswap.org/concepts/protocol/fees)


On this page
# Fees
note
While v3 uses predefined fee tiers (0.01%, 0.05%, 0.3%, and 1%), v4 introduces flexible fees that can range from 0% to 100%, offering greater customization options for pools.
## Swap Fees[​](https://docs.uniswap.org/concepts/protocol/fees#swap-fees "Direct link to Swap Fees")
Swap fees are distributed pro-rata to all in-range[1](https://docs.uniswap.org/concepts/protocol/fees#user-content-fn-1) liquidity at the time of the swap. If the spot price moves out of a position’s range, the given liquidity is no longer active and does not generate any fees. If the spot price reverses and reenters the position’s range, the position’s liquidity becomes active again and will generate fees.
Swap fees are not automatically reinvested as they were in previous versions of Uniswap. Instead, they are collected separately from the pool and must be manually redeemed when the owner wishes to collect their fees.
## Pool Fees Tiers[​](https://docs.uniswap.org/concepts/protocol/fees#pool-fees-tiers "Direct link to Pool Fees Tiers")
Uniswap v3 introduces multiple pools for each token pair, each with a different swapping fee. Liquidity providers may initially create pools at three fee levels: 0.05%, 0.30%, and 1%. More fee levels may be added by UNI governance, e.g. the 0.01% fee level added by [this](https://vote.uniswapfoundation.org/proposals/9) governance proposal in December 2021, as executed [here](https://etherscan.io/tx/0x5c84f89a67237db7500538b81af61ebd827c081302dd73a1c20c8f6efaaf4f3c).
Breaking pairs into separate pools was previously untenable due to the issue of liquidity fragmentation. Any incentive alignments achieved by more fee optionality invariably resulted in a net loss to traders, due to lower pairwise liquidity and the resulting increase in price impact upon swapping.
The introduction of concentrated liquidity decouples total liquidity from price impact. With price impact concerns out of the way, breaking pairs into multiple pools becomes a feasible approach to improving the functionality of a pool for assets previously underserved by the 0.30% swap fee.
## Finding The Right Pool Fee[​](https://docs.uniswap.org/concepts/protocol/fees#finding-the-right-pool-fee "Direct link to Finding The Right Pool Fee")
We anticipate that certain types of assets will gravitate towards specific fee tiers, based on where the incentives for both swappers and liquidity providers come nearest to alignment.
We expect low volatility assets (stable coins) will likely congregate in the lowest fee tier, as the price risk for liquidity providers holding these assets is very low, and those swapping will be motivated to pursue an execution price closest to 1:1 as they can get.
Similarly, we anticipate more exotic assets, or those traded rarely, will naturally gravitate towards a higher fee - as liquidity providers will be motivated to offset the cost risk of holding these assets for the duration of their position.
## Protocol Fees[​](https://docs.uniswap.org/concepts/protocol/fees#protocol-fees "Direct link to Protocol Fees")
Both Uniswap v3 and v4 include a protocol fee mechanism that can be activated through UNI governance. This fee structure offers greater flexibility compared to v2, allowing governance to adjust the fraction of swap fees allocated to the protocol. For detailed information about protocol fees, refer to the [v3 whitepaper](https://uniswap.org/whitepaper-v3.pdf) and [v4 whitepaper](https://uniswap.org/whitepaper-v4.pdf).
## Footnotes[​](https://docs.uniswap.org/concepts/protocol/fees#footnote-label "Direct link to Footnotes")
  1. In-range liquidity refers to the liquidity contained in any positions which span both sides of the spot price. [↩](https://docs.uniswap.org/concepts/protocol/fees#user-content-fnref-1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/fees.md)
Was this helpful?
[PreviousSwaps](https://docs.uniswap.org/concepts/protocol/swaps)[NextOracle](https://docs.uniswap.org/concepts/protocol/oracle)
On this page
  * [Swap Fees](https://docs.uniswap.org/concepts/protocol/fees#swap-fees)
  * [Pool Fees Tiers](https://docs.uniswap.org/concepts/protocol/fees#pool-fees-tiers)
  * [Finding The Right Pool Fee](https://docs.uniswap.org/concepts/protocol/fees#finding-the-right-pool-fee)
  * [Protocol Fees](https://docs.uniswap.org/concepts/protocol/fees#protocol-fees)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/fees.md)
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
