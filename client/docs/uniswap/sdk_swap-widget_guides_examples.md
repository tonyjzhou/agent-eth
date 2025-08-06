# https://docs.uniswap.org/sdk/swap-widget/guides/examples

[Skip to main content](https://docs.uniswap.org/sdk/swap-widget/guides/examples#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
        * [Swaps](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
        * [Position Management](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
        * [Advanced](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
      * [Technical Reference](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
    * [v3 SDK](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
      * [Overview](https://docs.uniswap.org/sdk/swap-widget/overview)
      * [Guides](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
        * [Getting Started](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started)
        * [Example Themes](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
      * [Technical Reference](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
    * [web3-react](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
    * [Core SDK](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
    * [v2 SDK](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
    * [v1 SDK](https://docs.uniswap.org/sdk/swap-widget/guides/examples)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Swap Widget
  * Guides
  * [Example Themes](https://docs.uniswap.org/sdk/swap-widget/guides/examples)


On this page
# Swap Widget Example Themes
The [Getting Started guide](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started) showed how to embed the swap widget and let your users trade tokens on the Uniswap Protocol without leaving your dApp.
Below you’ll find a few examples showing how you can [customize the widget theme](https://docs.uniswap.org/sdk/swap-widget/guides/examples#customizing-theme) to match the look and feel of your dApp. All of them can be integrated using the following code snippet where you can set your `theme`:
```
import{Theme,SwapWidget}from'@uniswap/widgets'import'@uniswap/widgets/dist/fonts.css'const theme:Theme={// Check out the theme examples below}functionApp(){<divclassName="Uniswap"><SwapWidgettheme={theme}/></div>}
```

## Theme Examples[​](https://docs.uniswap.org/sdk/swap-widget/guides/examples#theme-examples "Direct link to Theme Examples")
Copy any of the `theme` object below to start making your own custom appearance for the swap widget. You might also need the right CSS import for any custom fonts.
![Swap Widget Theme Example 1](https://docs.uniswap.org/img/widgets/swap-widget-theme-example-1.png)| ```
const theme:Theme={ primary:'#1F4A05', secondary:'#5F7D52', interactive:'#CBD6BA', container:'#D9ECD9', module:'#E9F7DF', accent:'#8E8B78', outline:'#CADDC2', dialog:'#FFF', fontFamily:'Nunito', borderRadius:0.8,}
```
  
---|---  
![Swap Widget Theme Example 2](https://docs.uniswap.org/img/widgets/swap-widget-theme-example-2.png)| ```
const theme:Theme={ primary:'#000', secondary:'#666', interactive:'#0089EC', container:'#FFF', module:'#E7E7E7', accent:'#3D3B31', outline:'#343D3A', dialog:'#FFF', fontFamily:'Verdana', borderRadius:0.8,}
```
  
![Swap Widget Theme Example 3](https://docs.uniswap.org/img/widgets/swap-widget-theme-example-3.png)| ```
const theme:Theme={ primary:'#FFF', secondary:'#A9A9A9', interactive:'#000', container:'#4E4E5A', module:'#222633', accent:'#71FF98', outline:'#CC1', dialog:'#000', fontFamily:'Josefin Sans', borderRadius:0.5,}
```
  
![Swap Widget Theme Example 4](https://docs.uniswap.org/img/widgets/swap-widget-theme-example-4.png)| ```
const theme:Theme={ primary:'#000', secondary:'#A9A9A9', interactive:'#1E4D3C', container:'#98D747', module:'#FFF', accent:'#FD5B00', outline:'#1E4D3C', dialog:'#000', fontFamily:'Inter', borderRadius:0.2,}
```
  
![Swap Widget Theme Example 5](https://docs.uniswap.org/img/widgets/swap-widget-theme-example-5.png)| ```
const theme:Theme={ primary:'#001D82', secondary:'#6677C1', interactive:'#005BAE', container:'#ABD6FE', module:'#FFF7FB', accent:'#FF7BC2', outline:'#ABD6FE', dialog:'#FFF', fontFamily:'Arvo', borderRadius:1,}
```
  
![Swap Widget Theme Example 6](https://docs.uniswap.org/img/widgets/swap-widget-theme-example-6.png)| ```
const theme:Theme={ primary:'#000', secondary:'#666', interactive:'#AFAFAF', container:'#DADADA', module:'#FFF', accent:'#0018F4', outline:'#000', dialog:'#FFF', fontFamily:'Comic Sans MS', borderRadius:0.2,}
```
  
Questions?
Join the [Discord channel](https://discord.com/channels/597638925346930701/941447445844463676) to ask questions and get support from the Uniswap community.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/swap-widget/guides/swap-widget-examples.mdx)
Was this helpful?
[PreviousGetting Started](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started)[NextAPI V2 Reference](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
On this page
  * [Theme Examples](https://docs.uniswap.org/sdk/swap-widget/guides/examples#theme-examples)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/swap-widget/guides/swap-widget-examples.mdx)
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
