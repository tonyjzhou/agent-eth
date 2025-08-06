# https://docs.uniswap.org/contracts/v4/guides/unlock-callback

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#__docusaurus_skipToContent_fallback)
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
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
        * [Unlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)
        * [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)
        * [Custom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Swap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Position Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)
        * [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Access msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
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
  * Guides
  * [Unlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)


On this page
# Unlock Callback & Deltas
### Refresher[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#refresher "Direct link to Refresher")
In order to have access to the liquidity inside the `PoolManager`, it needs to be _unlocked_ to begin with. After being unlocked, any number of operations can be executed, which at the end of must be _locked_ again. At this point, if there are any _non-zero deltas_ , meaning the PoolManager is owed or owes tokens back to some address, the whole execution reverts. Otherwise, both parties have paid or received the right amount of tokens and the operations have successfully carried out.
# Unlocking the PoolManager
### Implementing the unlock callback[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#implementing-the-unlock-callback "Direct link to Implementing the unlock callback")
Prior to unlocking the PoolManager, the integrating contract must implement the `unlockCallback` function. This function will be called by the PoolManager after being unlocked. An easy way to do this is to inherit the `SafeCallback` abstract contract.
```
import{SafeCallback}from"v4-periphery/src/base/SafeCallback.sol";contractIntegratingContractis SafeCallback {constructor(IPoolManager _poolManager)SafeCallback(_poolManager){}}
```

### Calling the unlock function[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#calling-the-unlock-function "Direct link to Calling the unlock function")
After implementing the callback, the integrating contract can now invoke the `unlock()` function. It receives a _bytes_ parameter that is further passed to your callback function as an argument. This parameter is used to encode the sequence of operations to be executed in the context of the `PoolManager`.
```
bytesmemory unlockData = abi.encode(encode_operations_here);bytesmemory unlockResultData = poolManager.unlock(unlockData);
```

Next, we must override the `_unlockCallback` function inherited from the `SafeCallback` contract. In your implementation, you should decode your operations and continue with the desired logic.
```
function_unlockCallback(bytescalldata data)internal override returns(bytesmemory){(...)= abi.decode(data,(...));}
```

# Operations
There are **9** operations that can be done in the `PoolManager` which fall in two categories: _liquidity-accessing_ and _delta-resolving_.
### Deltas[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#deltas "Direct link to Deltas")
Deltas are the `PoolManager`'s method to keep track of token amounts it needs to receive, respectively to distribute. A negative delta signals that the `PoolManager` is owed tokens, while a positive one expresses a token balance that needs to be paid to its user.
### Liquidity-accessing[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#liquidity-accessing "Direct link to Liquidity-accessing")
_Liquidity-accessing_ operations will create non-zero _deltas_ and produce a state transition of the selected pool. They are the following:
  * _modify liquidity_ - used to increase or decrease liquidity; increasing liquidity will result in a negative token delta, while decreasing yields a positive one
  * _swap_ - used to trade one token for another; will result in a negative tokenA delta and a positive tokenB delta
  * _donate_ - used to provide direct token revenue to positions in range; will result in a negative delta for the pool's tokens the user wishes to provide


### Delta-resolving[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#delta-resolving "Direct link to Delta-resolving")
_Delta-resolving_ operations are used to even out the deltas created by the _liquidity-accessing_ operations. They are the following:
  * _settle_ - used following token transfers to the manager or burning of ERC6909 claims to resolve negative deltas
  * _take_ - transfer tokens from the manager, used to resolve positive deltas but also provide token loans, producing negative deltas
  * _mint_ - used to create ERC6909 claims, creating a negative delta that needs to be resolved by transferring the corresponding token and _settling_ afterwards
  * _burn_ - removes ERC6909 claims, creating a positive delta for tokens to be transferred back to the owner or used in settling negative balances
  * _clear_ - used to zero out positive token deltas, helpful to forfeit insignificant token amounts in order to avoid paying further transfer costs


### Handling Deltas for Liquidity Modifications[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#handling-deltas-for-liquidity-modifications "Direct link to Handling Deltas for Liquidity Modifications")
#### When it happens[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#when-it-happens "Direct link to When it happens")
  * **Building custom routers** that pre-calculate token amounts.
  * **Estimating values** for user interfaces or simulations.


#### Why It Happens[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#why-it-happens "Direct link to Why It Happens")
  * **Pre-calculated amounts** (e.g., from `LiquidityAmounts.getAmountsForLiquidity()`) use static math.
  * **Actual deltas** (from `modifyLiquidity()`) reflect real-time pool state, including: 
    * Tick crossings during execution.
    * Rounding in fixed-point arithmetic (`Q128.128`).


#### Why LiquidityAmounts ≠ Liquidity Delta[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#why-liquidityamounts--liquidity-delta "Direct link to Why LiquidityAmounts ≠ Liquidity Delta")
The discrepancy occurs because:
  * **Price Movement:** The pool's price changes between pre-calculation and execution
  * **Tick Crossings:** Transactions may cross ticks, changing liquidity math
  * **Rounding:** Static calculations use idealized math while execution uses Q128.128 fixed-point


#### 📊 Price Movement Example[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#-price-movement-example "Direct link to 📊 Price Movement Example")
When ETH/USDC price changes during transaction execution:
```
// Static math calculationLiquidityAmounts.getAmountsForLiquidity(  sqrtRatioX96:3000,// Fixed price...);// Interacts with the pool and uses actual execution (reflects real-time price)poolManager.modifyLiquidity(  sqrtRatioX96:3001,// Updated price...);
```

getAmountsForLiquidity() assumes static 3000 price modifyLiquidity() reflects actual 3001 price
#### Key Impact[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#key-impact "Direct link to Key Impact")
Scenario| Risk  
---|---  
**Underestimating deltas**|  Transactions revert with `CurrencyNotSettled`.  
**Overestimating deltas**|  Users overpay and lose funds to residual dust.  
**No slippage check**|  Significant financial losses.  
#### Best Practices for Custom Routers[​](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#best-practices-for-custom-routers "Direct link to Best Practices for Custom Routers")
  1. **Never settle without validating against slippage**
Supposing slippage tolerance is 50 (basis point)
```
require(    actualAmount0 >= expectedAmount0 *(10_000 - slippageTolerance)/10_000,"Slippage too high (token0)");require(    actualAmount1 >= expectedAmount1 *(10_000 - slippageTolerance)/10_000,"Slippage too high (token1)");
```

  2. **Use Deltas for Settlement** Always derive final amounts from `modifyLiquidity()` deltas:
```
   CallbackData memory _data = abi.decode(data,(CallbackData));(BalanceDelta delta,)= poolManager.modifyLiquidity(     _data.key,     _data.params,     hex"");   _data.key.currency0.settle(poolManager, _data.key.hookAddress, delta.amount0()<0?uint256(uint128(-delta.amount0())):uint256(uint128(delta.amount0())),false);   _data.key.currency1.settle(poolManager, _data.key.hookAddress, delta.amount1()<0?uint256(uint128(-delta.amount1())):uint256(uint128(delta.amount1())),false);
```



> ⚠️ **Custom Router Pitfall** When pre-calculating liquidity changes, always account for rounding differences. **Never** assume `getAmountsForLiquidity() == modifyLiquidity()` deltas. Enforce slippage post-execution.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/06-unlock-callback.mdx)
Was this helpful?
[PreviousHook Deployment](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)[NextReading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)
On this page
  * [Refresher](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#refresher)
  * [Implementing the unlock callback](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#implementing-the-unlock-callback)
  * [Calling the unlock function](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#calling-the-unlock-function)
  * [Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#deltas)
  * [Liquidity-accessing](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#liquidity-accessing)
  * [Delta-resolving](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#delta-resolving)
  * [Handling Deltas for Liquidity Modifications](https://docs.uniswap.org/contracts/v4/guides/unlock-callback#handling-deltas-for-liquidity-modifications)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/06-unlock-callback.mdx)
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
