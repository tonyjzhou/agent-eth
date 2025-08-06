# https://docs.uniswap.org/sdk/v4/reference/classes/Hook

[Skip to main content](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#__docusaurus_skipToContent_fallback)
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
        * [Overview](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
          * [Hook](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
          * [Pool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
          * [Position](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
          * [Route](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
          * [Trade](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
          * [V4BaseActionsParser](https://docs.uniswap.org/sdk/v4/reference/classes/V4BaseActionsParser)
          * [V4Planner](https://docs.uniswap.org/sdk/v4/reference/classes/V4Planner)
          * [V4PositionManager](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
          * [V4PositionPlanner](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionPlanner)
        * [enumerations](https://docs.uniswap.org/sdk/v4/reference/enumerations/Actions)
        * [interfaces](https://docs.uniswap.org/sdk/v4/reference/interfaces/AllowanceTransferPermitBatch)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/overview)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Technical Reference
  * classes
  * [Hook](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)


On this page
# Hook
[@uniswap/v4-sdk](https://docs.uniswap.org/sdk/v4/reference/overview) / Hook
Defined in: [utils/hook.ts:40](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/utils/hook.ts#L40)
## Constructors[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#constructors "Direct link to Constructors")
### new Hook()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#new-hook "Direct link to new Hook\(\)")
> **new Hook**(): [`Hook`](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#returns "Direct link to Returns")
[`Hook`](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
## Methods[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#methods "Direct link to Methods")
### hasDonatePermissions()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#hasdonatepermissions "Direct link to hasDonatePermissions\(\)")
> `static` **hasDonatePermissions**(`address`): `boolean`
Defined in: [utils/hook.ts:91](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/utils/hook.ts#L91)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#parameters "Direct link to Parameters")
Parameter| Type  
---|---  
`address`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#returns-1 "Direct link to Returns")
`boolean`
### hasInitializePermissions()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#hasinitializepermissions "Direct link to hasInitializePermissions\(\)")
> `static` **hasInitializePermissions**(`address`): `boolean`
Defined in: [utils/hook.ts:66](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/utils/hook.ts#L66)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#parameters-1 "Direct link to Parameters")
Parameter| Type  
---|---  
`address`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#returns-2 "Direct link to Returns")
`boolean`
### hasLiquidityPermissions()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#hasliquiditypermissions "Direct link to hasLiquidityPermissions\(\)")
> `static` **hasLiquidityPermissions**(`address`): `boolean`
Defined in: [utils/hook.ts:74](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/utils/hook.ts#L74)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#parameters-2 "Direct link to Parameters")
Parameter| Type  
---|---  
`address`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#returns-3 "Direct link to Returns")
`boolean`
### hasPermission()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#haspermission "Direct link to hasPermission\(\)")
> `static` **hasPermission**(`address`, `hookOption`): `boolean`
Defined in: [utils/hook.ts:61](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/utils/hook.ts#L61)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#parameters-3 "Direct link to Parameters")
Parameter| Type  
---|---  
`address`| `string`  
`hookOption`| [`HookOptions`](https://docs.uniswap.org/sdk/v4/reference/enumerations/HookOptions)  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#returns-4 "Direct link to Returns")
`boolean`
### hasSwapPermissions()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#hasswappermissions "Direct link to hasSwapPermissions\(\)")
> `static` **hasSwapPermissions**(`address`): `boolean`
Defined in: [utils/hook.ts:85](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/utils/hook.ts#L85)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#parameters-4 "Direct link to Parameters")
Parameter| Type  
---|---  
`address`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#returns-5 "Direct link to Returns")
`boolean`
### permissions()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#permissions "Direct link to permissions\(\)")
> `static` **permissions**(`address`): [`HookPermissions`](https://docs.uniswap.org/sdk/v4/reference/overview#hookpermissions)
Defined in: [utils/hook.ts:41](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/utils/hook.ts#L41)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#parameters-5 "Direct link to Parameters")
Parameter| Type  
---|---  
`address`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#returns-6 "Direct link to Returns")
[`HookPermissions`](https://docs.uniswap.org/sdk/v4/reference/overview#hookpermissions)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Hook.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/sdk/v4/reference/overview)[NextPool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
On this page
  * [Constructors](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#constructors)
    * [new Hook()](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#new-hook)
  * [Methods](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#methods)
    * [hasDonatePermissions()](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#hasdonatepermissions)
    * [hasInitializePermissions()](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#hasinitializepermissions)
    * [hasLiquidityPermissions()](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#hasliquiditypermissions)
    * [hasPermission()](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#haspermission)
    * [hasSwapPermissions()](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#hasswappermissions)
    * [permissions()](https://docs.uniswap.org/sdk/v4/reference/classes/Hook#permissions)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Hook.md)
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
