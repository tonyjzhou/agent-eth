# https://docs.uniswap.org/sdk/core/reference/classes/Price

[Skip to main content](https://docs.uniswap.org/sdk/core/reference/classes/Price#__docusaurus_skipToContent_fallback)
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
          * [CurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
          * [Ether](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
          * [Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
          * [NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
          * [Percent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
          * [Price](https://docs.uniswap.org/sdk/core/reference/classes/Price)
          * [Token](https://docs.uniswap.org/sdk/core/reference/classes/Token)
        * [enums](https://docs.uniswap.org/sdk/core/reference/enums/ChainId)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Core SDK
  * Technical Reference
  * classes
  * [Price](https://docs.uniswap.org/sdk/core/reference/classes/Price)


On this page
[@uniswap/sdk-core](https://docs.uniswap.org/sdk/core/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/core/reference/modules.md) / Price
# Class: Price<TBase, TQuote>
## Type parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#type-parameters "Direct link to Type parameters")
Name| Type  
---|---  
`TBase`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
`TQuote`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
## Hierarchy[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#hierarchy "Direct link to Hierarchy")
  * [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
↳ **`Price`**


## Table of contents[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Price#constructor)


### Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#properties "Direct link to Properties")
  * [baseCurrency](https://docs.uniswap.org/sdk/core/reference/classes/Price#basecurrency)
  * [denominator](https://docs.uniswap.org/sdk/core/reference/classes/Price#denominator)
  * [numerator](https://docs.uniswap.org/sdk/core/reference/classes/Price#numerator)
  * [quoteCurrency](https://docs.uniswap.org/sdk/core/reference/classes/Price#quotecurrency)
  * [scalar](https://docs.uniswap.org/sdk/core/reference/classes/Price#scalar)


### Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#accessors "Direct link to Accessors")
  * [adjustedForDecimals](https://docs.uniswap.org/sdk/core/reference/classes/Price#adjustedfordecimals)
  * [asFraction](https://docs.uniswap.org/sdk/core/reference/classes/Price#asfraction)
  * [quotient](https://docs.uniswap.org/sdk/core/reference/classes/Price#quotient)
  * [remainder](https://docs.uniswap.org/sdk/core/reference/classes/Price#remainder)


### Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#methods "Direct link to Methods")
  * [add](https://docs.uniswap.org/sdk/core/reference/classes/Price#add)
  * [divide](https://docs.uniswap.org/sdk/core/reference/classes/Price#divide)
  * [equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Price#equalto)
  * [greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Price#greaterthan)
  * [invert](https://docs.uniswap.org/sdk/core/reference/classes/Price#invert)
  * [lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Price#lessthan)
  * [multiply](https://docs.uniswap.org/sdk/core/reference/classes/Price#multiply)
  * [quote](https://docs.uniswap.org/sdk/core/reference/classes/Price#quote)
  * [subtract](https://docs.uniswap.org/sdk/core/reference/classes/Price#subtract)
  * [toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Price#tofixed)
  * [toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Price#tosignificant)


## Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#constructor "Direct link to constructor")
• **new Price** <`TBase`, `TQuote`>(...`args`)
Construct a price, either with the base and quote currency amount, or the
#### Type parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#type-parameters-1 "Direct link to Type parameters")
Name| Type  
---|---  
`TBase`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
`TQuote`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters "Direct link to Parameters")
Name| Type  
---|---  
`...args`| [`TBase`, `TQuote`, [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish), [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)] | [{ `baseAmount`: [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`TBase`> ; `quoteAmount`: [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`TQuote`> }]  
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#overrides "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[constructor](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructor)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in "Direct link to Defined in")
[entities/fractions/price.ts:18](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L18)
## Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#properties-1 "Direct link to Properties")
### baseCurrency[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#basecurrency "Direct link to baseCurrency")
• `Readonly` **baseCurrency** : `TBase`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-1 "Direct link to Defined in")
[entities/fractions/price.ts:10](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L10)
### denominator[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#denominator "Direct link to denominator")
• `Readonly` **denominator** : `default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[denominator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#denominator)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-2 "Direct link to Defined in")
[entities/fractions/fraction.ts:26](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L26)
### numerator[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#numerator "Direct link to numerator")
• `Readonly` **numerator** : `default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-1 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[numerator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#numerator)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-3 "Direct link to Defined in")
[entities/fractions/fraction.ts:25](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L25)
### quoteCurrency[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#quotecurrency "Direct link to quoteCurrency")
• `Readonly` **quoteCurrency** : `TQuote`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-4 "Direct link to Defined in")
[entities/fractions/price.ts:11](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L11)
### scalar[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#scalar "Direct link to scalar")
• `Readonly` **scalar** : [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-5 "Direct link to Defined in")
[entities/fractions/price.ts:12](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L12)
## Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#accessors-1 "Direct link to Accessors")
### adjustedForDecimals[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#adjustedfordecimals "Direct link to adjustedForDecimals")
• `Private` `get` **adjustedForDecimals**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
Get the value scaled by decimals for formatting
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-6 "Direct link to Defined in")
[entities/fractions/price.ts:77](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L77)
### asFraction[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#asfraction "Direct link to asFraction")
• `get` **asFraction**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
Helper method for converting any super class back to a fraction
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-1 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-2 "Direct link to Inherited from")
Fraction.asFraction
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-7 "Direct link to Defined in")
[entities/fractions/fraction.ts:154](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L154)
### quotient[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#quotient "Direct link to quotient")
• `get` **quotient**(): `default`
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-2 "Direct link to Returns")
`default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-3 "Direct link to Inherited from")
Fraction.quotient
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-8 "Direct link to Defined in")
[entities/fractions/fraction.ts:42](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L42)
### remainder[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#remainder "Direct link to remainder")
• `get` **remainder**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-3 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-4 "Direct link to Inherited from")
Fraction.remainder
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-9 "Direct link to Defined in")
[entities/fractions/fraction.ts:47](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L47)
## Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#methods-1 "Direct link to Methods")
### add[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#add "Direct link to add")
▸ **add**(`other`): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-1 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-4 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-5 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[add](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#add)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-10 "Direct link to Defined in")
[entities/fractions/fraction.ts:55](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L55)
### divide[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#divide "Direct link to divide")
▸ **divide**(`other`): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-2 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-5 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-6 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[divide](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#divide)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-11 "Direct link to Defined in")
[entities/fractions/fraction.ts:115](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L115)
### equalTo[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#equalto "Direct link to equalTo")
▸ **equalTo**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-3 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-6 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-7 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#equalto)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-12 "Direct link to Defined in")
[entities/fractions/fraction.ts:91](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L91)
### greaterThan[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#greaterthan "Direct link to greaterThan")
▸ **greaterThan**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-4 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-7 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-8 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#greaterthan)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-13 "Direct link to Defined in")
[entities/fractions/fraction.ts:99](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L99)
### invert[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#invert "Direct link to invert")
▸ **invert**(): [`Price`](https://docs.uniswap.org/sdk/core/reference/classes/Price)<`TQuote`, `TBase`>
Flip the price, switching the base and quote currency
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-8 "Direct link to Returns")
[`Price`](https://docs.uniswap.org/sdk/core/reference/classes/Price)<`TQuote`, `TBase`>
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#overrides-1 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[invert](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#invert)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-14 "Direct link to Defined in")
[entities/fractions/price.ts:49](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L49)
### lessThan[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#lessthan "Direct link to lessThan")
▸ **lessThan**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-5 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-9 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-9 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#lessthan)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-15 "Direct link to Defined in")
[entities/fractions/fraction.ts:83](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L83)
### multiply[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#multiply "Direct link to multiply")
▸ **multiply** <`TOtherQuote`>(`other`): [`Price`](https://docs.uniswap.org/sdk/core/reference/classes/Price)<`TBase`, `TOtherQuote`>
Multiply the price by another price, returning a new price. The other price must have the same base currency as this price's quote currency
#### Type parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#type-parameters-2 "Direct link to Type parameters")
Name| Type  
---|---  
`TOtherQuote`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-6 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`other`| [`Price`](https://docs.uniswap.org/sdk/core/reference/classes/Price)<`TQuote`, `TOtherQuote`>| the other price  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-10 "Direct link to Returns")
[`Price`](https://docs.uniswap.org/sdk/core/reference/classes/Price)<`TBase`, `TOtherQuote`>
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#overrides-2 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[multiply](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#multiply)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-16 "Direct link to Defined in")
[entities/fractions/price.ts:57](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L57)
### quote[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#quote "Direct link to quote")
▸ **quote**(`currencyAmount`): [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`TQuote`>
Return the amount of quote currency corresponding to a given amount of the base currency
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-7 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`currencyAmount`| [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`TBase`>| the amount of base currency to quote against the price  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-11 "Direct link to Returns")
[`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`TQuote`>
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-17 "Direct link to Defined in")
[entities/fractions/price.ts:67](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L67)
### subtract[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#subtract "Direct link to subtract")
▸ **subtract**(`other`): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-8 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-12 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#inherited-from-10 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[subtract](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#subtract)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-18 "Direct link to Defined in")
[entities/fractions/fraction.ts:69](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L69)
### toFixed[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#tofixed "Direct link to toFixed")
▸ **toFixed**(`decimalPlaces?`, `format?`, `rounding?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-9 "Direct link to Parameters")
Name| Type| Default value  
---|---|---  
`decimalPlaces`| `number`| `4`  
`format?`| `object`| `undefined`  
`rounding?`| [`Rounding`](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)| `undefined`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-13 "Direct link to Returns")
`string`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#overrides-3 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tofixed)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-19 "Direct link to Defined in")
[entities/fractions/price.ts:85](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L85)
### toSignificant[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#tosignificant "Direct link to toSignificant")
▸ **toSignificant**(`significantDigits?`, `format?`, `rounding?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#parameters-10 "Direct link to Parameters")
Name| Type| Default value  
---|---|---  
`significantDigits`| `number`| `6`  
`format?`| `object`| `undefined`  
`rounding?`| [`Rounding`](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)| `undefined`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#returns-14 "Direct link to Returns")
`string`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#overrides-4 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tosignificant)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Price#defined-in-20 "Direct link to Defined in")
[entities/fractions/price.ts:81](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/price.ts#L81)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Price.md)
Was this helpful?
[PreviousPercent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)[NextToken](https://docs.uniswap.org/sdk/core/reference/classes/Token)
On this page
  * [Type parameters](https://docs.uniswap.org/sdk/core/reference/classes/Price#type-parameters)
  * [Hierarchy](https://docs.uniswap.org/sdk/core/reference/classes/Price#hierarchy)
  * [Table of contents](https://docs.uniswap.org/sdk/core/reference/classes/Price#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Price#constructors)
    * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Price#properties)
    * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Price#accessors)
    * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Price#methods)
  * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Price#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Price#constructor)
  * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Price#properties-1)
    * [baseCurrency](https://docs.uniswap.org/sdk/core/reference/classes/Price#basecurrency)
    * [denominator](https://docs.uniswap.org/sdk/core/reference/classes/Price#denominator)
    * [numerator](https://docs.uniswap.org/sdk/core/reference/classes/Price#numerator)
    * [quoteCurrency](https://docs.uniswap.org/sdk/core/reference/classes/Price#quotecurrency)
    * [scalar](https://docs.uniswap.org/sdk/core/reference/classes/Price#scalar)
  * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Price#accessors-1)
    * [adjustedForDecimals](https://docs.uniswap.org/sdk/core/reference/classes/Price#adjustedfordecimals)
    * [asFraction](https://docs.uniswap.org/sdk/core/reference/classes/Price#asfraction)
    * [quotient](https://docs.uniswap.org/sdk/core/reference/classes/Price#quotient)
    * [remainder](https://docs.uniswap.org/sdk/core/reference/classes/Price#remainder)
  * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Price#methods-1)
    * [add](https://docs.uniswap.org/sdk/core/reference/classes/Price#add)
    * [divide](https://docs.uniswap.org/sdk/core/reference/classes/Price#divide)
    * [equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Price#equalto)
    * [greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Price#greaterthan)
    * [invert](https://docs.uniswap.org/sdk/core/reference/classes/Price#invert)
    * [lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Price#lessthan)
    * [multiply](https://docs.uniswap.org/sdk/core/reference/classes/Price#multiply)
    * [quote](https://docs.uniswap.org/sdk/core/reference/classes/Price#quote)
    * [subtract](https://docs.uniswap.org/sdk/core/reference/classes/Price#subtract)
    * [toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Price#tofixed)
    * [toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Price#tosignificant)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Price.md)
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
