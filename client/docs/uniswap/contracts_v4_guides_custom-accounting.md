# https://docs.uniswap.org/contracts/v4/guides/custom-accounting

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Unlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)
        * [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)
        * [Custom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Swap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Position Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)
        * [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Access msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
    * [Permit2](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Guides
  * [Custom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)


On this page
# Introduction
Uniswap v4 introduces a set of powerful, interconnected features that proposes a new way automated market makers (AMMs) can function. Custom accounting, hook fees, custom curves, and return deltas might seem like distinct concepts, but they form a cohesive system that enables unprecedented flexibility in decentralized exchange mechanisms.
These features are grouped together because they collectively represent the core of Uniswap v4’s customizability. They all relate to how pool state is managed and modified, working in tandem to allow developers to create highly tailored AMM experiences. From dynamic fee structures to unique bonding curves.
# Brief Overview of Concepts
Before we dive into the details of custom accounting, hook fees, custom curves, and return deltas, let’s explore how these features work in Uniswap v4.
## Delta Accounting in v4[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#delta-accounting-inv4 "Direct link to Delta Accounting in v4")
As described in [Flash Accounting](https://docs.uniswap.org/contracts/v4/concepts/flash-accounting) Uniswap v4 tracks net token transfers with transient storage. Unlike previous versions that tracked absolute token balances, v4 records changes to these balances (_deltas_). This approach is at the core of v4’s enhanced flexibility and efficiency.
In the v4 architecture, the [`PoolManager`](https://docs.uniswap.org/contracts/v4/concepts/PoolManager) manages credits or debits per address. After a swap router contract interacts with the PoolManager, the core contract determines that the swap router owes input tokens and must claim output tokens. Token balances are tracked as accumulated deltas in transient storage; and only the final deltas incur token transfers
Delta accounting provides several key benefits:
  1. More efficient state management, especially for complex operations involving multiple steps.
  2. Easier integration with hooks, allowing for custom logic to be applied to state changes.
  3. Improved gas efficiency for many operations, as it reduces the number of storage writes.


This system forms the foundation upon which other v4 features, such as hook fees and custom curves, are built. It allows for more complex pool behaviors while maintaining efficiency and flexibility.
## Hook Fees in v4[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#hook-fees-inv4 "Direct link to Hook Fees in v4")
Hook fees are a feature in Uniswap v4 that allow hook developers to monetize their hooks or implement custom value distribution mechanisms. Unlike pool fees or dynamic fees, hook fees are entirely separate and are implemented through custom logic in the hook itself.
Key characteristics of hook fees in Uniswap v4:
**Separate from Pool Fees**
Hook fees are distinct from the standard pool fees. They can be implemented alongside pool fees without interfering with the core fee structure.
**Implemented in beforeSwap**
Hook fees are typically calculated and applied in the `beforeSwap` function, allowing them to modify the swap parameters before the core swap logic is executed.
**Use of BeforeSwapDelta**
Hook fees often utilize the [`BeforeSwapDelta`](https://docs.uniswap.org/contracts/v4/reference/core/types/beforeswapdelta) mechanism to adjust swap amounts and transfer deltas from the hook to the swap router, enabling precise control over how the fee affects the swap.
**Flexible Implementation**
Developers have full control over how hook fees are calculated, collected, and distributed. This allows for complex fee structures tailored to specific use cases. In other words, developers can implement static fees, percentage-based fees, or even a fee that changes.
**Potential Use Cases**
  * Monetization of hook development
  * Implementation of withdrawal penalties (e.g., to penalize just-in-time liquidity provision)
  * Custom value distribution for liquidity providers


Keep reading because at the bottom we are providing a step by step guide on how to implement hook fees.
## Custom Curves in v4[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#custom-curves-inv4 "Direct link to Custom Curves in v4")
Custom Curves in Uniswap v4 represent a big change in AMM design flexibility. Unlike [Uniswap v2](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works) where the x*y=k formula was hardcoded, v4 allows developers to implement a wide variety of pricing models.
This is made possible through the hook system, particularly hooks that can interact with the swap process. Custom curves allow developers to eject the native concentrated liquidity pricing mechanism. These hooks can intercept swap requests, apply custom pricing logic, and return modified swap parameters. This enables the creation of pools with unique characteristics, such as:
  * Stable asset pairs with minimal price impact
  * Curves for special token types like rebasing tokens, RWAs, vault tokens


For example, creating a custom curve for a stable swap pool would involve designing a pricing function that maintains tighter price ranges when assets are near parity. This could be achieved by implementing a curve that's flatter in the middle (where assets are at their expected 1:1 ratio) and steeper at the edges (to discourage large imbalances).
This type of custom curve could significantly improve capital efficiency for stable asset pairs, reducing slippage for traders and potentially attracting more liquidity to the pool. It showcases how Uniswap v4's flexible architecture allows for tailored solutions to specific trading scenarios, opening up new possibilities in decentralized exchange design.
## Return Deltas in v4[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#return-deltas-inv4 "Direct link to Return Deltas in v4")
Return deltas are a fundamental mechanism in Uniswap v4's custom accounting system. They allow for precise, programmatic adjustments to the outcomes of operations within the protocol.
Key aspects of return deltas:
  1. **Dual Adjustment** : Return deltas simultaneously modify the balance changes (deltas) for both the hook contract and the swap router. This dual nature ensures that custom logic is accurately reflected across the entire system.
  2. **Credits and Debts Modification** : By adjusting these deltas, return deltas effectively alter the credits and debts owed by the hook and the swap router. This allows for complex economic models to be implemented directly within the protocol.
  3. **Native Pricing Bypass** : Return deltas enable hooks to implement custom curves that can completely bypass Uniswap's native pricing mechanism. This opens up possibilities for entirely new types of automated market makers within the Uniswap ecosystem.
  4. **Hook Fee Implementation** : Through return deltas, hooks can implement their own fee structures, separate from the core protocol fees.


In essence, return deltas allow for bespoke modification of an operation's result -- enabling features that were previously impossible in earlier versions of the protocol.
# Implementing Hook Fees: A Step-by-Step Guide
In this guide, we'll walk through the process of implementing a custom fee hook in Uniswap v4. We'll not only show you how to write the code but also explain what's happening under the hood at each step.
## Step 1: Setting Up the Hook[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-1-setting-up-the-hook "Direct link to Step 1: Setting Up the Hook")
First, let's create our basic hook structure:
```
// SPDX-License-Identifier: MITpragmasolidity0.8.26;import{BaseHook}from"v4-periphery/src/BaseHook.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{PoolKey}from"v4-core/src/types/PoolKey.sol";import{Currency}from"v4-core/src/types/Currency.sol";import{BeforeSwapDelta, toBeforeSwapDelta}from"v4-core/src/types/BeforeSwapDelta.sol";contractHookFeeExampleis BaseHook {uint256publicconstant HOOK_FEE_PERCENTAGE =10;// 0.01% feeuint256publicconstant FEE_DENOMINATOR =100000;constructor(IPoolManager _poolManager)BaseHook(_poolManager){}functiongetHookPermissions()publicpure override returns(Hooks.Permissions memory){return Hooks.Permissions({      beforeInitialize:false,      afterInitialize:false,      beforeAddLiquidity:false,      beforeRemoveLiquidity:false,      afterAddLiquidity:false,      afterRemoveLiquidity:false,      beforeSwap:true,      afterSwap:false,      beforeDonate:false,      afterDonate:false,      beforeSwapReturnDelta:true,      afterSwapReturnDelta:false,      afterAddLiquidityReturnDelta:false,      afterRemoveLiquidityReturnDelta:false});}}
```

Here, we're setting up our hook with a constant fee of 0.01% and enabling the `beforeSwap` and `beforeSwapReturnDelta` permissions.
## Step 2: Implementing the beforeSwap Function[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-2-implementing-the-beforeswap-function "Direct link to Step 2: Implementing the beforeSwap Function")
Now, let's implement our `beforeSwap` function:
```
function_beforeSwap(address,  PoolKey calldata key,  IPoolManager.SwapParams calldata params,bytescalldata)internal override returns(bytes4, BeforeSwapDelta,uint24){// Implementation details will be explained in the following sub-steps}
```

### Step 2.1: Calculate the swap amount and fee[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-21-calculate-the-swap-amount-and-fee "Direct link to Step 2.1: Calculate the swap amount and fee")
We determine the absolute swap amount and calculate our fee based on it.
```
uint256 swapAmount = params.amountSpecified <0?uint256(-params.amountSpecified):uint256(params.amountSpecified);uint256 feeAmount =(swapAmount * HOOK_FEE_PERCENTAGE)/ FEE_DENOMINATOR;
```

### Step 2.2: Collect the fee[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-22-collect-the-fee "Direct link to Step 2.2: Collect the fee")
We use `poolManager.take` to collect the fee. This creates a debt for our hook in the specified currency.
```
Currency feeCurrency;feeCurrency = params.amountSpecified <0== params.zeroForOne ? key.currency0 : key.currency1;poolManager.take(feeCurrency,address(this), feeAmount);
```

note
> Using `poolManager.take()` requires an ERC20 balance on the PoolManager, i.e. via other liquidity pools. If the `.take()` amount exceeds the ERC20 balance, the code will revert. As a workaround, use either:
>   1. `poolManager.mint()` to obtain ERC6909, which are also more gas efficient
>   2. A custom swap router, where input tokens are transferred to PoolManager before the router calls `poolManager.swap()`
> 

### Step 2.3: Create the BeforeSwapDelta[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-23-create-the-beforeswapdelta "Direct link to Step 2.3: Create the BeforeSwapDelta")
This is where the magic happens. We create a `BeforeSwapDelta` that represents our fee. The `toBeforeSwapDelta` function takes two parameters:
  * The specified delta: This is our fee amount. It's positive because we're adding it to the hook's balance.
  * The unspecified delta: We set this to 0 as we're not affecting the other currency.


```
BeforeSwapDelta returnDelta =toBeforeSwapDelta(int128(int256(feeAmount)),// Specified delta (fee amount)0// Unspecified delta (no change));
```

### Step 2.4: Return values[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-24-return-values "Direct link to Step 2.4: Return values")
We return the function selector, our `returnDelta`, and 0 for the fee override.
```
return(BaseHook.beforeSwap.selector, returnDelta,0);
```

## Step 3: Understanding the BeforeSwapDelta Mechanism[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-3-understanding-the-beforeswapdelta-mechanism "Direct link to Step 3: Understanding the BeforeSwapDelta Mechanism")
Now, let's dive deeper into how the `BeforeSwapDelta` works and how it affects the overall swap process.
  1. **Initial State** : Let's say a user wants to swap 100 USDC for USDT 
     * Hook's delta: (0, 0)
     * User's swap request: -100 USDC (negative because they're selling)
  2. **After Hook Execution** : Our hook has taken a 1 USDC fee (assuming 1% for simplicity): 
     * Hook's delta: (-1 USDC, 0) // The hook now owes 1 USDC to the pool
     * BeforeSwapDelta returned: (1 USDC, 0) // This will be added to the hook's delta and subtracted from the swap delta
  3. **PoolManager Processing** : The PoolManager applies our `BeforeSwapDelta` The pool then swaps 99 USDC for, let's say, 98 USDT.


```
amountToSwap = params.amountSpecified + hookDelta.getSpecifiedDelta();-99 USDC =-100 USDC +1 USDC
```

  1. **Delta Resolution** : The PoolManager then resolves the deltas:


```
// Hook's new deltanewHookDelta = oldHookDelta + returnDelta(0,0)=(-1 USDC,0)+(1 USDC,0)// Swap delta for routerswapDelta =(-99 USDC,98 USDT)-(1 USDC,0)=(-100 USDC,98 USDT)
```

  1. **Final Outcome** : 
     * The hook's debt is cleared: It took 1 USDC as a fee, but "returned" it to the swap process.
     * The router (on behalf of the user) must pay 100 USDC (original amount including fee) and receives 98 USDT.


This process demonstrates how `BeforeSwapDelta` effectively "transfers" the hook's outstanding delta to the swap router, ensuring that the user pays the fee while the hook collects it, all within a single atomic transaction.
## Conclusion[​](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#conclusion "Direct link to Conclusion")
By implementing hook fees this way, we've leveraged Uniswap v4's delta accounting system to create a seamless fee collection mechanism. This approach allows for complex fee structures and behaviors without disrupting the core swap process or requiring separate fee transfers.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/08-custom-accounting.mdx)
Was this helpful?
[PreviousReading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)[NextSwap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)
On this page
  * [Delta Accounting in v4](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#delta-accounting-inv4)
  * [Hook Fees in v4](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#hook-fees-inv4)
  * [Custom Curves in v4](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#custom-curves-inv4)
  * [Return Deltas in v4](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#return-deltas-inv4)
  * [Step 1: Setting Up the Hook](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-1-setting-up-the-hook)
  * [Step 2: Implementing the beforeSwap Function](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-2-implementing-the-beforeswap-function)
    * [Step 2.1: Calculate the swap amount and fee](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-21-calculate-the-swap-amount-and-fee)
    * [Step 2.2: Collect the fee](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-22-collect-the-fee)
    * [Step 2.3: Create the BeforeSwapDelta](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-23-create-the-beforeswapdelta)
    * [Step 2.4: Return values](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-24-return-values)
  * [Step 3: Understanding the BeforeSwapDelta Mechanism](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#step-3-understanding-the-beforeswapdelta-mechanism)
  * [Conclusion](https://docs.uniswap.org/contracts/v4/guides/custom-accounting#conclusion)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/08-custom-accounting.mdx)
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
