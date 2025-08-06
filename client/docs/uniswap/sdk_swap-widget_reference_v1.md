# https://docs.uniswap.org/sdk/swap-widget/reference/v1

[Skip to main content](https://docs.uniswap.org/sdk/swap-widget/reference/v1#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/sdk/swap-widget/overview)
      * [Guides](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
        * [API V2 Reference](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
        * [API V1 Reference (Deprecated)](https://docs.uniswap.org/sdk/swap-widget/reference/v1)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Swap Widget
  * Technical Reference
  * [API V1 Reference (Deprecated)](https://docs.uniswap.org/sdk/swap-widget/reference/v1)


On this page
# Swap Widget API V1 Reference (Deprecated)
## Required Parameters[​](https://docs.uniswap.org/sdk/swap-widget/reference/v1#required-parameters "Direct link to Required Parameters")
Prop Name| Prop Type| Default Value| Description  
---|---|---|---  
`jsonRpcEndpoint`| `string`| `undefined`| URI of your JSON-RPC endpoint. Strongly recommended in order to provide trade quotes prior to the user connecting a wallet. If none is provided, the widget will be completely disabled until the user connects a wallet. Once a wallet is connected, the widget will use the wallet’s JSON-RPC. See [Understanding the Swap Widget States](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#understanding-widget-states).  
`provider`| `any`| `undefined`| An [EIP-1193](https://eips.ethereum.org/EIPS/eip-1193) provider. This is required to swap.  
## Optional Parameters[​](https://docs.uniswap.org/sdk/swap-widget/reference/v1#optional-parameters "Direct link to Optional Parameters")
Prop Name| Prop Type| Default Value| Description  
---|---|---|---  
`convenienceFee`| `number`| `undefined`| Optionally, you may charge a convenience fee on top of swaps executed through your web app. The allowed range is 1 to 100 basis points (inclusive of 100) consistent with the Uniswap v3 Periphery contract.  
`convenienceFeeRecipient`| `{[chainId: number]: string}`| `undefined`| The address to receive the convenience fee on each network. Required if `convenienceFee` is provided.  
`defaultInputTokenAddress`| `{[chainId: number]: string}`| `string` or `'NATIVE'`| Address of the token to be selected by default in the input field (e.g. USDC) for each network chain ID. If left empty the widget will use the native token of the connected chain as default. This can be explicitly defined by the special string `'NATIVE'`. For convenience you may pass a single string instead of a `chainId` mapping. In this case, the widget will assume that string corresponds to an L1 Ethereum address with `chaindId=1`. Any addresses provided in this parameter must be included in the `tokenList`.  
`defaultInputAmount`| `number`| `0`| Default amount for the input field (e.g. 1 ETH). This value will respect the decimals of the `defaultInputTokenAddress`. This parameter is valid only if `defaultInputTokenAddress` is also set. This parameter is mutually exclusive with `defaultOutputAmount`, so you may set only one of `defaultInputAmount` and `defaultOutputAmount`.  
`defaultOutputTokenAddress`| `{[chainId: number]: string}`| `string` or `undefined`| Address of the token to be selected by default in the input field (e.g. USDC) for each network chain ID. None if left empty. Any addresses provided in this parameter must be included in the `tokenList`.  
`defaultOutputAmount`| `number`| `0`| Default amount for the input field (e.g. 100 USDC). This value will respect the decimals of the `defaultOutputTokenAddress`. This parameter is mutually exclusive with `defaultInputAmount`, so you may set only one of `defaultInputAmount` and `defaultOutputAmount`.  
`locale`| `SupportedLocale`| `en-US`| Specifies an explicit locale to use for the widget interface. This can be set to one of the values exported by the library in [`SUPPORTED_LOCALES`](https://github.com/Uniswap/widgets/blob/main/src/constants/locales.ts).  
`onConnectWallet`| `() => void`| `undefined`| If passed, the “Connect your wallet” message will be clickable, and clicking it will trigger this handler function. This can be used to trigger your own wallet connection flow from the widget.  
`onError`| `ErrorHandler`| `undefined`| An error handler which receives any errors that occur in the widget. This can be used for collecting error metrics.  
`theme`| `Theme`| `lightTheme`| Specifies a custom theme (colors, font, and border radii). See [Customizing the Theme](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#customizing-theme).  
`tokenList`| `string`| `TokenInfo[]`| Specifies the set of tokens that appear by default in the token selector list. Accepts either a URI of a token list as defined by the Token Lists standard, or an inline array of tokens. If none is provided, the Uniswap Labs default token list will be used. See [Customizing the Default Token List](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#customizing-default-token-list).  
`width`| `number` or `string`| `360`| Specifies the width of the widget. If specified as a number, this is in pixels; otherwise, it is interpreted as a CSS `<length>` data type. Recommended width is 360px. Minimum width is 270px. See [Customizing the Width](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#customizing-width).  
## Subscribing to Events[​](https://docs.uniswap.org/sdk/swap-widget/reference/v1#subscribing-to-events "Direct link to Subscribing to Events")
During the lifecycle of the swap widget, most of the events you will need are available on the web3 provider. For example, the below snippet shows how to listen for events when the wallet account changes or a new wallet connects. You can see more event examples in the [MetaMask](https://docs.metamask.io/guide/ethereum-provider.html) docs.
```
// Subscribe to messagesinterfaceProviderMessage{type: string;data: unknown;}ethereum.on('message',handler:(message:ProviderMessage)=>void);
```

Questions?
Join the [Discord channel](https://discord.com/channels/597638925346930701/941447445844463676) to ask questions and get support from the Uniswap community.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/swap-widget/reference/v1.md)
Was this helpful?
[PreviousAPI V2 Reference](https://docs.uniswap.org/sdk/swap-widget/reference/v2)[NextOverview](https://docs.uniswap.org/sdk/web3-react/overview)
On this page
  * [Required Parameters](https://docs.uniswap.org/sdk/swap-widget/reference/v1#required-parameters)
  * [Optional Parameters](https://docs.uniswap.org/sdk/swap-widget/reference/v1#optional-parameters)
  * [Subscribing to Events](https://docs.uniswap.org/sdk/swap-widget/reference/v1#subscribing-to-events)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/swap-widget/reference/v1.md)
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
