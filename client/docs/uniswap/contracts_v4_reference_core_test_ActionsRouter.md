# https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#__docusaurus_skipToContent_fallback)
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
  * [Actions](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter)


On this page
# Actions
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/test/ActionsRouter.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
```
enumActions{  SETTLE,  SETTLE_NATIVE,  SETTLE_FOR,  TAKE,  PRANK_TAKE_FROM,  SYNC,  MINT,  CLEAR,  ASSERT_BALANCE_EQUALS,  ASSERT_RESERVES_EQUALS,  ASSERT_DELTA_EQUALS,  ASSERT_NONZERO_DELTA_COUNT_EQUALS,  TRANSFER_FROM,  COLLECT_PROTOCOL_FEES}
```

# ActionsRouter
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/test/ActionsRouter.sol)
**Inherits:** [IUnlockCallback](https://docs.uniswap.org/src/interfaces/callback/IUnlockCallback.sol/interface.IUnlockCallback.md), Test
A router that handles an arbitrary input of actions. TODO: Can continue to add functions per action.
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#state-variables "Direct link to State Variables")
### manager[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#manager "Direct link to manager")
```
IPoolManager manager;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#constructor "Direct link to constructor")
```
constructor(IPoolManager _manager);
```

### unlockCallback[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#unlockcallback "Direct link to unlockCallback")
```
functionunlockCallback(bytescalldata data)externalreturns(bytesmemory);
```

### executeActions[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#executeactions "Direct link to executeActions")
```
functionexecuteActions(Actions[]memory actions,bytes[]memory params)externalpayable;
```

### _settle[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_settle "Direct link to _settle")
```
function_settle()internal;
```

### _settleNative[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_settlenative "Direct link to _settleNative")
```
function_settleNative(bytesmemory params)internal;
```

### _settleFor[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_settlefor "Direct link to _settleFor")
```
function_settleFor(bytesmemory params)internal;
```

### _take[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_take "Direct link to _take")
```
function_take(bytesmemory params)internal;
```

### _prankTakeFrom[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_pranktakefrom "Direct link to _prankTakeFrom")
```
function_prankTakeFrom(bytesmemory params)internal;
```

### _sync[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_sync "Direct link to _sync")
```
function_sync(bytesmemory params)internal;
```

### _mint[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_mint "Direct link to _mint")
```
function_mint(bytesmemory params)internal;
```

### _clear[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_clear "Direct link to _clear")
```
function_clear(bytesmemory params)internal;
```

### _assertBalanceEquals[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_assertbalanceequals "Direct link to _assertBalanceEquals")
```
function_assertBalanceEquals(bytesmemory params)internalview;
```

### _assertReservesEquals[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_assertreservesequals "Direct link to _assertReservesEquals")
```
function_assertReservesEquals(bytesmemory params)internalview;
```

### _assertDeltaEquals[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_assertdeltaequals "Direct link to _assertDeltaEquals")
```
function_assertDeltaEquals(bytesmemory params)internalview;
```

### _assertNonzeroDeltaCountEquals[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_assertnonzerodeltacountequals "Direct link to _assertNonzeroDeltaCountEquals")
```
function_assertNonzeroDeltaCountEquals(bytesmemory params)internalview;
```

### _transferFrom[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_transferfrom "Direct link to _transferFrom")
```
function_transferFrom(bytesmemory params)internal;
```

### _collectProtocolFees[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_collectprotocolfees "Direct link to _collectProtocolFees")
```
function_collectProtocolFees(bytesmemory params)internal;
```

## Errors[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#errors "Direct link to Errors")
### ActionNotSupported[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#actionnotsupported "Direct link to ActionNotSupported")
```
error ActionNotSupported();
```

### CheckParameters[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#checkparameters "Direct link to CheckParameters")
```
error CheckParameters();
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/ActionsRouter.md)
Was this helpful?
[PreviousIUnlockCallback](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IUnlockCallback)[NextBaseTestHooks](https://docs.uniswap.org/contracts/v4/reference/core/test/BaseTestHooks)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#state-variables)
    * [manager](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#manager)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#constructor)
    * [unlockCallback](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#unlockcallback)
    * [executeActions](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#executeactions)
    * [_settle](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_settle)
    * [_settleNative](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_settlenative)
    * [_settleFor](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_settlefor)
    * [_take](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_take)
    * [_prankTakeFrom](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_pranktakefrom)
    * [_sync](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_sync)
    * [_mint](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_mint)
    * [_clear](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_clear)
    * [_assertBalanceEquals](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_assertbalanceequals)
    * [_assertReservesEquals](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_assertreservesequals)
    * [_assertDeltaEquals](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_assertdeltaequals)
    * [_assertNonzeroDeltaCountEquals](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_assertnonzerodeltacountequals)
    * [_transferFrom](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_transferfrom)
    * [_collectProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#_collectprotocolfees)
  * [Errors](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#errors)
    * [ActionNotSupported](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#actionnotsupported)
    * [CheckParameters](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter#checkparameters)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/ActionsRouter.md)
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
