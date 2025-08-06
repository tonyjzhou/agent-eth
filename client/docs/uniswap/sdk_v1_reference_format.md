# https://docs.uniswap.org/sdk/v1/reference/format

[Skip to main content](https://docs.uniswap.org/sdk/v1/reference/format#__docusaurus_skipToContent_fallback)
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
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
      * [Overview](https://docs.uniswap.org/sdk/v1/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/guides/getting-started)
        * [Getting Started](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Data](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Computation](https://docs.uniswap.org/sdk/v1/reference/computation)
        * [Format](https://docs.uniswap.org/sdk/v1/reference/format)
        * [Orchestration](https://docs.uniswap.org/sdk/v1/reference/orchestration)
        * [Transact](https://docs.uniswap.org/sdk/v1/reference/transact)
        * [Constants](https://docs.uniswap.org/sdk/v1/reference/constants)
        * [Types](https://docs.uniswap.org/sdk/v1/reference/types)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v1 SDK
  * Technical Reference
  * [Format](https://docs.uniswap.org/sdk/v1/reference/format)


On this page
# formatSignificant
This function formats values to a specified number of significant digits.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/format#function-signature "Direct link to Function Signature")
```
exportfunctionformatSignificant(bigNumberish: BigNumberish, options?: FormatSignificantOptions):string
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/format#input-parameters "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
bigNumberish| `BigNumberish`| The value to be formatted.  
options?| `FormatSignificantOptions`| Formatting options.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/format#example-usage "Direct link to Example Usage")
```
const formatted:string=formatSignificant('123456',{ significantDigits:3})// 1.23
```

# formatSignificantDecimals
This function formats token and ethereum values to a specified number of significant digits.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/format#function-signature-1 "Direct link to Function Signature")
```
exportfunctionformatSignificantDecimals( bigNumberish: BigNumberish, decimals:number, options?: FormatSignificantOptions):string
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/format#input-parameters-1 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
bigNumberish| `BigNumberish`| The value to be formatted.  
decimals| `number`| The decimals of the passed value.  
options?| `FormatSignificantOptions`| Formatting options.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/format#example-usage-1 "Direct link to Example Usage")
```
const formatted:string=formatSignificantDecimals('1234560000000000000',18,{ significantDigits:3,})// 1.23
```

# formatFixed
This function formats values to a specified number of decimal places.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/format#function-signature-2 "Direct link to Function Signature")
```
exportfunctionformatFixed(bigNumberish: BigNumberish, options?: FormatFixedOptions):string
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/format#input-parameters-2 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
bigNumberish| `BigNumberish`| The value to be formatted.  
options?| `FormatFixedOptions`| Formatting options.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/format#example-usage-2 "Direct link to Example Usage")
```
const formatted:string=formatFixed('1.2345',{ decimalPlaces:2})// 1.23
```

# formatFixedDecimals
This function formats token and ethereum values to a specified number of decimal places.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/format#function-signature-3 "Direct link to Function Signature")
```
exportfunctionformatFixedDecimals(bigNumberish: BigNumberish, decimals:number, options?: FormatFixedOptions):string
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/format#input-parameters-3 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
bigNumberish| `BigNumberish`| The value to be formatted.  
decimals| `number`| The decimals of the passed value.  
options?| `FormatFixedOptions`| Formatting options.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/format#example-usage-3 "Direct link to Example Usage")
```
const formatted:string=formatFixedDecimals('1234560000000000000',18,{ decimalPlaces:2,})// 1.23
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/04-format.md)
Was this helpful?
[PreviousComputation](https://docs.uniswap.org/sdk/v1/reference/computation)[NextOrchestration](https://docs.uniswap.org/sdk/v1/reference/orchestration)
On this page
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/format#function-signature)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/format#input-parameters)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/format#example-usage)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/format#function-signature-1)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/format#input-parameters-1)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/format#example-usage-1)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/format#function-signature-2)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/format#input-parameters-2)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/format#example-usage-2)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/format#function-signature-3)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/format#input-parameters-3)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/format#example-usage-3)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/04-format.md)
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
