# https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/ecosystem-participants

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/ecosystem-participants#__docusaurus_skipToContent_fallback)
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
  * [Ecosystem Participants](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/ecosystem-participants)


![](https://docs.uniswap.org/assets/images/participants-a3e150f3c98a0b402c2063de3e160f2e.jpg)
The Uniswap ecosystem is primarily comprised of three types of users: liquidity providers, traders, and developers. Liquidity providers are incentivized to contribute [ERC-20](https://eips.ethereum.org/EIPS/eip-20) tokens to common liquidity pools. Traders can swap these tokens for one another for a fixed [0.30% fee](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees) (which goes to liquidity providers). Developers can integrate directly with Uniswap smart contracts to power new and exciting interactions with tokens, trading interfaces, retail experiences, and more.
In total, interactions between these classes create a positive feedback loop, fueling digital economies by defining a common language through which tokens can be pooled, traded and used.
# Liquidity Providers
Liquidity providers, or LPs, are not a homogeneous group:
  * Passive LPs are token holders who wish to passively invest their assets to accumulate trading fees.
  * Professional LPs are focused on market making as their primary strategy. They usually develop custom tools and ways of tracking their liquidity positions across different DeFi projects.
  * Token projects sometimes choose to become LPs to create a liquid marketplace for their token. This allows tokens to be bought and sold more easily, and unlocks interoperability with other DeFi projects through Uniswap.
  * Finally, some DeFi pioneers are exploring complex liquidity provision interactions like incentivized liquidity, liquidity as collateral, and other experimental strategies. Uniswap is the perfect protocol for projects to experiment with these kinds of ideas.


# Traders
There are a several categories of traders in the protocol ecosystem:
  * Speculators use a variety of community built tools and products to swap tokens using liquidity pulled from the Uniswap protocol.
  * Arbitrage bots seek profits by comparing prices across different platforms to find an edge. (Though it might seem extractive, these bots actually help equalize prices across broader Ethereum markets and keep things fair.)
  * DAPP users buy tokens on Uniswap for use in other applications on Ethereum.
  * Smart contracts that execute trades on the protocol by implementing swap functionality (from products like DEX aggregators to custom Solidity scripts).


In all cases, trades are subject to the same flat fee for trading on the protocol. Each is important for increasing the accuracy of prices and incentivizing liquidity.
# Developers/Projects
There are far too many ways Uniswap is used in the wider Ethereum ecosystem to count, but some examples include:
  * The open-source, accessible nature of Uniswap means there are countless UX experiments and front-ends built to offer access to Uniswap functionality. You can find Uniswap functions in most of the major DeFi dashboard projects. There are also many [Uniswap-specific tools](https://github.com/Uniswap/universe) built by the community.
  * Wallets often integrate swapping and liquidity provision functionality as a core offering of their product.
  * DEX (decentralized exchange) aggregators pull liquidity from many liquidity protocols to offer traders the best prices by splitting their trades. Uniswap is the biggest single decentralized liquidity source for these projects.
  * Smart contract developers use the suite of functions available to invent new DeFi tools and other various experimental ideas. See projects like [Unisocks](https://unisocks.exchange/) or [Zora](https://ourzora.com/), among many, many others.


# Uniswap Team and Community
The Uniswap team along with the broader Uniswap community drives development of the protocol and ecosystem.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/01-protocol-overview/02-ecosystem-participants.md)
Was this helpful?
[PreviousHow Uniswap works](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)[NextSmart contracts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/01-protocol-overview/02-ecosystem-participants.md)
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
