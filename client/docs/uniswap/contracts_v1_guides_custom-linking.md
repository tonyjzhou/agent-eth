# https://docs.uniswap.org/contracts/v1/guides/custom-linking

[Skip to main content](https://docs.uniswap.org/contracts/v1/guides/custom-linking#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
      * [Quickstart](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
    * [UniswapX](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
    * [Universal Router](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
    * [Permit2](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
      * [Overview](https://docs.uniswap.org/contracts/v1/overview)
      * [Guides](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Connect to Uniswap](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)
        * [Pool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)
        * [Trade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Custom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Iframe Integration](https://docs.uniswap.org/contracts/v1/guides/iframe-integration)
        * [Token Listing](https://docs.uniswap.org/contracts/v1/guides/token-listing)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Factory](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Exchange](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Interfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v1 Protocol
  * Guides
  * [Custom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking)


On this page
# Query Parameters
The Uniswap front-end supports URL query parameters to allow for custom linking to the Uniswap exchange. Users and developers can use these query parameters to link to the Uniswap exchange with custom prefilled settings.
Each Page has specific available URL parameters that can be set. Global parameters can be used on all pages.
A parameter used on an incorrect page will have no effect on exchange settings. Parameters not set with a URL parameter will be set to standard exchange defaults.
## Global[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#global "Direct link to Global")
Parameter| Type| Description  
---|---|---  
theme| `String`| Sets them to dark or light mode.  
### Theme Options[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#theme-options "Direct link to Theme Options")
Theme can be set as `light` or `dark`.
### Example Usage[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage "Direct link to Example Usage")
`https://app.uniswap.org/#/swap?theme=dark&use=v1`
## Swap Page[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#swap-page "Direct link to Swap Page")
Parameter| Type| Description  
---|---|---  
inputCurrency| `address`| Input currency that will be swapped for output currency.  
outputCurrency| `address or ETH`| Output currency that input currency will be swapped for.  
slippage| `number`| Max slippage to be used during transaction (in bips)  
exactAmount| `number`| The custom token amount to buy or sell.  
exactField| `string`| The field to set custom token amount for. Must be `input` or `output`.  
### Defaults[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#defaults "Direct link to Defaults")
ETH defaults as the input currency. When a different token is selected for either input or output ETH will default as the opposite selected currency.
### Constraints[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#constraints "Direct link to Constraints")
Addresses must be valid ERC20 addresses. Slippage and amount values must be valid numbers accepted by the exchange (or error will prevent from swapping). Slippage can 0, or within the range 10->9999 bips (which converts to 0%, 0.01%->99%)
When selecting ETH as the output currency a user must also choose an inputCurrency that is not ETH (to prevent ETH being populated in both fields)
### Setting Amounts[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#setting-amounts "Direct link to Setting Amounts")
Two parameters, exactField and exactAmount can be used to set specific token amounts to be sold or bought. Both fields must be set in the URL or there will be no effect on the settings.
### Example Usage[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-1 "Direct link to Example Usage")
`https://app.uniswap.org/#/swap?exactField=input&exactAmount=10&inputCurrency=0x0F5D2fB29fb7d3CFeE444a200298f468908cC942?use=v1`
## Send Page[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#send-page "Direct link to Send Page")
The send page has the same options available as the Swap page, plus one additional parameter, `recipient`.
Parameter| Type| Description  
---|---|---  
recipient| `address`| Address of the recipient of a send transaction.  
### Example Usage[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-2 "Direct link to Example Usage")
`https://app.uniswap.org/#/send?recipient=0x74Aa01d162E6dC6A657caC857418C403D48E2D77?use=v1`
## Pool Page[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#pool-page "Direct link to Pool Page")
The Pool page is made up of 3 subroutes: `add-liquidity`, `remove-liquidity`, `create-exchange`.
### Add Liquidity[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#add-liquidity "Direct link to Add Liquidity")
Parameter| Type| Description  
---|---|---  
ethAmount| `number`| Amount of ETH to deposit into the pool.  
token| `address`| ERC20 address of the pool to add liquidity to.  
tokenAmount| `number`| Amount of the selected token to deposit into the pool.  
### Example Usage[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-3 "Direct link to Example Usage")
`https://app.uniswap.org/#/add-liquidity?ethAmount=2.34&token=0x42456D7084eacF4083f1140d3229471bbA2949A8&tokenAmount=300?use=v1`
## Remove Liquidity[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#remove-liquidity "Direct link to Remove Liquidity")
Parameter| Type| Description  
---|---|---  
poolTokenAddress| `address`| Pool to withdraw liquidity from. (Must be an ERC20 address with an existing exchange)  
poolTokenAmount| `number`| Amount of pool token to be withdrawn from liquidity pool.  
### Example Usage[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-4 "Direct link to Example Usage")
`https://app.uniswap.org/#/remove-liquidity?poolTokenAmount=1.23&use=v1`
## Create Exchange[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#create-exchange "Direct link to Create Exchange")
Parameter| Type| Description  
---|---|---  
tokenAddress| `address`| ERC20 token to create the exchange for. Must be valid ERC20 token for which there is no existing exchange.  
### Example Usage[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-5 "Direct link to Example Usage")
`https://app.uniswap.org/#/swap?use=v1&create-exchange?tokenAddress=0x0F5D2fB29fb7d3CFeE444a200298f468908cC942`
## Custom Routes[​](https://docs.uniswap.org/contracts/v1/guides/custom-linking#custom-routes "Direct link to Custom Routes")
Custom token routes can still be used in combination with URL parameters. URL parameters are higher in the settings hierarchy than custom routes.
An example using custom token route and URL parameters.
`https://app.uniswap.org/#/swap/0x0F5D2fB29fb7d3CFeE444a200298f468908cC942?exactField=input&exactAmount=10&use=v1`
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/04-custom-linking.md)
Was this helpful?
[PreviousTrade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)[NextIframe Integration](https://docs.uniswap.org/contracts/v1/guides/iframe-integration)
On this page
  * [Global](https://docs.uniswap.org/contracts/v1/guides/custom-linking#global)
    * [Theme Options](https://docs.uniswap.org/contracts/v1/guides/custom-linking#theme-options)
    * [Example Usage](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage)
  * [Swap Page](https://docs.uniswap.org/contracts/v1/guides/custom-linking#swap-page)
    * [Defaults](https://docs.uniswap.org/contracts/v1/guides/custom-linking#defaults)
    * [Constraints](https://docs.uniswap.org/contracts/v1/guides/custom-linking#constraints)
    * [Setting Amounts](https://docs.uniswap.org/contracts/v1/guides/custom-linking#setting-amounts)
    * [Example Usage](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-1)
  * [Send Page](https://docs.uniswap.org/contracts/v1/guides/custom-linking#send-page)
    * [Example Usage](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-2)
  * [Pool Page](https://docs.uniswap.org/contracts/v1/guides/custom-linking#pool-page)
    * [Add Liquidity](https://docs.uniswap.org/contracts/v1/guides/custom-linking#add-liquidity)
    * [Example Usage](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-3)
  * [Remove Liquidity](https://docs.uniswap.org/contracts/v1/guides/custom-linking#remove-liquidity)
    * [Example Usage](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-4)
  * [Create Exchange](https://docs.uniswap.org/contracts/v1/guides/custom-linking#create-exchange)
    * [Example Usage](https://docs.uniswap.org/contracts/v1/guides/custom-linking#example-usage-5)
  * [Custom Routes](https://docs.uniswap.org/contracts/v1/guides/custom-linking#custom-routes)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/04-custom-linking.md)
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
