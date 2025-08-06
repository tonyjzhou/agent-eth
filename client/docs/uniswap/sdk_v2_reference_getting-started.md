# https://docs.uniswap.org/sdk/v2/reference/getting-started

[Skip to main content](https://docs.uniswap.org/sdk/v2/reference/getting-started#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v2/reference/getting-started)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Swaps](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Position Management](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Advanced](https://docs.uniswap.org/sdk/v2/reference/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/reference/getting-started)
    * [v3 SDK](https://docs.uniswap.org/sdk/v2/reference/getting-started)
    * [Swap Widget](https://docs.uniswap.org/sdk/v2/reference/getting-started)
    * [web3-react](https://docs.uniswap.org/sdk/v2/reference/getting-started)
    * [Core SDK](https://docs.uniswap.org/sdk/v2/reference/getting-started)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/reference/getting-started)
      * [Overview](https://docs.uniswap.org/sdk/v2/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/reference/getting-started)
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
    * [v1 SDK](https://docs.uniswap.org/sdk/v2/reference/getting-started)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v2 SDK
  * Technical Reference
  * [Getting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)


On this page
# Getting Started
The pages that follow contain technical reference information on the Uniswap SDK. Looking for a [quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start) instead? You may also want to jump into a [guide](https://docs.uniswap.org/sdk/v2/guides/fetching-data), which offers a friendlier introduction to the SDK!
The SDK is written in TypeScript, has a robust test suite, performs arbitrary precision arithmetic, and supports rounding to significant digits or fixed decimal places. The principal exports of the SDK are _entities_ : classes that contain initialization and validation checks, necessary data fields, and helper functions.
An important concept in the SDK is _fractions_. Because Solidity performs integer math, care must be taken in non-EVM environments to faithfully replicate the actual computation carried out on-chain. The first concern here is to ensure that an overflow-safe integer implementation is used. Ideally, the SDK would be able to use native [BigInt](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt)s. However, until support becomes more widespread, [JSBI](https://github.com/GoogleChromeLabs/jsbi) objects are used instead, with the idea that once BigInts proliferate, this dependency can be compiled away. The second concern is precision loss due to, for example, chained price ratio calculations. To address this issue, all math operations are performed as fraction operations, ensuring arbitrary precision up until the point that values are rounded for display purposes, or truncated to fit inside a fixed bit width.
The Fractions class, among others that the V2 SDK depends on, are exported from the SDK Core to allow interoperability with the V3 SDK. Refer to the [Core SDK section of the docs](https://docs.uniswap.org/sdk/core/overview) to learn more about these classes.
The SDK works for all chains on which the [factory](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#address) is deployed.
## Code[​](https://docs.uniswap.org/sdk/v2/reference/getting-started#code "Direct link to Code")
The [source code is available on GitHub](https://github.com/Uniswap/uniswap-sdk).
## Dependencies[​](https://docs.uniswap.org/sdk/v2/reference/getting-started#dependencies "Direct link to Dependencies")
The SDK installs a small number of dependencies(<https://github.com/Uniswap/v2-sdk/blob/main/package.json#L24>). The most important dependency of the V2 SDK is the SDK core, which was previously part of the V2 SDK itself, but later released as its own package to avoid duplicate code between the V2 and V3 SDK.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/reference/01-getting-started.md)
Was this helpful?
[PreviousPair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)[NextPair](https://docs.uniswap.org/sdk/v2/reference/pair)
On this page
  * [Code](https://docs.uniswap.org/sdk/v2/reference/getting-started#code)
  * [Dependencies](https://docs.uniswap.org/sdk/v2/reference/getting-started#dependencies)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/reference/01-getting-started.md)
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
