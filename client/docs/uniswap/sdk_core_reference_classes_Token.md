# https://docs.uniswap.org/sdk/core/reference/classes/Token

[Skip to main content](https://docs.uniswap.org/sdk/core/reference/classes/Token#__docusaurus_skipToContent_fallback)
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
  * [Token](https://docs.uniswap.org/sdk/core/reference/classes/Token)


On this page
[@uniswap/sdk-core](https://docs.uniswap.org/sdk/core/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/core/reference/modules.md) / Token
# Class: Token
Represents an ERC20 token with a unique address and some metadata.
## Hierarchy[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#hierarchy "Direct link to Hierarchy")
  * `BaseCurrency`
↳ **`Token`**


## Table of contents[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Token#constructor)


### Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#properties "Direct link to Properties")
  * [address](https://docs.uniswap.org/sdk/core/reference/classes/Token#address)
  * [chainId](https://docs.uniswap.org/sdk/core/reference/classes/Token#chainid)
  * [decimals](https://docs.uniswap.org/sdk/core/reference/classes/Token#decimals)
  * [isNative](https://docs.uniswap.org/sdk/core/reference/classes/Token#isnative)
  * [isToken](https://docs.uniswap.org/sdk/core/reference/classes/Token#istoken)
  * [name](https://docs.uniswap.org/sdk/core/reference/classes/Token#name)
  * [symbol](https://docs.uniswap.org/sdk/core/reference/classes/Token#symbol)


### Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#accessors "Direct link to Accessors")
  * [wrapped](https://docs.uniswap.org/sdk/core/reference/classes/Token#wrapped)


### Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#methods "Direct link to Methods")
  * [equals](https://docs.uniswap.org/sdk/core/reference/classes/Token#equals)
  * [sortsBefore](https://docs.uniswap.org/sdk/core/reference/classes/Token#sortsbefore)


## Constructors[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#constructor "Direct link to constructor")
• **new Token**(`chainId`, `address`, `decimals`, `symbol?`, `name?`, `bypassChecksum?`)
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#parameters "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`chainId`| `number`| BaseCurrency#chainId  
`address`| `string`| The contract address on the chain on which this token lives  
`decimals`| `number`| BaseCurrency#decimals  
`symbol?`| `string`| BaseCurrency#symbol  
`name?`| `string`| BaseCurrency#name  
`bypassChecksum?`| `boolean`| If true it only checks for length === 42, startsWith 0x and contains only hex characters  
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#overrides "Direct link to Overrides")
BaseCurrency.constructor
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in "Direct link to Defined in")
[entities/token.ts:27](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/token.ts#L27)
## Properties[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#properties-1 "Direct link to Properties")
### address[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#address "Direct link to address")
• `Readonly` **address** : `string`
The contract address on the chain on which this token lives
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-1 "Direct link to Defined in")
[entities/token.ts:16](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/token.ts#L16)
### chainId[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#chainid "Direct link to chainId")
• `Readonly` **chainId** : `number`
The chain ID on which this currency resides
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#inherited-from "Direct link to Inherited from")
BaseCurrency.chainId
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-2 "Direct link to Defined in")
[entities/baseCurrency.ts:21](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L21)
### decimals[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#decimals "Direct link to decimals")
• `Readonly` **decimals** : `number`
The decimals used in representing currency amounts
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#inherited-from-1 "Direct link to Inherited from")
BaseCurrency.decimals
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-3 "Direct link to Defined in")
[entities/baseCurrency.ts:25](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L25)
### isNative[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#isnative "Direct link to isNative")
• `Readonly` **isNative** : `false`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#overrides-1 "Direct link to Overrides")
BaseCurrency.isNative
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-4 "Direct link to Defined in")
[entities/token.ts:10](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/token.ts#L10)
### isToken[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#istoken "Direct link to isToken")
• `Readonly` **isToken** : `true`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#overrides-2 "Direct link to Overrides")
BaseCurrency.isToken
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-5 "Direct link to Defined in")
[entities/token.ts:11](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/token.ts#L11)
### name[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#name "Direct link to name")
• `Optional` `Readonly` **name** : `string`
The name of the currency, i.e. a descriptive textual non-unique identifier
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#inherited-from-2 "Direct link to Inherited from")
BaseCurrency.name
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-6 "Direct link to Defined in")
[entities/baseCurrency.ts:33](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L33)
### symbol[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#symbol "Direct link to symbol")
• `Optional` `Readonly` **symbol** : `string`
The symbol of the currency, i.e. a short textual non-unique identifier
#### Inherited from[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#inherited-from-3 "Direct link to Inherited from")
BaseCurrency.symbol
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-7 "Direct link to Defined in")
[entities/baseCurrency.ts:29](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/baseCurrency.ts#L29)
## Accessors[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#accessors-1 "Direct link to Accessors")
### wrapped[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#wrapped "Direct link to wrapped")
• `get` **wrapped**(): [`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)
Return this token, which does not need to be wrapped
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#returns "Direct link to Returns")
[`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#overrides-3 "Direct link to Overrides")
BaseCurrency.wrapped
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-8 "Direct link to Defined in")
[entities/token.ts:66](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/token.ts#L66)
## Methods[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#methods-1 "Direct link to Methods")
### equals[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#equals "Direct link to equals")
▸ **equals**(`other`): `boolean`
Returns true if the two tokens are equivalent, i.e. have the same chainId and address.
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#parameters-1 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`other`| [`Currency`](https://docs.uniswap.org/sdk/core/reference/modules.md#currency)| other token to compare  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#returns-1 "Direct link to Returns")
`boolean`
#### Overrides[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#overrides-4 "Direct link to Overrides")
BaseCurrency.equals
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-9 "Direct link to Defined in")
[entities/token.ts:47](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/token.ts#L47)
### sortsBefore[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#sortsbefore "Direct link to sortsBefore")
▸ **sortsBefore**(`other`): `boolean`
Returns true if the address of this token sorts before the address of the other token
**`Throws`**
if the tokens have the same address
**`Throws`**
if the tokens are on different chains
#### Parameters[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#parameters-2 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`other`| [`Token`](https://docs.uniswap.org/sdk/core/reference/classes/Token)| other token to compare  
#### Returns[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#returns-2 "Direct link to Returns")
`boolean`
#### Defined in[​](https://docs.uniswap.org/sdk/core/reference/classes/Token#defined-in-10 "Direct link to Defined in")
[entities/token.ts:57](https://github.com/Uniswap/sdk-core/blob/9997e88/src/entities/token.ts#L57)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Token.md)
Was this helpful?
[PreviousPrice](https://docs.uniswap.org/sdk/core/reference/classes/Price)[NextChainId](https://docs.uniswap.org/sdk/core/reference/enums/ChainId)
On this page
  * [Hierarchy](https://docs.uniswap.org/sdk/core/reference/classes/Token#hierarchy)
  * [Table of contents](https://docs.uniswap.org/sdk/core/reference/classes/Token#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Token#constructors)
    * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Token#properties)
    * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Token#accessors)
    * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Token#methods)
  * [Constructors](https://docs.uniswap.org/sdk/core/reference/classes/Token#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/core/reference/classes/Token#constructor)
  * [Properties](https://docs.uniswap.org/sdk/core/reference/classes/Token#properties-1)
    * [address](https://docs.uniswap.org/sdk/core/reference/classes/Token#address)
    * [chainId](https://docs.uniswap.org/sdk/core/reference/classes/Token#chainid)
    * [decimals](https://docs.uniswap.org/sdk/core/reference/classes/Token#decimals)
    * [isNative](https://docs.uniswap.org/sdk/core/reference/classes/Token#isnative)
    * [isToken](https://docs.uniswap.org/sdk/core/reference/classes/Token#istoken)
    * [name](https://docs.uniswap.org/sdk/core/reference/classes/Token#name)
    * [symbol](https://docs.uniswap.org/sdk/core/reference/classes/Token#symbol)
  * [Accessors](https://docs.uniswap.org/sdk/core/reference/classes/Token#accessors-1)
    * [wrapped](https://docs.uniswap.org/sdk/core/reference/classes/Token#wrapped)
  * [Methods](https://docs.uniswap.org/sdk/core/reference/classes/Token#methods-1)
    * [equals](https://docs.uniswap.org/sdk/core/reference/classes/Token#equals)
    * [sortsBefore](https://docs.uniswap.org/sdk/core/reference/classes/Token#sortsbefore)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/core/reference/classes/Token.md)
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
