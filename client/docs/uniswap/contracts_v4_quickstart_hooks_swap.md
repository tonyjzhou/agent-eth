# https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#__docusaurus_skipToContent_fallback)
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
  * [Swap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap)


On this page
# Swap Hooks
Swaps are the most common interaction with the Uniswap protocol. When it comes to swap there are two hook functions available to customize and extend its behavior:
  * `beforeSwap`
  * `afterSwap`


As the names suggest `beforeSwap`/`afterSwap` are functions called before or after a swap is executed on a pool.
This guide will go through the parameters for `beforeSwap` and `afterSwap`, and a simple example of a swap hook.
Note: The swap hook is not production ready code, and is implemented in a simplistic manner for the purpose of learning.
## Set Up the Contract[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#set-up-the-contract "Direct link to Set Up the Contract")
Declare the solidity version used to compile the contract, here we will use `>=0.8.24` for the solidity version as transient storage is used.
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.24;
```

Import the relevant dependencies from `v4-core` and `v4-periphery`:
```
import{BaseHook}from"v4-periphery/src/utils/BaseHook.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{PoolKey}from"v4-core/src/types/PoolKey.sol";import{PoolId, PoolIdLibrary}from"v4-core/src/types/PoolId.sol";import{BalanceDelta}from"v4-core/src/types/BalanceDelta.sol";import{BeforeSwapDelta, BeforeSwapDeltaLibrary}from"v4-core/src/types/BeforeSwapDelta.sol";
```

Create a contract called `SwapHook`, use `PoolIdLibrary` to attach functions of computing `poolId` for `PoolKey`. Declare two mappings as counters for `beforeSwap` and `afterSwap`.
```
contractSwapHookis BaseHook {usingPoolIdLibraryfor PoolKey;// NOTE: ---------------------------------------------------------// state variables should typically be unique to a pool// a single hook contract should be able to service multiple pools// ---------------------------------------------------------------mapping(PoolId =>uint256 count)public beforeSwapCount;mapping(PoolId =>uint256 count)public afterSwapCount;constructor(IPoolManager _poolManager)BaseHook(_poolManager){}
```

Override `getHookPermissions()` from `BaseHook` to return a struct of permissions to signal which hook functions are to be implemented. It will also be used at deployment to validate the hook address correctly represents the expected permissions.
```
functiongetHookPermissions()publicpure override returns(Hooks.Permissions memory){return Hooks.Permissions({    beforeInitialize:false,    afterInitialize:false,    beforeAddLiquidity:false,    afterAddLiquidity:false,    beforeRemoveLiquidity:false,    afterRemoveLiquidity:false,    beforeSwap:true,    afterSwap:true,    beforeDonate:false,    afterDonate:false,    beforeSwapReturnDelta:false,    afterSwapReturnDelta:false,    afterAddLiquidityReturnDelta:false,    afterRemoveLiquidityReturnDelta:false});}
```

## beforeSwap[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#beforeswap "Direct link to beforeSwap")
Here the example shows that every time **before** a swap is executed in a pool, `beforeSwapCount` for that pool will be incremented by one.
```
function_beforeSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata,bytescalldata)internal  overridereturns(bytes4, BeforeSwapDelta,uint24){  beforeSwapCount[key.toId()]++;return(BaseHook.beforeSwap.selector, BeforeSwapDeltaLibrary.ZERO_DELTA,0);}
```

### `beforeSwap` Parameters[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#beforeswap-parameters "Direct link to beforeswap-parameters")
When triggering the `beforeSwap` hook function, there are some parameters we can make use of to customize or extend the behavior of `swap`. These parameters are described in [`beforeSwap`](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks#beforeswap) from `IHooks`.
A brief overview of the parameters:
  * `sender` The initial `msg.sender` for the `PoolManager.swap` call - typically a swap router
  * `key` The key for the pool
  * `params` The parameters for the swap i.e. [`SwapParams`](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swapparams) from `IPoolManager`
  * `hookData` Arbitrary data handed into the `PoolManager` by the swapper to be passed on to the hook


## afterSwap[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#afterswap "Direct link to afterSwap")
Similiar as above, every time **after** a swap is executed in a pool, `afterSwapCount` for that pool will be incremented by one.
```
function_afterSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata, BalanceDelta,bytescalldata)internal  overridereturns(bytes4,int128){  afterSwapCount[key.toId()]++;return(BaseHook.afterSwap.selector,0);}
```

### `afterSwap` Parameters[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#afterswap-parameters "Direct link to afterswap-parameters")
When triggering the `afterSwap` hook function, there are some parameters we can make use of to customize or extend the behavior of `swap`. These parameters are described in [`afterSwap`](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swapparams) from `IHooks`.
A brief overview of the parameters:
  * `sender` The initial `msg.sender` for the `PoolManager.swap` call - typically a swap router
  * `key` The key for the pool
  * `params` The parameters for the swap i.e. [`SwapParams`](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swapparams) from `IPoolManager`
  * `delta` The amount owed to the caller (positive) or owed to the pool (negative)
  * `hookData` Arbitrary data handed into the `PoolManager` by the swapper to be passed on to the hook


## A Complete Swap Hook Contract[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#a-complete-swap-hook-contract "Direct link to A Complete Swap Hook Contract")
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.24;import{BaseHook}from"v4-periphery/src/utils/BaseHook.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{PoolKey}from"v4-core/src/types/PoolKey.sol";import{PoolId, PoolIdLibrary}from"v4-core/src/types/PoolId.sol";import{BalanceDelta}from"v4-core/src/types/BalanceDelta.sol";import{BeforeSwapDelta, BeforeSwapDeltaLibrary}from"v4-core/src/types/BeforeSwapDelta.sol";contractSwapHookis BaseHook {usingPoolIdLibraryfor PoolKey;// NOTE: ---------------------------------------------------------// state variables should typically be unique to a pool// a single hook contract should be able to service multiple pools// ---------------------------------------------------------------mapping(PoolId =>uint256 count)public beforeSwapCount;mapping(PoolId =>uint256 count)public afterSwapCount;constructor(IPoolManager _poolManager)BaseHook(_poolManager){}functiongetHookPermissions()publicpure override returns(Hooks.Permissions memory){return Hooks.Permissions({      beforeInitialize:false,      afterInitialize:false,      beforeAddLiquidity:true,      afterAddLiquidity:false,      beforeRemoveLiquidity:true,      afterRemoveLiquidity:false,      beforeSwap:true,      afterSwap:true,      beforeDonate:false,      afterDonate:false,      beforeSwapReturnDelta:false,      afterSwapReturnDelta:false,      afterAddLiquidityReturnDelta:false,      afterRemoveLiquidityReturnDelta:false});}// -----------------------------------------------// NOTE: see IHooks.sol for function documentation// -----------------------------------------------function_beforeSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata,bytescalldata)internal    overridereturns(bytes4, BeforeSwapDelta,uint24){    beforeSwapCount[key.toId()]++;return(BaseHook.beforeSwap.selector, BeforeSwapDeltaLibrary.ZERO_DELTA,0);}function_afterSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata, BalanceDelta,bytescalldata)internal    overridereturns(bytes4,int128){    afterSwapCount[key.toId()]++;return(BaseHook.afterSwap.selector,0);}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/04-hooks/01-swap.mdx)
Was this helpful?
[PreviousSet Up Local Environment](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)[NextLiquidity Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
On this page
  * [Set Up the Contract](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#set-up-the-contract)
  * [beforeSwap](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#beforeswap)
    * [`beforeSwap` Parameters](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#beforeswap-parameters)
  * [afterSwap](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#afterswap)
    * [`afterSwap` Parameters](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#afterswap-parameters)
  * [A Complete Swap Hook Contract](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap#a-complete-swap-hook-contract)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/04-hooks/01-swap.mdx)
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
