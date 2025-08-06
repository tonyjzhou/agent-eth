# https://docs.uniswap.org/sdk/v2/guides/quick-start

[Skip to main content](https://docs.uniswap.org/sdk/v2/guides/quick-start#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/overview)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/overview)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/overview)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
      * [Overview](https://docs.uniswap.org/sdk/v2/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [SDK Quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [Fetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)
        * [Pricing](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Trading](https://docs.uniswap.org/sdk/v2/guides/trading)
        * [Pair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Getting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Pair](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Route](https://docs.uniswap.org/sdk/v2/reference/route)
        * [Trade](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Other Exports](https://docs.uniswap.org/sdk/v2/reference/other-exports)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v2 SDK
  * Guides
  * [SDK Quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start)


On this page
The Uniswap SDK exists to help developers build on top of Uniswap. It's designed to run in any environment that can execute JavaScript (think websites, node scripts, etc.). While simple enough to use in a hackathon project, it's also robust enough to power production applications.
# Installation
The easiest way to consume the SDK is via [npm](https://github.com/Uniswap/uniswap-v2-sdk). To install it in your project, simply run `yarn add @uniswap/v2-sdk` (or `npm install @uniswap/v2-sdk`). This also installs the sdk-core package that is used by both the V2 and V3 SDK and ethers as dependencies.
# Usage
To run code from the SDK in your application, use an `import` or `require` statement, depending on which your environment supports. Note that the guides following this page will use ES6 syntax.
## ES6 (import)[​](https://docs.uniswap.org/sdk/v2/guides/quick-start#es6-import "Direct link to ES6 \(import\)")
```
import{ ChainId }from'@uniswap/sdk-core'import{Pair}from'@uniswap/v2-sdk'console.log(`The chainId of mainnet is ${ChainId.MAINNET}.`)
```

## CommonJS (require)[​](https://docs.uniswap.org/sdk/v2/guides/quick-start#commonjs-require "Direct link to CommonJS \(require\)")
```
constCORE=require('@uniswap/sdk-core')constV2_SDK=require('@uniswap/v2-sdk')console.log(`The chainId of mainnet is ${CORE.ChainId.MAINNET}.`)
```

# Reference
Comprehensive reference material for the SDK is publicly available on the [Uniswap Labs GitHub](https://github.com/Uniswap).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/01-quick-start.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/sdk/v2/overview)[NextFetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)
On this page
  * [ES6 (import)](https://docs.uniswap.org/sdk/v2/guides/quick-start#es6-import)
  * [CommonJS (require)](https://docs.uniswap.org/sdk/v2/guides/quick-start#commonjs-require)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/01-quick-start.md)
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
