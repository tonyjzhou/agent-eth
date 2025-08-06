# https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
    * [Permit2](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
          * [Using the API](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
          * [Custom Linking](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
          * [Iframe Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [API](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Governance](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Interface Integration
  * [Custom Linking](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking)


On this page
# Query Parameters
The Uniswap front-end supports URL query parameters to allow for custom linking to the Uniswap frontend. Users and developers can use these query parameters to link to the Uniswap frontend with custom prefilled settings.
Each Page has specific available URL parameters that can be set. Global parameters can be used on all pages.
A parameter used on an incorrect page will have no effect on frontend settings. Parameters not set with a URL parameter will be set to standard frontend defaults.
## Global[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#global "Direct link to Global")
Parameter| Type| Description  
---|---|---  
theme| `String`| Sets them to dark or light mode.  
### Theme Options[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#theme-options "Direct link to Theme Options")
Theme can be set as `light` or `dark`.
### Example Usage[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#example-usage "Direct link to Example Usage")
`https://app.uniswap.org/#/swap?theme=dark`
## Swap Page[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#swap-page "Direct link to Swap Page")
Parameter| Type| Description  
---|---|---  
inputCurrency| `address`| Input currency that will be swapped for output currency.  
outputCurrency| `address or ETH`| Output currency that input currency will be swapped for.  
exactAmount| `number`| The custom token amount to buy or sell.  
exactField| `string`| The field to set custom token amount for. Must be `input` or `output`.  
### Defaults[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#defaults "Direct link to Defaults")
ETH defaults as the input currency. When a different token is selected for either input or output ETH will default as the opposite selected currency.
### Constraints[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#constraints "Direct link to Constraints")
Addresses must be valid ERC20 addresses. Slippage and amount values must be valid numbers accepted by the frontend (or error will prevent from swapping). Slippage can 0, or within the range 10->9999 bips (which converts to 0%, 0.01%->99%)
When selecting ETH as the output currency a user must also choose an inputCurrency that is not ETH (to prevent ETH being populated in both fields)
### Setting Amounts[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#setting-amounts "Direct link to Setting Amounts")
Two parameters, exactField and exactAmount can be used to set specific token amounts to be sold or bought. Both fields must be set in the URL or there will be no effect on the settings.
### Example Usage[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#example-usage-1 "Direct link to Example Usage")
`https://app.uniswap.org/#/swap?exactField=input&exactAmount=10&inputCurrency=0x0F5D2fB29fb7d3CFeE444a200298f468908cC942`
## Pool Page[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#pool-page "Direct link to Pool Page")
The Pool page is made up of 2 subroutes: `add`, `remove`.
### Add Liquidity[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#add-liquidity "Direct link to Add Liquidity")
Parameter| Type| Description  
---|---|---  
Token0| `address`| Pool to withdraw liquidity from. (Must be an ERC20 address with an existing token)  
Token1| `address`| Pool to withdraw liquidity from. (Must be an ERC20 address with an existing token)  
### Example Usage[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#example-usage-2 "Direct link to Example Usage")
`https://app.uniswap.org/#/add/v2/0x6B175474E89094C44Da98b954EedeAC495271d0F/0xdAC17F958D2ee523a2206206994597C13D831ec7`
## Remove Liquidity[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#remove-liquidity "Direct link to Remove Liquidity")
Parameter| Type| Description  
---|---|---  
Token0| `address`| Pool to withdraw liquidity from. (Must be an ERC20 address with an existing token)  
Token1| `address`| Pool to withdraw liquidity from. (Must be an ERC20 address with an existing token)  
### Example Usage[​](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#example-usage-3 "Direct link to Example Usage")
`https://app.uniswap.org/#/remove/0x6B175474E89094C44Da98b954EedeAC495271d0F-0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2`
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/interface-integration/02-custom-interface-linking.md)
Was this helpful?
[PreviousUsing the API](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)[NextIframe Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)
On this page
  * [Global](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#global)
    * [Theme Options](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#theme-options)
    * [Example Usage](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#example-usage)
  * [Swap Page](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#swap-page)
    * [Defaults](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#defaults)
    * [Constraints](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#constraints)
    * [Setting Amounts](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#setting-amounts)
    * [Example Usage](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#example-usage-1)
  * [Pool Page](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#pool-page)
    * [Add Liquidity](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#add-liquidity)
    * [Example Usage](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#example-usage-2)
  * [Remove Liquidity](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#remove-liquidity)
    * [Example Usage](https://docs.uniswap.org/contracts/v2/guides/interface-integration/custom-interface-linking#example-usage-3)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/interface-integration/02-custom-interface-linking.md)
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
