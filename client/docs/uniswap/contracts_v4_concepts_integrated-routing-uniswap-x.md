# https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x

[Skip to main content](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
        * [v4 vs v3](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/concepts/flash-accounting)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks)
        * [Subscribers](https://docs.uniswap.org/contracts/v4/concepts/subscribers)
        * [PoolManager](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
        * [Dynamic Fees](https://docs.uniswap.org/contracts/v4/concepts/dynamic-fees)
        * [Integrated Routing with UniswapX](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
        * [v4 Fee Structure Guide](https://docs.uniswap.org/contracts/v4/concepts/fees)
        * [Security](https://docs.uniswap.org/contracts/v4/concepts/security)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
    * [Permit2](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Concepts
  * [Integrated Routing with UniswapX](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)


# Integrated Routing with UniswapX
The [Uniswap Interface](https://app.uniswap.org) will be ramping up support for hooks in its standard routing system progressively over time. Hook builders looking to get immediate access to flow from the interface can do so by running a UniswapX filler for their hooked pools.
At a high level, hook builders' filler implementations will need to do the following:
  1. (On Mainnet) Subscribe to the UniswapX RFQ system and submit fillable bids from orders they receive
  2. Listen to the public feed for orders they won or that are open to be filled publicly
  3. Execute those orders against pools that use their hooks


Developers should check [UniswapX Documentation](https://docs.uniswap.org/contracts/uniswapx/overview) to get started.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/08-integrated-routing-uniswap-x.mdx)
Was this helpful?
[PreviousDynamic Fees](https://docs.uniswap.org/contracts/v4/concepts/dynamic-fees)[Nextv4 Fee Structure Guide](https://docs.uniswap.org/contracts/v4/concepts/fees)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/08-integrated-routing-uniswap-x.mdx)
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
