# https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
    * [Permit2](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
          * [Fees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
          * [Pricing](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
          * [Understanding Returns](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/understanding-returns)
          * [Security](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/security)
          * [Math](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/math)
          * [Research](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [API](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Governance](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Concepts
  * Advanced Topics
  * [Research](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)


The automated market maker is a new concept, and as such, new research comes out frequently. We've selected some of the most thoughtful here.
# Uniswap's Financial Alchemy
Authors: Dave White, Martin Tassy, Charlie Noyes, and Dan Robinson
> An automated market maker is a type of decentralized exchange that lets customers trade between on-chain assets like USDC and ETH. Uniswap is the most popular AMM on Ethereum. Like most AMMs, Uniswap facilitates trading between a particular pair of assets by holding reserves of both assets. It sets the trading price between them based on the size of its reserves in such a way that prices will stay in line with the broader market. Anybody who would like to can join the “pool” for a particular pair and become a liquidity provider, or LP, so-called because they provide liquid assets for others to trade against. LPs contribute assets to both reserves simultaneously, taking on some of the risk of trading in exchange for a share of the returns.
  * [Uniswap's Financial Alchemy](https://research.paradigm.xyz/uniswaps-alchemy)


# An analysis of Uniswap markets
Authors: Guillermo Angeris, Hsien-Tang Kao, Rei Chiang, Charlie Noyes, Tarun Chitra
> Uniswap---and other constant product markets---appear to work well in practice despite their simplicity. In this paper, we give a simple formal analysis of constant product markets and their generalizations, showing that, under some common conditions, these markets must closely track the reference market price. We also show that Uniswap satisfies many other desirable properties and numerically demonstrate, via a large-scale agent-based simulation, that Uniswap is stable under a wide range of market conditions.
  * [An analysis of Uniswap markets](https://arxiv.org/abs/1911.03380)


# Improved Price Oracles: Constant Function Market Makers
Authors: Guillermo Angeris, Tarun Chitra
> Automated market makers, first popularized by Hanson's logarithmic market scoring rule (or LMSR) for prediction markets, have become important building blocks, called 'primitives,' for decentralized finance. A particularly useful primitive is the ability to measure the price of an asset, a problem often known as the pricing oracle problem. In this paper, we focus on the analysis of a very large class of automated market makers, called constant function market makers (or CFMMs) which includes existing popular market makers such as Uniswap, Balancer, and Curve, whose yearly transaction volume totals to billions of dollars. We give sufficient conditions such that, under fairly general assumptions, agents who interact with these constant function market makers are incentivized to correctly report the price of an asset and that they can do so in a computationally efficient way. We also derive several other useful properties that were previously not known. These include lower bounds on the total value of assets held by CFMMs and lower bounds guaranteeing that no agent can, by any set of trades, drain the reserves of assets held by a given CFMM.
  * [Improved Price Oracles: Constant Function Market Makers](https://arxiv.org/abs/2003.10001)


# Pintail research
Published [medium](https://medium.com/@pintail) articles by Pintail.
  * [Understanding Uniswap Returns](https://medium.com/@pintail/understanding-uniswap-returns-cc593f3499ef)
  * [Uniswap: A Good Deal for Liquidity Providers?](https://medium.com/@pintail/uniswap-a-good-deal-for-liquidity-providers-104c0b6816f2)


# Liquidity Provider Returns in Geometric Mean Markets
Authors: Alex Evans
> Geometric mean market makers (G3Ms), such as Uniswap and Balancer, comprise a popular class of automated market makers (AMMs) defined by the following rule: the reserves of the AMM before and after each trade must have the same (weighted) geometric mean. This paper extends several results known for constant-weight G3Ms to the general case of G3Ms with time-varying and potentially stochastic weights. These results include the returns and no-arbitrage prices of liquidity pool (LP) shares that investors receive for supplying liquidity to G3Ms. Using these expressions, we show how to create G3Ms whose LP shares replicate the payoffs of financial derivatives. The resulting hedges are model-independent and exact for derivative contracts whose payoff functions satisfy an elasticity constraint. These strategies allow LP shares to replicate various trading strategies and financial contracts, including standard options. G3Ms are thus shown to be capable of recreating a variety of active trading strategies through passive positions in LP shares.
  * [Liquidity Provider Returns in Geometric Mean Markets](https://arxiv.org/abs/2006.08806)


# The Replicating Portfolio of a Constant Product Market
Authors: Joseph Clark
> We derive the replicating portfolio of a constant product market. This is structurally short volatility (selling options) which explains why positive transaction costs are needed to induce liquidity providers to participate. Where futures and options markets do not exist, this payoff can be used to create them.
  * <https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3550601>


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/03-advanced-topics/06-research.md)
Was this helpful?
[PreviousMath](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/math)[NextUsing the API](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/03-advanced-topics/06-research.md)
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
