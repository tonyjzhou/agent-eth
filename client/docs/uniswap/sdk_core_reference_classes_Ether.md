# https://docs.uniswap.org/sdk/core/reference/classes/Ether

[Skip to main content](https://docs.uniswap.org/sdk/core/reference/classes/Ether#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
        * [Swaps](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
        * [Position Management](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
        * [Advanced](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
    * [v3 SDK](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
    * [Swap Widget](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
    * [web3-react](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
    * [Core SDK](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
      * [Overview](https://docs.uniswap.org/sdk/core/overview)
      * [Technical Reference](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
        * [Overview](https://docs.uniswap.org/sdk/core/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
          * [CurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)
          * [Ether](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
          * [Fraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
          * [NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
          * [Percent](https://docs.uniswap.org/sdk/core/reference/classes/Percent)
          * [Price](https://docs.uniswap.org/sdk/core/reference/classes/Price)
          * [Token](https://docs.uniswap.org/sdk/core/reference/classes/Token)
        * [enums](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
    * [v2 SDK](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
    * [v1 SDK](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * Core SDK
  * Technical Reference
  * classes
  * [Ether](https://docs.uniswap.org/sdk/core/reference/classes/Ether)


On this page
[@uniswap/sdk-core](https://docs.uniswap.org/sdk/core/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/core/reference/modules.md) / Ether
# Class: Ether
Ether is the main usage of a 'native' currency, i.e. for Ethereum mainnet and all testnets
## Hierarchy[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#hierarchy "Direct link to Hierarchy")
  * [`NativeCurrency`](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency)
↳ **`Ether`**


## Table of contents[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Ether#constructor)


### Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#properties "Direct link to Properties")
  * [chainId](https://docs.uniswap.org/sdk/core/reference/classes/Ether#chainid)
  * [decimals](https://docs.uniswap.org/sdk/core/reference/classes/Ether#decimals)
  * [isNative](https://docs.uniswap.org/sdk/core/reference/classes/Ether#isnative)
  * [isToken](https://docs.uniswap.org/sdk/core/reference/classes/Ether#istoken)
  * [name](https://docs.uniswap.org/sdk/core/reference/classes/Ether#name)
  * [symbol](https://docs.uniswap.org/sdk/core/reference/classes/Ether#symbol)
  * [_etherCache](https://docs.uniswap.org/sdk/core/reference/classes/Ether#_ethercache)


### Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#accessors "Direct link to Accessors")
  * [wrapped](https://docs.uniswap.org/sdk/core/reference/classes/Ether#wrapped)


### Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#methods "Direct link to Methods")
  * [equals](https://docs.uniswap.org/sdk/core/reference/classes/Ether#equals)
  * [onChain](https://docs.uniswap.org/sdk/core/reference/classes/Ether#onchain)


## Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#constructor "Direct link to constructor")
• `Protected` **new Ether**(`chainId`)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#parameters "Direct link to Parameters")
Name| Type  
---|---  
`chainId`| `number`  
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#overrides "Direct link to Overrides")
[NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency).[constructor](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#constructor)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in "Direct link to Defined in")
[entities/ether.ts:11](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/ether.ts#L11)
## Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#properties-1 "Direct link to Properties")
### chainId[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#chainid "Direct link to chainId")
• `Readonly` **chainId** : `number`
The chain ID on which this currency resides
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#inherited-from "Direct link to Inherited from")
[NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency).[chainId](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#chainid)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-1 "Direct link to Defined in")
[entities/baseCurrency.ts:21](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L21)
### decimals[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#decimals "Direct link to decimals")
• `Readonly` **decimals** : `number`
The decimals used in representing currency amounts
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#inherited-from-1 "Direct link to Inherited from")
[NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency).[decimals](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#decimals)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-2 "Direct link to Defined in")
[entities/baseCurrency.ts:25](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L25)
### isNative[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#isnative "Direct link to isNative")
• `Readonly` **isNative** : `true`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#inherited-from-2 "Direct link to Inherited from")
[NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency).[isNative](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#isnative)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-3 "Direct link to Defined in")
[entities/nativeCurrency.ts:7](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/nativeCurrency.ts#L7)
### isToken[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#istoken "Direct link to isToken")
• `Readonly` **isToken** : `false`
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#inherited-from-3 "Direct link to Inherited from")
[NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency).[isToken](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#istoken)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-4 "Direct link to Defined in")
[entities/nativeCurrency.ts:8](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/nativeCurrency.ts#L8)
### name[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#name "Direct link to name")
• `Optional` `Readonly` **name** : `string`
The name of the currency, i.e. a descriptive textual non-unique identifier
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#inherited-from-4 "Direct link to Inherited from")
[NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency).[name](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#name)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-5 "Direct link to Defined in")
[entities/baseCurrency.ts:33](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L33)
### symbol[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#symbol "Direct link to symbol")
• `Optional` `Readonly` **symbol** : `string`
The symbol of the currency, i.e. a short textual non-unique identifier
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#inherited-from-5 "Direct link to Inherited from")
[NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency).[symbol](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#symbol)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-6 "Direct link to Defined in")
[entities/baseCurrency.ts:29](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L29)
### _etherCache[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#_ethercache "Direct link to _etherCache")
▪ `Static` `Private` **_etherCache** : `Object` = `{}`
#### Index signature[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#index-signature "Direct link to Index signature")
▪ [chainId: `number`]: [`Ether`](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-7 "Direct link to Defined in")
[entities/ether.ts:21](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/ether.ts#L21)
## Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#accessors-1 "Direct link to Accessors")
### wrapped[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#wrapped "Direct link to wrapped")
• `get` **wrapped**(): [`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)
Return the wrapped version of this currency that can be used with the Uniswap contracts. Currencies must implement this to be used in Uniswap
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#returns "Direct link to Returns")
[`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#overrides-1 "Direct link to Overrides")
NativeCurrency.wrapped
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-8 "Direct link to Defined in")
[entities/ether.ts:15](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/ether.ts#L15)
## Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#methods-1 "Direct link to Methods")
### equals[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#equals "Direct link to equals")
▸ **equals**(`other`): `boolean`
Returns whether this currency is functionally equivalent to the other currency
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#parameters-1 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`other`| [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)| the other currency  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#returns-1 "Direct link to Returns")
`boolean`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#overrides-2 "Direct link to Overrides")
[NativeCurrency](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency).[equals](https://docs.uniswap.org/sdk/core/reference/classes/NativeCurrency#equals)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-9 "Direct link to Defined in")
[entities/ether.ts:27](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/ether.ts#L27)
### onChain[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#onchain "Direct link to onChain")
▸ `Static` **onChain**(`chainId`): [`Ether`](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#parameters-2 "Direct link to Parameters")
Name| Type  
---|---  
`chainId`| `number`  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#returns-2 "Direct link to Returns")
[`Ether`](https://docs.uniswap.org/sdk/core/reference/classes/Ether)
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Ether#defined-in-10 "Direct link to Defined in")
[entities/ether.ts:23](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/ether.ts#L23)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Ether.md)
Was this helpful?
[PreviousCurrencyAmount](https://docs.uniswap.org/sdk/core/reference/classes/CurrencyAmount)[NextFraction](https://docs.uniswap.org/sdk/core/reference/classes/Fraction)
On this page
  * [Hierarchy](https://docs.uniswap.org/sdk/core/reference/classes/Ether#hierarchy)
  * [Table of contents](https://docs.uniswap.org/sdk/core/reference/classes/Ether#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Ether#constructors)
    * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Ether#properties)
    * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Ether#accessors)
    * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Ether#methods)
  * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Ether#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Ether#constructor)
  * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Ether#properties-1)
    * [chainId](https://docs.uniswap.org/sdk/core/reference/classes/Ether#chainid)
    * [decimals](https://docs.uniswap.org/sdk/core/reference/classes/Ether#decimals)
    * [isNative](https://docs.uniswap.org/sdk/core/reference/classes/Ether#isnative)
    * [isToken](https://docs.uniswap.org/sdk/core/reference/classes/Ether#istoken)
    * [name](https://docs.uniswap.org/sdk/core/reference/classes/Ether#name)
    * [symbol](https://docs.uniswap.org/sdk/core/reference/classes/Ether#symbol)
    * [_etherCache](https://docs.uniswap.org/sdk/core/reference/classes/Ether#_ethercache)
  * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Ether#accessors-1)
    * [wrapped](https://docs.uniswap.org/sdk/core/reference/classes/Ether#wrapped)
  * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Ether#methods-1)
    * [equals](https://docs.uniswap.org/sdk/core/reference/classes/Ether#equals)
    * [onChain](https://docs.uniswap.org/sdk/core/reference/classes/Ether#onchain)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Ether.md)
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
