# https://docs.uniswap.org/contracts/v2/reference/API/overview

[Skip to main content](https://docs.uniswap.org/contracts/v2/reference/API/overview#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/API/overview)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/API/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/overview)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/reference/API/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/reference/API/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/reference/API/overview)
    * [Permit2](https://docs.uniswap.org/contracts/v2/reference/API/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/overview)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/reference/API/overview)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/reference/API/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/overview)
          * [API Overview](https://docs.uniswap.org/contracts/v2/reference/API/overview)
          * [Entities](https://docs.uniswap.org/contracts/v2/reference/API/entities)
          * [Queries](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/API/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Technical Reference
  * API
  * [API Overview](https://docs.uniswap.org/contracts/v2/reference/API/overview)


On this page
# API Overview
This section explains the Uniswap Subgraph and how to interact with it. The Uniswap subgraph indexes data from the Uniswap contracts over time. It organizes data about pairs, tokens, Uniswap as a whole, and more. The subgraph updates any time a transaction is made on Uniswap. The subgraph runs on [The Graph](https://thegraph.com/) protocol's hosted service and can be openly queried.
## Resources[​](https://docs.uniswap.org/contracts/v2/reference/API/overview#resources "Direct link to Resources")
[Subgraph Explorer](https://thegraph.com/explorer/subgraph/uniswap/uniswap-v2) - sandbox for querying data and endpoints for developers.
[Uniswap V2 Subgraph](https://github.com/Uniswap/uniswap-v2-subgraph) - source code for deployed subgraph.
## Usage[​](https://docs.uniswap.org/contracts/v2/reference/API/overview#usage "Direct link to Usage")
The subgraph provides a snapshot of the current state of Uniswap and also tracks historical data. It is currently used to power [uniswap.info](https://uniswap.info/). **It is not intended to be used as a data source for structuring transactions (contracts should be referenced directly for the most reliable live data).**
## Making Queries[​](https://docs.uniswap.org/contracts/v2/reference/API/overview#making-queries "Direct link to Making Queries")
To learn more about querying a subgraph refer to [The Graph's documentation](https://thegraph.com/docs/about/introduction).
## Versions[​](https://docs.uniswap.org/contracts/v2/reference/API/overview#versions "Direct link to Versions")
The [Uniswap V2 Subgraph](https://thegraph.com/explorer/subgraph/uniswap/uniswap-v2) only tracks data on Uniswap V2. For Uniswap V1 information see the [V1 Subgraph](https://thegraph.com/explorer/subgraph/graphprotocol/uniswap).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/API/01-overview.md)
Was this helpful?
[PreviousSupporting meta transactions](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions)[NextEntities](https://docs.uniswap.org/contracts/v2/reference/API/entities)
On this page
  * [Resources](https://docs.uniswap.org/contracts/v2/reference/API/overview#resources)
  * [Usage](https://docs.uniswap.org/contracts/v2/reference/API/overview#usage)
  * [Making Queries](https://docs.uniswap.org/contracts/v2/reference/API/overview#making-queries)
  * [Versions](https://docs.uniswap.org/contracts/v2/reference/API/overview#versions)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/API/01-overview.md)
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
