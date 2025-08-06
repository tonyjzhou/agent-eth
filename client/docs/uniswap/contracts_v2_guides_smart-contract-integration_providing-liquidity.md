# https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
    * [Permit2](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
          * [Smart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
          * [Implement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
          * [Providing Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
          * [Building an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)
          * [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses)
          * [Supporting meta transactions](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [API](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Governance](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Smart Contract Integration
  * [Providing Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)


On this page
# Providing Liquidity
## Introduction[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity#introduction "Direct link to Introduction")
When providing liquidity from a smart contract, the most important thing to keep in mind is that tokens deposited into a pool at any rate other than the current reserve ratio _are vulnerable to being arbitraged_. As an example, if the ratio of x:y in a pair is 10:2 (i.e. the price is 5), and someone naively adds liquidity at 5:2 (a price of 2.5), the contract will simply accept all tokens (changing the price to 3.75 and opening up the market to arbitrage), but only issue pool tokens entitling the sender to the amount of assets sent at the proper ratio, in this case 5:1. To avoid donating to arbitrageurs, it is imperative to add liquidity at the current price. Luckily, it's easy to ensure that this condition is met!
## Using the Router[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity#using-the-router "Direct link to Using the Router")
The easiest way to safely add liquidity to a pool is to use the [router](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02), which provides simple methods to safely add liquidity to a pool. If the liquidity is to be added to an ERC-20/ERC-20 pair, use [addLiquidity](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#addliquidity). If WETH is involved, use [addLiquidityETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#addliquidityeth).
These methods both require the caller to commit to a _belief about the current price_ , which is encoded in the `amount*Desired` parameters. Typically, it's fairly safe to assume that the current fair market price is around what the current reserve ratio is for a pair (because of arbitrage). So, if a user wants to add 1 ETH to a pool, and the current DAI/WETH ratio of the pool is 200/1, it's reasonable to calculate that 200 DAI must be sent along with the ETH, which is an implicit commitment to the price of 200 DAI/1 WETH. However, it's important to note that this must be calculated _before the transaction is submitted_. It is _not safe_ to look up the reserve ratio from within a transaction and rely on it as a price belief, as this ratio can be cheaply manipulated to your detriment.
However, it is still possible to submit a transaction which encodes a belief about the price which ends up being wrong because of a larger change in the true market price before the transaction is confirmed. For that reason, it's necessary to pass an additional set of parameters which encode the caller's tolerance to price changes. These `amount*Min` parameters should typically be set to percentages of the calculated desired price. So, at a 1% tolerance level, if our user sends a transaction with 1 ETH and 200 DAI, `amountETHMin` should be set to e.g. .99 ETH, and `amountTokenMin` should be set to 198 DAI. This means that, at worst, liquidity will be added at a rate between 198 DAI/1 ETH and 202.02 DAI/1 ETH (200 DAI/.99 ETH).
Once the price calculations have been made, it's important to ensure that your contract a) controls at least as many tokens/ETH as were passed as `amount*Desired` parameters, and b) has granted approval to the router to withdraw this many tokens.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/03-providing-liquidity.md)
Was this helpful?
[PreviousImplement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)[NextBuilding an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
On this page
  * [Introduction](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity#introduction)
  * [Using the Router](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity#using-the-router)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/03-providing-liquidity.md)
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
