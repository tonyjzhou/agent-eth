# https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/periphery/base/BlockTimestamp)
          * [Base](https://docs.uniswap.org/contracts/v3/reference/periphery/base/BlockTimestamp)
          * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/IERC20Metadata)
          * [Lens](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter)
          * [Libraries](https://docs.uniswap.org/contracts/v3/reference/periphery/libraries/Base64)
          * [Staker](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
          * [Test](https://docs.uniswap.org/contracts/v3/reference/periphery/test/Base64Test)
          * [NonfungiblePositionManager](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager)
          * [NonfungibleTokenPositionDescriptor](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungibleTokenPositionDescriptor)
          * [SwapRouter](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter)
          * [V3Migrator](https://docs.uniswap.org/contracts/v3/reference/periphery/V3Migrator)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * Periphery
  * [NonfungiblePositionManager](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager)


On this page
# NonfungiblePositionManager
Wraps Uniswap V3 positions in the ERC721 non-fungible token interface
## Functions[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#constructor "Direct link to constructor")
```
functionconstructor()public
```

### positions[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#positions "Direct link to positions")
```
functionpositions(uint256 tokenId)externalviewreturns(uint96 nonce,address operator,address token0,address token1,uint24 fee,int24 tickLower,int24 tickUpper,uint128 liquidity,uint256 feeGrowthInside0LastX128,uint256 feeGrowthInside1LastX128,uint128 tokensOwed0,uint128 tokensOwed1)
```

Returns the position information associated with a given token ID.
Throws if the token ID is not valid.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#parameters "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenId`| uint256| The ID of the token that represents the position  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#return-values "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`nonce`| uint96| The nonce for permits  
`operator`| address| The address that is approved for spending  
`token0`| address| The address of the token0 for a specific pool  
`token1`| address| The address of the token1 for a specific pool  
`fee`| uint24| The fee associated with the pool  
`tickLower`| int24| The lower end of the tick range for the position  
`tickUpper`| int24| The higher end of the tick range for the position  
`liquidity`| uint128| The liquidity of the position  
`feeGrowthInside0LastX128`| uint256| The fee growth of token0 as of the last action on the individual position  
`feeGrowthInside1LastX128`| uint256| The fee growth of token1 as of the last action on the individual position  
`tokensOwed0`| uint128| The uncollected amount of token0 owed to the position as of the last computation  
`tokensOwed1`| uint128| The uncollected amount of token1 owed to the position as of the last computation  
### mint[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#mint "Direct link to mint")
```
functionmint(structINonfungiblePositionManager.MintParams params)externalreturns(uint256 tokenId,uint128 liquidity,uint256 amount0,uint256 amount1)
```

Creates a new position wrapped in a NFT
Call this when the pool does exist and is initialized. Note that if the pool is created but not initialized a method does not exist, i.e. the pool is assumed to be initialized.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#parameters-1 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`params`| struct INonfungiblePositionManager.MintParams| The params necessary to mint a position, encoded as `MintParams` in calldata  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#return-values-1 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`tokenId`| uint256| The ID of the token that represents the minted position  
`liquidity`| uint128| The amount of liquidity for this position  
`amount0`| uint256| The amount of token0  
`amount1`| uint256| The amount of token1  
### tokenURI[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#tokenuri "Direct link to tokenURI")
```
functiontokenURI(uint256 tokenId)publicviewreturns(string)
```

Returns a URI describing a particular token ID
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#parameters-2 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenId`| uint256| The ID of the token that represents the minted position  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#return-values-2 "Direct link to Return Values:")
A base64 string with the URI data.
### baseURI[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#baseuri "Direct link to baseURI")
```
functionbaseURI()publicreturns(string)
```

### increaseLiquidity[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#increaseliquidity "Direct link to increaseLiquidity")
```
functionincreaseLiquidity(structINonfungiblePositionManager.IncreaseLiquidityParams params)externalreturns(uint128 liquidity,uint256 amount0,uint256 amount1)
```

Increases the amount of liquidity in a position, with tokens paid by the `msg.sender`
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#parameters-3 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`params`| struct INonfungiblePositionManager.IncreaseLiquidityParams| tokenId The ID of the token for which liquidity is being increased,  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#return-values-3 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`liquidity`| uint128| The new liquidity amount as a result of the increase  
`amount0`| uint256| The amount of token0 to achieve resulting liquidity  
`amount1`| uint256| The amount of token1 to achieve resulting liquidity  
### decreaseLiquidity[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#decreaseliquidity "Direct link to decreaseLiquidity")
```
functiondecreaseLiquidity(structINonfungiblePositionManager.DecreaseLiquidityParams params)externalreturns(uint256 amount0,uint256 amount1)
```

Decreases the amount of liquidity in a position and accounts it to the position
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#parameters-4 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`params`| struct INonfungiblePositionManager.DecreaseLiquidityParams| tokenId The ID of the token for which liquidity is being decreased,  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#return-values-4 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amount0`| uint256| The amount of token0 accounted to the position's tokens owed  
`amount1`| uint256| The amount of token1 accounted to the position's tokens owed  
### collect[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#collect "Direct link to collect")
```
functioncollect(structINonfungiblePositionManager.CollectParams params)externalreturns(uint256 amount0,uint256 amount1)
```

Collects up to a maximum amount of fees owed to a specific position to the recipient
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#parameters-5 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`params`| struct INonfungiblePositionManager.CollectParams| tokenId The ID of the NFT for which tokens are being collected,  
recipient The account that should receive the tokens, amount0Max The maximum amount of token0 to collect, amount1Max The maximum amount of token1 to collect
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#return-values-5 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amount0`| uint256| The amount of fees collected in token0  
`amount1`| uint256| The amount of fees collected in token1  
### burn[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#burn "Direct link to burn")
```
functionburn(uint256 tokenId)external
```

Burns a token ID, which deletes it from the NFT contract. The token must have 0 liquidity and all tokens must be collected first.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#parameters-6 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenId`| uint256| The ID of the token that is being burned  
### _getAndIncrementNonce[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#_getandincrementnonce "Direct link to _getAndIncrementNonce")
```
function_getAndIncrementNonce()internalreturns(uint256)
```

### getApproved[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#getapproved "Direct link to getApproved")
```
functiongetApproved()publicviewreturns(address)
```

Returns the account approved for `tokenId` token. Requirements:
  * `tokenId` must exist.


### _approve[​](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#_approve "Direct link to _approve")
```
function_approve()internal
```

Overrides _approve to use the operator in the position, which is packed with the position permit nonce
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/NonfungiblePositionManager.md)
Was this helpful?
[PreviousTickLensTest](https://docs.uniswap.org/contracts/v3/reference/periphery/test/TickLensTest)[NextNonfungibleTokenPositionDescriptor](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungibleTokenPositionDescriptor)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#functions)
    * [constructor](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#constructor)
    * [positions](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#positions)
    * [mint](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#mint)
    * [tokenURI](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#tokenuri)
    * [baseURI](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#baseuri)
    * [increaseLiquidity](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#increaseliquidity)
    * [decreaseLiquidity](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#decreaseliquidity)
    * [collect](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#collect)
    * [burn](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#burn)
    * [_getAndIncrementNonce](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#_getandincrementnonce)
    * [getApproved](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#getapproved)
    * [_approve](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#_approve)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/NonfungiblePositionManager.md)
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
