# https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager

[Skip to main content](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
        * [Swaps](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
        * [Position Management](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
        * [Advanced](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
        * [Overview](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
          * [Hook](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
          * [Pool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
          * [Position](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
          * [Route](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
          * [Trade](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
          * [V4BaseActionsParser](https://docs.uniswap.org/sdk/v4/reference/classes/V4BaseActionsParser)
          * [V4Planner](https://docs.uniswap.org/sdk/v4/reference/classes/V4Planner)
          * [V4PositionManager](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
          * [V4PositionPlanner](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionPlanner)
        * [enumerations](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
        * [interfaces](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
    * [web3-react](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Technical Reference
  * classes
  * [V4PositionManager](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)


On this page
# V4PositionManager
[@uniswap/v4-sdk](https://docs.uniswap.org/sdk/v4/reference/overview) / V4PositionManager
Defined in: [PositionManager.ts:206](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L206)
## Properties[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#properties "Direct link to Properties")
### INTERFACE[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#interface "Direct link to INTERFACE")
> `static` **INTERFACE** : `Interface`
Defined in: [PositionManager.ts:207](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L207)
## Methods[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#methods "Direct link to Methods")
### addCallParameters()[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#addcallparameters "Direct link to addCallParameters\(\)")
> `static` **addCallParameters**(`position`, `options`): [`MethodParameters`](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
Defined in: [PositionManager.ts:224](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L224)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#parameters "Direct link to Parameters")
Parameter| Type  
---|---  
`position`| [`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)  
`options`| [`AddLiquidityOptions`](https://docs.uniswap.org/sdk/v4/reference/overview#addliquidityoptions)  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#returns "Direct link to Returns")
[`MethodParameters`](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
### collectCallParameters()[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#collectcallparameters "Direct link to collectCallParameters\(\)")
> `static` **collectCallParameters**(`position`, `options`): [`MethodParameters`](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
Defined in: [PositionManager.ts:387](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L387)
Produces the calldata for collecting fees from a position
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#parameters-1 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`position`| [`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)| The position to collect fees from  
`options`| [`CollectOptions`](https://docs.uniswap.org/sdk/v4/reference/overview#collectoptions)| Additional information necessary for generating the calldata  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#returns-1 "Direct link to Returns")
[`MethodParameters`](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
The call parameters
### createCallParameters()[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#createcallparameters "Direct link to createCallParameters\(\)")
> `static` **createCallParameters**(`poolKey`, `sqrtPriceX96`): [`MethodParameters`](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
Defined in: [PositionManager.ts:217](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L217)
Public methods to encode method parameters for different actions on the PositionManager contract
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#parameters-2 "Direct link to Parameters")
Parameter| Type  
---|---  
`poolKey`| [`PoolKey`](https://docs.uniswap.org/sdk/v4/reference/overview#poolkey)  
`sqrtPriceX96`| `BigintIsh`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#returns-2 "Direct link to Returns")
[`MethodParameters`](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
### encodeERC721Permit()[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#encodeerc721permit "Direct link to encodeERC721Permit\(\)")
> `static` **encodeERC721Permit**(`spender`, `tokenId`, `deadline`, `nonce`, `signature`): `string`
Defined in: [PositionManager.ts:435](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L435)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#parameters-3 "Direct link to Parameters")
Parameter| Type  
---|---  
`spender`| `string`  
`tokenId`| `BigintIsh`  
`deadline`| `BigintIsh`  
`nonce`| `BigintIsh`  
`signature`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#returns-3 "Direct link to Returns")
`string`
### encodeModifyLiquidities()[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#encodemodifyliquidities "Direct link to encodeModifyLiquidities\(\)")
> `static` **encodeModifyLiquidities**(`unlockData`, `deadline`): `string`
Defined in: [PositionManager.ts:421](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L421)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#parameters-4 "Direct link to Parameters")
Parameter| Type  
---|---  
`unlockData`| `string`  
`deadline`| `BigintIsh`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#returns-4 "Direct link to Returns")
`string`
### encodePermitBatch()[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#encodepermitbatch "Direct link to encodePermitBatch\(\)")
> `static` **encodePermitBatch**(`owner`, `permitBatch`, `signature`): `string`
Defined in: [PositionManager.ts:426](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L426)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#parameters-5 "Direct link to Parameters")
Parameter| Type  
---|---  
`owner`| `string`  
`permitBatch`| [`AllowanceTransferPermitBatch`](https://docs.uniswap.org/sdk/v4/reference/interfaces/AllowanceTransferPermitBatch)  
`signature`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#returns-5 "Direct link to Returns")
`string`
### getPermitData()[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#getpermitdata "Direct link to getPermitData\(\)")
> `static` **getPermitData**(`permit`, `positionManagerAddress`, `chainId`): [`NFTPermitData`](https://docs.uniswap.org/sdk/v4/reference/interfaces/NFTPermitData)
Defined in: [PositionManager.ts:452](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L452)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#parameters-6 "Direct link to Parameters")
Parameter| Type  
---|---  
`permit`| [`NFTPermitValues`](https://docs.uniswap.org/sdk/v4/reference/interfaces/NFTPermitValues)  
`positionManagerAddress`| `string`  
`chainId`| `number`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#returns-6 "Direct link to Returns")
[`NFTPermitData`](https://docs.uniswap.org/sdk/v4/reference/interfaces/NFTPermitData)
### removeCallParameters()[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#removecallparameters "Direct link to removeCallParameters\(\)")
> `static` **removeCallParameters**(`position`, `options`): [`MethodParameters`](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
Defined in: [PositionManager.ts:314](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L314)
Produces the calldata for completely or partially exiting a position
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#parameters-7 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`position`| [`Position`](https://docs.uniswap.org/sdk/v4/reference/classes/Position)| The position to exit  
`options`| [`RemoveLiquidityOptions`](https://docs.uniswap.org/sdk/v4/reference/overview#removeliquidityoptions)| Additional information necessary for generating the calldata  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#returns-7 "Direct link to Returns")
[`MethodParameters`](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
The call parameters
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/V4PositionManager.md)
Was this helpful?
[PreviousV4Planner](https://docs.uniswap.org/sdk/v4/reference/classes/V4Planner)[NextV4PositionPlanner](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionPlanner)
On this page
  * [Properties](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#properties)
    * [INTERFACE](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#interface)
  * [Methods](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#methods)
    * [addCallParameters()](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#addcallparameters)
    * [collectCallParameters()](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#collectcallparameters)
    * [createCallParameters()](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#createcallparameters)
    * [encodeERC721Permit()](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#encodeerc721permit)
    * [encodeModifyLiquidities()](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#encodemodifyliquidities)
    * [encodePermitBatch()](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#encodepermitbatch)
    * [getPermitData()](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#getpermitdata)
    * [removeCallParameters()](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager#removecallparameters)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/V4PositionManager.md)
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
