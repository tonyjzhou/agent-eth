# https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
          * [Set Up Local Environment](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
          * [Swap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap)
          * [Liquidity Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
          * [AsyncSwap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/async-swap)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
    * [Permit2](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Quickstart
  * Hooks
  * [Liquidity Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)


On this page
# Liquidity Hooks
This guide will walk through on an example of adding and removing liquidity. There are four hook functions available to customize and extend these behavior:
  * `beforeAddLiquidity`
  * `afterAddLiquidity`
  * `beforeRemoveLiquidity`
  * `afterRemoveLiquidity`


As the names suggest `beforeAddLiquidity`/`afterAddLiquidity` are functions called before or after liquidity is added to a pool. Similarly `beforeRemoveLiquidity`/`afterRemoveLiquidity` are functions called before or after liquidity is removed from a pool.
This guide will go through the parameters and examples specifically for `beforeAddLiquidity` and `beforeRemoveLiquidity`.
Note: The liquidity examples are not production ready code, and are implemented in a simplistic manner for the purpose of learning.
## Set Up the Contract[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#set-up-the-contract "Direct link to Set Up the Contract")
Declare the solidity version used to compile the contract, since transient storage is used the solidity version will be `>=0.8.24`.
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.24;
```

Import the relevant dependencies from `v4-core` and `v4-periphery`:
```
import{BaseHook}from"v4-periphery/src/utils/BaseHook.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{PoolKey}from"v4-core/src/types/PoolKey.sol";import{PoolId, PoolIdLibrary}from"v4-core/src/types/PoolId.sol";import{BalanceDelta}from"v4-core/src/types/BalanceDelta.sol";import{BeforeSwapDelta, BeforeSwapDeltaLibrary}from"v4-core/src/types/BeforeSwapDelta.sol";
```

Create a contract called LiquidityHook, use `PoolIdLibrary` to attach functions of computing ID of a pool to `PoolKey`. Declare two mappings to act as counters when calling `beforeAddLiquidity` and `beforeRemoveLiquidity`.
```
contractLiquidityHookis BaseHook {usingPoolIdLibraryfor PoolKey;// NOTE: ---------------------------------------------------------// state variables should typically be unique to a pool// a single hook contract should be able to service multiple pools// ---------------------------------------------------------------mapping(PoolId =>uint256 count)public beforeAddLiquidityCount;mapping(PoolId =>uint256 count)public beforeRemoveLiquidityCount;constructor(IPoolManager _poolManager)BaseHook(_poolManager){}
```

Override `getHookPermissions` from `BaseHook.sol` to return a struct of permissions to signal which hook functions are to be implemented. It will also be used at deployment to validate the address correctly represents the expected permissions.
```
functiongetHookPermissions()publicpure override returns(Hooks.Permissions memory){return Hooks.Permissions({    beforeInitialize:false,    afterInitialize:false,    beforeAddLiquidity:true,    afterAddLiquidity:false,    beforeRemoveLiquidity:true,    afterRemoveLiquidity:false,    beforeSwap:false,    afterSwap:false,    beforeDonate:false,    afterDonate:false,    beforeAddLiquidityReturnDelta:false,    afterSwapReturnDelta:false,    afterAddLiquidityReturnDelta:false,    afterRemoveLiquidityReturnDelta:false});}
```

## beforeAddLiquidity[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#beforeaddliquidity "Direct link to beforeAddLiquidity")
Here the example shows that every time **before** liquidity is added to a pool, `beforeAddLiquidityCount` for that pool will be incremented by one.
```
function_beforeAddLiquidity(address,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata,bytescalldata)internal override returns(bytes4){  beforeAddLiquidityCount[key.toId()]++;return BaseHook.beforeAddLiquidity.selector;}
```

### `beforeAddLiquidity` Parameters[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#beforeaddliquidity-parameters "Direct link to beforeaddliquidity-parameters")
When triggering the `beforeAddLiquidity` hook function, there are some parameters we can make use of to customize or extend the behavior of `modifyLiquidity`. These parameters are described in `beforeAddLiquidity` from [`IHooks.sol`](https://github.com/Uniswap/v4-core/blob/main/src/interfaces/IHooks.sol#L47).
A brief overview of the parameters:
  * `sender` The initial `msg.sender` for the add liquidity call
  * `key` The key for the pool
  * `params` The parameters for adding liquidity i.e. `ModifyLiquidityParams` from [`IPoolManager.sol`](https://github.com/Uniswap/v4-core/blob/main/src/interfaces/IPoolManager.sol#L125C12-L125C33)
  * `hookData` Arbitrary data handed into the `PoolManager` by the liquidity provider to be be passed on to the hook


## beforeRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#beforeremoveliquidity "Direct link to beforeRemoveLiquidity")
Similiar as above, every time **before** liquidity is removed from a pool, `beforeRemoveLiquidityCount` for that pool will be incremented by one.
```
function_beforeRemoveLiquidity(address,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata,bytescalldata)internal override returns(bytes4){  beforeRemoveLiquidityCount[key.toId()]++;return BaseHook.beforeRemoveLiquidity.selector;}
```

### `beforeRemoveLiquidity` Parameters[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#beforeremoveliquidity-parameters "Direct link to beforeremoveliquidity-parameters")
When triggering the `beforeRemoveLiquidity` hook function, there are some parameters we can make use of to customize or extend the behavior of `modifyLiquidity`. These parameters are described in `beforeRemoveLiquidity` from [`IHooks.sol`](https://github.com/Uniswap/v4-core/blob/main/src/interfaces/IHooks.sol#L78).
A brief overview of the parameters:
  * `sender` The initial msg.sender for the remove liquidity call
  * `key` The key for the pool
  * `params` The parameters for removing liquidity i.e. `ModifyLiquidityParams` from [`IPoolManager.sol`](https://github.com/Uniswap/v4-core/blob/main/src/interfaces/IPoolManager.sol#L125C12-L125C33)
  * `hookData` Arbitrary data handed into the `PoolManager` by the liquidity provider to be be passed on to the hook


## A Complete Liquidity Hook Contract[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#a-complete-liquidity-hook-contract "Direct link to A Complete Liquidity Hook Contract")
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.24;import{BaseHook}from"v4-periphery/src/utils/BaseHook.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{PoolKey}from"v4-core/src/types/PoolKey.sol";import{PoolId, PoolIdLibrary}from"v4-core/src/types/PoolId.sol";import{BalanceDelta}from"v4-core/src/types/BalanceDelta.sol";import{BeforeSwapDelta, BeforeSwapDeltaLibrary}from"v4-core/src/types/BeforeSwapDelta.sol";contractLiquidityHookis BaseHook {usingPoolIdLibraryfor PoolKey;// NOTE: ---------------------------------------------------------// state variables should typically be unique to a pool// a single hook contract should be able to service multiple pools// ---------------------------------------------------------------mapping(PoolId =>uint256 count)public beforeAddLiquidityCount;mapping(PoolId =>uint256 count)public beforeRemoveLiquidityCount;constructor(IPoolManager _poolManager)BaseHook(_poolManager){}functiongetHookPermissions()publicpure override returns(Hooks.Permissions memory){return Hooks.Permissions({      beforeInitialize:false,      afterInitialize:false,      beforeAddLiquidity:true,      afterAddLiquidity:false,      beforeRemoveLiquidity:true,      afterRemoveLiquidity:false,      beforeSwap:false,      afterSwap:false,      beforeDonate:false,      afterDonate:false,      beforeAddLiquidityReturnDelta:false,      afterSwapReturnDelta:false,      afterAddLiquidityReturnDelta:false,      afterRemoveLiquidityReturnDelta:false});}// -----------------------------------------------// NOTE: see IHooks.sol for function documentation// -----------------------------------------------function_beforeAddLiquidity(address,    PoolKey calldata key,    IPoolManager.ModifyLiquidityParams calldata,bytescalldata)internal override returns(bytes4){    beforeAddLiquidityCount[key.toId()]++;return BaseHook.beforeAddLiquidity.selector;}function_beforeRemoveLiquidity(address,    PoolKey calldata key,    IPoolManager.ModifyLiquidityParams calldata,bytescalldata)internal override returns(bytes4){    beforeRemoveLiquidityCount[key.toId()]++;return BaseHook.beforeRemoveLiquidity.selector;}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/04-hooks/02-liquidity.mdx)
Was this helpful?
[PreviousSwap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap)[NextAsyncSwap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/async-swap)
On this page
  * [Set Up the Contract](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#set-up-the-contract)
  * [beforeAddLiquidity](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#beforeaddliquidity)
    * [`beforeAddLiquidity` Parameters](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#beforeaddliquidity-parameters)
  * [beforeRemoveLiquidity](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#beforeremoveliquidity)
    * [`beforeRemoveLiquidity` Parameters](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#beforeremoveliquidity-parameters)
  * [A Complete Liquidity Hook Contract](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity#a-complete-liquidity-hook-contract)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/04-hooks/02-liquidity.mdx)
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
