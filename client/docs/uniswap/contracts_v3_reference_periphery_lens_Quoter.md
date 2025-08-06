# https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#__docusaurus_skipToContent_fallback)
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
            * [Quoter](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter)
            * [QuoterV2](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/QuoterV2)
            * [TickLens](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/TickLens)
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
  * Lens
  * [Quoter](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter)


On this page
# Quoter
Allows getting the expected amount out or amount in for a given swap without executing the swap
These functions are not gas efficient and should _not_ be called on chain. Instead, optimistically execute the swap and check the amounts in the callback.
## Functions[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#constructor "Direct link to constructor")
```
functionconstructor()public
```

### uniswapV3SwapCallback[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#uniswapv3swapcallback "Direct link to uniswapV3SwapCallback")
```
functionuniswapV3SwapCallback(int256 amount0Delta,int256 amount1Delta,bytes data)external
```

Called to `msg.sender` after executing a swap via IUniswapV3Pool#swap.
In the implementation you must pay the pool tokens owed for the swap. The caller of this method must be checked to be a UniswapV3Pool deployed by the canonical UniswapV3Factory. amount0Delta and amount1Delta can both be 0 if no tokens were swapped.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#parameters "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`amount0Delta`| int256| The amount of token0 that was sent (negative) or must be received (positive) by the pool by  
the end of the swap. If positive, the callback must send that amount of token0 to the pool. |`amount1Delta` | int256 | The amount of token1 that was sent (negative) or must be received (positive) by the pool by the end of the swap. If positive, the callback must send that amount of token1 to the pool. |`data` | bytes | Any data passed through by the caller via the IUniswapV3PoolActions#swap call
### quoteExactInputSingle[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#quoteexactinputsingle "Direct link to quoteExactInputSingle")
```
functionquoteExactInputSingle(address tokenIn,address tokenOut,uint24 fee,uint256 amountIn,uint160 sqrtPriceLimitX96)publicreturns(uint256 amountOut)
```

Returns the amount out received for a given exact input but for a swap of a single pool
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#parameters-1 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenIn`| address| The token being swapped in  
`tokenOut`| address| The token being swapped out  
`fee`| uint24| The fee of the token pool to consider for the pair  
`amountIn`| uint256| The desired input amount  
`sqrtPriceLimitX96`| uint160| The price limit of the pool that cannot be exceeded by the swap  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#return-values "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amountOut`| uint256| The amount of `tokenOut` that would be received  
### quoteExactInput[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#quoteexactinput "Direct link to quoteExactInput")
```
functionquoteExactInput(bytes path,uint256 amountIn)externalreturns(uint256 amountOut)
```

Returns the amount out received for a given exact input swap without executing the swap
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#parameters-2 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`path`| bytes| The path of the swap, i.e. each token pair and the pool fee  
`amountIn`| uint256| The amount of the first token to swap  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#return-values-1 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amountOut`| uint256| The amount of the last token that would be received  
### quoteExactOutputSingle[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#quoteexactoutputsingle "Direct link to quoteExactOutputSingle")
```
functionquoteExactOutputSingle(address tokenIn,address tokenOut,uint24 fee,uint256 amountOut,uint160 sqrtPriceLimitX96)publicreturns(uint256 amountIn)
```

Returns the amount in required to receive the given exact output amount but for a swap of a single pool
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#parameters-3 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenIn`| address| The token being swapped in  
`tokenOut`| address| The token being swapped out  
`fee`| uint24| The fee of the token pool to consider for the pair  
`amountOut`| uint256| The desired output amount  
`sqrtPriceLimitX96`| uint160| The price limit of the pool that cannot be exceeded by the swap  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#return-values-2 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amountIn`| uint256| The amount required as the input for the swap in order to receive `amountOut`  
### quoteExactOutput[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#quoteexactoutput "Direct link to quoteExactOutput")
```
functionquoteExactOutput(bytes path,uint256 amountOut)externalreturns(uint256 amountIn)
```

Returns the amount in required for a given exact output swap without executing the swap
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#parameters-4 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`path`| bytes| The path of the swap, i.e. each token pair and the pool fee  
`amountOut`| uint256| The amount of the last token to receive  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#return-values-3 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`amountIn`| uint256| The amount of first token required to be paid  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/lens/Quoter.md)
Was this helpful?
[PreviousIWETH9](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/external/IWETH9)[NextQuoterV2](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/QuoterV2)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#functions)
    * [constructor](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#constructor)
    * [uniswapV3SwapCallback](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#uniswapv3swapcallback)
    * [quoteExactInputSingle](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#quoteexactinputsingle)
    * [quoteExactInput](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#quoteexactinput)
    * [quoteExactOutputSingle](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#quoteexactoutputsingle)
    * [quoteExactOutput](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter#quoteexactoutput)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/lens/Quoter.md)
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
