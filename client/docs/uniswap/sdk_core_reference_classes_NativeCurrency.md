# https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency

[Skip to main content](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
        * [Swaps](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
        * [Position Management](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
        * [Advanced](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
    * [v3 SDK](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
    * [Swap Widget](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
    * [web3-react](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
    * [Core SDK](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
      * [Overview](https://docs.uniswap.org/sdk/core/overview)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
        * [Overview](https://docs.uniswap.org/sdk/core/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
          * [CurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
          * [Ether](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
          * [Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
          * [NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
          * [Percent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
          * [Price](https://docs.uniswap.org/sdk/core/reference/classes/Price)
          * [Token](https://docs.uniswap.org/sdk/core/reference/classes/Token)
        * [enums](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
    * [v2 SDK](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
    * [v1 SDK](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Core SDK
  * Technical Reference
  * classes
  * [NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)


On this page
[@uniswap/sdk-core](https://docs.uniswap.org/sdk/core/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/core/reference/modules.md) / NativeCurrency
# Class: NativeCurrency
Represents the native currency of the chain on which it resides, e.g.
## Hierarchy[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#hierarchy "Direct link to Hierarchy")
  * `BaseCurrency`
↳ **`NativeCurrency`**
↳↳ [`Ether`](https://docs.uniswap.org/sdk/core/reference/classes/Ether)


## Table of contents[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#constructor)


### Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#properties "Direct link to Properties")
  * [chainId](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#chainid)
  * [decimals](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#decimals)
  * [isNative](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#isnative)
  * [isToken](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#istoken)
  * [name](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#name)
  * [symbol](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#symbol)


### Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#accessors "Direct link to Accessors")
  * [wrapped](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#wrapped)


### Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#methods "Direct link to Methods")
  * [equals](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#equals)


## Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#constructor "Direct link to constructor")
• `Protected` **new NativeCurrency**(`chainId`, `decimals`, `symbol?`, `name?`)
Constructs an instance of the base class `BaseCurrency`.
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#parameters "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`chainId`| `number`| the chain ID on which this currency resides  
`decimals`| `number`| decimals of the currency  
`symbol?`| `string`| symbol of the currency  
`name?`| `string`| of the currency  
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#inherited-from "Direct link to Inherited from")
BaseCurrency.constructor
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in "Direct link to Defined in")
[entities/baseCurrency.ts:42](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L42)
## Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#properties-1 "Direct link to Properties")
### chainId[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#chainid "Direct link to chainId")
• `Readonly` **chainId** : `number`
The chain ID on which this currency resides
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#inherited-from-1 "Direct link to Inherited from")
BaseCurrency.chainId
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in-1 "Direct link to Defined in")
[entities/baseCurrency.ts:21](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L21)
### decimals[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#decimals "Direct link to decimals")
• `Readonly` **decimals** : `number`
The decimals used in representing currency amounts
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#inherited-from-2 "Direct link to Inherited from")
BaseCurrency.decimals
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in-2 "Direct link to Defined in")
[entities/baseCurrency.ts:25](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L25)
### isNative[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#isnative "Direct link to isNative")
• `Readonly` **isNative** : `true`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#overrides "Direct link to Overrides")
BaseCurrency.isNative
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in-3 "Direct link to Defined in")
[entities/nativeCurrency.ts:7](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/nativeCurrency.ts#L7)
### isToken[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#istoken "Direct link to isToken")
• `Readonly` **isToken** : `false`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#overrides-1 "Direct link to Overrides")
BaseCurrency.isToken
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in-4 "Direct link to Defined in")
[entities/nativeCurrency.ts:8](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/nativeCurrency.ts#L8)
### name[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#name "Direct link to name")
• `Optional` `Readonly` **name** : `string`
The name of the currency, i.e. a descriptive textual non-unique identifier
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#inherited-from-3 "Direct link to Inherited from")
BaseCurrency.name
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in-5 "Direct link to Defined in")
[entities/baseCurrency.ts:33](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L33)
### symbol[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#symbol "Direct link to symbol")
• `Optional` `Readonly` **symbol** : `string`
The symbol of the currency, i.e. a short textual non-unique identifier
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#inherited-from-4 "Direct link to Inherited from")
BaseCurrency.symbol
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in-6 "Direct link to Defined in")
[entities/baseCurrency.ts:29](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L29)
## Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#accessors-1 "Direct link to Accessors")
### wrapped[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#wrapped "Direct link to wrapped")
• `Abstract` `get` **wrapped**(): [`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)
Return the wrapped version of this currency that can be used with the Uniswap contracts. Currencies must implement this to be used in Uniswap
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#returns "Direct link to Returns")
[`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#inherited-from-5 "Direct link to Inherited from")
BaseCurrency.wrapped
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in-7 "Direct link to Defined in")
[entities/baseCurrency.ts:62](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L62)
## Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#methods-1 "Direct link to Methods")
### equals[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#equals "Direct link to equals")
▸ `Abstract` **equals**(`other`): `boolean`
Returns whether this currency is functionally equivalent to the other currency
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#parameters-1 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`other`| [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)| the other currency  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#returns-1 "Direct link to Returns")
`boolean`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#inherited-from-6 "Direct link to Inherited from")
BaseCurrency.equals
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#defined-in-8 "Direct link to Defined in")
[entities/baseCurrency.ts:56](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L56)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/NativeCurrency.md)
Was this helpful?
[PreviousFraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)[NextPercent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
On this page
  * [Hierarchy](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#hierarchy)
  * [Table of contents](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#constructors)
    * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#properties)
    * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#accessors)
    * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#methods)
  * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#constructor)
  * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#properties-1)
    * [chainId](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#chainid)
    * [decimals](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#decimals)
    * [isNative](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#isnative)
    * [isToken](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#istoken)
    * [name](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#name)
    * [symbol](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#symbol)
  * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#accessors-1)
    * [wrapped](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#wrapped)
  * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#methods-1)
    * [equals](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#equals)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/NativeCurrency.md)
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
