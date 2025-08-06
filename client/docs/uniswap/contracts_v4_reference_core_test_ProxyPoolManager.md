# https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#__docusaurus_skipToContent_fallback)
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
  * [ProxyPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager)


On this page
# ProxyPoolManager
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/test/ProxyPoolManager.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager), [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees), [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall), [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims), [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload), [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
A proxy pool manager that delegates calls to the real/delegate pool manager
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#state-variables "Direct link to State Variables")
### MAX_TICK_SPACING[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#max_tick_spacing "Direct link to MAX_TICK_SPACING")
```
int24privateconstant MAX_TICK_SPACING = TickMath.MAX_TICK_SPACING;
```

### MIN_TICK_SPACING[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#min_tick_spacing "Direct link to MIN_TICK_SPACING")
```
int24privateconstant MIN_TICK_SPACING = TickMath.MIN_TICK_SPACING;
```

### _pools[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_pools "Direct link to _pools")
```
mapping(PoolId id => Pool.State)internal _pools;
```

### _delegateManager[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_delegatemanager "Direct link to _delegateManager")
```
addressinternal immutable _delegateManager;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#constructor "Direct link to constructor")
```
constructor(address delegateManager)ProtocolFees(msg.sender);
```

### onlyWhenUnlocked[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#onlywhenunlocked "Direct link to onlyWhenUnlocked")
This will revert if the contract is locked
```
modifieronlyWhenUnlocked();
```

### unlock[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#unlock "Direct link to unlock")
All interactions on the contract that account deltas require unlocking. A caller that calls `unlock` must implement `IUnlockCallback(msg.sender).unlockCallback(data)`, where they interact with the remaining functions on this contract.
_The only functions callable without an unlocking are`initialize` and `updateDynamicLPFee`_
```
functionunlock(bytescalldata data)external noDelegateCall returns(bytesmemory result);
```

**Parameters**
Name| Type| Description  
---|---|---  
`data`| `bytes`| Any data to pass to the callback, via `IUnlockCallback(msg.sender).unlockCallback(data)`  
**Returns**
Name| Type| Description  
---|---|---  
`result`| `bytes`| The data returned by the call to `IUnlockCallback(msg.sender).unlockCallback(data)`  
### initialize[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#initialize "Direct link to initialize")
Initialize the state for a given pool ID
_A swap fee totaling MAX_SWAP_FEE (100%) makes exact output swaps impossible since the input is entirely consumed by the fee_
```
functioninitialize(PoolKey memory key,uint160 sqrtPriceX96)external noDelegateCall returns(int24 tick);
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The pool key for the pool to initialize  
`sqrtPriceX96`| `uint160`| The initial square root price  
**Returns**
Name| Type| Description  
---|---|---  
`tick`| `int24`| The initial tick of the pool  
### modifyLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#modifyliquidity "Direct link to modifyLiquidity")
Modify the liquidity for the given pool
_Poke by calling with a zero liquidityDelta_
```
functionmodifyLiquidity(PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params,bytescalldata hookData)external  onlyWhenUnlocked  noDelegateCallreturns(BalanceDelta callerDelta, BalanceDelta feesAccrued);
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The pool to modify liquidity in  
`params`| `IPoolManager.ModifyLiquidityParams`| The parameters for modifying the liquidity  
`hookData`| `bytes`| The data to pass through to the add/removeLiquidity hooks  
**Returns**
Name| Type| Description  
---|---|---  
`callerDelta`| `BalanceDelta`| The balance delta of the caller of modifyLiquidity. This is the total of both principal, fee deltas, and hook deltas if applicable  
`feesAccrued`| `BalanceDelta`| The balance delta of the fees generated in the liquidity range. Returned for informational purposes  
### swap[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#swap "Direct link to swap")
Swap against the given pool
_Swapping on low liquidity pools may cause unexpected swap amounts when liquidity available is less than amountSpecified. Additionally note that if interacting with hooks that have the BEFORE_SWAP_RETURNS_DELTA_FLAG or AFTER_SWAP_RETURNS_DELTA_FLAG the hook may alter the swap input/output. Integrators should perform checks on the returned swapDelta._
```
functionswap(PoolKey memory key, IPoolManager.SwapParams memory params,bytescalldata hookData)external  onlyWhenUnlocked  noDelegateCallreturns(BalanceDelta swapDelta);
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The pool to swap in  
`params`| `IPoolManager.SwapParams`| The parameters for swapping  
`hookData`| `bytes`| The data to pass through to the swap hooks  
**Returns**
Name| Type| Description  
---|---|---  
`swapDelta`| `BalanceDelta`| The balance delta of the address swapping  
### donate[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#donate "Direct link to donate")
Donate the given currency amounts to the in-range liquidity providers of a pool
_Calls to donate can be frontrun adding just-in-time liquidity, with the aim of receiving a portion donated funds. Donors should keep this in mind when designing donation mechanisms._
```
functiondonate(PoolKey memory key,uint256 amount0,uint256 amount1,bytescalldata hookData)external  onlyWhenUnlocked  noDelegateCallreturns(BalanceDelta delta);
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The key of the pool to donate to  
`amount0`| `uint256`| The amount of currency0 to donate  
`amount1`| `uint256`| The amount of currency1 to donate  
`hookData`| `bytes`| The data to pass through to the donate hooks  
**Returns**
Name| Type| Description  
---|---|---  
`delta`| `BalanceDelta`| BalanceDelta The delta of the caller after the donate  
### sync[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#sync "Direct link to sync")
Writes the current ERC20 balance of the specified currency to transient storage This is used to checkpoint balances for the manager and derive deltas for the caller.
_This MUST be called before any ERC20 tokens are sent into the contract, but can be skipped for native tokens because the amount to settle is determined by the sent value. However, if an ERC20 token has been synced and not settled, and the caller instead wants to settle native funds, this function can be called with the native currency to then be able to settle the native currency_
```
functionsync(Currency currency)public;
```

### take[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#take "Direct link to take")
Called by the user to net out some value owed to the user
_Will revert if the requested amount is not available, consider using`mint` instead_
```
functiontake(Currency currency,address to,uint256 amount)external onlyWhenUnlocked noDelegateCall;
```

**Parameters**
Name| Type| Description  
---|---|---  
`currency`| `Currency`| The currency to withdraw from the pool manager  
`to`| `address`| The address to withdraw to  
`amount`| `uint256`| The amount of currency to withdraw  
### settle[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#settle "Direct link to settle")
Called by the user to pay what is owed
```
functionsettle()externalpayable onlyWhenUnlocked noDelegateCall returns(uint256 paid);
```

**Returns**
Name| Type| Description  
---|---|---  
`paid`| `uint256`| The amount of currency settled  
### settleFor[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#settlefor "Direct link to settleFor")
Called by the user to pay on behalf of another address
```
functionsettleFor(address recipient)externalpayable onlyWhenUnlocked noDelegateCall returns(uint256 paid);
```

**Parameters**
Name| Type| Description  
---|---|---  
`recipient`| `address`| The address to credit for the payment  
**Returns**
Name| Type| Description  
---|---|---  
`paid`| `uint256`| The amount of currency settled  
### clear[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#clear "Direct link to clear")
WARNING - Any currency that is cleared, will be non-retrievable, and locked in the contract permanently. A call to clear will zero out a positive balance WITHOUT a corresponding transfer.
_This could be used to clear a balance that is considered dust. Additionally, the amount must be the exact positive balance. This is to enforce that the caller is aware of the amount being cleared._
```
functionclear(Currency currency,uint256 amount)external onlyWhenUnlocked;
```

### mint[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#mint "Direct link to mint")
Called by the user to move value into ERC6909 balance
_The id is converted to a uint160 to correspond to a currency address If the upper 12 bytes are not 0, they will be 0-ed out_
```
functionmint(address to,uint256 id,uint256 amount)external onlyWhenUnlocked noDelegateCall;
```

**Parameters**
Name| Type| Description  
---|---|---  
`to`| `address`| The address to mint the tokens to  
`id`| `uint256`| The currency address to mint to ERC6909s, as a uint256  
`amount`| `uint256`| The amount of currency to mint  
### burn[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#burn "Direct link to burn")
Called by the user to move value from ERC6909 balance
_The id is converted to a uint160 to correspond to a currency address If the upper 12 bytes are not 0, they will be 0-ed out_
```
functionburn(addressfrom,uint256 id,uint256 amount)external onlyWhenUnlocked noDelegateCall;
```

**Parameters**
Name| Type| Description  
---|---|---  
`from`| `address`| The address to burn the tokens from  
`id`| `uint256`| The currency address to burn from ERC6909s, as a uint256  
`amount`| `uint256`| The amount of currency to burn  
### updateDynamicLPFee[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#updatedynamiclpfee "Direct link to updateDynamicLPFee")
Updates the pools lp fees for the a pool that has enabled dynamic lp fees.
_A swap fee totaling MAX_SWAP_FEE (100%) makes exact output swaps impossible since the input is entirely consumed by the fee_
```
functionupdateDynamicLPFee(PoolKey memory key,uint24 newDynamicLPFee)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The key of the pool to update dynamic LP fees for  
`newDynamicLPFee`| `uint24`| The new dynamic pool LP fee  
### _delegateCall[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_delegatecall "Direct link to _delegateCall")
Make a delegate call, bubble up any error or return the result
```
function_delegateCall(address target,bytesmemory data)internalreturns(bytesmemory result);
```

### _getPool[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_getpool "Direct link to _getPool")
Implementation of the _getPool function defined in ProtocolFees
```
function_getPool(PoolId id)internalview override returns(Pool.State storage);
```

### _isUnlocked[​](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_isunlocked "Direct link to _isUnlocked")
Implementation of the _isUnlocked function defined in ProtocolFees
```
function_isUnlocked()internalview override returns(bool);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/ProxyPoolManager.md)
Was this helpful?
[PreviousProtocolFeesImplementation](https://docs.uniswap.org/contracts/v4/reference/core/test/ProtocolFeesImplementation)[NextSkipCallsTestHook](https://docs.uniswap.org/contracts/v4/reference/core/test/SkipCallsTestHook)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#state-variables)
    * [MAX_TICK_SPACING](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#max_tick_spacing)
    * [MIN_TICK_SPACING](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#min_tick_spacing)
    * [_pools](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_pools)
    * [_delegateManager](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_delegatemanager)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#constructor)
    * [onlyWhenUnlocked](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#onlywhenunlocked)
    * [unlock](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#unlock)
    * [initialize](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#initialize)
    * [modifyLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#modifyliquidity)
    * [swap](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#swap)
    * [donate](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#donate)
    * [sync](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#sync)
    * [take](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#take)
    * [settle](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#settle)
    * [settleFor](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#settlefor)
    * [clear](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#clear)
    * [mint](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#mint)
    * [burn](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#burn)
    * [updateDynamicLPFee](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#updatedynamiclpfee)
    * [_delegateCall](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_delegatecall)
    * [_getPool](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_getpool)
    * [_isUnlocked](https://docs.uniswap.org/contracts/v4/reference/core/test/ProxyPoolManager#_isunlocked)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/test/ProxyPoolManager.md)
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
