# https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider

[Skip to main content](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/advanced/introduction)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/overview)
        * [Overview](https://docs.uniswap.org/sdk/v3/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v3/reference/classes/FullMath)
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
        * [enums](https://docs.uniswap.org/sdk/v3/reference/enums/FeeAmount)
        * [interfaces](https://docs.uniswap.org/sdk/v3/reference/interfaces/AllowedPermitArguments)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Technical Reference
  * classes
  * [TickListDataProvider](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider)


On this page
[@uniswap/v3-sdk](https://docs.uniswap.org/sdk/v3/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/v3/reference/modules.md) / TickListDataProvider
# Class: TickListDataProvider
A data provider for ticks that is backed by an in-memory array of ticks.
## Implements[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#implements "Direct link to Implements")
  * [`TickDataProvider`](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider)


## Table of contents[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#constructor)


### Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#properties "Direct link to Properties")
  * [ticks](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#ticks)


### Methods[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#methods "Direct link to Methods")
  * [getTick](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#gettick)
  * [nextInitializedTickWithinOneWord](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#nextinitializedtickwithinoneword)


## Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#constructor "Direct link to constructor")
• **new TickListDataProvider**(`ticks`, `tickSpacing`)
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#parameters "Direct link to Parameters")
Name| Type  
---|---  
`ticks`| ([`Tick`](https://docs.uniswap.org/sdk/v3/reference/classes/Tick) | [`TickConstructorArgs`](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickConstructorArgs))[]  
`tickSpacing`| `number`  
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#defined-in "Direct link to Defined in")
[entities/tickListDataProvider.ts:12](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/tickListDataProvider.ts#L12)
## Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#properties-1 "Direct link to Properties")
### ticks[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#ticks "Direct link to ticks")
• `Private` **ticks** : readonly [`Tick`](https://docs.uniswap.org/sdk/v3/reference/classes/Tick)[]
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#defined-in-1 "Direct link to Defined in")
[entities/tickListDataProvider.ts:10](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/tickListDataProvider.ts#L10)
## Methods[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#methods-1 "Direct link to Methods")
### getTick[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#gettick "Direct link to getTick")
▸ **getTick**(`tick`): `Promise`<{ `liquidityGross`: `BigintIsh` ; `liquidityNet`: `BigintIsh` }>
Return information corresponding to a specific tick
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#parameters-1 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`tick`| `number`| the tick to load  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#returns "Direct link to Returns")
`Promise`<{ `liquidityGross`: `BigintIsh` ; `liquidityNet`: `BigintIsh` }>
#### Implementation of[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#implementation-of "Direct link to Implementation of")
[TickDataProvider](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider).[getTick](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider#gettick)
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#defined-in-2 "Direct link to Defined in")
[entities/tickListDataProvider.ts:18](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/tickListDataProvider.ts#L18)
### nextInitializedTickWithinOneWord[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#nextinitializedtickwithinoneword "Direct link to nextInitializedTickWithinOneWord")
▸ **nextInitializedTickWithinOneWord**(`tick`, `lte`, `tickSpacing`): `Promise`<[`number`, `boolean`]>
Return the next tick that is initialized within a single word
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#parameters-2 "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`tick`| `number`| The current tick  
`lte`| `boolean`| Whether the next tick should be lte the current tick  
`tickSpacing`| `number`| The tick spacing of the pool  
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#returns-1 "Direct link to Returns")
`Promise`<[`number`, `boolean`]>
#### Implementation of[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#implementation-of-1 "Direct link to Implementation of")
[TickDataProvider](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider).[nextInitializedTickWithinOneWord](https://docs.uniswap.org/sdk/v3/reference/interfaces/TickDataProvider#nextinitializedtickwithinoneword)
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#defined-in-3 "Direct link to Defined in")
[entities/tickListDataProvider.ts:22](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/tickListDataProvider.ts#L22)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/TickListDataProvider.md)
Was this helpful?
[PreviousTickList](https://docs.uniswap.org/sdk/v3/reference/classes/TickList)[NextTickMath](https://docs.uniswap.org/sdk/v3/reference/classes/TickMath)
On this page
  * [Implements](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#implements)
  * [Table of contents](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#constructors)
    * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#properties)
    * [Methods](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#methods)
  * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#constructor)
  * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#properties-1)
    * [ticks](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#ticks)
  * [Methods](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#methods-1)
    * [getTick](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#gettick)
    * [nextInitializedTickWithinOneWord](https://docs.uniswap.org/sdk/v3/reference/classes/TickListDataProvider#nextinitializedtickwithinoneword)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/TickListDataProvider.md)
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
