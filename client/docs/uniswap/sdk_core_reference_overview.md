# https://docs.uniswap.org/sdk/core/reference/overview

[Skip to main content](https://docs.uniswap.org/sdk/core/reference/overview#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/sdk/core/overview)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/overview)
        * [Overview](https://docs.uniswap.org/sdk/core/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
        * [enums](https://docs.uniswap.org/sdk/core/reference/enums/ChainId)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Core SDK
  * Technical Reference
  * [Overview](https://docs.uniswap.org/sdk/core/reference/overview)


On this page
# Overview
## Table of contents[​](https://docs.uniswap.org/sdk/core/reference/overview#table-of-contents "Direct link to Table of contents")
### Enumerations[​](https://docs.uniswap.org/sdk/core/reference/overview#enumerations "Direct link to Enumerations")
  * [Rounding](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)
  * [ChainId](https://docs.uniswap.org/sdk/core/reference/enums/ChainId)
  * [NativeCurrencyName](https://docs.uniswap.org/sdk/core/reference/enums/NativeCurrencyName)
  * [TradeType](https://docs.uniswap.org/sdk/core/reference/enums/TradeType)


### Classes[​](https://docs.uniswap.org/sdk/core/reference/overview#classes "Direct link to Classes")
  * [CurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
  * [Ether](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
  * [Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
  * [NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
  * [Percent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
  * [Price](https://docs.uniswap.org/sdk/core/reference/classes/Price)
  * [Token](https://docs.uniswap.org/sdk/core/reference/classes/Token)


### Type Aliases[​](https://docs.uniswap.org/sdk/core/reference/overview#type-aliases "Direct link to Type Aliases")
  * [BigintIsh](https://docs.uniswap.org/sdk/core/reference/overview#bigintish)
  * [Currency](https://docs.uniswap.org/sdk/core/reference/overview#currency)


### Variables[​](https://docs.uniswap.org/sdk/core/reference/overview#variables "Direct link to Variables")
  * [MaxUint256](https://docs.uniswap.org/sdk/core/reference/overview#maxuint256)
  * [WETH9](https://docs.uniswap.org/sdk/core/reference/overview#weth9)


### Functions[​](https://docs.uniswap.org/sdk/core/reference/overview#functions "Direct link to Functions")
  * [computePriceImpact](https://docs.uniswap.org/sdk/core/reference/overview#computepriceimpact)
  * [sortedInsert](https://docs.uniswap.org/sdk/core/reference/overview#sortedinsert)
  * [sqrt](https://docs.uniswap.org/sdk/core/reference/overview#sqrt)
  * [validateAndParseAddress](https://docs.uniswap.org/sdk/core/reference/overview#validateandparseaddress)


## Type Aliases[​](https://docs.uniswap.org/sdk/core/reference/overview#type-aliases-1 "Direct link to Type Aliases")
### BigintIsh[​](https://docs.uniswap.org/sdk/core/reference/overview#bigintish "Direct link to BigintIsh")
Ƭ **BigintIsh** : `JSBI` | `string` | `number`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/overview#defined-in "Direct link to Defined in")
[constants.ts:24](https://github.com/Uniswap/sdk-core/blob/9997e88/src/constants.ts#L24)
### Currency[​](https://docs.uniswap.org/sdk/core/reference/overview#currency "Direct link to Currency")
Ƭ **Currency** : [`NativeCurrency`](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency) | [`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/overview#defined-in-1 "Direct link to Defined in")
[entities/currency.ts:4](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/currency.ts#L4)
## Variables[​](https://docs.uniswap.org/sdk/core/reference/overview#variables-1 "Direct link to Variables")
### MaxUint256[​](https://docs.uniswap.org/sdk/core/reference/overview#maxuint256 "Direct link to MaxUint256")
• `Const` **MaxUint256** : `default`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/overview#defined-in-2 "Direct link to Defined in")
[constants.ts:37](https://github.com/Uniswap/sdk-core/blob/9997e88/src/constants.ts#L37)
### WETH9[​](https://docs.uniswap.org/sdk/core/reference/overview#weth9 "Direct link to WETH9")
• `Const` **WETH9** : `Object`
Known WETH9 implementation addresses, used in our implementation of Ether#wrapped
#### Index signature[​](https://docs.uniswap.org/sdk/core/reference/overview#index-signature "Direct link to Index signature")
▪ [chainId: `number`]: [`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/overview#defined-in-3 "Direct link to Defined in")
[entities/weth9.ts:6](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/weth9.ts#L6)
## Functions[​](https://docs.uniswap.org/sdk/core/reference/overview#functions-1 "Direct link to Functions")
### computePriceImpact[​](https://docs.uniswap.org/sdk/core/reference/overview#computepriceimpact "Direct link to computePriceImpact")
▸ **computePriceImpact** <`TBase`, `TQuote`>(`midPrice`, `inputAmount`, `outputAmount`): [`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
Returns the percent difference between the mid price and the execution price, i.e. price impact.
#### Type parameters[​](https://docs.uniswap.org/sdk/core/reference/overview#type-parameters "Direct link to Type parameters")
Name| Type  
---|---  
`TBase`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
`TQuote`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/overview#parameters "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`midPrice`| [`Price`](https://docs.uniswap.org/sdk/core/reference/classes/Price)<`TBase`, `TQuote`>| mid price before the trade  
`inputAmount`| [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`TBase`>| the input amount of the trade  
`outputAmount`| [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`TQuote`>| the output amount of the trade  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/overview#returns "Direct link to Returns")
[`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/overview#defined-in-4 "Direct link to Defined in")
[utils/computePriceImpact.ts:9](https://github.com/Uniswap/sdk-core/blob/9997e88/src/utils/computePriceImpact.ts#L9)
### sortedInsert[​](https://docs.uniswap.org/sdk/core/reference/overview#sortedinsert "Direct link to sortedInsert")
▸ **sortedInsert** <`T`>(`items`, `add`, `maxSize`, `comparator`): `T` | `null`
#### Type parameters[​](https://docs.uniswap.org/sdk/core/reference/overview#type-parameters-1 "Direct link to Type parameters")
Name  
---  
`T`  
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/overview#parameters-1 "Direct link to Parameters")
Name| Type  
---|---  
`items`| `T`[]  
`add`| `T`  
`maxSize`| `number`  
`comparator`| (`a`: `T`, `b`: `T`) => `number`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/overview#returns-1 "Direct link to Returns")
`T` | `null`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/overview#defined-in-5 "Direct link to Defined in")
[utils/sortedInsert.ts:5](https://github.com/Uniswap/sdk-core/blob/9997e88/src/utils/sortedInsert.ts#L5)
### sqrt[​](https://docs.uniswap.org/sdk/core/reference/overview#sqrt "Direct link to sqrt")
▸ **sqrt**(`value`): `JSBI`
Computes floor(sqrt(value))
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/overview#parameters-2 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`value`| `default`| the value for which to compute the square root, rounded down  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/overview#returns-2 "Direct link to Returns")
`JSBI`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/overview#defined-in-6 "Direct link to Defined in")
[utils/sqrt.ts:14](https://github.com/Uniswap/sdk-core/blob/9997e88/src/utils/sqrt.ts#L14)
### validateAndParseAddress[​](https://docs.uniswap.org/sdk/core/reference/overview#validateandparseaddress "Direct link to validateAndParseAddress")
▸ **validateAndParseAddress**(`address`): `string`
Validates an address and returns the parsed (checksummed) version of that address
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/overview#parameters-3 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`address`| `string`| the unchecksummed hex address  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/overview#returns-3 "Direct link to Returns")
`string`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/overview#defined-in-7 "Direct link to Defined in")
[utils/validateAndParseAddress.ts:7](https://github.com/Uniswap/sdk-core/blob/9997e88/src/utils/validateAndParseAddress.ts#L7)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/overview.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/sdk/core/overview)[NextCurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
On this page
  * [Table of contents](https://docs.uniswap.org/sdk/core/reference/overview#table-of-contents)
    * [Enumerations](https://docs.uniswap.org/sdk/core/reference/overview#enumerations)
    * [Classes](https://docs.uniswap.org/sdk/core/reference/overview#classes)
    * [Type Aliases](https://docs.uniswap.org/sdk/core/reference/overview#type-aliases)
    * [Variables](https://docs.uniswap.org/sdk/core/reference/overview#variables)
    * [Functions](https://docs.uniswap.org/sdk/core/reference/overview#functions)
  * [Type Aliases](https://docs.uniswap.org/sdk/core/reference/overview#type-aliases-1)
    * [BigintIsh](https://docs.uniswap.org/sdk/core/reference/overview#bigintish)
    * [Currency](https://docs.uniswap.org/sdk/core/reference/overview#currency)
  * [Variables](https://docs.uniswap.org/sdk/core/reference/overview#variables-1)
    * [MaxUint256](https://docs.uniswap.org/sdk/core/reference/overview#maxuint256)
    * [WETH9](https://docs.uniswap.org/sdk/core/reference/overview#weth9)
  * [Functions](https://docs.uniswap.org/sdk/core/reference/overview#functions-1)
    * [computePriceImpact](https://docs.uniswap.org/sdk/core/reference/overview#computepriceimpact)
    * [sortedInsert](https://docs.uniswap.org/sdk/core/reference/overview#sortedinsert)
    * [sqrt](https://docs.uniswap.org/sdk/core/reference/overview#sqrt)
    * [validateAndParseAddress](https://docs.uniswap.org/sdk/core/reference/overview#validateandparseaddress)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/overview.md)
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
