# https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter

[Skip to main content](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
        * [Swaps](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
        * [Position Management](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
        * [Advanced](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
        * [Advanced](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
        * [Overview](https://docs.uniswap.org/sdk/v3/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
          * [FullMath](https://docs.uniswap.org/sdk/v3/reference/classes/FullMath)
          * [LiquidityMath](https://docs.uniswap.org/sdk/v3/reference/classes/LiquidityMath)
          * [Multicall](https://docs.uniswap.org/sdk/v3/reference/classes/Multicall)
          * [NoTickDataProvider](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
          * [NonfungiblePositionManager](https://docs.uniswap.org/sdk/v3/reference/classes/NonfungiblePositionManager)
          * [Payments](https://docs.uniswap.org/sdk/v3/reference/classes/Payments)
          * [Pool](https://docs.uniswap.org/sdk/v3/reference/classes/Pool)
          * [Position](https://docs.uniswap.org/sdk/v3/reference/classes/Position)
          * [PositionLibrary](https://docs.uniswap.org/sdk/v3/reference/classes/PositionLibrary)
          * [Route](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
          * [SelfPermit](https://docs.uniswap.org/sdk/v3/reference/classes/SelfPermit)
          * [SqrtPriceMath](https://docs.uniswap.org/sdk/v3/reference/classes/SqrtPriceMath)
          * [Staker](https://docs.uniswap.org/sdk/v3/reference/classes/Staker)
          * [SwapMath](https://docs.uniswap.org/sdk/v3/reference/classes/SwapMath)
          * [SwapQuoter](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
          * [SwapRouter](https://docs.uniswap.org/sdk/v3/reference/classes/SwapRouter)
          * [Tick](https://docs.uniswap.org/sdk/v3/reference/classes/Tick)
          * [TickLibrary](https://docs.uniswap.org/sdk/v3/reference/classes/TickLibrary)
          * [TickList](https://docs.uniswap.org/sdk/v3/reference/classes/TickList)
          * [TickListDataProvider](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider)
          * [TickMath](https://docs.uniswap.org/sdk/v3/reference/classes/TickMath)
          * [Trade](https://docs.uniswap.org/sdk/v3/reference/classes/Trade)
        * [enums](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
        * [interfaces](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
    * [web3-react](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Technical Reference
  * classes
  * [SwapQuoter](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter)


On this page
[@uniswap/v3-sdk](https://docs.uniswap.org/sdk/v3/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/v3/reference/modules.md) / SwapQuoter
# Class: SwapQuoter
Represents the Uniswap V3 QuoterV1 contract with a method for returning the formatted calldata needed to call the quoter contract.
## Table of contents[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#constructor)


### Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#properties "Direct link to Properties")
  * [V1INTERFACE](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#v1interface)
  * [V2INTERFACE](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#v2interface)


### Methods[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#methods "Direct link to Methods")
  * [quoteCallParameters](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#quotecallparameters)


## Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#constructor "Direct link to constructor")
• **new SwapQuoter**()
## Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#properties-1 "Direct link to Properties")
### V1INTERFACE[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#v1interface "Direct link to V1INTERFACE")
▪ `Static` **V1INTERFACE** : `Interface`
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#defined-in "Direct link to Defined in")
[quoter.ts:37](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/quoter.ts#L37)
### V2INTERFACE[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#v2interface "Direct link to V2INTERFACE")
▪ `Static` **V2INTERFACE** : `Interface`
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#defined-in-1 "Direct link to Defined in")
[quoter.ts:38](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/quoter.ts#L38)
## Methods[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#methods-1 "Direct link to Methods")
### quoteCallParameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#quotecallparameters "Direct link to quoteCallParameters")
▸ `Static` **quoteCallParameters** <`TInput`, `TOutput`>(`route`, `amount`, `tradeType`, `options?`): [`MethodParameters`](https://docs.uniswap.org/sdk/v3/reference/interfaces/MethodParameters)
Produces the on-chain method name of the appropriate function within QuoterV2, and the relevant hex encoded parameters.
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#type-parameters "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token, either Ether or an ERC-20  
`TOutput`| extends `Currency`| The output token, either Ether or an ERC-20  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#parameters "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`route`| [`Route`](https://docs.uniswap.org/sdk/v3/reference/classes/Route)<`TInput`, `TOutput`>| The swap route, a list of pools through which a swap can occur  
`amount`| `CurrencyAmount`<`TInput` | `TOutput`>| The amount of the quote, either an amount in, or an amount out  
`tradeType`| `TradeType`| The trade type, either exact input or exact output  
`options`| [`QuoteOptions`](https://docs.uniswap.org/sdk/v3/reference/interfaces/QuoteOptions)| The optional params including price limit and Quoter contract switch  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#returns "Direct link to Returns")
[`MethodParameters`](https://docs.uniswap.org/sdk/v3/reference/interfaces/MethodParameters)
The formatted calldata
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#defined-in-2 "Direct link to Defined in")
[quoter.ts:51](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/quoter.ts#L51)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/SwapQuoter.md)
Was this helpful?
[PreviousSwapMath](https://docs.uniswap.org/sdk/v3/reference/classes/SwapMath)[NextSwapRouter](https://docs.uniswap.org/sdk/v3/reference/classes/SwapRouter)
On this page
  * [Table of contents](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#constructors)
    * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#properties)
    * [Methods](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#methods)
  * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#constructor)
  * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#properties-1)
    * [V1INTERFACE](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#v1interface)
    * [V2INTERFACE](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#v2interface)
  * [Methods](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#methods-1)
    * [quoteCallParameters](https://docs.uniswap.org/sdk/v3/reference/classes/SwapQuoter#quotecallparameters)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/SwapQuoter.md)
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
