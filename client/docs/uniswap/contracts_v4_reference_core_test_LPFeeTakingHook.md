# https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
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
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * test
  * [LPFeeTakingHook](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)


On this page
# LPFeeTakingHook
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/test/LPFeeTakingHook.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [BaseTestHooks](https://docs.uniswap.org/contracts/v4/reference/core/test/BaseTestHooks)
a hook that takes all of the LP fee revenue
_an example test hook to validate the data is provided correctly_
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#state-variables "Direct link to State Variables")
### manager[​](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#manager "Direct link to manager")
```
IPoolManager immutable manager;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#constructor "Direct link to constructor")
```
constructor(IPoolManager _manager);
```

### onlyPoolManager[​](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#onlypoolmanager "Direct link to onlyPoolManager")
```
modifieronlyPoolManager();
```

### afterRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#afterremoveliquidity "Direct link to afterRemoveLiquidity")
```
functionafterRemoveLiquidity(address,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata,  BalanceDelta,  BalanceDelta feeDelta,bytescalldata)external override onlyPoolManager returns(bytes4, BalanceDelta);
```

### afterAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#afteraddliquidity "Direct link to afterAddLiquidity")
```
functionafterAddLiquidity(address,  PoolKey calldata key,  IPoolManager.ModifyLiquidityParams calldata,  BalanceDelta,  BalanceDelta feeDelta,bytescalldata)external override onlyPoolManager returns(bytes4, BalanceDelta);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/LPFeeTakingHook.md)
Was this helpful?
[PreviousHooksTest](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)[NextLiquidityMathTest](https://docs.uniswap.org/contracts/v4/reference/core/test/LiquidityMathTest)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#state-variables)
    * [manager](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#manager)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#constructor)
    * [onlyPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#onlypoolmanager)
    * [afterRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#afterremoveliquidity)
    * [afterAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook#afteraddliquidity)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/LPFeeTakingHook.md)
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
