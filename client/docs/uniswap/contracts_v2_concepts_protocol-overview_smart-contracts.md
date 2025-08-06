# https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#__docusaurus_skipToContent_fallback)
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
          * [How Uniswap works](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
          * [Ecosystem Participants](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/ecosystem-participants)
          * [Smart contracts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts)
          * [Glossary](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
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
  * Protocol Overview
  * [Smart contracts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts)


On this page
Uniswap V2 is a binary smart contract system. [Core](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#core) contracts provide fundamental safety guarantees for all parties interacting with Uniswap. [Periphery](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#periphery) contracts interact with one or more core contracts but are not themselves part of the core.
# Core
[Source code](https://github.com/Uniswap/uniswap-v2-core)
The core consists of a singleton [factory](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#factory) and many [pairs](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#pairs), which the factory is responsible for creating and indexing. These contracts are quite minimal, even brutalist. The simple rationale for this is that contracts with a smaller surface area are easier to reason about, less bug-prone, and more functionally elegant. Perhaps the biggest upside of this design is that many desired properties of the system can be asserted directly in the code, leaving little room for error. One downside, however, is that core contracts are somewhat user-unfriendly. In fact, interacting directly with these contracts is not recommended for most use cases. Instead, a periphery contract should be used.
## Factory[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#factory "Direct link to Factory")
[Reference documentation](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
The factory holds the generic bytecode responsible for powering pairs. Its primary job is to create one and only one smart contract per unique token pair. It also contains logic to turn on the protocol charge.
## Pairs[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#pairs "Direct link to Pairs")
[Reference documentation](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
[Reference documentation (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair-erc-20)
Pairs have two primary purposes: serving as automated market makers and keeping track of pool token balances. They also expose data which can be used to build decentralized price oracles.
# Periphery
[Source code](https://github.com/Uniswap/uniswap-v2-periphery)
The periphery is a constellation of smart contracts designed to support domain-specific interactions with the core. Because of Uniswap's permissionless nature, the contracts described below have no special privileges, and are in fact only a small subset of the universe of possible periphery-like contracts. However, they are useful examples of how to safely and efficiently interact with Uniswap V2.
## Library[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#library "Direct link to Library")
[Reference documentation](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library)
The library provides a variety of convenience functions for fetching data and pricing.
## Router[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#router "Direct link to Router")
[Reference documentation](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
The router, which uses the library, fully supports all the basic requirements of a front-end offering trading and liquidity management functionality. Notably, it natively supports multi-pair trades (e.g. x to y to z), treats ETH as a first-class citizen, and offers meta-transactions for removing liquidity.
# Design Decisions
The following sections describe some of the notable design decisions made in Uniswap V2. These are safe to skip unless you're interested in gaining a deep technical understanding of how V2 works under the hood, or writing smart contract integrations!
## Sending Tokens[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#sending-tokens "Direct link to Sending Tokens")
Typically, smart contracts which need tokens to perform some functionality require would-be interactors to first make an approval on the token contract, then call a function that in turn calls transferFrom on the token contract. This is _not_ how V2 pairs accept tokens. Instead, pairs check their token balances at the _end_ of every interaction. Then, at the beginning of the _next_ interaction, current balances are differenced against the stored values to determine the amount of tokens that were sent by the current interactor. See the [whitepaper](https://docs.uniswap.org/whitepaper.pdf) for a justification of why this is the case, but the takeaway is that **tokens must be transferred to the pair before calling any token-requiring method** (the one exception to this rule is [Flash Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps).
## WETH[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#weth "Direct link to WETH")
Unlike Uniswap V1 pools, V2 pairs do not support ETH directly, so ETH⇄ERC-20 pairs must be emulated with WETH. The motivation behind this choice was to remove ETH-specific code in the core, resulting in a cleaner codebase. End users can be kept fully ignorant of this implementation detail, however, by simply wrapping/unwrapping ETH in the periphery.
The router fully supports interacting with any WETH pair via ETH.
## Minimum Liquidity[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#minimum-liquidity "Direct link to Minimum Liquidity")
To ameliorate rounding errors and increase the theoretical minimum tick size for liquidity provision, pairs burn the first [MINIMUM_LIQUIDITY](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#minimum_liquidity) pool tokens. For the vast majority of pairs, this will represent a trivial value. The burning happens automatically during the first liquidity provision, after which point the [totalSupply](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair-erc-20#totalsupply) is forevermore bounded.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/01-protocol-overview/03-smart-contracts.md)
Was this helpful?
[PreviousEcosystem Participants](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/ecosystem-participants)[NextGlossary](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary)
On this page
  * [Factory](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#factory)
  * [Pairs](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#pairs)
  * [Library](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#library)
  * [Router](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#router)
  * [Sending Tokens](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#sending-tokens)
  * [WETH](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#weth)
  * [Minimum Liquidity](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#minimum-liquidity)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/01-protocol-overview/03-smart-contracts.md)
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
