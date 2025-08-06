# https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#__docusaurus_skipToContent_fallback)
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
            * [Actions](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter)
            * [BaseTestHooks](https://docs.uniswap.org/contracts/v4/reference/core/test/BaseTestHooks)
            * [CurrencyTest](https://docs.uniswap.org/contracts/v4/reference/core/test/CurrencyTest)
            * [CustomCurveHook](https://docs.uniswap.org/contracts/v4/reference/core/test/CustomCurveHook)
            * [DeltaReturningHook](https://docs.uniswap.org/contracts/v4/reference/core/test/DeltaReturningHook)
            * [DynamicFeesTestHook](https://docs.uniswap.org/contracts/v4/reference/core/test/DynamicFeesTestHook)
            * [DynamicReturnFeeTestHook](https://docs.uniswap.org/contracts/v4/reference/core/test/DynamicReturnFeeTestHook)
            * [EmptyRevertContract](https://docs.uniswap.org/contracts/v4/reference/core/test/EmptyRevertContract)
            * [EmptyTestHooks](https://docs.uniswap.org/contracts/v4/reference/core/test/EmptyTestHooks)
            * [FeeTakingHook](https://docs.uniswap.org/contracts/v4/reference/core/test/FeeTakingHook)
            * [Fuzzers](https://docs.uniswap.org/contracts/v4/reference/core/test/Fuzzers)
            * [HooksTest](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
            * [LPFeeTakingHook](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
            * [LiquidityMathTest](https://docs.uniswap.org/contracts/v4/reference/core/test/LiquidityMathTest)
            * [MockContract](https://docs.uniswap.org/contracts/v4/reference/core/test/MockContract)
            * [MockERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/test/MockERC6909Claims)
            * [MockHooks](https://docs.uniswap.org/contracts/v4/reference/core/test/MockHooks)
            * [NativeERC20](https://docs.uniswap.org/contracts/v4/reference/core/test/NativeERC20)
            * [NoDelegateCallTest](https://docs.uniswap.org/contracts/v4/reference/core/test/NoDelegateCallTest)
            * [PoolClaimsTest](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolClaimsTest)
            * [PoolDonateTest](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolDonateTest)
            * [PoolEmptyUnlockTest](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolEmptyUnlockTest)
            * [PoolModifyLiquidityTest](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolModifyLiquidityTest)
            * [PoolModifyLiquidityTestNoChecks](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolModifyLiquidityTestNoChecks)
            * [Action](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolNestedActionsTest)
            * [PoolSwapTest](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolSwapTest)
            * [PoolTakeTest](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolTakeTest)
            * [PoolTestBase](https://docs.uniswap.org/contracts/v4/reference/core/test/PoolTestBase)
            * [ProtocolFeesImplementation](https://docs.uniswap.org/contracts/v4/reference/core/test/ProtocolFeesImplementation)
            * [ProxyPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager)
            * [SkipCallsTestHook](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook)
            * [SqrtPriceMathEchidnaTest](https://docs.uniswap.org/contracts/v4/reference/core/test/SqrtPriceMathEchidnaTest)
            * [SwapRouterNoChecks](https://docs.uniswap.org/contracts/v4/reference/core/test/SwapRouterNoChecks)
            * [TestERC20](https://docs.uniswap.org/contracts/v4/reference/core/test/TestERC20)
            * [TestInvalidERC20](https://docs.uniswap.org/contracts/v4/reference/core/test/TestInvalidERC20)
            * [TickMathEchidnaTest](https://docs.uniswap.org/contracts/v4/reference/core/test/TickMathEchidnaTest)
            * [TickMathTest](https://docs.uniswap.org/contracts/v4/reference/core/test/TickMathTest)
            * [TickOverflowSafetyEchidnaTest](https://docs.uniswap.org/contracts/v4/reference/core/test/TickOverflowSafetyEchidnaTest)
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
  * test
  * [SkipCallsTestHook](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook)


On this page
# SkipCallsTestHook
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/test/SkipCallsTestHook.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [BaseTestHooks](https://docs.uniswap.org/contracts/v4/reference/core/test/BaseTestHooks), Test
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#state-variables "Direct link to State Variables")
### counter[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#counter "Direct link to counter")
```
uint256public counter;
```

### manager[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#manager "Direct link to manager")
```
IPoolManager manager;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#functions "Direct link to Functions")
### setManager[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#setmanager "Direct link to setManager")
```
functionsetManager(IPoolManager _manager)external;
```

### beforeInitialize[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforeinitialize "Direct link to beforeInitialize")
```
functionbeforeInitialize(address, PoolKey calldata key,uint160 sqrtPriceX96)external override returns(bytes4);
```

### afterInitialize[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afterinitialize "Direct link to afterInitialize")
```
functionafterInitialize(address, PoolKey calldata key,uint160 sqrtPriceX96,int24)external  overridereturns(bytes4);
```

### beforeAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforeaddliquidity "Direct link to beforeAddLiquidity")
```
functionbeforeAddLiquidity(address,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,bytescalldata hookData)external override returns(bytes4);
```

### afterAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afteraddliquidity "Direct link to afterAddLiquidity")
```
functionafterAddLiquidity(address,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,  BalanceDelta,  BalanceDelta,bytescalldata hookData)external override returns(bytes4, BalanceDelta);
```

### beforeRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforeremoveliquidity "Direct link to beforeRemoveLiquidity")
```
functionbeforeRemoveLiquidity(address,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,bytescalldata hookData)external override returns(bytes4);
```

### afterRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afterremoveliquidity "Direct link to afterRemoveLiquidity")
```
functionafterRemoveLiquidity(address,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata params,  BalanceDelta,  BalanceDelta,bytescalldata hookData)external override returns(bytes4, BalanceDelta);
```

### beforeSwap[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforeswap "Direct link to beforeSwap")
```
functionbeforeSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata params,bytescalldata hookData)external  overridereturns(bytes4, BeforeSwapDelta,uint24);
```

### afterSwap[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afterswap "Direct link to afterSwap")
```
functionafterSwap(address,  PoolKey calldata key,  IPoolManager.SwapParams calldata params,  BalanceDelta,bytescalldata hookData)external override returns(bytes4,int128);
```

### beforeDonate[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforedonate "Direct link to beforeDonate")
```
functionbeforeDonate(address, PoolKey calldata key,uint256 amt0,uint256 amt1,bytescalldata hookData)external  overridereturns(bytes4);
```

### afterDonate[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afterdonate "Direct link to afterDonate")
```
functionafterDonate(address, PoolKey calldata key,uint256 amt0,uint256 amt1,bytescalldata hookData)external  overridereturns(bytes4);
```

### _initialize[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_initialize "Direct link to _initialize")
```
function_initialize(PoolKey memory key,uint160 sqrtPriceX96)public;
```

### _swap[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_swap "Direct link to _swap")
```
function_swap(PoolKey calldata key, IPoolManager.SwapParams memory params,bytescalldata hookData)public;
```

### _addLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_addliquidity "Direct link to _addLiquidity")
```
function_addLiquidity(PoolKey calldata key, IPoolManager.ModifyLiquidityParams memory params,bytescalldata hookData)public;
```

### _removeLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_removeliquidity "Direct link to _removeLiquidity")
```
function_removeLiquidity(  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams memory params,bytescalldata hookData)public;
```

### _donate[​](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_donate "Direct link to _donate")
```
function_donate(PoolKey calldata key,uint256 amt0,uint256 amt1,bytescalldata hookData)public;
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/SkipCallsTestHook.md)
Was this helpful?
[PreviousProxyPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager)[NextSqrtPriceMathEchidnaTest](https://docs.uniswap.org/contracts/v4/reference/core/test/SqrtPriceMathEchidnaTest)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#state-variables)
    * [counter](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#counter)
    * [manager](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#manager)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#functions)
    * [setManager](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#setmanager)
    * [beforeInitialize](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforeinitialize)
    * [afterInitialize](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afterinitialize)
    * [beforeAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforeaddliquidity)
    * [afterAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afteraddliquidity)
    * [beforeRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforeremoveliquidity)
    * [afterRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afterremoveliquidity)
    * [beforeSwap](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforeswap)
    * [afterSwap](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afterswap)
    * [beforeDonate](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#beforedonate)
    * [afterDonate](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#afterdonate)
    * [_initialize](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_initialize)
    * [_swap](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_swap)
    * [_addLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_addliquidity)
    * [_removeLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_removeliquidity)
    * [_donate](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook#_donate)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/SkipCallsTestHook.md)
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
