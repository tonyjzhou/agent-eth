# https://docs.uniswap.org/contracts/v4/guides/ERC-6909

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Unlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)
        * [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)
        * [Custom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Swap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Position Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)
        * [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Access msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
    * [Permit2](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Guides
  * [ERC-6909](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)


On this page
# Introduction
Uniswap v4 uses [ERC-6909](https://docs.uniswap.org/contracts/v4/concepts/erc6909), a token standard that works alongside the protocol’s flash accounting system. This guide explains how ERC-6909 functions within v4, when to use mint versus burn operations, and how developers can implement them effectively.
# What is ERC-6909?
ERC-6909 is a token standard that enables efficient token management within a single contract through multiple token balances per user. Where ERC-20 requires separate approve and transfer calls for token interactions, ERC-6909 provides native support for multi-token operations through mint/burn mechanics that integrate with v4’s flash accounting system.
Here’s how the approaches differ:
```
// Traditional ERC-20 approachIERC20(tokenA).transferFrom(owner, poolManager, amount);// ERC-6909 approach in v4poolManager.burn(owner, currency.toId(), amount);
```

## Integration with Flash Accounting[​](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#integration-with-flash-accounting "Direct link to Integration with Flash Accounting")
While flash accounting tracks balance changes as deltas throughout a transaction, ERC-6909 provides an additional primitive to resolve deltas.
This enables:
  1. Simplified transaction flows through direct mint/burn operations
  2. Efficient handling of multi-step operations
  3. Streamlined token management when using the PoolManager


## Gas Efficiency[​](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#gas-efficiency "Direct link to Gas Efficiency")
ERC-6909 provides gas savings compared to ERC-20 tokens, making it particularly valuable for use cases requiring frequent token transfers like:
  * Day trading operations
  * MEV bot transactions
  * Active liquidity management


This efficiency is especially beneficial when performing multiple token operations in rapid succession.
## Simplified Token Management[​](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#simplified-token-management "Direct link to Simplified Token Management")
The traditional ERC-20 workflow requires careful management of allowances and transfers, often leading to complex transaction sequences and potential security concerns.
ERC-6909 takes a different approach by providing direct balance modifications through mint and burn operations.
By working through the PoolManager, all token operations are consolidated into a single interface. This means you don’t need to worry about managing multiple token approvals or keeping track of allowances across different contracts. Instead, you can focus on the core logic of your application while the PoolManager handles the token management aspects.
# Understanding ERC-6909 in v4
Let's explore how ERC-6909 is used across different v4 operations and understand when to use each of its operations.
## Operations and Token Movement[​](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#operations-and-token-movement "Direct link to Operations and Token Movement")
Different pool operations create different types of deltas that need to be resolved:
  * **Swaps** : Create negative deltas for input tokens and positive deltas for output tokens
  * **Adding Liquidity** : Creates negative deltas (tokens you need to provide)
  * **Removing Liquidity** : Creates positive deltas (tokens you receive)
  * **Donations** : Creates negative deltas (tokens you're donating)


## Using Mint and Burn[​](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#using-mint-and-burn "Direct link to Using Mint and Burn")
The choice between mint and burn operations depends on your token movement needs:
```
// When you have positive deltas (withdrawing value from PoolManager):poolManager.mint(currency,address(this), amount);// When you have negative deltas (transferring value to PoolManager):poolManager.burn(currency,address(this), amount);
```

This pattern is used throughout v4's operations:
  * Use mint when withdrawing value from the pool (like receiving tokens from swaps)
  * Use burn when transferring value to the pool (like providing tokens)


## Hook Integration[​](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#hook-integration "Direct link to Hook Integration")
When building hooks, ERC-6909 operations help manage token movements within your hook's logic:
```
function_beforeSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata params)internalreturns(bytes4, BeforeSwapDelta,uint24){  poolManager.mint(key.currency0,address(this), amount);return(   BaseHook.beforeSwap.selector,   BeforeSwapDeltaLibrary.ZERO_DELTA,0);}
```

Other common cases would be to use `mint` for fee collection or `burn` for token distribution.
# Implementation
Let's build a contract that handles donations in v4 using ERC-6909. We'll create a donation router that follows this flow:
  1. Users call our donation function with their desired amounts
  2. Our contract packages this data and uses the PoolManager's unlock pattern
  3. In the callback, we unpack the data and execute the donation, handling token movements using ERC-6909


First, let's set up our contract with the necessary imports and create a struct to help us pass data between functions:
```
// SPDX-License-Identifier: MITpragmasolidity0.8.24;import{ IPoolManager }from"@uniswap/v4-core/src/interfaces/IPoolManager.sol";import{ PoolKey }from"@uniswap/v4-core/src/types/PoolKey.sol";import{ BalanceDelta }from"@uniswap/v4-core/src/types/BalanceDelta.sol";import{ Currency }from"@uniswap/v4-core/src/types/Currency.sol";contractDonationRouter{  IPoolManager public immutable poolManager;// This struct helps us pack donation parameters to pass through// the unlock/callback patternstructCallbackData{    PoolKey key;uint256 amount0;uint256 amount1;bytes hookData;}constructor(IPoolManager _poolManager){    poolManager = _poolManager;}}
```

Now let's implement the external donation function. Here we'll pack our parameters into the CallbackData struct and start the unlock process:
```
/// @notice Donates tokens to a pool/// @param key The pool to donate to/// @param amount0 Amount of token0 to donate/// @param amount1 Amount of token1 to donate/// @param hookData Optional data to pass to hooksfunctiondonate(  PoolKey memory key,uint256 amount0,uint256 amount1,bytesmemory hookData)externalreturns(BalanceDelta delta){// 1. Create a CallbackData struct with all our parameters  CallbackData memory data =CallbackData({    key: key,    amount0: amount0,    amount1: amount1,    hookData: hookData});// 2. Encode our struct into bytes to pass through unlockbytesmemory encodedData = abi.encode(data);// 3. Call unlock with our encoded data// 4. unlock will call our callback, which returns encoded delta// 5. Decode the returned bytes back into a BalanceDelta  delta = abi.decode(    poolManager.unlock(encodedData),(BalanceDelta));}
```

When the PoolManager calls our callback, we need to decode our data:
```
functionunlockCallback(bytescalldata rawData)externalreturns(bytesmemory){// Only the PoolManager can trigger our callbackrequire(msg.sender ==address(poolManager));// Decode the bytes back into our CallbackData struct// (CallbackData) tells abi.decode what type to expect  CallbackData memory data = abi.decode(rawData,(CallbackData));
```

Now `data` contains the same values we packed in donate():
  * `data.key`: The pool to donate to
  * `data.amount0`: Amount of first token
  * `data.amount1`: Amount of second token
  * `data.hookData`: Any hook data


And we can execute the donation:
```
// Execute the donation through PoolManager// This creates negative deltas for the tokens we're donating  BalanceDelta delta = poolManager.donate(    data.key,    data.amount0,    data.amount1,    data.hookData);
```

After executing the donation through the PoolManager, we need to handle the token transfers. The donation creates negative deltas, which represent tokens that we owe to the PoolManager. This is where ERC-6909's burn operation comes into play.
Instead of using traditional token transfers, we can use ERC-6909's burn operation to settle this debt. We check each token's delta and, if negative, burn the corresponding amount of ERC-6909 tokens. And finally return the encoded delta. Let’s see how:
```
// Handle any negative deltas by burning ERC-6909 tokensif(delta.amount0()<0){    poolManager.burn(      data.key.currency0,address(this),uint256(-delta.amount0()));}if(delta.amount1()<0){    poolManager.burn(      data.key.currency1,address(this),uint256(-delta.amount1()));}// Encode and return the delta so donate() can decode itreturn abi.encode(delta);}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/10-ERC-6909.mdx)
Was this helpful?
[PreviousSwap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)[NextPosition Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)
On this page
  * [Integration with Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#integration-with-flash-accounting)
  * [Gas Efficiency](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#gas-efficiency)
  * [Simplified Token Management](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#simplified-token-management)
  * [Operations and Token Movement](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#operations-and-token-movement)
  * [Using Mint and Burn](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#using-mint-and-burn)
  * [Hook Integration](https://docs.uniswap.org/contracts/v4/guides/ERC-6909#hook-integration)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/10-ERC-6909.mdx)
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
