# https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
    * [Permit2](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
          * [Fees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
          * [Pricing](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
          * [Understanding Returns](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/understanding-returns)
          * [Security](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/security)
          * [Math](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/math)
          * [Research](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [API](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Governance](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Concepts
  * Advanced Topics
  * [Fees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)


On this page
# Fees
## Liquidity provider fees[​](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees#liquidity-provider-fees "Direct link to Liquidity provider fees")
There is a **0.3%** fee for swapping tokens. **This fee is split by liquidity providers proportional to their contribution to liquidity reserves.**
Swapping fees are immediately deposited into liquidity reserves. This increases the value of liquidity tokens, functioning as a payout to all liquidity providers proportional to their share of the pool. Fees are collected by burning liquidity tokens to remove a proportional share of the underlying reserves.
Since fees are added to liquidity pools, the invariant increases at the end of every trade. Within a single transaction, the invariant represents `token0_pool * token1_pool` at the end of the previous transaction.
There are many community-developed tools to determine returns. You can also read more in the docs about how to think about [LP returns](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/understanding-returns).
## Protocol Fees[​](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees#protocol-fees "Direct link to Protocol Fees")
At the moment there are no protocol fees. However, it is possible for a 0.05% fee to be turned on in the future.
More information about a potential future protocol fee can be found [here](https://uniswap.org/blog/uniswap-v2/#path-to-sustainability).
## Protocol Charge Calculation[​](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees#protocol-charge-calculation "Direct link to Protocol Charge Calculation")
In the future, it is possible that a protocol-wide charge of 0.05% per trade will take effect. This represents ⅙th (16.6̅%) of the 0.30% fee. The fee is in effect if [feeTo](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory/#feeto) is not `address(0)` (`0x0000000000000000000000000000000000000000`), indicating that feeTo is the recipient of the charge.
This amount would not affect the fee paid by traders, but would affect the amount received by liquidity providers.
Rather than calculating this charge on swaps, which would significantly increase gas costs for all users, the charge is instead calculated when liquidity is added or removed. See the [whitepaper](https://docs.uniswap.org/whitepaper.pdf) for more details.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/03-advanced-topics/01-fees.md)
Was this helpful?
[PreviousOracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles)[NextPricing](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
On this page
  * [Liquidity provider fees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees#liquidity-provider-fees)
  * [Protocol Fees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees#protocol-fees)
  * [Protocol Charge Calculation](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees#protocol-charge-calculation)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/03-advanced-topics/01-fees.md)
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
