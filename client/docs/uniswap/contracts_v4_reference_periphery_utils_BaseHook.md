# https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [test](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [PositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
          * [PositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [UniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/UniswapV4DeployerCompetition)
          * [V4Router](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [base](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [lens](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
          * [utils](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
            * [BaseHook](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Periphery
  * utils
  * [BaseHook](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)


On this page
# BaseHook
[Git Source](https://github.com/uniswap/v4-periphery/blob/ea2bf2e1ba6863bb809fc2ff791744f308c4a26d/src/utils/BaseHook.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** IHooks, [ImmutableState](https://docs.uniswap.org/contracts/v4/reference/periphery/base/ImmutableState)
abstract contract for hook implementations
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#constructor "Direct link to constructor")
```
constructor(IPoolManager _manager)ImmutableState(_manager);
```

### getHookPermissions[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#gethookpermissions "Direct link to getHookPermissions")
Returns a struct of permissions to signal which hook functions are to be implemented
_Used at deployment to validate the address correctly represents the expected permissions_
```
functiongetHookPermissions()publicpure virtual returns(Hooks.Permissions memory);
```

### validateHookAddress[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#validatehookaddress "Direct link to validateHookAddress")
Validates the deployed hook address agrees with the expected permissions of the hook
_this function is virtual so that we can override it during testing, which allows us to deploy an implementation to any address and then etch the bytecode into the correct address_
```
functionvalidateHookAddress(BaseHook _this)internalpure virtual;
```

### beforeInitialize[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforeinitialize "Direct link to beforeInitialize")
The hook called before the state of a pool is initialized
```
functionbeforeInitialize(address sender, PoolKey calldata key,uint160 sqrtPriceX96)external  onlyPoolManagerreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the initialize call  
`key`| `PoolKey`| The key for the pool being initialized  
`sqrtPriceX96`| `uint160`| The sqrt(price) of the pool as a Q64.96  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### _beforeInitialize[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforeinitialize "Direct link to _beforeInitialize")
```
function_beforeInitialize(address, PoolKey calldata,uint160)internal virtual returns(bytes4);
```

### afterInitialize[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afterinitialize "Direct link to afterInitialize")
The hook called after the state of a pool is initialized
```
functionafterInitialize(address sender, PoolKey calldata key,uint160 sqrtPriceX96,int24 tick)external  onlyPoolManagerreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the initialize call  
`key`| `PoolKey`| The key for the pool being initialized  
`sqrtPriceX96`| `uint160`| The sqrt(price) of the pool as a Q64.96  
`tick`| `int24`| The current tick after the state of a pool is initialized  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### _afterInitialize[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afterinitialize "Direct link to _afterInitialize")
```
function_afterInitialize(address, PoolKey calldata,uint160,int24)internal virtual returns(bytes4);
```

### beforeAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforeaddliquidity "Direct link to beforeAddLiquidity")
The hook called before liquidity is added
```
functionbeforeAddLiquidity(address sender,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,bytescalldata hookData)external onlyPoolManager returns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the add liquidity call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for adding liquidity  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### _beforeAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforeaddliquidity "Direct link to _beforeAddLiquidity")
```
function_beforeAddLiquidity(address, PoolKey calldata, IPoolManager.ModifyLiquidityParams calldata,bytescalldata)internal  virtualreturns(bytes4);
```

### beforeRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforeremoveliquidity "Direct link to beforeRemoveLiquidity")
The hook called before liquidity is removed
```
functionbeforeRemoveLiquidity(address sender,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,bytescalldata hookData)external onlyPoolManager returns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the remove liquidity call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for removing liquidity  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### _beforeRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforeremoveliquidity "Direct link to _beforeRemoveLiquidity")
```
function_beforeRemoveLiquidity(address, PoolKey calldata, IPoolManager.ModifyLiquidityParams calldata,bytescalldata)internal  virtualreturns(bytes4);
```

### afterAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afteraddliquidity "Direct link to afterAddLiquidity")
The hook called after liquidity is added
```
functionafterAddLiquidity(address sender,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,  BalanceDelta delta,  BalanceDelta feesAccrued,bytescalldata hookData)external onlyPoolManager returns(bytes4, BalanceDelta);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the add liquidity call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for adding liquidity  
`delta`| `BalanceDelta`| The caller's balance delta after adding liquidity; the sum of principal delta, fees accrued, and hook delta  
`feesAccrued`| `BalanceDelta`| The fees accrued since the last time fees were collected from this position  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
`<none>`| `BalanceDelta`| BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency  
### _afterAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afteraddliquidity "Direct link to _afterAddLiquidity")
```
function_afterAddLiquidity(address,  PoolKey calldata,  IPoolManager.ModifyLiquidityParams calldata,  BalanceDelta,  BalanceDelta,bytescalldata)internal virtual returns(bytes4, BalanceDelta);
```

### afterRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afterremoveliquidity "Direct link to afterRemoveLiquidity")
The hook called after liquidity is removed
```
functionafterRemoveLiquidity(address sender,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,  BalanceDelta delta,  BalanceDelta feesAccrued,bytescalldata hookData)external onlyPoolManager returns(bytes4, BalanceDelta);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the remove liquidity call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for removing liquidity  
`delta`| `BalanceDelta`| The caller's balance delta after removing liquidity; the sum of principal delta, fees accrued, and hook delta  
`feesAccrued`| `BalanceDelta`| The fees accrued since the last time fees were collected from this position  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
`<none>`| `BalanceDelta`| BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency  
### _afterRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afterremoveliquidity "Direct link to _afterRemoveLiquidity")
```
function_afterRemoveLiquidity(address,  PoolKey calldata,  IPoolManager.ModifyLiquidityParams calldata,  BalanceDelta,  BalanceDelta,bytescalldata)internal virtual returns(bytes4, BalanceDelta);
```

### beforeSwap[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforeswap "Direct link to beforeSwap")
The hook called before a swap
```
functionbeforeSwap(address sender,  PoolKey calldata key,  IPoolManager.SwapParams calldata params,bytescalldata hookData)external onlyPoolManager returns(bytes4, BeforeSwapDelta,uint24);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the swap call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.SwapParams`| The parameters for the swap  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
`<none>`| `BeforeSwapDelta`| BeforeSwapDelta The hook's delta in specified and unspecified currencies. Positive: the hook is owed/took currency, negative: the hook owes/sent currency  
`<none>`| `uint24`| uint24 Optionally override the lp fee, only used if three conditions are met: 1. the Pool has a dynamic fee, 2. the value's 2nd highest bit is set (23rd bit, 0x400000), and 3. the value is less than or equal to the maximum fee (1 million)  
### _beforeSwap[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforeswap "Direct link to _beforeSwap")
```
function_beforeSwap(address, PoolKey calldata, IPoolManager.SwapParams calldata,bytescalldata)internal  virtualreturns(bytes4, BeforeSwapDelta,uint24);
```

### afterSwap[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afterswap "Direct link to afterSwap")
The hook called after a swap
```
functionafterSwap(address sender,  PoolKey calldata key,  IPoolManager.SwapParams calldata params,  BalanceDelta delta,bytescalldata hookData)external onlyPoolManager returns(bytes4,int128);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the swap call  
`key`| `PoolKey`| The key for the pool  
`params`| `IPoolManager.SwapParams`| The parameters for the swap  
`delta`| `BalanceDelta`| The amount owed to the caller (positive) or owed to the pool (negative)  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
`<none>`| `int128`| int128 The hook's delta in unspecified currency. Positive: the hook is owed/took currency, negative: the hook owes/sent currency  
### _afterSwap[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afterswap "Direct link to _afterSwap")
```
function_afterSwap(address, PoolKey calldata, IPoolManager.SwapParams calldata, BalanceDelta,bytescalldata)internal  virtualreturns(bytes4,int128);
```

### beforeDonate[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforedonate "Direct link to beforeDonate")
The hook called before donate
```
functionbeforeDonate(address sender, PoolKey calldata key,uint256 amount0,uint256 amount1,bytescalldata hookData)external  onlyPoolManagerreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the donate call  
`key`| `PoolKey`| The key for the pool  
`amount0`| `uint256`| The amount of token0 being donated  
`amount1`| `uint256`| The amount of token1 being donated  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### _beforeDonate[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforedonate "Direct link to _beforeDonate")
```
function_beforeDonate(address, PoolKey calldata,uint256,uint256,bytescalldata)internal virtual returns(bytes4);
```

### afterDonate[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afterdonate "Direct link to afterDonate")
The hook called after donate
```
functionafterDonate(address sender, PoolKey calldata key,uint256 amount0,uint256 amount1,bytescalldata hookData)external  onlyPoolManagerreturns(bytes4);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The initial msg.sender for the donate call  
`key`| `PoolKey`| The key for the pool  
`amount0`| `uint256`| The amount of token0 being donated  
`amount1`| `uint256`| The amount of token1 being donated  
`hookData`| `bytes`| Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes4`| bytes4 The function selector for the hook  
### _afterDonate[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afterdonate "Direct link to _afterDonate")
```
function_afterDonate(address, PoolKey calldata,uint256,uint256,bytescalldata)internal virtual returns(bytes4);
```

## Errors[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#errors "Direct link to Errors")
### HookNotImplemented[​](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#hooknotimplemented "Direct link to HookNotImplemented")
```
error HookNotImplemented();
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/utils/BaseHook.md)
Was this helpful?
[PreviousVanityAddressLib](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/VanityAddressLib)[NextOverview](https://docs.uniswap.org/contracts/v3/overview)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#constructor)
    * [getHookPermissions](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#gethookpermissions)
    * [validateHookAddress](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#validatehookaddress)
    * [beforeInitialize](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforeinitialize)
    * [_beforeInitialize](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforeinitialize)
    * [afterInitialize](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afterinitialize)
    * [_afterInitialize](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afterinitialize)
    * [beforeAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforeaddliquidity)
    * [_beforeAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforeaddliquidity)
    * [beforeRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforeremoveliquidity)
    * [_beforeRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforeremoveliquidity)
    * [afterAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afteraddliquidity)
    * [_afterAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afteraddliquidity)
    * [afterRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afterremoveliquidity)
    * [_afterRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afterremoveliquidity)
    * [beforeSwap](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforeswap)
    * [_beforeSwap](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforeswap)
    * [afterSwap](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afterswap)
    * [_afterSwap](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afterswap)
    * [beforeDonate](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#beforedonate)
    * [_beforeDonate](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_beforedonate)
    * [afterDonate](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#afterdonate)
    * [_afterDonate](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#_afterdonate)
  * [Errors](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#errors)
    * [HookNotImplemented](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook#hooknotimplemented)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/utils/BaseHook.md)
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
