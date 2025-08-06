# https://docs.uniswap.org/sdk/v4/reference/classes/Route

[Skip to main content](https://docs.uniswap.org/sdk/v4/reference/classes/Route#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
        * [Swaps](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
        * [Position Management](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
        * [Advanced](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
        * [Overview](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
          * [Hook](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
          * [Pool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
          * [Position](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
          * [Route](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
          * [Trade](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
          * [V4BaseActionsParser](https://docs.uniswap.org/sdk/v4/reference/classes/V4BaseActionsParser)
          * [V4Planner](https://docs.uniswap.org/sdk/v4/reference/classes/V4Planner)
          * [V4PositionManager](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
          * [V4PositionPlanner](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionPlanner)
        * [enumerations](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
        * [interfaces](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
    * [web3-react](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Technical Reference
  * classes
  * [Route](https://docs.uniswap.org/sdk/v4/reference/classes/Route)


On this page
# Route
[@uniswap/v4-sdk](https://docs.uniswap.org/sdk/v4/reference/overview) / Route
Defined in: [entities/route.ts:12](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L12)
Represents a list of pools through which a swap can occur
## Type Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#type-parameters "Direct link to Type Parameters")
Type Parameter| Description  
---|---  
`TInput` _extends_ `Currency`| The input currency  
`TOutput` _extends_ `Currency`| The output currency  
## Constructors[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#constructors "Direct link to Constructors")
### new Route()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#new-route "Direct link to new Route\(\)")
> **new Route** <`TInput`, `TOutput`>(`pools`, `input`, `output`): [`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>
Defined in: [entities/route.ts:28](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L28)
Creates an instance of route.
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#parameters "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`pools`| [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)[]| An array of `Pool` objects, ordered by the route the swap will take  
`input`| `TInput`| The input currency  
`output`| `TOutput`| The output currency  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#returns "Direct link to Returns")
[`Route`](https://docs.uniswap.org/sdk/v4/reference/classes/Route)<`TInput`, `TOutput`>
## Properties[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#properties "Direct link to Properties")
### currencyPath[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#currencypath "Direct link to currencyPath")
> `readonly` **currencyPath** : `Currency`[]
Defined in: [entities/route.ts:14](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L14)
### input[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#input "Direct link to input")
> `readonly` **input** : `TInput`
Defined in: [entities/route.ts:15](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L15)
### output[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#output "Direct link to output")
> `readonly` **output** : `TOutput`
Defined in: [entities/route.ts:16](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L16)
### pathInput[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#pathinput "Direct link to pathInput")
> `readonly` **pathInput** : `Currency`
Defined in: [entities/route.ts:17](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L17)
### pathOutput[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#pathoutput "Direct link to pathOutput")
> `readonly` **pathOutput** : `Currency`
Defined in: [entities/route.ts:18](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L18)
### pools[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#pools "Direct link to pools")
> `readonly` **pools** : [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)[]
Defined in: [entities/route.ts:13](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L13)
## Accessors[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#accessors "Direct link to Accessors")
### chainId[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#chainid "Direct link to chainId")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#get-signature "Direct link to Get Signature")
> **get** **chainId**(): `number`
Defined in: [entities/route.ts:58](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L58)
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#returns-1 "Direct link to Returns")
`number`
### midPrice[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#midprice "Direct link to midPrice")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#get-signature-1 "Direct link to Get Signature")
> **get** **midPrice**(): `Price`<`TInput`, `TOutput`>
Defined in: [entities/route.ts:65](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/route.ts#L65)
Returns the mid price of the route
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Route#returns-2 "Direct link to Returns")
`Price`<`TInput`, `TOutput`>
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Route.md)
Was this helpful?
[PreviousPosition](https://docs.uniswap.org/sdk/v4/reference/classes/Position)[NextTrade](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
On this page
  * [Type Parameters](https://docs.uniswap.org/sdk/v4/reference/classes/Route#type-parameters)
  * [Constructors](https://docs.uniswap.org/sdk/v4/reference/classes/Route#constructors)
    * [new Route()](https://docs.uniswap.org/sdk/v4/reference/classes/Route#new-route)
  * [Properties](https://docs.uniswap.org/sdk/v4/reference/classes/Route#properties)
    * [currencyPath](https://docs.uniswap.org/sdk/v4/reference/classes/Route#currencypath)
    * [input](https://docs.uniswap.org/sdk/v4/reference/classes/Route#input)
    * [output](https://docs.uniswap.org/sdk/v4/reference/classes/Route#output)
    * [pathInput](https://docs.uniswap.org/sdk/v4/reference/classes/Route#pathinput)
    * [pathOutput](https://docs.uniswap.org/sdk/v4/reference/classes/Route#pathoutput)
    * [pools](https://docs.uniswap.org/sdk/v4/reference/classes/Route#pools)
  * [Accessors](https://docs.uniswap.org/sdk/v4/reference/classes/Route#accessors)
    * [chainId](https://docs.uniswap.org/sdk/v4/reference/classes/Route#chainid)
    * [midPrice](https://docs.uniswap.org/sdk/v4/reference/classes/Route#midprice)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Route.md)
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
