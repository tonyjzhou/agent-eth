# https://docs.uniswap.org/sdk/core/reference/classes/Percent

[Skip to main content](https://docs.uniswap.org/sdk/core/reference/classes/Percent#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
        * [Swaps](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
        * [Position Management](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
        * [Advanced](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
    * [v3 SDK](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
    * [Swap Widget](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
    * [web3-react](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
    * [Core SDK](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
      * [Overview](https://docs.uniswap.org/sdk/core/overview)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
        * [Overview](https://docs.uniswap.org/sdk/core/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
          * [CurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
          * [Ether](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
          * [Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
          * [NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
          * [Percent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
          * [Price](https://docs.uniswap.org/sdk/core/reference/classes/Price)
          * [Token](https://docs.uniswap.org/sdk/core/reference/classes/Token)
        * [enums](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
    * [v2 SDK](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
    * [v1 SDK](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Core SDK
  * Technical Reference
  * classes
  * [Percent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)


On this page
[@uniswap/sdk-core](https://docs.uniswap.org/sdk/core/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/core/reference/modules.md) / Percent
# Class: Percent
## Hierarchy[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#hierarchy "Direct link to Hierarchy")
  * [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
↳ **`Percent`**


## Table of contents[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Percent#constructor)


### Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#properties "Direct link to Properties")
  * [denominator](https://docs.uniswap.org/sdk/core/reference/classes/Percent#denominator)
  * [isPercent](https://docs.uniswap.org/sdk/core/reference/classes/Percent#ispercent)
  * [numerator](https://docs.uniswap.org/sdk/core/reference/classes/Percent#numerator)


### Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#accessors "Direct link to Accessors")
  * [asFraction](https://docs.uniswap.org/sdk/core/reference/classes/Percent#asfraction)
  * [quotient](https://docs.uniswap.org/sdk/core/reference/classes/Percent#quotient)
  * [remainder](https://docs.uniswap.org/sdk/core/reference/classes/Percent#remainder)


### Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#methods "Direct link to Methods")
  * [add](https://docs.uniswap.org/sdk/core/reference/classes/Percent#add)
  * [divide](https://docs.uniswap.org/sdk/core/reference/classes/Percent#divide)
  * [equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Percent#equalto)
  * [greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Percent#greaterthan)
  * [invert](https://docs.uniswap.org/sdk/core/reference/classes/Percent#invert)
  * [lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Percent#lessthan)
  * [multiply](https://docs.uniswap.org/sdk/core/reference/classes/Percent#multiply)
  * [subtract](https://docs.uniswap.org/sdk/core/reference/classes/Percent#subtract)
  * [toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Percent#tofixed)
  * [toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Percent#tosignificant)


## Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#constructor "Direct link to constructor")
• **new Percent**(`numerator`, `denominator?`)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters "Direct link to Parameters")
Name| Type  
---|---  
`numerator`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)  
`denominator`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)  
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[constructor](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructor)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in "Direct link to Defined in")
[entities/fractions/fraction.ts:28](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L28)
## Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#properties-1 "Direct link to Properties")
### denominator[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#denominator "Direct link to denominator")
• `Readonly` **denominator** : `default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-1 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[denominator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#denominator)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-1 "Direct link to Defined in")
[entities/fractions/fraction.ts:26](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L26)
### isPercent[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#ispercent "Direct link to isPercent")
• `Readonly` **isPercent** : `true`
This boolean prevents a fraction from being interpreted as a Percent
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-2 "Direct link to Defined in")
[entities/fractions/percent.ts:19](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/percent.ts#L19)
### numerator[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#numerator "Direct link to numerator")
• `Readonly` **numerator** : `default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-2 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[numerator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#numerator)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-3 "Direct link to Defined in")
[entities/fractions/fraction.ts:25](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L25)
## Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#accessors-1 "Direct link to Accessors")
### asFraction[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#asfraction "Direct link to asFraction")
• `get` **asFraction**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
Helper method for converting any super class back to a fraction
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-3 "Direct link to Inherited from")
Fraction.asFraction
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-4 "Direct link to Defined in")
[entities/fractions/fraction.ts:154](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L154)
### quotient[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#quotient "Direct link to quotient")
• `get` **quotient**(): `default`
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-1 "Direct link to Returns")
`default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-4 "Direct link to Inherited from")
Fraction.quotient
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-5 "Direct link to Defined in")
[entities/fractions/fraction.ts:42](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L42)
### remainder[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#remainder "Direct link to remainder")
• `get` **remainder**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-2 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-5 "Direct link to Inherited from")
Fraction.remainder
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-6 "Direct link to Defined in")
[entities/fractions/fraction.ts:47](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L47)
## Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#methods-1 "Direct link to Methods")
### add[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#add "Direct link to add")
▸ **add**(`other`): [`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-1 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-3 "Direct link to Returns")
[`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#overrides "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[add](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#add)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-7 "Direct link to Defined in")
[entities/fractions/percent.ts:21](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/percent.ts#L21)
### divide[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#divide "Direct link to divide")
▸ **divide**(`other`): [`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-2 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-4 "Direct link to Returns")
[`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#overrides-1 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[divide](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#divide)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-8 "Direct link to Defined in")
[entities/fractions/percent.ts:33](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/percent.ts#L33)
### equalTo[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#equalto "Direct link to equalTo")
▸ **equalTo**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-3 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-5 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-6 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#equalto)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-9 "Direct link to Defined in")
[entities/fractions/fraction.ts:91](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L91)
### greaterThan[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#greaterthan "Direct link to greaterThan")
▸ **greaterThan**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-4 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-6 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-7 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#greaterthan)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-10 "Direct link to Defined in")
[entities/fractions/fraction.ts:99](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L99)
### invert[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#invert "Direct link to invert")
▸ **invert**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-7 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-8 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[invert](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#invert)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-11 "Direct link to Defined in")
[entities/fractions/fraction.ts:51](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L51)
### lessThan[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#lessthan "Direct link to lessThan")
▸ **lessThan**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-5 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-8 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#inherited-from-9 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#lessthan)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-12 "Direct link to Defined in")
[entities/fractions/fraction.ts:83](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L83)
### multiply[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#multiply "Direct link to multiply")
▸ **multiply**(`other`): [`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-6 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-9 "Direct link to Returns")
[`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#overrides-2 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[multiply](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#multiply)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-13 "Direct link to Defined in")
[entities/fractions/percent.ts:29](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/percent.ts#L29)
### subtract[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#subtract "Direct link to subtract")
▸ **subtract**(`other`): [`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-7 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-10 "Direct link to Returns")
[`Percent`](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#overrides-3 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[subtract](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#subtract)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-14 "Direct link to Defined in")
[entities/fractions/percent.ts:25](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/percent.ts#L25)
### toFixed[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#tofixed "Direct link to toFixed")
▸ **toFixed**(`decimalPlaces?`, `format?`, `rounding?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-8 "Direct link to Parameters")
Name| Type| Default value  
---|---|---  
`decimalPlaces`| `number`| `2`  
`format?`| `object`| `undefined`  
`rounding?`| [`Rounding`](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)| `undefined`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-11 "Direct link to Returns")
`string`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#overrides-4 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tofixed)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-15 "Direct link to Defined in")
[entities/fractions/percent.ts:41](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/percent.ts#L41)
### toSignificant[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#tosignificant "Direct link to toSignificant")
▸ **toSignificant**(`significantDigits?`, `format?`, `rounding?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#parameters-9 "Direct link to Parameters")
Name| Type| Default value  
---|---|---  
`significantDigits`| `number`| `5`  
`format?`| `object`| `undefined`  
`rounding?`| [`Rounding`](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)| `undefined`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#returns-12 "Direct link to Returns")
`string`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#overrides-5 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tosignificant)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Percent#defined-in-16 "Direct link to Defined in")
[entities/fractions/percent.ts:37](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/percent.ts#L37)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Percent.md)
Was this helpful?
[PreviousNativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)[NextPrice](https://docs.uniswap.org/sdk/core/reference/classes/Price)
On this page
  * [Hierarchy](https://docs.uniswap.org/sdk/core/reference/classes/Percent#hierarchy)
  * [Table of contents](https://docs.uniswap.org/sdk/core/reference/classes/Percent#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Percent#constructors)
    * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Percent#properties)
    * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Percent#accessors)
    * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Percent#methods)
  * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Percent#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Percent#constructor)
  * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Percent#properties-1)
    * [denominator](https://docs.uniswap.org/sdk/core/reference/classes/Percent#denominator)
    * [isPercent](https://docs.uniswap.org/sdk/core/reference/classes/Percent#ispercent)
    * [numerator](https://docs.uniswap.org/sdk/core/reference/classes/Percent#numerator)
  * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Percent#accessors-1)
    * [asFraction](https://docs.uniswap.org/sdk/core/reference/classes/Percent#asfraction)
    * [quotient](https://docs.uniswap.org/sdk/core/reference/classes/Percent#quotient)
    * [remainder](https://docs.uniswap.org/sdk/core/reference/classes/Percent#remainder)
  * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Percent#methods-1)
    * [add](https://docs.uniswap.org/sdk/core/reference/classes/Percent#add)
    * [divide](https://docs.uniswap.org/sdk/core/reference/classes/Percent#divide)
    * [equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Percent#equalto)
    * [greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Percent#greaterthan)
    * [invert](https://docs.uniswap.org/sdk/core/reference/classes/Percent#invert)
    * [lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Percent#lessthan)
    * [multiply](https://docs.uniswap.org/sdk/core/reference/classes/Percent#multiply)
    * [subtract](https://docs.uniswap.org/sdk/core/reference/classes/Percent#subtract)
    * [toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Percent#tofixed)
    * [toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Percent#tosignificant)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Percent.md)
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
