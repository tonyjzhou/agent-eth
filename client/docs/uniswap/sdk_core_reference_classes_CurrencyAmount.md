# https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount

[Skip to main content](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
        * [Swaps](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
        * [Position Management](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
        * [Advanced](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
    * [v3 SDK](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
    * [Swap Widget](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
    * [web3-react](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
    * [Core SDK](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
      * [Overview](https://docs.uniswap.org/sdk/core/overview)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
        * [Overview](https://docs.uniswap.org/sdk/core/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
          * [CurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
          * [Ether](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
          * [Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
          * [NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
          * [Percent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
          * [Price](https://docs.uniswap.org/sdk/core/reference/classes/Price)
          * [Token](https://docs.uniswap.org/sdk/core/reference/classes/Token)
        * [enums](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
    * [v2 SDK](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
    * [v1 SDK](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Core SDK
  * Technical Reference
  * classes
  * [CurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)


On this page
[@uniswap/sdk-core](https://docs.uniswap.org/sdk/core/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/core/reference/modules.md) / CurrencyAmount
# Class: CurrencyAmount<T>
## Type parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#type-parameters "Direct link to Type parameters")
Name| Type  
---|---  
`T`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
## Hierarchy[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#hierarchy "Direct link to Hierarchy")
  * [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
↳ **`CurrencyAmount`**


## Table of contents[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#constructor)


### Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#properties "Direct link to Properties")
  * [currency](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#currency)
  * [decimalScale](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#decimalscale)
  * [denominator](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#denominator)
  * [numerator](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#numerator)


### Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#accessors "Direct link to Accessors")
  * [asFraction](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#asfraction)
  * [quotient](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#quotient)
  * [remainder](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#remainder)
  * [wrapped](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#wrapped)


### Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#methods "Direct link to Methods")
  * [add](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#add)
  * [divide](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#divide)
  * [equalTo](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#equalto)
  * [greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#greaterthan)
  * [invert](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#invert)
  * [lessThan](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#lessthan)
  * [multiply](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#multiply)
  * [subtract](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#subtract)
  * [toExact](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#toexact)
  * [toFixed](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#tofixed)
  * [toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#tosignificant)
  * [fromFractionalAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#fromfractionalamount)
  * [fromRawAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#fromrawamount)


## Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#constructor "Direct link to constructor")
• `Protected` **new CurrencyAmount** <`T`>(`currency`, `numerator`, `denominator?`)
#### Type parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#type-parameters-1 "Direct link to Type parameters")
Name| Type  
---|---  
`T`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters "Direct link to Parameters")
Name| Type  
---|---  
`currency`| `T`  
`numerator`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)  
`denominator?`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)  
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#overrides "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[constructor](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#constructor)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:40](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L40)
## Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#properties-1 "Direct link to Properties")
### currency[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#currency "Direct link to currency")
• `Readonly` **currency** : `T`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-1 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:14](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L14)
### decimalScale[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#decimalscale "Direct link to decimalScale")
• `Readonly` **decimalScale** : `default`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-2 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:15](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L15)
### denominator[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#denominator "Direct link to denominator")
• `Readonly` **denominator** : `default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[denominator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#denominator)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-3 "Direct link to Defined in")
[entities/fractions/fraction.ts:26](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L26)
### numerator[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#numerator "Direct link to numerator")
• `Readonly` **numerator** : `default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from-1 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[numerator](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#numerator)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-4 "Direct link to Defined in")
[entities/fractions/fraction.ts:25](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L25)
## Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#accessors-1 "Direct link to Accessors")
### asFraction[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#asfraction "Direct link to asFraction")
• `get` **asFraction**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
Helper method for converting any super class back to a fraction
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from-2 "Direct link to Inherited from")
Fraction.asFraction
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-5 "Direct link to Defined in")
[entities/fractions/fraction.ts:154](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L154)
### quotient[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#quotient "Direct link to quotient")
• `get` **quotient**(): `default`
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-1 "Direct link to Returns")
`default`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from-3 "Direct link to Inherited from")
Fraction.quotient
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-6 "Direct link to Defined in")
[entities/fractions/fraction.ts:42](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L42)
### remainder[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#remainder "Direct link to remainder")
• `get` **remainder**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-2 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from-4 "Direct link to Inherited from")
Fraction.remainder
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-7 "Direct link to Defined in")
[entities/fractions/fraction.ts:47](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L47)
### wrapped[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#wrapped "Direct link to wrapped")
• `get` **wrapped**(): [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<[`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)>
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-3 "Direct link to Returns")
[`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<[`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)>
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-8 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:91](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L91)
## Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#methods-1 "Direct link to Methods")
### add[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#add "Direct link to add")
▸ **add**(`other`): [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-1 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-4 "Direct link to Returns")
[`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#overrides-1 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[add](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#add)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-9 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:47](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L47)
### divide[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#divide "Direct link to divide")
▸ **divide**(`other`): [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-2 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-5 "Direct link to Returns")
[`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#overrides-2 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[divide](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#divide)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-10 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:64](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L64)
### equalTo[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#equalto "Direct link to equalTo")
▸ **equalTo**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-3 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-6 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from-5 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[equalTo](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#equalto)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-11 "Direct link to Defined in")
[entities/fractions/fraction.ts:91](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L91)
### greaterThan[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#greaterthan "Direct link to greaterThan")
▸ **greaterThan**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-4 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-7 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from-6 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#greaterthan)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-12 "Direct link to Defined in")
[entities/fractions/fraction.ts:99](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L99)
### invert[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#invert "Direct link to invert")
▸ **invert**(): [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-8 "Direct link to Returns")
[`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from-7 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[invert](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#invert)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-13 "Direct link to Defined in")
[entities/fractions/fraction.ts:51](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L51)
### lessThan[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#lessthan "Direct link to lessThan")
▸ **lessThan**(`other`): `boolean`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-5 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-9 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#inherited-from-8 "Direct link to Inherited from")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[lessThan](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#lessthan)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-14 "Direct link to Defined in")
[entities/fractions/fraction.ts:83](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/fraction.ts#L83)
### multiply[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#multiply "Direct link to multiply")
▸ **multiply**(`other`): [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-6 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish) | [`Fraction`](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-10 "Direct link to Returns")
[`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#overrides-3 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[multiply](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#multiply)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-15 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:59](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L59)
### subtract[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#subtract "Direct link to subtract")
▸ **subtract**(`other`): [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-7 "Direct link to Parameters")
Name| Type  
---|---  
`other`| [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-11 "Direct link to Returns")
[`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#overrides-4 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[subtract](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#subtract)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-16 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:53](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L53)
### toExact[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#toexact "Direct link to toExact")
▸ **toExact**(`format?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-8 "Direct link to Parameters")
Name| Type  
---|---  
`format`| `object`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-12 "Direct link to Returns")
`string`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-17 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:86](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L86)
### toFixed[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#tofixed "Direct link to toFixed")
▸ **toFixed**(`decimalPlaces?`, `format?`, `rounding?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-9 "Direct link to Parameters")
Name| Type| Default value  
---|---|---  
`decimalPlaces`| `number`| `undefined`  
`format?`| `object`| `undefined`  
`rounding`| [`Rounding`](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)| `Rounding.ROUND_DOWN`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-13 "Direct link to Returns")
`string`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#overrides-5 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[toFixed](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tofixed)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-18 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:77](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L77)
### toSignificant[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#tosignificant "Direct link to toSignificant")
▸ **toSignificant**(`significantDigits?`, `format?`, `rounding?`): `string`
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-10 "Direct link to Parameters")
Name| Type| Default value  
---|---|---  
`significantDigits`| `number`| `6`  
`format?`| `object`| `undefined`  
`rounding`| [`Rounding`](https://docs.uniswap.org/sdk/core/reference/enums/Rounding)| `Rounding.ROUND_DOWN`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-14 "Direct link to Returns")
`string`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#overrides-6 "Direct link to Overrides")
[Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction).[toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/Fraction#tosignificant)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-19 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:69](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L69)
### fromFractionalAmount[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#fromfractionalamount "Direct link to fromFractionalAmount")
▸ `Static` **fromFractionalAmount** <`T`>(`currency`, `numerator`, `denominator`): [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
Construct a currency amount with a denominator that is not equal to 1
#### Type parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#type-parameters-2 "Direct link to Type parameters")
Name| Type  
---|---  
`T`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-11 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`currency`| `T`| the currency  
`numerator`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)| the numerator of the fractional token amount  
`denominator`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)| the denominator of the fractional token amount  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-15 "Direct link to Returns")
[`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-20 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:32](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L32)
### fromRawAmount[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#fromrawamount "Direct link to fromRawAmount")
▸ `Static` **fromRawAmount** <`T`>(`currency`, `rawAmount`): [`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
Returns a new currency amount instance from the unitless amount of token, i.e. the raw amount
#### Type parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#type-parameters-3 "Direct link to Type parameters")
Name| Type  
---|---  
`T`| extends [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)  
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#parameters-12 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`currency`| `T`| the currency in the amount  
`rawAmount`| [`BigintIsh`](https://docs.uniswap.org/sdk/core/reference/modules.md#bigintish)| the raw token or ether amount  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#returns-16 "Direct link to Returns")
[`CurrencyAmount`](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)<`T`>
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#defined-in-21 "Direct link to Defined in")
[entities/fractions/currencyAmount.ts:22](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/fractions/currencyAmount.ts#L22)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/CurrencyAmount.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/sdk/core/reference/overview)[NextEther](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
On this page
  * [Type parameters](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#type-parameters)
  * [Hierarchy](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#hierarchy)
  * [Table of contents](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#constructors)
    * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#properties)
    * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#accessors)
    * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#methods)
  * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#constructor)
  * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#properties-1)
    * [currency](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#currency)
    * [decimalScale](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#decimalscale)
    * [denominator](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#denominator)
    * [numerator](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#numerator)
  * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#accessors-1)
    * [asFraction](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#asfraction)
    * [quotient](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#quotient)
    * [remainder](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#remainder)
    * [wrapped](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#wrapped)
  * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#methods-1)
    * [add](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#add)
    * [divide](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#divide)
    * [equalTo](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#equalto)
    * [greaterThan](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#greaterthan)
    * [invert](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#invert)
    * [lessThan](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#lessthan)
    * [multiply](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#multiply)
    * [subtract](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#subtract)
    * [toExact](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#toexact)
    * [toFixed](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#tofixed)
    * [toSignificant](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#tosignificant)
    * [fromFractionalAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#fromfractionalamount)
    * [fromRawAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount#fromrawamount)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/CurrencyAmount.md)
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
