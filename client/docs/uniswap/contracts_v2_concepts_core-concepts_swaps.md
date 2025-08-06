# https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
          * [Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
          * [Pools](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
          * [Oracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/Governance/governance-reference)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Concepts
  * Core Concepts
  * [Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)


On this page
![](https://docs.uniswap.org/assets/images/trade-b19a05be2c43a62708ab498766dc6d13.jpg)
# Introduction
Token swaps in Uniswap are a simple way to trade one ERC-20 token for another.
For end-users, swapping is intuitive: a user picks an input token and an output token. They specify an input amount, and the protocol calculates how much of the output token they’ll receive. They then execute the swap with one click, receiving the output token in their wallet immediately.
In this guide, we’ll look at what happens during a swap at the protocol level in order to gain a deeper understanding of how Uniswap works.
Swaps in Uniswap are different from trades on traditional platforms. Uniswap does not use an order book to represent liquidity or determine prices. Uniswap uses an automated market maker mechanism to provide instant feedback on rates and slippage.
As we learned in [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works), each pair on Uniswap is actually underpinned by a liquidity pool. Liquidity pools are smart contracts that hold balances of two unique tokens and enforces rules around depositing and withdrawing them.
This rule is the [constant product formula](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#constant-product-formula). When either token is withdrawn (purchased), a proportional amount of the other must be deposited (sold), in order to maintain the constant.
## Anatomy of a swap[​](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps#anatomy-of-a-swap "Direct link to Anatomy of a swap")
At the most basic level, all swaps in Uniswap V2 happen within a single function, aptly named `swap`:
```
functionswap(uint amount0Out,uint amount1Out,address to,bytescalldata data);
```

# Receiving tokens
As is probably clear from the function signature, Uniswap requires `swap` callers to _specify how many output tokens they would like to receive_ via the `amount{0,1}Out` parameters, which correspond to the desired amount of `token{0,1}`.
# Sending Tokens
What’s not as clear is how Uniswap _receives_ tokens as payment for the swap. Typically, smart contracts which need tokens to perform some functionality require callers to first make an approval on the token contract, then call a function that in turn calls transferFrom on the token contract. This is _not_ how V2 pairs accept tokens. Instead, pairs check their token balances at the _end_ of every interaction. Then, at the beginning of the _next_ interaction, current balances are differenced against the stored values to determine the amount of tokens that were sent by the current interactor. See the [whitepaper](https://docs.uniswap.org/whitepaper.pdf) for a justification of why this is the case.
The takeaway is that **tokens must be transferred to pairs before swap is called** (the one exception to this rule is [Flash Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)). This means that to safely use the `swap` function, it must be called from _another smart contract_. The alternative (transferring tokens to the pair and then calling `swap`) is not safe to do non-atomically because the sent tokens would be vulnerable to arbitrage.
# Developer resources
  * To see how to implement token swaps in a smart contract read [Trading from a smart contract](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract).
  * To see how to execute a swap from an interface read [Trading (SDK)](https://docs.uniswap.org/sdk/2.0.0/guides/trading)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/02-core-concepts/01-swaps.md)
Was this helpful?
[PreviousGlossary](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary)[NextPools](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
On this page
  * [Anatomy of a swap](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps#anatomy-of-a-swap)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/02-core-concepts/01-swaps.md)
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
