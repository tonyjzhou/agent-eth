# https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider

[Skip to main content](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
        * [Swaps](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
        * [Position Management](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
        * [Advanced](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
        * [Advanced](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
        * [Overview](https://docs.uniswap.org/sdk/v3/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
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
        * [enums](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
        * [interfaces](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
    * [web3-react](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Technical Reference
  * classes
  * [NoTickDataProvider](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider)


On this page
[@uniswap/v3-sdk](https://docs.uniswap.org/sdk/v3/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/v3/reference/modules.md) / NoTickDataProvider
# Class: NoTickDataProvider
This tick data provider does not know how to fetch any tick data. It throws whenever it is required. Useful if you do not need to load tick data for your use case.
## Implements[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#implements "Direct link to Implements")
  * [`TickDataProvider`](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider)


## Table of contents[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#constructor)


### Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#properties "Direct link to Properties")
  * [ERROR_MESSAGE](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#error_message)


### Methods[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#methods "Direct link to Methods")
  * [getTick](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#gettick)
  * [nextInitializedTickWithinOneWord](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#nextinitializedtickwithinoneword)


## Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#constructor "Direct link to constructor")
• **new NoTickDataProvider**()
## Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#properties-1 "Direct link to Properties")
### ERROR_MESSAGE[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#error_message "Direct link to ERROR_MESSAGE")
▪ `Static` `Private` **ERROR_MESSAGE** : `string` = `'No tick data provider was given'`
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#defined-in "Direct link to Defined in")
[entities/tickDataProvider.ts:27](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/tickDataProvider.ts#L27)
## Methods[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#methods-1 "Direct link to Methods")
### getTick[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#gettick "Direct link to getTick")
▸ **getTick**(`_tick`): `Promise`<{ `liquidityNet`: `BigintIsh` \\}>
Return information corresponding to a specific tick
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#parameters "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`_tick`| `number`| the tick to load  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#returns "Direct link to Returns")
`Promise`<{ `liquidityNet`: `BigintIsh` }>
#### Implementation of[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#implementation-of "Direct link to Implementation of")
[TickDataProvider](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider).[getTick](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider#gettick)
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#defined-in-1 "Direct link to Defined in")
[entities/tickDataProvider.ts:28](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/tickDataProvider.ts#L28)
### nextInitializedTickWithinOneWord[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#nextinitializedtickwithinoneword "Direct link to nextInitializedTickWithinOneWord")
▸ **nextInitializedTickWithinOneWord**(`_tick`, `_lte`, `_tickSpacing`): `Promise`<[`number`, `boolean`]>
Return the next tick that is initialized within a single word
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#parameters-1 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`_tick`| `number`| The current tick  
`_lte`| `boolean`| Whether the next tick should be lte the current tick  
`_tickSpacing`| `number`| The tick spacing of the pool  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#returns-1 "Direct link to Returns")
`Promise`<[`number`, `boolean`]>
#### Implementation of[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#implementation-of-1 "Direct link to Implementation of")
[TickDataProvider](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider).[nextInitializedTickWithinOneWord](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider#nextinitializedtickwithinoneword)
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#defined-in-2 "Direct link to Defined in")
[entities/tickDataProvider.ts:32](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/tickDataProvider.ts#L32)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/NoTickDataProvider.md)
Was this helpful?
[PreviousMulticall](https://docs.uniswap.org/sdk/v3/reference/classes/Multicall)[NextNonfungiblePositionManager](https://docs.uniswap.org/sdk/v3/reference/classes/NonfungiblePositionManager)
On this page
  * [Implements](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#implements)
  * [Table of contents](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#constructors)
    * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#properties)
    * [Methods](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#methods)
  * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#constructor)
  * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#properties-1)
    * [ERROR_MESSAGE](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#error_message)
  * [Methods](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#methods-1)
    * [getTick](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#gettick)
    * [nextInitializedTickWithinOneWord](https://docs.uniswap.org/sdk/v3/reference/classes/NoTickDataProvider#nextinitializedtickwithinoneword)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/NoTickDataProvider.md)
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
