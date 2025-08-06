# https://docs.uniswap.org/sdk/v3/reference/classes/Route

[Skip to main content](https://docs.uniswap.org/sdk/v3/reference/classes/Route#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
        * [Swaps](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
        * [Position Management](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
        * [Advanced](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
        * [Advanced](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
        * [Overview](https://docs.uniswap.org/sdk/v3/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
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
        * [enums](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
        * [interfaces](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
    * [web3-react](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/reference/classes/Route)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Technical Reference
  * classes
  * [Route](https://docs.uniswap.org/sdk/v3/reference/classes/Route)


On this page
[@uniswap/v3-sdk](https://docs.uniswap.org/sdk/v3/reference/README.md) / [Exports](https://docs.uniswap.org/sdk/v3/reference/modules.md) / Route
# Class: Route<TInput, TOutput>
Represents a list of pools through which a swap can occur
## Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#type-parameters "Direct link to Type parameters")
Name| Type| Description  
---|---|---  
`TInput`| extends `Currency`| The input token  
`TOutput`| extends `Currency`| The output token  
## Table of contents[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#table-of-contents "Direct link to Table of contents")
### Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#constructors "Direct link to Constructors")
  * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/Route#constructor)


### Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#properties "Direct link to Properties")
  * [_midPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Route#_midprice)
  * [input](https://docs.uniswap.org/sdk/v3/reference/classes/Route#input)
  * [output](https://docs.uniswap.org/sdk/v3/reference/classes/Route#output)
  * [pools](https://docs.uniswap.org/sdk/v3/reference/classes/Route#pools)
  * [tokenPath](https://docs.uniswap.org/sdk/v3/reference/classes/Route#tokenpath)


### Accessors[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#accessors "Direct link to Accessors")
  * [chainId](https://docs.uniswap.org/sdk/v3/reference/classes/Route#chainid)
  * [midPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Route#midprice)


## Constructors[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#constructors-1 "Direct link to Constructors")
### constructor[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#constructor "Direct link to constructor")
• **new Route** <`TInput`, `TOutput`>(`pools`, `input`, `output`)
Creates an instance of route.
#### Type parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#type-parameters-1 "Direct link to Type parameters")
Name| Type  
---|---  
`TInput`| extends `Currency`  
`TOutput`| extends `Currency`  
#### Parameters[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#parameters "Direct link to Parameters")
Name| Type| Description  
---|---|---  
`pools`| [`Pool`](https://docs.uniswap.org/sdk/v3/reference/classes/Pool)[]| An array of `Pool` objects, ordered by the route the swap will take  
`input`| `TInput`| The input token  
`output`| `TOutput`| The output token  
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#defined-in "Direct link to Defined in")
[entities/route.ts:25](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/route.ts#L25)
## Properties[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#properties-1 "Direct link to Properties")
### _midPrice[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#_midprice "Direct link to _midPrice")
• `Private` **_midPrice** : `null` | `Price`<`TInput`, `TOutput`> = `null`
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#defined-in-1 "Direct link to Defined in")
[entities/route.ts:17](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/route.ts#L17)
### input[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#input "Direct link to input")
• `Readonly` **input** : `TInput`
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#defined-in-2 "Direct link to Defined in")
[entities/route.ts:14](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/route.ts#L14)
### output[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#output "Direct link to output")
• `Readonly` **output** : `TOutput`
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#defined-in-3 "Direct link to Defined in")
[entities/route.ts:15](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/route.ts#L15)
### pools[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#pools "Direct link to pools")
• `Readonly` **pools** : [`Pool`](https://docs.uniswap.org/sdk/v3/reference/classes/Pool)[]
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#defined-in-4 "Direct link to Defined in")
[entities/route.ts:12](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/route.ts#L12)
### tokenPath[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#tokenpath "Direct link to tokenPath")
• `Readonly` **tokenPath** : `Token`[]
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#defined-in-5 "Direct link to Defined in")
[entities/route.ts:13](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/route.ts#L13)
## Accessors[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#accessors-1 "Direct link to Accessors")
### chainId[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#chainid "Direct link to chainId")
• `get` **chainId**(): `number`
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#returns "Direct link to Returns")
`number`
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#defined-in-6 "Direct link to Defined in")
[entities/route.ts:54](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/route.ts#L54)
### midPrice[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#midprice "Direct link to midPrice")
• `get` **midPrice**(): `Price`<`TInput`, `TOutput`>
Returns the mid price of the route
#### Returns[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#returns-1 "Direct link to Returns")
`Price`<`TInput`, `TOutput`>
#### Defined in[​](https://docs.uniswap.org/sdk/v3/reference/classes/Route#defined-in-7 "Direct link to Defined in")
[entities/route.ts:61](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/route.ts#L61)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/Route.md)
Was this helpful?
[PreviousPositionLibrary](https://docs.uniswap.org/sdk/v3/reference/classes/PositionLibrary)[NextSelfPermit](https://docs.uniswap.org/sdk/v3/reference/classes/SelfPermit)
On this page
  * [Type parameters](https://docs.uniswap.org/sdk/v3/reference/classes/Route#type-parameters)
  * [Table of contents](https://docs.uniswap.org/sdk/v3/reference/classes/Route#table-of-contents)
    * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/Route#constructors)
    * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/Route#properties)
    * [Accessors](https://docs.uniswap.org/sdk/v3/reference/classes/Route#accessors)
  * [Constructors](https://docs.uniswap.org/sdk/v3/reference/classes/Route#constructors-1)
    * [constructor](https://docs.uniswap.org/sdk/v3/reference/classes/Route#constructor)
  * [Properties](https://docs.uniswap.org/sdk/v3/reference/classes/Route#properties-1)
    * [_midPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Route#_midprice)
    * [input](https://docs.uniswap.org/sdk/v3/reference/classes/Route#input)
    * [output](https://docs.uniswap.org/sdk/v3/reference/classes/Route#output)
    * [pools](https://docs.uniswap.org/sdk/v3/reference/classes/Route#pools)
    * [tokenPath](https://docs.uniswap.org/sdk/v3/reference/classes/Route#tokenpath)
  * [Accessors](https://docs.uniswap.org/sdk/v3/reference/classes/Route#accessors-1)
    * [chainId](https://docs.uniswap.org/sdk/v3/reference/classes/Route#chainid)
    * [midPrice](https://docs.uniswap.org/sdk/v3/reference/classes/Route#midprice)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/reference/classes/Route.md)
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
