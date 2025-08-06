# https://docs.uniswap.org/sdk/core/reference/classes/Fraction

[Skip to main content](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#__docusaurus_skipToContent_fallback)
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
  * [Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)


On this page
[@uniswap/sdk-core](https://docs.uniswap.org/sdk/core/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/core/reference/modules.md) / Fraction
# Class: Fraction
## Hierarchy[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#hierarchy "Direct link to Hierarchy")
  * **`Fraction`**
↳ [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
↳ [`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
↳ [`Price`](https://docs.uniswap.org/sdk/core/reference/classes/Price)


## Table of contents[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructor)


### Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#properties "Direct link to Properties")
  * [denominator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#denominator)
  * [numerator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#numerator)


### Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#accessors "Direct link to Accessors")
  * [asFraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#asfraction)
  * [quotient](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#quotient)
  * [remainder](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#remainder)


### Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#methods "Direct link to Methods")
  * [add](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#add)
  * [divide](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#divide)
  * [equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#equalto)
  * [greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#greaterthan)
  * [invert](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#invert)
  * [lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#lessthan)
  * [multiply](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#multiply)
  * [subtract](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#subtract)
  * [toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tofixed)
  * [toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tosignificant)
  * [tryParseFraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tryparsefraction)


## Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructor "Direct link to constructor")
• **new Fraction**(`numerator`, `denominator?`)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters "Direct link to Parameters")
Name| Type  
---|---  
`numerator`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)  
`denominator`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)  
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in "Direct link to Defined in")
[entities/fractions/fraction.ts:28](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L28)
## Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#properties-1 "Direct link to Properties")
### denominator[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#denominator "Direct link to denominator")
• `Readonly` **denominator** : `default`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-1 "Direct link to Defined in")
[entities/fractions/fraction.ts:26](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L26)
### numerator[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#numerator "Direct link to numerator")
• `Readonly` **numerator** : `default`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-2 "Direct link to Defined in")
[entities/fractions/fraction.ts:25](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L25)
## Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#accessors-1 "Direct link to Accessors")
### asFraction[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#asfraction "Direct link to asFraction")
• `get` **asFraction**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
Helper method for converting any super class back to a fraction
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-3 "Direct link to Defined in")
[entities/fractions/fraction.ts:154](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L154)
### quotient[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#quotient "Direct link to quotient")
• `get` **quotient**(): `default`
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-1 "Direct link to Returns")
`default`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-4 "Direct link to Defined in")
[entities/fractions/fraction.ts:42](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L42)
### remainder[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#remainder "Direct link to remainder")
• `get` **remainder**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-2 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-5 "Direct link to Defined in")
[entities/fractions/fraction.ts:47](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L47)
## Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#methods-1 "Direct link to Methods")
### add[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#add "Direct link to add")
▸ **add**(`other`): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-1 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-3 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-6 "Direct link to Defined in")
[entities/fractions/fraction.ts:55](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L55)
### divide[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#divide "Direct link to divide")
▸ **divide**(`other`): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-2 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-4 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-7 "Direct link to Defined in")
[entities/fractions/fraction.ts:115](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L115)
### equalTo[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#equalto "Direct link to equalTo")
▸ **equalTo**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-3 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-5 "Direct link to Returns")
`boolean`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-8 "Direct link to Defined in")
[entities/fractions/fraction.ts:91](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L91)
### greaterThan[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#greaterthan "Direct link to greaterThan")
▸ **greaterThan**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-4 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-6 "Direct link to Returns")
`boolean`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-9 "Direct link to Defined in")
[entities/fractions/fraction.ts:99](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L99)
### invert[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#invert "Direct link to invert")
▸ **invert**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-7 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-10 "Direct link to Defined in")
[entities/fractions/fraction.ts:51](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L51)
### lessThan[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#lessthan "Direct link to lessThan")
▸ **lessThan**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-5 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-8 "Direct link to Returns")
`boolean`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-11 "Direct link to Defined in")
[entities/fractions/fraction.ts:83](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L83)
### multiply[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#multiply "Direct link to multiply")
▸ **multiply**(`other`): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-6 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-9 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-12 "Direct link to Defined in")
[entities/fractions/fraction.ts:107](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L107)
### subtract[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#subtract "Direct link to subtract")
▸ **subtract**(`other`): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-7 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-10 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-13 "Direct link to Defined in")
[entities/fractions/fraction.ts:69](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L69)
### toFixed[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tofixed "Direct link to toFixed")
▸ **toFixed**(`decimalPlaces`, `format?`, `rounding?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-8 "Direct link to Parameters")
Name| Type| Default value  
---|---|---  
`decimalPlaces`| `number`| `undefined`  
`format`| `object`| `undefined`  
`rounding`| [`Rounding`](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)| `Rounding.ROUND_HALF_UP`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-11 "Direct link to Returns")
`string`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-14 "Direct link to Defined in")
[entities/fractions/fraction.ts:138](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L138)
### toSignificant[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tosignificant "Direct link to toSignificant")
▸ **toSignificant**(`significantDigits`, `format?`, `rounding?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-9 "Direct link to Parameters")
Name| Type| Default value  
---|---|---  
`significantDigits`| `number`| `undefined`  
`format`| `object`| `undefined`  
`rounding`| [`Rounding`](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)| `Rounding.ROUND_HALF_UP`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-12 "Direct link to Returns")
`string`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-15 "Direct link to Defined in")
[entities/fractions/fraction.ts:123](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L123)
### tryParseFraction[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tryparsefraction "Direct link to tryParseFraction")
▸ `Static` `Private` **tryParseFraction**(`fractionish`): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#parameters-10 "Direct link to Parameters")
Name| Type  
---|---  
`fractionish`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#returns-13 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#defined-in-16 "Direct link to Defined in")
[entities/fractions/fraction.ts:33](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L33)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Fraction.md)
Was this helpful?
[PreviousEther](https://docs.uniswap.org/sdk/core/reference/classes/Ether)[NextNativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
On this page
  * [Hierarchy](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#hierarchy)
  * [Table of contents](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructors)
    * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#properties)
    * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#accessors)
    * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#methods)
  * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructor)
  * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#properties-1)
    * [denominator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#denominator)
    * [numerator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#numerator)
  * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#accessors-1)
    * [asFraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#asfraction)
    * [quotient](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#quotient)
    * [remainder](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#remainder)
  * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#methods-1)
    * [add](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#add)
    * [divide](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#divide)
    * [equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#equalto)
    * [greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#greaterthan)
    * [invert](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#invert)
    * [lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#lessthan)
    * [multiply](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#multiply)
    * [subtract](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#subtract)
    * [toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tofixed)
    * [toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tosignificant)
    * [tryParseFraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tryparsefraction)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Fraction.md)
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
