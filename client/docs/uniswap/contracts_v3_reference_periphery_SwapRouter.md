# https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#__docusaurus_skipToContent_fallback)
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
  * [SwapRouter](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter)


On this page
# SwapRouter
Router for stateless execution of swaps against Uniswap V3
> Input parameters are viewable on the [**Swap Router Interface**](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/ISwapRouter)
## Functions[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#constructor "Direct link to constructor")
```
functionconstructor()public
```

### uniswapV3SwapCallback[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#uniswapv3swapcallback "Direct link to uniswapV3SwapCallback")
```
functionuniswapV3SwapCallback(int256 amount0Delta,int256 amount1Delta,bytes data)external
```

Called to `msg.sender` after executing a swap via IUniswapV3Pool#swap.
In the implementation you must pay the pool tokens owed for the swap. The caller of this method must be checked to be a UniswapV3Pool deployed by the canonical UniswapV3Factory. amount0Delta and amount1Delta can both be 0 if no tokens were swapped.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#parameters "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`amount0Delta`| int256| The amount of token0 that was sent (negative) or must be received (positive) by the pool by the end of the swap. If positive, the callback must send that amount of token0 to the pool.  
`amount1Delta`| int256| The amount of token1 that was sent (negative) or must be received (positive) by the pool by the end of the swap. If positive, the callback must send that amount of token1 to the pool.  
`data`| bytes| Any data passed through by the caller via the IUniswapV3PoolActions#swap call  
### exactInputSingle[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#exactinputsingle "Direct link to exactInputSingle")
```
functionexactInputSingle(structISwapRouter.ExactInputSingleParams params)externalreturns(uint256 amountOut)
```

Swaps `amountIn` of one token for as much as possible of another token
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#parameters-1 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`params`| struct ISwapRouter.ExactInputSingleParams| The parameters necessary for the swap, encoded as `ExactInputSingleParams` in calldata  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#return-values "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amountOut`| uint256| The amount of the received token  
### exactInput[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#exactinput "Direct link to exactInput")
```
functionexactInput(structISwapRouter.ExactInputParams params)externalreturns(uint256 amountOut)
```

Swaps `amountIn` of one token for as much as possible of another along the specified path
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#parameters-2 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`params`| struct ISwapRouter.ExactInputParams| The parameters necessary for the multi-hop swap, encoded as `ExactInputParams` in calldata  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#return-values-1 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amountOut`| uint256| The amount of the received token  
### exactOutputSingle[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#exactoutputsingle "Direct link to exactOutputSingle")
```
functionexactOutputSingle(structISwapRouter.ExactOutputSingleParams params)externalreturns(uint256 amountIn)
```

Swaps as little as possible of one token for `amountOut` of another token
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#parameters-3 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`params`| struct ISwapRouter.ExactOutputSingleParams| The parameters necessary for the swap, encoded as `ExactOutputSingleParams` in calldata  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#return-values-2 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amountIn`| uint256| The amount of the input token  
### exactOutput[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#exactoutput "Direct link to exactOutput")
```
functionexactOutput(structISwapRouter.ExactOutputParams params)externalreturns(uint256 amountIn)
```

Swaps as little as possible of one token for `amountOut` of another along the specified path (reversed)
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#parameters-4 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`params`| struct ISwapRouter.ExactOutputParams| The parameters necessary for the multi-hop swap, encoded as `ExactOutputParams` in calldata  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#return-values-3 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amountIn`| uint256| The amount of the input token  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/SwapRouter.md)
Was this helpful?
[PreviousNonfungibleTokenPositionDescriptor](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungibleTokenPositionDescriptor)[NextV3Migrator](https://docs.uniswap.org/contracts/v3/reference/periphery/V3Migrator)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#functions)
    * [constructor](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#constructor)
    * [uniswapV3SwapCallback](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#uniswapv3swapcallback)
    * [exactInputSingle](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#exactinputsingle)
    * [exactInput](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#exactinput)
    * [exactOutputSingle](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#exactoutputsingle)
    * [exactOutput](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter#exactoutput)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/SwapRouter.md)
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
