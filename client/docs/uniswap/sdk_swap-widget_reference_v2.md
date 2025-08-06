# https://docs.uniswap.org/sdk/swap-widget/reference/v2

[Skip to main content](https://docs.uniswap.org/sdk/swap-widget/reference/v2#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
        * [Swaps](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
        * [Position Management](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
        * [Advanced](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
      * [Technical Reference](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
    * [v3 SDK](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
      * [Overview](https://docs.uniswap.org/sdk/swap-widget/overview)
      * [Guides](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
      * [Technical Reference](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
        * [API V2 Reference](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
        * [API V1 Reference (Deprecated)](https://docs.uniswap.org/sdk/swap-widget/reference/v1)
    * [web3-react](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
    * [Core SDK](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
    * [v2 SDK](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
    * [v1 SDK](https://docs.uniswap.org/sdk/swap-widget/reference/v2)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Swap Widget
  * Technical Reference
  * [API V2 Reference](https://docs.uniswap.org/sdk/swap-widget/reference/v2)


On this page
# Swap Widget API V2 Reference
## Recommended Parameters[​](https://docs.uniswap.org/sdk/swap-widget/reference/v2#recommended-parameters "Direct link to Recommended Parameters")
Prop Name| Prop Type| Default Value| Description  
---|---|---|---  
`provider`| `JsonRpcProvider` or `Eip1193Provider`| `undefined`| An [EIP-1193](https://eips.ethereum.org/EIPS/eip-1193) provider. See [Web3 provider](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#web3-provider).  
`jsonRpcUrlMap`| `{ [chainId: number]: string[] }`| `JSON_RPC_FALLBACK_ENDPOINTS`| Mapping of your JSON-RPC endpoint URLs indexed by chainId, used to provide trade quotes prior to the user connecting a wallet. If none are provided for a chain, the widget will fallback to public JSON-RPC endpoints, which are unreliable and rate-limited. See [JSON-RPC Endpoints](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#json-rpc-endpoint).  
## Optional Parameters[​](https://docs.uniswap.org/sdk/swap-widget/reference/v2#optional-parameters "Direct link to Optional Parameters")
Prop Name| Prop Type| Default Value| Description  
---|---|---|---  
`convenienceFee`| `number`| `undefined`| Optionally, you may charge a convenience fee on top of swaps executed through your web app. The allowed range is 1 to 100 basis points paid in the output token of a swap, consistent with the Uniswap v3 Periphery contract.  
`convenienceFeeRecipient`| `{[chainId: number]: string}`| `undefined`| The address to receive the convenience fee on each network. Required if `convenienceFee` is provided.  
`defaultChainId`| `number`| `1`| You may specify which chainId you want to prompt a user to connect their wallet to. [See a list of all chains supported on widget.](https://github.com/Uniswap/widgets/blob/main/src/constants/chains.ts#L4)  
`defaultInputTokenAddress`| `{[chainId: number]: string}`| `string` or `'NATIVE'`| Address of the token to be selected by default in the input field (e.g. USDC) for each network chain ID. If left empty the widget will use the native token of the connected chain as default. This can be explicitly defined by the special string `'NATIVE'`. For convenience you may pass a single string instead of a `chainId` mapping. In this case, the widget will assume that string corresponds to an L1 Ethereum address with `chaindId=1`. Any addresses provided in this parameter must be included in the `tokenList`.  
`defaultInputAmount`| `number`| `0`| Default amount for the input field (e.g. 1 ETH). This value will respect the decimals of the `defaultInputTokenAddress`. This parameter is valid only if `defaultInputTokenAddress` is also set. This parameter is mutually exclusive with `defaultOutputTokenAmount`, so you may set only one of `defaultInputTokenAmount` and `defaultOutputTokenAmount`.  
`defaultOutputTokenAddress`| `{[chainId: number]: string}`| `string` or `undefined`| Address of the token to be selected by default in the input field (e.g. USDC) for each network chain ID. None if left empty. Any addresses provided in this parameter must be included in the `tokenList`.  
`defaultOutputAmount`| `number`| `0`| Default amount for the input field (e.g. 100 USDC). This value will respect the decimals of the `defaultOutputTokenAddress`. This parameter is mutually exclusive with `defaultInputTokenAmount`, so you may set only one `of defaultInputTokenAmount and` `defaultOutputTokenAmount`.  
`hideConnectionUI`| `boolean`| `false`| Hide the widget's built-in wallet connection UI, including the connected account chip & 'Connect wallet to swap' button.  
`locale`| `SupportedLocale`| `en-US`| Specifies an explicit locale to use for the widget interface. This can be set to one of the values exported by the library in [`SUPPORTED_LOCALES`](https://github.com/Uniswap/widgets/blob/main/src/constants/locales.ts).  
`onConnectWalletClick`| `() => void` or `() => Promise<boolean>`| `undefined`| If passed, allows you to add custom behavior when the user clicks on the 'Connect your wallet to swap' button. To manage displaying the widget's built-in wallet connection modal, return a resolved promise with `resolve(true/false)`.  
`onSwitchChain`| `(addChainParameter: AddEthereumChainParameter) => void` or `Promise<void>`| `undefined`| An integration hook called when the user tries to switch chains. If the hook returns a Promise, it is assumed the integrator is attempting to switch the chain, and no further attempts will be made. If that Promise rejects, the error will be ignored so as not to crash the widget.  
`onError`| `ErrorHandler`| `undefined`| An error handler which receives any Javascript errors that occur in the widget. This can be used for collecting error metrics.  
`onReviewSwapClick`| `() => void` or `() => Promise<boolean>`| `undefined`| If passed, allows you to add custom behavior when the user clicks on the 'review swap' button. To manage progression to the review screen (i.e. to add a pre-swap warning speedbump), return a resolved promise with `resolve(true/false)`.  
`onTokenSelectorClick`| `(f: Field) => void | (f: Field) => Promise<boolean | void>`| `undefined`| A click handler fired with the selected `Field` (`'INPUT'|'OUTPUT'`) when the user clicks on a token selector dropdown. To manage progression to the native token selector view (i.e. to utilize your own external token selector UI), return a resolved promise with resolve(true/false).  
`onTxFail`| `(error: Error, data: any) => void`| `undefined`| An error handler which receives error data for on-chain transaction failures. Does not include when user cancels a transaction or if a transaction isn't able to be submitted.  
`onTxSubmit`| `(txHash: string, data: any) => void`| `undefined`| A handler that receives the transaction hash and related data when a user submits a transaction.  
`onTxSuccess`| `(txHash: string, data: any) => void`| `undefined`| A handler that receives the transaction hash and related data when a transaction succeeds on-chain.  
`routerUrl`| `string`| `undefined`| Optionally provide a base URL to your own hosted instance of the Uniswap Router API. If none is provided, the optimal trade route is computed by running the @uniswap/smart-order-router package locally in the browser; this can take a few seconds to load & is slower. You also may be able to find more optimal routes using the Router API! See more about [deploying the Router API](https://github.com/Uniswap/routing-api#deploying-the-api).  
`theme`| `Theme`| `lightTheme`| Specifies a custom theme (colors, font, and border radii). See [Customizing the Theme](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#customizing-theme).  
`tokenList`| `string`| `TokenInfo[]`| Specifies the set of tokens that appear by default in the token selector list. Accepts either a URI of a token list as defined by the Token Lists standard, or an inline array of tokens. If none is provided, the Uniswap Labs default token list will be used. See [Customizing the Default Token List](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#customizing-default-token-list).  
`width`| `number` or `string`| `360`| Specifies the width of the widget. If specified as a number, this is in pixels; otherwise, it is interpreted as a CSS `<length>` data type. Recommended width is 360px. Minimum width is 300px. See [Customizing the Width](https://docs.uniswap.org/sdk/swap-widget/guides/getting-started#customizing-width).  
`brandedFooter`| `boolean`| `true`| Enables the "Powered by Uniswap" footer at the bottom of the widget.  
`dialog`| `HTMLDivElement`| `undefined`| Specifies the element to portal widget dialogs (e.g. Summary, Transaction dialogs) into.  
`dialogOptions`| `DialogOptions`| `undefined`| Specifies more custom dialog behavior, like transition animations.  
## Subscribing to Events[​](https://docs.uniswap.org/sdk/swap-widget/reference/v2#subscribing-to-events "Direct link to Subscribing to Events")
During the lifecycle of the swap widget, most of the events you will need are available on the web3 provider. For example, the below snippet shows how to listen for events when the wallet account changes or a new wallet connects. You can see more event examples in the [MetaMask](https://docs.metamask.io/guide/ethereum-provider.html) docs.
```
// Subscribe to messagesinterfaceProviderMessage{type: string;data: unknown;}ethereum.on('message',handler:(message:ProviderMessage)=>void);
```

Questions?
Join the [Discord channel](https://discord.com/channels/597638925346930701/941447445844463676) to ask questions and get support from the Uniswap community.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/swap-widget/reference/v2.md)
Was this helpful?
[PreviousExample Themes](https://docs.uniswap.org/sdk/swap-widget/guides/examples)[NextAPI V1 Reference (Deprecated)](https://docs.uniswap.org/sdk/swap-widget/reference/v1)
On this page
  * [Recommended Parameters](https://docs.uniswap.org/sdk/swap-widget/reference/v2#recommended-parameters)
  * [Optional Parameters](https://docs.uniswap.org/sdk/swap-widget/reference/v2#optional-parameters)
  * [Subscribing to Events](https://docs.uniswap.org/sdk/swap-widget/reference/v2#subscribing-to-events)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/swap-widget/reference/v2.md)
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
