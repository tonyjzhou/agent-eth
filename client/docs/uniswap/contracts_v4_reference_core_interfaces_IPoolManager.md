# https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
            * [IERC20Minimal](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
            * [IERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims)
            * [IExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload)
            * [IExttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload)
            * [IHooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
            * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
            * [IProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
            * [IUnlockCallback](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IUnlockCallback)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * interfaces
  * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)


On this page
# IPoolManager
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/interfaces/IPoolManager.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [IProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees), [IERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims), [IExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload), [IExttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload)
Interface for the PoolManager
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#functions "Direct link to Functions")
### unlock[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#unlock "Direct link to unlock")
All interactions on the contract that account deltas require unlocking. A caller that calls `unlock` must implement `IUnlockCallback(msg.sender).unlockCallback(data)`, where they interact with the remaining functions on this contract.
_The only functions callable without an unlocking are`initialize` and `updateDynamicLPFee`_
```
functionunlock(bytescalldata data)externalreturns(bytesmemory);
```

**Parameters**
Name| Type| Description  
---|---|---  
`data`| `bytes`| Any data to pass to the callback, via `IUnlockCallback(msg.sender).unlockCallback(data)`  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes`| The data returned by the call to `IUnlockCallback(msg.sender).unlockCallback(data)`  
### initialize[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#initialize "Direct link to initialize")
Initialize the state for a given pool ID
_A swap fee totaling MAX_SWAP_FEE (100%) makes exact output swaps impossible since the input is entirely consumed by the fee_
```
functioninitialize(PoolKey memory key,uint160 sqrtPriceX96)externalreturns(int24 tick);
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
### modifyLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#modifyliquidity "Direct link to modifyLiquidity")
Modify the liquidity for the given pool
_Poke by calling with a zero liquidityDelta_
_Note that feesAccrued can be artificially inflated by a malicious actor and integrators should be careful using the value For pools with a single liquidity position, actors can donate to themselves to inflate feeGrowthGlobal (and consequently feesAccrued) atomically donating and collecting fees in the same unlockCallback may make the inflated value more extreme_
```
functionmodifyLiquidity(PoolKey memory key, ModifyLiquidityParams memory params,bytescalldata hookData)externalreturns(BalanceDelta callerDelta, BalanceDelta feesAccrued);
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The pool to modify liquidity in  
`params`| `ModifyLiquidityParams`| The parameters for modifying the liquidity  
`hookData`| `bytes`| The data to pass through to the add/removeLiquidity hooks  
**Returns**
Name| Type| Description  
---|---|---  
`callerDelta`| `BalanceDelta`| The balance delta of the caller of modifyLiquidity. This is the total of both principal, fee deltas, and hook deltas if applicable  
`feesAccrued`| `BalanceDelta`| The balance delta of the fees generated in the liquidity range. Returned for informational purposes  
### swap[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swap "Direct link to swap")
Swap against the given pool
_Swapping on low liquidity pools may cause unexpected swap amounts when liquidity available is less than amountSpecified. Additionally note that if interacting with hooks that have the BEFORE_SWAP_RETURNS_DELTA_FLAG or AFTER_SWAP_RETURNS_DELTA_FLAG the hook may alter the swap input/output. Integrators should perform checks on the returned swapDelta._
```
functionswap(PoolKey memory key, SwapParams memory params,bytescalldata hookData)externalreturns(BalanceDelta swapDelta);
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The pool to swap in  
`params`| `SwapParams`| The parameters for swapping  
`hookData`| `bytes`| The data to pass through to the swap hooks  
**Returns**
Name| Type| Description  
---|---|---  
`swapDelta`| `BalanceDelta`| The balance delta of the address swapping  
### donate[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#donate "Direct link to donate")
Donate the given currency amounts to the in-range liquidity providers of a pool
_Calls to donate can be frontrun adding just-in-time liquidity, with the aim of receiving a portion donated funds. Donors should keep this in mind when designing donation mechanisms._
_This function donates to in-range LPs at slot0.tick. In certain edge-cases of the swap algorithm, the`sqrtPrice` of a pool can be at the lower boundary of tick `n`, but the `slot0.tick` of the pool is already `n - 1`. In this case a call to `donate` would donate to tick `n - 1` (slot0.tick) not tick `n` (getTickAtSqrtPrice(slot0.sqrtPriceX96)). Read the comments in `Pool.swap()` for more information about this._
```
functiondonate(PoolKey memory key,uint256 amount0,uint256 amount1,bytescalldata hookData)externalreturns(BalanceDelta);
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
`<none>`| `BalanceDelta`| BalanceDelta The delta of the caller after the donate  
### sync[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#sync "Direct link to sync")
Writes the current ERC20 balance of the specified currency to transient storage This is used to checkpoint balances for the manager and derive deltas for the caller.
_This MUST be called before any ERC20 tokens are sent into the contract, but can be skipped for native tokens because the amount to settle is determined by the sent value. However, if an ERC20 token has been synced and not settled, and the caller instead wants to settle native funds, this function can be called with the native currency to then be able to settle the native currency_
```
functionsync(Currency currency)external;
```

### take[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#take "Direct link to take")
Called by the user to net out some value owed to the user
_Will revert if the requested amount is not available, consider using`mint` instead_
_Can also be used as a mechanism for free flash loans_
```
functiontake(Currency currency,address to,uint256 amount)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`currency`| `Currency`| The currency to withdraw from the pool manager  
`to`| `address`| The address to withdraw to  
`amount`| `uint256`| The amount of currency to withdraw  
### settle[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#settle "Direct link to settle")
Called by the user to pay what is owed
```
functionsettle()externalpayablereturns(uint256 paid);
```

**Returns**
Name| Type| Description  
---|---|---  
`paid`| `uint256`| The amount of currency settled  
### settleFor[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#settlefor "Direct link to settleFor")
Called by the user to pay on behalf of another address
```
functionsettleFor(address recipient)externalpayablereturns(uint256 paid);
```

**Parameters**
Name| Type| Description  
---|---|---  
`recipient`| `address`| The address to credit for the payment  
**Returns**
Name| Type| Description  
---|---|---  
`paid`| `uint256`| The amount of currency settled  
### clear[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#clear "Direct link to clear")
WARNING - Any currency that is cleared, will be non-retrievable, and locked in the contract permanently. A call to clear will zero out a positive balance WITHOUT a corresponding transfer.
_This could be used to clear a balance that is considered dust. Additionally, the amount must be the exact positive balance. This is to enforce that the caller is aware of the amount being cleared._
```
functionclear(Currency currency,uint256 amount)external;
```

### mint[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#mint "Direct link to mint")
Called by the user to move value into ERC6909 balance
_The id is converted to a uint160 to correspond to a currency address If the upper 12 bytes are not 0, they will be 0-ed out_
```
functionmint(address to,uint256 id,uint256 amount)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`to`| `address`| The address to mint the tokens to  
`id`| `uint256`| The currency address to mint to ERC6909s, as a uint256  
`amount`| `uint256`| The amount of currency to mint  
### burn[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#burn "Direct link to burn")
Called by the user to move value from ERC6909 balance
_The id is converted to a uint160 to correspond to a currency address If the upper 12 bytes are not 0, they will be 0-ed out_
```
functionburn(addressfrom,uint256 id,uint256 amount)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`from`| `address`| The address to burn the tokens from  
`id`| `uint256`| The currency address to burn from ERC6909s, as a uint256  
`amount`| `uint256`| The amount of currency to burn  
### updateDynamicLPFee[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#updatedynamiclpfee "Direct link to updateDynamicLPFee")
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
## Events[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#events "Direct link to Events")
### Initialize[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#initialize-1 "Direct link to Initialize")
Emitted when a new pool is initialized
```
eventInitialize(  PoolId indexed id,  Currency indexed currency0,  Currency indexed currency1,uint24 fee,int24 tickSpacing,  IHooks hooks,uint160 sqrtPriceX96,int24 tick);
```

**Parameters**
Name| Type| Description  
---|---|---  
`id`| `PoolId`| The abi encoded hash of the pool key struct for the new pool  
`currency0`| `Currency`| The first currency of the pool by address sort order  
`currency1`| `Currency`| The second currency of the pool by address sort order  
`fee`| `uint24`| The fee collected upon every swap in the pool, denominated in hundredths of a bip  
`tickSpacing`| `int24`| The minimum number of ticks between initialized ticks  
`hooks`| `IHooks`| The hooks contract address for the pool, or address(0) if none  
`sqrtPriceX96`| `uint160`| The price of the pool on initialization  
`tick`| `int24`| The initial tick of the pool corresponding to the initialized price  
### ModifyLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#modifyliquidity-1 "Direct link to ModifyLiquidity")
Emitted when a liquidity position is modified
```
eventModifyLiquidity(  PoolId indexed id,addressindexed sender,int24 tickLower,int24 tickUpper,int256 liquidityDelta,bytes32 salt);
```

**Parameters**
Name| Type| Description  
---|---|---  
`id`| `PoolId`| The abi encoded hash of the pool key struct for the pool that was modified  
`sender`| `address`| The address that modified the pool  
`tickLower`| `int24`| The lower tick of the position  
`tickUpper`| `int24`| The upper tick of the position  
`liquidityDelta`| `int256`| The amount of liquidity that was added or removed  
`salt`| `bytes32`| The extra data to make positions unique  
### Swap[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swap-1 "Direct link to Swap")
Emitted for swaps between currency0 and currency1
```
eventSwap(  PoolId indexed id,addressindexed sender,int128 amount0,int128 amount1,uint160 sqrtPriceX96,uint128 liquidity,int24 tick,uint24 fee);
```

**Parameters**
Name| Type| Description  
---|---|---  
`id`| `PoolId`| The abi encoded hash of the pool key struct for the pool that was modified  
`sender`| `address`| The address that initiated the swap call, and that received the callback  
`amount0`| `int128`| The delta of the currency0 balance of the pool  
`amount1`| `int128`| The delta of the currency1 balance of the pool  
`sqrtPriceX96`| `uint160`| The sqrt(price) of the pool after the swap, as a Q64.96  
`liquidity`| `uint128`| The liquidity of the pool after the swap  
`tick`| `int24`| The log base 1.0001 of the price of the pool after the swap  
`fee`| `uint24`| The swap fee in hundredths of a bip  
### Donate[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#donate-1 "Direct link to Donate")
Emitted for donations
```
eventDonate(PoolId indexed id,addressindexed sender,uint256 amount0,uint256 amount1);
```

**Parameters**
Name| Type| Description  
---|---|---  
`id`| `PoolId`| The abi encoded hash of the pool key struct for the pool that was donated to  
`sender`| `address`| The address that initiated the donate call  
`amount0`| `uint256`| The amount donated in currency0  
`amount1`| `uint256`| The amount donated in currency1  
## Errors[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#errors "Direct link to Errors")
### CurrencyNotSettled[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#currencynotsettled "Direct link to CurrencyNotSettled")
Thrown when a currency is not netted out after the contract is unlocked
```
error CurrencyNotSettled();
```

### PoolNotInitialized[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#poolnotinitialized "Direct link to PoolNotInitialized")
Thrown when trying to interact with a non-initialized pool
```
error PoolNotInitialized();
```

### AlreadyUnlocked[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#alreadyunlocked "Direct link to AlreadyUnlocked")
Thrown when unlock is called, but the contract is already unlocked
```
error AlreadyUnlocked();
```

### ManagerLocked[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#managerlocked "Direct link to ManagerLocked")
Thrown when a function is called that requires the contract to be unlocked, but it is not
```
error ManagerLocked();
```

### TickSpacingTooLarge[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#tickspacingtoolarge "Direct link to TickSpacingTooLarge")
Pools are limited to type(int16).max tickSpacing in #initialize, to prevent overflow
```
error TickSpacingTooLarge(int24 tickSpacing);
```

### TickSpacingTooSmall[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#tickspacingtoosmall "Direct link to TickSpacingTooSmall")
Pools must have a positive non-zero tickSpacing passed to #initialize
```
error TickSpacingTooSmall(int24 tickSpacing);
```

### CurrenciesOutOfOrderOrEqual[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#currenciesoutoforderorequal "Direct link to CurrenciesOutOfOrderOrEqual")
PoolKey must have currencies where address(currency0) < address(currency1)
```
error CurrenciesOutOfOrderOrEqual(address currency0,address currency1);
```

### UnauthorizedDynamicLPFeeUpdate[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#unauthorizeddynamiclpfeeupdate "Direct link to UnauthorizedDynamicLPFeeUpdate")
Thrown when a call to updateDynamicLPFee is made by an address that is not the hook, or on a pool that does not have a dynamic swap fee.
```
error UnauthorizedDynamicLPFeeUpdate();
```

### SwapAmountCannotBeZero[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swapamountcannotbezero "Direct link to SwapAmountCannotBeZero")
Thrown when trying to swap amount of 0
```
error SwapAmountCannotBeZero();
```

### NonzeroNativeValue[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#nonzeronativevalue "Direct link to NonzeroNativeValue")
Thrown when native currency is passed to a non native settlement
```
error NonzeroNativeValue();
```

### MustClearExactPositiveDelta[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#mustclearexactpositivedelta "Direct link to MustClearExactPositiveDelta")
Thrown when `clear` is called with an amount that is not exactly equal to the open currency delta.
```
error MustClearExactPositiveDelta();
```

## Structs[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#structs "Direct link to Structs")
### ModifyLiquidityParams[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#modifyliquidityparams "Direct link to ModifyLiquidityParams")
```
structModifyLiquidityParams{int24 tickLower;int24 tickUpper;int256 liquidityDelta;bytes32 salt;}
```

### SwapParams[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swapparams "Direct link to SwapParams")
```
structSwapParams{bool zeroForOne;int256 amountSpecified;uint160 sqrtPriceLimitX96;}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IPoolManager.md)
Was this helpful?
[PreviousIHooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)[NextIProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#functions)
    * [unlock](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#unlock)
    * [initialize](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#initialize)
    * [modifyLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#modifyliquidity)
    * [swap](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swap)
    * [donate](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#donate)
    * [sync](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#sync)
    * [take](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#take)
    * [settle](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#settle)
    * [settleFor](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#settlefor)
    * [clear](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#clear)
    * [mint](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#mint)
    * [burn](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#burn)
    * [updateDynamicLPFee](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#updatedynamiclpfee)
  * [Events](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#events)
    * [Initialize](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#initialize-1)
    * [ModifyLiquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#modifyliquidity-1)
    * [Swap](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swap-1)
    * [Donate](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#donate-1)
  * [Errors](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#errors)
    * [CurrencyNotSettled](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#currencynotsettled)
    * [PoolNotInitialized](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#poolnotinitialized)
    * [AlreadyUnlocked](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#alreadyunlocked)
    * [ManagerLocked](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#managerlocked)
    * [TickSpacingTooLarge](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#tickspacingtoolarge)
    * [TickSpacingTooSmall](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#tickspacingtoosmall)
    * [CurrenciesOutOfOrderOrEqual](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#currenciesoutoforderorequal)
    * [UnauthorizedDynamicLPFeeUpdate](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#unauthorizeddynamiclpfeeupdate)
    * [SwapAmountCannotBeZero](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swapamountcannotbezero)
    * [NonzeroNativeValue](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#nonzeronativevalue)
    * [MustClearExactPositiveDelta](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#mustclearexactpositivedelta)
  * [Structs](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#structs)
    * [ModifyLiquidityParams](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#modifyliquidityparams)
    * [SwapParams](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager#swapparams)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IPoolManager.md)
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
