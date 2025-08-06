# https://docs.uniswap.org/contracts/v4/quickstart/hooks/async-swap

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/hooks/async-swap#__docusaurus_skipToContent_fallback)
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
          * [Set Up Local Environment](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
          * [Swap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap)
          * [Liquidity Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
          * [AsyncSwap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/async-swap)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
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
  * Quickstart
  * Hooks
  * [AsyncSwap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/async-swap)


One feature enabled by [custom accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting) is​​​​‌ AsyncSwap swap. This feature allows hook developers to replace the v4 (v3-style) swap logic.
This means developers can replace Uniswap's internal core logic for how to handle swaps. Two emergent use-cases are possible with custom accounting:
  1. Asynchronous swaps and swap-ordering. Delay the v4 swap logic for fulfillment at a later time.
  2. Custom Curves. Replace the v4 swap logic with different swap logic. The custom logic is flexible and developers can implement symmetric curves, asymmetric curves, or custom quoting.


> AsyncSwap is typically described as taking the full input to replace the internal swap logic, partially taking the input is better described as _custom accounting_
Note: The flexibility of AsyncSwap means hook developers can implement harmful behavior (such as taking all swap amounts for themselves, charging extra fees, etc.). Hooks with AsyncSwap behavior should be examined very closely by both developers and users.
# Configure a AsyncSwap Hook
To enable AsyncSwap, developers will need the hook permission `BEFORE_SWAP_RETURNS_DELTA_FLAG`
```
import{BaseHook}from"v4-periphery/BaseHook.sol";// ...contractAsyncSwapHookis BaseHook {// ...functiongetHookPermissions()publicpure virtual override returns(Hooks.Permissions memory){return Hooks.Permissions({      beforeInitialize:false,      afterInitialize:false,      beforeAddLiquidity:false,      afterAddLiquidity:false,      beforeRemoveLiquidity:false,      afterRemoveLiquidity:false,      beforeSwap:false,      afterSwap:true,      beforeDonate:false,      afterDonate:false,      beforeSwapReturnDelta:true,      afterSwapReturnDelta:false,      afterAddLiquidityReturnDelta:false,      afterRemoveLiquidityReturnDelta:false});}// ...}
```

# beforeSwap
AsyncSwap only works on exact-input swaps and the _beforeSwap_ **must** take the input currency and return [`BeforeSwapDelta`](https://docs.uniswap.org/contracts/v4/reference/core/types/beforeswapdelta). The hook should `IPoolManager.mint` itself the corresponding tokens equal to the amount of the input (`amountSpecified`). It should then return a `BeforeSwapDelta` where `deltaSpecified = -amountSpecified` (the positive amount).
The funds' movements are as follows:
  1. User initiates a swap, specifying -100 tokenA as input
  2. The hook's beforeSwap takes 100 tokenA for itself, and returns a value of 100 to PoolManager.
  3. The PoolManager accounts the 100 tokens against the swap input, leaving 0 tokens remaining
  4. The PoolManager does not execute swap logic, as there are no tokens left to swap
  5. The PoolManager transfers the delta from the hook to the swap router, in step 2 the hook created a debt (that must be paid)
  6. The swap router pays off the debt using the user's tokens


```
contractAsyncSwapHookis BaseHook {// ...function_beforeSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata params,bytescalldata)internal    overridereturns(bytes4, BeforeSwapDelta,uint24){// AsyncSwap only works on exact-input swapsif(params.amountSpecified <0){// take the input token so that v3-swap is skipped...      Currency input = params.zeroForOne ? key.currency0 : key.currency1;uint256 amountTaken =uint256(-params.amountSpecified);      poolManager.mint(address(this), input.toId(), amountTaken);// to AsyncSwap the exact input, we return the amount that's taken by the hookreturn(BaseHook.beforeSwap.selector,toBeforeSwapDelta(amountTaken.toInt128(),0),0);}else{return(BaseHook.beforeSwap.selector, BeforeSwapDeltaLibrary.ZERO,0);}}}
```

# Testing
To verify the AsyncSwap behaved properly, developers should test the swap and that token balances match expected behavior.
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.19;import"forge-std/Test.sol";import{Deployers}from"v4-core/test/utils/Deployers.sol";// ...contractAsyncSwapTestis Test, Deployers {// ...functionsetUp()public{// ... }functiontest_asyncSwap()public{assertEq(hook.beforeSwapCount(poolId),0);uint256 balance0Before = currency0.balanceOfSelf();uint256 balance1Before = currency1.balanceOfSelf();// Perform a test swap //int256 amount =-1e18;bool zeroForOne =true;    BalanceDelta swapDelta =swap(poolKey, zeroForOne, amount, ZERO_BYTES);// ------------------- //uint256 balance0After = currency0.balanceOfSelf();uint256 balance1After = currency1.balanceOfSelf();// user paid token0assertEq(balance0Before - balance0After,1e18);// user did not recieve token1 (AsyncSwap)assertEq(balance1Before, balance1After);}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/04-hooks/03-async-swap.mdx)
Was this helpful?
[PreviousLiquidity Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)[NextSubscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/04-hooks/03-async-swap.mdx)
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
