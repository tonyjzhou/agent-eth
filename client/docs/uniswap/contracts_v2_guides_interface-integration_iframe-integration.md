# https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
    * [Permit2](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
          * [Using the API](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
          * [Custom Linking](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
          * [Iframe Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [API](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Governance](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Interface Integration
  * [Iframe Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)


Swap Widget
These docs are deprecated. Please refer to [Swap Widget](https://docs.uniswap.org/contracts/swap-widget/guides/getting-started) to embed the Uniswap swap widget as a React component in your website in 2 minutes.
Uniswap can be used within other sites as an iframe. An iframe shows an exact version of the Uniswap frontend site and can have custom prefilled settings.
# Why You May Want This
Integrating the Uniswap site directly into your web application can be useful for a variety of reasons.
The interface allows users to buy, sell, send, or provide liquidity for ERC20 tokens. An iframe integration may be useful if your application provides services around these ERC20 tokens. (For example, users can buy DAI through a Uniswap iframe on your site, then allow users to lend that DAI on your site).
It can also be useful if your application requires users to acquire some token in order to use some service (For example, allow users to buy "REP" token so they can engage in prediction markets on the Augur Dapp).
# iframe vs. custom UI
One benefit of an iframe integration is that the your site will automatically keep up with any improvements/additions to the site. After the initial integration is setup no further work is needed to pull in updates as the exchange site is updated over time.
# Example
```
<iframe src="https://app.uniswap.org/#/swap?exactField=input&exactAmount=10&inputCurrency=0x6b175474e89094c44da98b954eedeac495271d0f" height="660px" width="100%" style="  border: 0;  margin: 0 auto;  margin-bottom: .5rem;  display: block;  border-radius: 10px;  max-width: 960px;  min-width: 300px; "/>
```

An example of an Iframe integration can be found on the FOAM site [https://map.foam.space/](https://map.foam.space/#/at/?lng=-74.0045300&lat=40.6771800&zoom=5.00)
To see the iframe, click the dropdown in the top right and click `Get FOAM`.
# Add To Your Site
To include a Uniswap iframe within your site just add an iframe element within your website code and link to the Uniswap frontend.
Linking to a ETH <-> DAI swap page would look something like this. To link to a token of your choice replace the address after `outputCurrency` with the token address of the token you want to link to.
```
<iframe src="https://app.uniswap.org/#/swap?outputCurrency=0x89d24a6b4ccb1b6faa2625fe562bdd9a23260359" height="660px" width="100%" style="  border: 0;  margin: 0 auto;  display: block;  border-radius: 10px;  max-width: 600px;  min-width: 300px; "/>
```

You can customize the selected page, selected custom tokens and more using URL query parameters. See [Custom Linking](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/interface-integration/03-iframe-integration.mdx)
Was this helpful?
[PreviousCustom Linking](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)[NextSmart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/interface-integration/03-iframe-integration.mdx)
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
