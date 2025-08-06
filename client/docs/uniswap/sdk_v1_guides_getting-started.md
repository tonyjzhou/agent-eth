# https://docs.uniswap.org/sdk/v1/guides/getting-started

[Skip to main content](https://docs.uniswap.org/sdk/v1/guides/getting-started#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/guides/getting-started)
        * [Swaps](https://docs.uniswap.org/sdk/v1/guides/getting-started)
        * [Position Management](https://docs.uniswap.org/sdk/v1/guides/getting-started)
        * [Advanced](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/guides/getting-started)
    * [v3 SDK](https://docs.uniswap.org/sdk/v1/guides/getting-started)
    * [Swap Widget](https://docs.uniswap.org/sdk/v1/guides/getting-started)
    * [web3-react](https://docs.uniswap.org/sdk/v1/guides/getting-started)
    * [Core SDK](https://docs.uniswap.org/sdk/v1/guides/getting-started)
    * [v2 SDK](https://docs.uniswap.org/sdk/v1/guides/getting-started)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Overview](https://docs.uniswap.org/sdk/v1/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/guides/getting-started)
        * [Getting Started](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/guides/getting-started)
        * [Data](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Computation](https://docs.uniswap.org/sdk/v1/reference/computation)
        * [Format](https://docs.uniswap.org/sdk/v1/reference/format)
        * [Orchestration](https://docs.uniswap.org/sdk/v1/reference/orchestration)
        * [Transact](https://docs.uniswap.org/sdk/v1/reference/transact)
        * [Constants](https://docs.uniswap.org/sdk/v1/reference/constants)
        * [Types](https://docs.uniswap.org/sdk/v1/reference/types)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v1 SDK
  * Guides
  * [Getting Started](https://docs.uniswap.org/sdk/v1/guides/getting-started)


The [Uniswap SDK](https://github.com/Uniswap/uniswap-sdk/tree/v1) is meant to simplify every aspect of integrating Uniswap into your project. It's written in [TypeScript](https://www.typescriptlang.org), has a [robust test suite](https://github.com/Uniswap/uniswap-sdk/tree/v1/src/__tests__), uses [bignumber.js](https://github.com/MikeMcl/bignumber.js) for math, and includes an optional data-fetching module which relies on [ethers.js](https://github.com/ethers-io/ethers.js/).
The SDK was built to be extremely easy to use, but also feature-rich. It offers various levels of abstraction that make it suitable for use nearly anywhere, from hackathon projects to production applications.
# Overview
The SDK is divided into several modular components that perform tightly scoped tasks:
  * [Data](https://docs.uniswap.org/sdk/1.0.0/reference/data) - Fetches Uniswap data from the blockchain
  * [Computation](https://docs.uniswap.org/sdk/1.0.0/reference/computation) - Computes market- and trade-specific statistics using blockchain data
  * [Format](https://docs.uniswap.org/sdk/1.0.0/reference/format) - Formats data for display
  * [Orchestration](https://docs.uniswap.org/sdk/1.0.0/reference/orchestration) - Offers named abstraction functions that seamlessly combine lower-level data- and computation-related functions
  * [Transact](https://docs.uniswap.org/sdk/1.0.0/reference/transact) - Prepares computed trades for execution against Uniswap smart contracts
  * [Constants](https://docs.uniswap.org/sdk/1.0.0/reference/constants) - Exports various helpful constants for use throughout the SDK


Additionally, it exports a number of custom types:
  * [Types](https://docs.uniswap.org/sdk/1.0.0/reference/types) - Exports all types used by the SDK


# Installation
To start using the SDK, simply install it into your project...
```
yarn add @uniswap/sdk
```

...import some functions...
```
import{...}from'@uniswap/sdk'
```

...and dive into the rest of the documentation to learn more!
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/guides/getting-started.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/sdk/v1/overview)[NextData](https://docs.uniswap.org/sdk/v1/reference/data)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/guides/getting-started.md)
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
