# https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
    * [Permit2](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
          * [Fees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
          * [Pricing](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
          * [Understanding Returns](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/understanding-returns)
          * [Security](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/security)
          * [Math](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/math)
          * [Research](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [API](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Governance](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Concepts
  * Advanced Topics
  * [Pricing](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)


On this page
# How are prices determined?
As we learned in [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works), each pair on Uniswap is actually underpinned by a liquidity pool. Liquidity pools are smart contracts that hold balances of two unique tokens and enforces rules around depositing and withdrawing them. The primary rule is the [constant product formula](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#constant-product-formula). When a token is withdrawn (bought), a proportional amount must be deposited (sold) to maintain the constant. The ratio of tokens in the pool, in combination with the constant product formula, ultimately determine the price that a swap executes at.
# How Uniswap handles prices
In Uniswap V1, trades are always executed at the "best possible" price, calculated at execution time. Somewhat confusingly, this calculation is actually accomplished with one of two different formulas, depending on whether the trade specifies an exact _input_ or _output_ amount. Functionally, the difference between these two functions is miniscule, but the very existence of a difference increases conceptual complexity. Initial attempts to support both functions in V2 proved inelegant, and the decision was made to **not provide any pricing functions in the core**. Instead, pairs directly check whether the invariant was satisfied (accounting for fees) after every trade. This means that rather than relying on a pricing function to _also_ enforce the invariant, V2 pairs simply and transparently ensure their own safety, a nice separation of concerns. One downstream benefit is that V2 pairs will more naturally support other flavors of trades which may emerge, (e.g. trading to a specific price at execution time).
At a high level, in Uniswap V2, _trades must be priced in the periphery_. The good news is that the [library](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library) provides a variety of functions designed to make this quite simple, and all swapping functions in the [router](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02) are designed with this in mind.
# Pricing Trades
When swapping tokens on Uniswap, it's common to want to receive as many output tokens as possible for an _exact input amount_ , or to pay as few input tokens as possible for an _exact output amount_. In order to calculate these amounts, a contract must look up the _current reserves_ of a pair, in order to understand what the current price is. However, it is _not safe to perform this lookup and rely on the results without access to an external price_.
Say a smart contract naively wants to send 10 DAI to the DAI/WETH pair and receive as much WETH as it can get, given the current reserve ratio. If, when called, the naive smart contract simply looks up the current price and executes the trade, it is _vulnerable to front-running and will likely suffer an economic loss_. To see why, consider a malicious actor who sees this transaction before it is confirmed. They could execute a swap which dramatically changes the DAI/WETH price immediately before the naive swap goes through, wait for the naive swap to execute at a bad rate, and then swap to change the price back to what it was before the naive swap. This attack is fairly cheap and low-risk, and can typically be performed for a profit.
To prevent these types of attacks, it's vital to submit swaps _that have access to knowledge about the "fair" price their swap should execute at_. In other words, swaps need access to an _oracle_ , to be sure that the best execution they can get from Uniswap is close enough to what the oracle considers the "true" price. While this may sound complicated, the oracle can be as simple as an _off-chain observation of the current market price of a pair_. Because of arbitrage, it's typically the case that the ratio of the intra-block reserves of a pair is close to the "true" market price. So, if a user submits a trade with this knowledge in mind, they can ensure that the losses due to front-running are tightly bounded. This is how, for example, the Uniswap frontend ensure trade safety. It calculates the optimal input/output amounts given observed intra-block prices, and uses the router to perform the swap, which guarantees the swap will execute at a rate no less that `x`% worse than the observed intra-block rate, where `x` is a user-specified slippage tolerance (0.5% by default).
There are, of course, other options for oracles, including [native V2 oracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles).
## Exact Input[​](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing#exact-input "Direct link to Exact Input")
If you'd like to send an exact amount of input tokens in exchange for as many output tokens as possible, you'll want to use [getAmountsOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountout). The equivalent SDK function is [getOutputAmount](https://docs.uniswap.org/sdk/2.0.0/reference/pair#getoutputamount), or [minimumAmountOut](https://docs.uniswap.org/sdk/2.0.0/reference/trade#minimumamountout-since-204) for slippage calculations.
## Exact Output[​](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing#exact-output "Direct link to Exact Output")
If you'd like to receive an exact amount of output tokens for as few input tokens as possible, you'll want to use [getAmountsIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountsin). The equivalent SDK function is [getInputAmount](https://docs.uniswap.org/sdk/2.0.0/reference/pair#getinputamount), or [maximumAmountIn](https://docs.uniswap.org/sdk/2.0.0/reference/trade#maximumamountin-since-204) for slippage calculations.
## Swap to Price[​](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing#swap-to-price "Direct link to Swap to Price")
For this more advanced use case, see [ExampleSwapToPrice.sol](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/examples/ExampleSwapToPrice.sol).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/03-advanced-topics/02-pricing.md)
Was this helpful?
[PreviousFees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)[NextUnderstanding Returns](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/understanding-returns)
On this page
  * [Exact Input](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing#exact-input)
  * [Exact Output](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing#exact-output)
  * [Swap to Price](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing#swap-to-price)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/03-advanced-topics/02-pricing.md)
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
