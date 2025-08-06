# https://docs.uniswap.org/contracts/v1/guides/token-listing

[Skip to main content](https://docs.uniswap.org/contracts/v1/guides/token-listing#__docusaurus_skipToContent_fallback)
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
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
      * [Overview](https://docs.uniswap.org/contracts/v1/overview)
      * [Guides](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)
        * [Connect to Uniswap](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)
        * [Pool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)
        * [Trade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Custom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Iframe Integration](https://docs.uniswap.org/contracts/v1/guides/iframe-integration)
        * [Token Listing](https://docs.uniswap.org/contracts/v1/guides/token-listing)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Factory](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Exchange](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Interfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v1 Protocol
  * Guides
  * [Token Listing](https://docs.uniswap.org/contracts/v1/guides/token-listing)


On this page
# Token Listing
It is possible that a token you are interested in is not included in the token dropdown on <https://app.uniswap.org/#/swap?use=v1>, however, all tokens that have a deployed uniswap exchange are supported on the front-end.
There are three ways to interact with tokens that are not yet included on the default list.
## 1. Paste the token address into the search box.[​](https://docs.uniswap.org/contracts/v1/guides/token-listing#1-paste-the-token-address-into-the-search-box "Direct link to 1. Paste the token address into the search box.")
If a token is not included in the list, try pasting the token address into the search box. It will populate the dropdown with the token you are looking for.
## 2. Custom Linking[​](https://docs.uniswap.org/contracts/v1/guides/token-listing#2-custom-linking "Direct link to 2. Custom Linking")
<https://app.uniswap.org/#/swap?use=v1> supports custom linking to all tokens that have a Uniswap exchange. See [Custom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking) for details on how to link.
For example, to populate the output token field with an unlisted token, we can specify the outputCurrency in the URL and pass in the token's address like this:
`https://app.uniswap.org/#/swap?use=v1?outputCurrency=0xfA3E941D1F6B7b10eD84A0C211bfA8aeE907965e`
## Token Details and Assets[​](https://docs.uniswap.org/contracts/v1/guides/token-listing#token-details-and-assets "Direct link to Token Details and Assets")
Token information (including decimals, symbol, name, etc.) is pulled from token contracts directly. Logo images are pulled from TrustWallet. If you'd like your token logo updated make a pull request into the TrustWallet assets repo <https://github.com/trustwallet/assets>.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/06-token-listing.md)
Was this helpful?
[PreviousIframe Integration](https://docs.uniswap.org/contracts/v1/guides/iframe-integration)[NextFactory](https://docs.uniswap.org/contracts/v1/reference/factory)
On this page
  * [1. Paste the token address into the search box.](https://docs.uniswap.org/contracts/v1/guides/token-listing#1-paste-the-token-address-into-the-search-box)
  * [2. Custom Linking](https://docs.uniswap.org/contracts/v1/guides/token-listing#2-custom-linking)
  * [Token Details and Assets](https://docs.uniswap.org/contracts/v1/guides/token-listing#token-details-and-assets)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/06-token-listing.md)
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
