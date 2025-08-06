# https://docs.uniswap.org/contracts/v4/guides/flash-accounting

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Unlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)
        * [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)
        * [Custom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Swap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Position Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)
        * [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Access msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
    * [Permit2](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Guides
  * [Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)


On this page
# Introduction
Flash accounting is v4’s mechanism for tracking token movements throughout a transaction. Unlike traditional token accounting which updates balances immediately after each operation, flash accounting accumulates changes (deltas) and settles them at the end of the transaction.
## How Flash Accounting Works[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#how-flash-accounting-works "Direct link to How Flash Accounting Works")
When interacting with v4's PoolManager, all token movements follow a consistent pattern: negative values represent tokens moving from users to the PoolManager, while positive values represent tokens moving from the PoolManager to users. This pattern appears in operations like swaps and liquidity management, where:
  * Negative values indicate tokens going to the PoolManager
  * Positive values indicate tokens coming from the PoolManager


These movements are tracked through deltas that represent token obligations:
  * Negative deltas indicate tokens owed to the PoolManager
  * Positive deltas indicate tokens the PoolManager owes to an address


## The PoolManager Lock Pattern[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#the-poolmanager-lockpattern "Direct link to The PoolManager Lock Pattern")
All operations that access pool liquidity must occur while the PoolManager is unlocked. This pattern ensures atomic execution and proper delta tracking:
  1. Unlock the PoolManager
  2. Execute operations (creating deltas)
  3. Resolve all deltas
  4. Context returns to the PoolManager which verifies no outstanding deltas


If any deltas remain unresolved when the PoolManager locks, the entire transaction reverts. This guarantees that all token movements balance out by the end of the transaction.
# Understanding the Basics
Before diving into implementation patterns, let’s look at the key concepts you’ll need to work with flash accounting. Each example includes common scenarios you’ll encounter when building on v4.
## Working with Deltas[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#working-withdeltas "Direct link to Working with Deltas")
Every operation in v4 that involves tokens creates deltas. These deltas track what the executor owes to the PoolManager and vice versa:
```
// Example: Executing a swap// Note: This assumes the PoolManager has been unlockedfunctionexecuteSwap(PoolKey calldata key)external{// A swap returns a BalanceDelta  BalanceDelta delta = poolManager.swap(    key,    IPoolManager.SwapParams({      zeroForOne:true,      amountSpecified:-1e18,// Negative means spending/providing 1 ETH      sqrtPriceLimitX96: MAX_SQRT_RATIO -1// Max price willing to accept}),"");// Delta shows:// delta.amount0() = -1e18  (executor owes 1 ETH)// delta.amount1() = +2000e6 (executor receives 2000 USDC)}
```

When a swap is executed, the PoolManager returns a `BalanceDelta` that shows your token obligations. In this example, the negative delta (-1e18) means the executor owes 1 ETH to the PoolManager, while the positive delta (+2000e6) means the executor is entitled to receive 2000 USDC. These deltas must be resolved before the transaction completes.
_Note how negative values in v4 consistently represent tokens going to the PoolManager - both in`amountSpecified` for the input amount and in the returned delta for tokens owed._
## Reading Delta States[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#reading-deltastates "Direct link to Reading Delta States")
A common pattern is checking current deltas before executing operations. The `TransientStateLibrary` helps you track these balances:
```
import{TransientStateLibrary}from"@uniswap/v4-core/src/libraries/TransientStateLibrary.sol";contractDeltaReader{usingTransientStateLibraryfor IPoolManager;functioncheckDeltaBeforeOperation(    Currency currency,address user)externalviewreturns(int256){// Important: This shows the current delta for this token/user pairreturn poolManager.getCurrentDelta(currency, user);// Negative: User owes tokens// Positive: User can claim tokens// Zero: No outstanding obligations}}
```

The `TransientStateLibrary` provides utilities to check the current state of deltas at any point in your transaction. The `getCurrentDelta` function returns an int256 where negative values indicate the user owes tokens to the PoolManager, positive values mean the user can claim tokens from the PoolManager, and zero means there are no outstanding obligations for this token/user pair.
## Resolving Deltas[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#resolving-deltas "Direct link to Resolving Deltas")
You must resolve all deltas before your transaction completes. There are two main approaches:
**1. Using ERC-20 Functions**
When using ERC-20 tokens, settling requires a specific sequence of operations:
```
functionresolveWithERC20(  Currency currency,uint256 amount)external{// For negative deltas (you owe tokens):if(!currency.isAddressZero()){// If not ETH    poolManager.sync(currency);// Sync currency balance firstIERC20Minimal(Currency.unwrap(currency)).transfer(address(poolManager),      amount);    poolManager.settle();// Complete the settlement}// For positive deltas (receiving tokens):  poolManager.take(currency,address(this), amount);}
```

When resolving negative deltas with ERC-20 tokens, you need to:
  1. Sync the currency balance with `sync()`
  2. Transfer the tokens to the PoolManager
  3. Complete the settlement with `settle()`


For positive deltas, simply use `take` to receive tokens from the PoolManager.
**2. Using ERC-6909 Functions**
```
functionresolveWithERC6909(  Currency currency,uint256 amount)external{// For negative deltas (you owe tokens):  poolManager.burn(currency,address(this), amount);// For positive deltas (receiving tokens):  poolManager.mint(currency,address(this), amount);}
```

ERC-6909 operations map to their ERC-20 equivalents in v4:
  * Use `burn` when you would use `settle` (for negative deltas)
  * Use `mint` when you would use `take` (for positive deltas)


Notice how this pattern requires no additional sync operations or separate token transfers.
> **Important** : _Every delta must be resolved before the transaction ends, or the entire transaction will revert. Use_ `TransientStateLibrary` _to verify your balances are properly settled._
> _Delta is a net balance resulting from token movements thus not bound to a certain token type i.e. can be resolved via mix-and-match with ERC-20 functions and ERC-6909 functions._
# Working with Flash Accounting
To interact with the PoolManager, we first need to create the functions our users will call. Then we'll implement the unlock callback pattern required to execute these operations.
## Using the Lock/Unlock Pattern[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#using-the-lockunlock-pattern "Direct link to Using the Lock/Unlock Pattern")
Let's start by creating our external function. First, we need to implement the callback that the `PoolManager` will use:
```
functionunlockCallback(bytescalldata data)externalreturns(bytesmemory){// To be implemented later}
```

Now let's implement our external function that users will call:
```
functionexecuteSwap(  PoolKey calldata key,uint256 amount)externalreturns(int256,int256){// Encode operation parametersbytesmemory data = abi.encode(key, amount);// Call unlock with encoded databytesmemory result = poolManager.unlock(data);// Optional: Decode any relevant return datareturn(0,0);// Replace with actual return values if needed}
```

When you call this function the flow followed is the following:
  1. `unlock` is called on the PoolManager
  2. PoolManager calls back to your `unlockCallback`
  3. Your callback executes the operations
  4. All deltas must be resolved before returning
  5. Execution of the logic returns to the PoolManager which verifies there are no outstanding deltas, and will relock itself


> **Warning** *: Always implement proper access control in your unlock callback. Only the PoolManager should be able to call it.*
## Implementing the Unlock Callback[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#implementing-the-unlockcallback "Direct link to Implementing the Unlock Callback")
First, let’s set up a contract with the proper unlock callback implementation:
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.24;import{IPoolManager}from"@uniswap/v4-core/src/interfaces/IPoolManager.sol";import{PoolKey}from"@uniswap/v4-core/src/types/PoolKey.sol";import{BalanceDelta}from"@uniswap/v4-core/src/types/BalanceDelta.sol";import{Currency}from"@uniswap/v4-core/src/types/Currency.sol";contractFlashAccountingExample{  IPoolManager public immutable poolManager;constructor(IPoolManager _poolManager){    poolManager = _poolManager;}functionexecuteSwap(    PoolKey calldata key,uint256 amount)externalreturns(int256,int256){...}functionunlockCallback(bytescalldata data)externalreturns(bytesmemory){// Important: Must check caller is PoolManagerrequire(msg.sender ==address(poolManager),"Not pool manager");// Decode and call our executeOperations function which // we'll implement next(bytesmemory result)=executeOperations(data);// Important: Must return bytes, even if emptyreturn result;}}
```

This base contract sets up the foundation for working with v4’s flash accounting. The `unlockCallback` function is required for any operations that access pool liquidity - when your contract calls `poolManager.unlock()`, the PoolManager calls back to this function to execute your operations.
The callback must verify it's being called by the PoolManager and return a bytes value (even if empty) to prevent transaction failures. Any actual pool operations (like swaps or liquidity changes) will be handled through the `executeOperations` function.
> **Critical Note** *: The most common mistake developers make is not returning a bytes value from unlockCallback. This will cause your transaction to revert. Always return a bytes value, even if it’s empty.*
Let’s add functionality to execute operations:
```
functionexecuteOperations(bytescalldata data)internalreturns(bytesmemory){// Decode operation parameters(PoolKey memory key,uint256 amount)= abi.decode(    data,(PoolKey,uint256));// Execute operation (e.g. swap)  BalanceDelta delta = poolManager.swap(    key,    IPoolManager.SwapParams({      zeroForOne:true,      amountSpecified:-int256(amount),      sqrtPriceLimitX96:0}),"");// Resolve deltasif(delta.amount0()<0){    poolManager.sync(key.currency0);IERC20Minimal(Currency.unwrap(key.currency0)).transfer(address(poolManager),uint256(-delta.amount0()));    poolManager.settle();}if(delta.amount1()>0){    poolManager.take(      key.currency1,address(this),uint256(delta.amount1()));}return"";// Return empty bytes if no specific result needed}
```

The `executeOperations` function handles the actual pool operations. It first decodes the data passed from the unlock call to get the operation parameters.
In this example, it executes a swap which creates deltas (token obligations) that must be resolved. For negative deltas (tokens we owe), we follow a specific sequence: first sync the currency state, then transfer the tokens to the PoolManager, and finally call settle. For positive deltas (tokens we receive), we use take to claim them. All deltas must be resolved before the function returns or the transaction will revert.
# Managing Liquidity with Flash Accounting
When adding or removing liquidity in v4, you’ll use `modifyLiquidity` which creates deltas that need to be handled through flash accounting. Let's understand how this works.
## Adding Liquidity[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#adding-liquidity "Direct link to Adding Liquidity")
```
// Example: Adding liquidity creates negative deltas (you need to provide tokens)BalanceDelta delta = poolManager.modifyLiquidity(  key,  IPoolManager.ModifyLiquidityParams({    tickLower: tickLower,// Lower price bound for position    tickUpper: tickUpper,// Upper price bound for position    liquidityDelta: liquidityAmount // Positive for adding liquidity}),""// No hook data needed);// Negative deltas for both tokens// delta.amount0() = -100 (need to provide token0)// delta.amount1() = -200 (need to provide token1)
```

When adding liquidity to a pool, you’ll need to provide both tokens in the pair. The `modifyLiquidity` function returns a [`BalanceDelta`](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta) that indicates how many tokens you need to provide. In this case:
  * The negative values in the delta (-100, -200) indicate you need to provide these amounts of each token
  * The values are proportional to the current pool price and your specified price range (tickLower to tickUpper)
  * These deltas must be resolved by providing the tokens before the transaction completes


## Removing Liquidity[​](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#removing-liquidity "Direct link to Removing Liquidity")
```
// Example: Removing liquidity creates positive deltas (you receive tokens)BalanceDelta delta = poolManager.modifyLiquidity(  key,  IPoolManager.ModifyLiquidityParams({    tickLower: tickLower,// Same position bounds as when added    tickUpper: tickUpper,    liquidityDelta:-liquidityAmount // Negative for removing liquidity}),""// No hook data needed);// Positive deltas for both tokens// delta.amount0() = +100 (receive token0)// delta.amount1() = +200 (receive token1)
```

When removing liquidity, the process is reversed. The negative `liquidityDelta` indicates you're removing liquidity, and the function returns positive deltas representing the tokens you'll receive:
  * The positive values (+100, +200) indicate the amounts you’ll receive of each token
  * The amounts depend on the pool’s current state and how much liquidity you’re removing
  * These positive deltas represent tokens you can claim from the pool


> **Important** *: Unlike single token operations, liquidity management typically involves handling deltas for both tokens in the pool.*
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/13-flash-accounting.mdx)
Was this helpful?
[PreviousStateView](https://docs.uniswap.org/contracts/v4/guides/state-view)[NextAccess msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)
On this page
  * [How Flash Accounting Works](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#how-flash-accounting-works)
  * [The PoolManager Lock Pattern](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#the-poolmanager-lockpattern)
  * [Working with Deltas](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#working-withdeltas)
  * [Reading Delta States](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#reading-deltastates)
  * [Resolving Deltas](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#resolving-deltas)
  * [Using the Lock/Unlock Pattern](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#using-the-lockunlock-pattern)
  * [Implementing the Unlock Callback](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#implementing-the-unlockcallback)
  * [Adding Liquidity](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#adding-liquidity)
  * [Removing Liquidity](https://docs.uniswap.org/contracts/v4/guides/flash-accounting#removing-liquidity)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/13-flash-accounting.mdx)
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
