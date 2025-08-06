# https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#__docusaurus_skipToContent_fallback)
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
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
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
  * v4 Protocol
  * Technical Reference
  * Core
  * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)


On this page
The `IPoolManager` interface defines the main methods for interacting with the Uniswap V4 pool manager contract. It exposes the core _swap lifecycle_ operations
# ModifyLiquidityParams
Structure used to modify liquidity in a pool.
  * `tickLower`: Lower tick boundary of the position
  * `tickUpper`: Upper tick boundary of the position
  * `liquidityDelta`: Amount of liquidity to add (positive) or remove (negative)
  * `salt`: A value to set if you want unique liquidity positions at the same range


Used in the `modifyLiquidity` function to add or remove liquidity from a specific position in the pool.
# SwapParams
Structure used to execute a swap in a pool.
  * `zeroForOne`: Direction of the swap (true for token0 to token1, false for token1 to token0)
  * `amountSpecified`: The desired input amount if negative (exactIn), or the desired output amount if positive (exactOut)
  * `sqrtPriceLimitX96`: Slippage limit represented as [Q64X96](https://uniswapv3book.com/milestone_3/more-on-fixed-point-numbers.html#:~:text=The%20Q64.,and%2018%20signify%20decimal%20places.) notation


Used in the `swap` function to define the behavior of our swap.
# Methods
## initialize[​](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#initialize "Direct link to initialize")
```
functioninitialize(PoolKey memory key,uint160 sqrtPriceX96)externalreturns(int24 tick);
```

Initialize a new pool by defining its parameters: token pair, fee tier, tick spacing, hook contract, and starting price
Param Name| Type| Description  
---|---|---  
key| PoolKey| The key defining the pool to initialize  
sqrtPriceX96| uint160| The initial sqrt price of the pool as a Q64.96 value  
Returns the initial tick value of the pool.
## unlock[​](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#unlock "Direct link to unlock")
```
functionunlock(bytescalldata data)externalreturns(bytesmemory);
```

Provides a single entry point for all pool operations. The provided data is passed to the callback for execution.
Param Name| Type| Description  
---|---|---  
data| bytes| Any data to pass to the callback via `IUnlockCallback(msg.sender).unlockCallback(data)`  
Returns the data returned by the callback.
## modifyLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#modifyliquidity "Direct link to modifyLiquidity")
```
functionmodifyLiquidity(  PoolKey memory key,  ModifyLiquidityParams memory params,bytescalldata hookData)externalreturns(BalanceDelta, BalanceDelta);
```

Modifies the liquidity for the given pool. Can be used to add or remove liquidity, or collect fees
> passing zero will collect fees for the given tick range
Param Name| Type| Description  
---|---|---  
key| PoolKey| The key of the pool to modify liquidity in  
params| ModifyLiquidityParams| The parameters for modifying the liquidity position  
hookData| bytes| Any data to pass to a hook contract on the before/add liquidity hooks  
Returns the balance delta for the caller (total of principal and fees) and the fee delta generated in the liquidity range.
## swap[​](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#swap "Direct link to swap")
```
functionswap(PoolKey memory key, SwapParams memory params,bytescalldata hookData)externalreturns(BalanceDelta);
```

Executes a swap against the given pool using the provided parameters.
Param Name| Type| Description  
---|---|---  
key| PoolKey| The key of the pool to swap in  
params| SwapParams| The parameters for executing the swap  
hookData| bytes| Any data to pass to a hook contract on the before/afterSwap hooks  
Returns the balance delta for the address initiating the swap. Swapping on low liquidity pools may cause unexpected swap amounts when liquidity available is less than amountSpecified. Additionally note that if interacting with hooks that have the BEFORE_SWAP_RETURNS_DELTA_FLAG or AFTER_SWAP_RETURNS_DELTA_FLAG, the hook may alter the swap input/output. Integrators should perform checks on the returned swapDelta.
## donate[​](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#donate "Direct link to donate")
```
functiondonate(PoolKey memory key,uint256 amount0,uint256 amount1,bytescalldata hookData)externalreturns(BalanceDelta);
```

Donates the specified currency amounts to the pool.
Param Name| Type| Description  
---|---|---  
key| PoolKey| The key of the pool to donate to  
amount0| uint256| The amount of token0 to donate  
amount1| uint256| The amount of token1 to donate  
hookData| bytes| Any data to pass to a hook contract on the before/afterDonate hooks  
Returns the balance delta representing the donated amounts.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/IPoolManager.mdx)
Was this helpful?
[PreviousExttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)[NextNoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
On this page
  * [initialize](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#initialize)
  * [unlock](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#unlock)
  * [modifyLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#modifyliquidity)
  * [swap](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#swap)
  * [donate](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager#donate)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/IPoolManager.mdx)
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
