# https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
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
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * test
  * [HooksTest](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest)


On this page
# HooksTest
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/test/HooksTest.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#functions "Direct link to Functions")
### validateHookPermissions[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#validatehookpermissions "Direct link to validateHookPermissions")
```
functionvalidateHookPermissions(address hookAddress, Hooks.Permissions calldata params)externalpure;
```

### isValidHookAddress[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#isvalidhookaddress "Direct link to isValidHookAddress")
```
functionisValidHookAddress(address hookAddress,uint24 fee)externalpurereturns(bool);
```

### shouldCallBeforeInitialize[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforeinitialize "Direct link to shouldCallBeforeInitialize")
```
functionshouldCallBeforeInitialize(address hookAddress)externalpurereturns(bool);
```

### shouldCallAfterInitialize[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafterinitialize "Direct link to shouldCallAfterInitialize")
```
functionshouldCallAfterInitialize(address hookAddress)externalpurereturns(bool);
```

### shouldCallBeforeSwap[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforeswap "Direct link to shouldCallBeforeSwap")
```
functionshouldCallBeforeSwap(address hookAddress)externalpurereturns(bool);
```

### shouldCallAfterSwap[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafterswap "Direct link to shouldCallAfterSwap")
```
functionshouldCallAfterSwap(address hookAddress)externalpurereturns(bool);
```

### shouldCallBeforeAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforeaddliquidity "Direct link to shouldCallBeforeAddLiquidity")
```
functionshouldCallBeforeAddLiquidity(address hookAddress)externalpurereturns(bool);
```

### shouldCallAfterAddLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafteraddliquidity "Direct link to shouldCallAfterAddLiquidity")
```
functionshouldCallAfterAddLiquidity(address hookAddress)externalpurereturns(bool);
```

### shouldCallBeforeRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforeremoveliquidity "Direct link to shouldCallBeforeRemoveLiquidity")
```
functionshouldCallBeforeRemoveLiquidity(address hookAddress)externalpurereturns(bool);
```

### shouldCallAfterRemoveLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafterremoveliquidity "Direct link to shouldCallAfterRemoveLiquidity")
```
functionshouldCallAfterRemoveLiquidity(address hookAddress)externalpurereturns(bool);
```

### shouldCallBeforeDonate[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforedonate "Direct link to shouldCallBeforeDonate")
```
functionshouldCallBeforeDonate(address hookAddress)externalpurereturns(bool);
```

### shouldCallAfterDonate[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafterdonate "Direct link to shouldCallAfterDonate")
```
functionshouldCallAfterDonate(address hookAddress)externalpurereturns(bool);
```

### getGasCostOfShouldCall[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#getgascostofshouldcall "Direct link to getGasCostOfShouldCall")
```
functiongetGasCostOfShouldCall(address hookAddress)externalviewreturns(uint256);
```

### getGasCostOfValidateHookAddress[​](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#getgascostofvalidatehookaddress "Direct link to getGasCostOfValidateHookAddress")
```
functiongetGasCostOfValidateHookAddress(address hookAddress, Hooks.Permissions calldata params)externalviewreturns(uint256);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/HooksTest.md)
Was this helpful?
[PreviousFuzzers](https://docs.uniswap.org/contracts/v4/reference/core/test/Fuzzers)[NextLPFeeTakingHook](https://docs.uniswap.org/contracts/v4/reference/core/test/LPFeeTakingHook)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#functions)
    * [validateHookPermissions](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#validatehookpermissions)
    * [isValidHookAddress](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#isvalidhookaddress)
    * [shouldCallBeforeInitialize](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforeinitialize)
    * [shouldCallAfterInitialize](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafterinitialize)
    * [shouldCallBeforeSwap](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforeswap)
    * [shouldCallAfterSwap](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafterswap)
    * [shouldCallBeforeAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforeaddliquidity)
    * [shouldCallAfterAddLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafteraddliquidity)
    * [shouldCallBeforeRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforeremoveliquidity)
    * [shouldCallAfterRemoveLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafterremoveliquidity)
    * [shouldCallBeforeDonate](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallbeforedonate)
    * [shouldCallAfterDonate](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#shouldcallafterdonate)
    * [getGasCostOfShouldCall](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#getgascostofshouldcall)
    * [getGasCostOfValidateHookAddress](https://docs.uniswap.org/contracts/v4/reference/core/test/HooksTest#getgascostofvalidatehookaddress)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/HooksTest.md)
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
