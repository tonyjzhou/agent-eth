# https://docs.uniswap.org/contracts/v4/guides/state-view

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/state-view#__docusaurus_skipToContent_fallback)
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
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
        * [Unlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)
        * [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)
        * [Custom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Swap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Position Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)
        * [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Access msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
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
  * Guides
  * [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view)


On this page
# Introduction
When building on **Uniswap v4** , you will often need to read pool state for both onchain and offchain use cases. Onchain contracts can directly invoke the [**StateLibrary**](https://github.com/Uniswap/v4-core/blob/main/src/libraries/StateLibrary.sol) to execute these reads during transactions, but offchain systems—such as frontends or analytics services—require a deployed contract with view functions. This is where [**StateView**](https://github.com/Uniswap/v4-periphery/blob/main/src/lens/StateView.sol) comes in.
> _In short: Use StateLibrary within onchain contracts and use StateView with an RPC for frontends and analytics._
By providing a dedicated interface for offchain reads, **StateView** helps:
  * Retrieve pool state without paying gas
  * Simplify integration for frontends, dashboards, and analytics
  * Ensure a clean separation between onchain logic and offchain queries


## Comparing onchain and offchain Access[​](https://docs.uniswap.org/contracts/v4/guides/state-view#comparing-onchain-and-offchain-access "Direct link to Comparing onchain and offchain Access")
If you’re familiar with [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state), you already know that Uniswap v4 uses **extsload** for efficient data access. For onchain usage, we rely on **StateLibrary** within contracts. However, offchain clients cannot rely on an onchain library for state reads.
Instead, **StateView** provides these same calls in a single contract designed explicitly for offchain consumption.
> _Because StateLibrary operates via onchain function calls, it’s not directly accessible to offchain clients. Hence, StateView provides a simple, gas-free interface designed for frontends and analytics._
For instance, an onchain contract might use the `StateLibrary` as follows:
```
// Onchain contract using StateLibrarycontractMyProtocol{usingStateLibraryfor IPoolManager;functioncheckPoolPrice(PoolId poolId)externalreturns(uint160){(uint160 sqrtPriceX96,,,)= poolManager.getSlot0(poolId);// ... use the price in contract logic ...return sqrtPriceX96;}}
```

By contrast, an offchain frontend or analytics service should interact with `StateView`:
```
// Frontend or analytics client using StateViewconst stateView =getContract({ address:STATE_VIEW_ADDRESS, abi: stateViewABI});const{ sqrtPriceX96 }=await stateView.read.getSlot0([poolId]);// ... use the price in your application ...
```

This separation ensures that each context (onchain vs. offchain) uses the most efficient data reading pattern.
# Usage With Frontend Clients
Frontend applications frequently display real-time information about pools, positions, and other market data—without incurring transaction costs. **StateView** addresses these requirements by exposing read-only functions tailored for offchain integrations.
## Setting Up With Viem[​](https://docs.uniswap.org/contracts/v4/guides/state-view#setting-up-with-viem "Direct link to Setting Up With Viem")
We’ll use [**viem**](https://viem.sh/), a TypeScript library for Ethereum, to demonstrate how to connect to **StateView**.
```
import{ createPublicClient, http }from'viem';import{ mainnet }from'viem/chains';// Initialize the clientconst client =createPublicClient({ chain: mainnet, transport:http()});// Set up StateView contract instanceconst stateView =getContract({ address:STATE_VIEW_ADDRESS, abi: stateViewABI, client})
```

> **Note:** _The stateView object comes from our getContract call above. Make sure you’ve imported stateViewABI correctly before attempting to read from the contract._
With this setup, you can now:
  * **Connect to an Ethereum network**
  * **Call StateView’s read functions**
  * **Retrieve pool information offchain at no gas cost**


### Handling Errors and Invalid Pool IDs[​](https://docs.uniswap.org/contracts/v4/guides/state-view#handling-errors-and-invalid-pool-ids "Direct link to Handling Errors and Invalid Pool IDs")
When calling `stateView.read.<function>([poolId])`, be mindful that:
  * If you pass an invalid `poolId` (typically a [`bytes32`](https://github.com/Uniswap/v4-core/blob/main/src/types/PoolId.sol#L6) in Uniswap v4), the call may revert or return unexpected data.
  * Consider adding try-catch (or equivalent error handling in your framework) to gracefully handle failures if the pool does not exist or if the call fails onchain.


# Reading Pool Data
Here are common examples of how to retrieve pool data using **StateView**.
## Getting Pool State[​](https://docs.uniswap.org/contracts/v4/guides/state-view#getting-pool-state "Direct link to Getting Pool State")
A pool’s core state, such as its current price or fees, is often necessary for frontends. Use `getSlot0`:
```
// Example: Reading pool price and feesconstgetPoolState=async(poolId:string)=>{// getSlot0 returns:// - Current price (sqrtPriceX96) in Q64.96 fixed-point format// - Active tick// - Protocol and LP fee settingsconst[  sqrtPriceX96,  tick,  protocolFee,  lpFee]=await stateView.read.getSlot0([poolId]);return{  price:calculatePrice(sqrtPriceX96),// implement your math logic for Q64.96  tick,  protocolFee,  lpFee};};
```

**What it Returns:**
  * **`sqrtPriceX96`**: The current pool price in Q64.96 fixed-point format.
  * **`tick`**: The current tick in which the pool is operating.
  * **`protocolFee`**and**`lpFee`**: Fee parameters for protocol and LP fee tiers.


## Getting Pool Liquidity[​](https://docs.uniswap.org/contracts/v4/guides/state-view#getting-pool-liquidity "Direct link to Getting Pool Liquidity")
To understand how much liquidity a pool holds:
```
// Example: Reading the total active liquidity of a poolconstgetPoolLiquidity=async(poolId:string)=>{// getLiquidity returns the total liquidity currently active in the poolconst liquidity =await stateView.read.getLiquidity([poolId]);return liquidity;};
```

**Why It Matters:**
  * Helps gauge the depth of the pool
  * Influences price impact calculations in trading
  * Provides context for the pool’s capacity to absorb trades


# Core Functions and Return Types
While **StateView** exposes many functions, here are several essential calls for most offchain applications. Each function typically takes a `poolId` (of type `bytes32`) as the key input, identifying which pool to query.
  1. **[`getSlot0(poolId)`](https://docs.uniswap.org/contracts/v4/reference/periphery/lens/StateView#getslot0)**
     * Returns `(uint160 sqrtPriceX96, int24 tick, uint8 protocolFee, uint8 lpFee)`.
     * Essential for displaying real-time price data and fees.
  2. **[`getLiquidity(poolId)`](https://docs.uniswap.org/contracts/v4/reference/periphery/lens/StateView#getliquidity)**
     * Returns `uint128 liquidity` (the total active pool liquidity).
     * Used to assess trading depth and volatility.
  3. **[`getPositionInfo(poolId, positionId)`](https://docs.uniswap.org/contracts/v4/reference/periphery/lens/StateView#getpositioninfo)**
     * Returns `(uint128 liquidity, uint256 feeGrowthInside0Last, uint256 feeGrowthInside1Last)`.
     * Critical for tracking user positions, especially to calculate earned fees over time.
  4. **[`getFeeGrowthGlobals(poolId)`](https://docs.uniswap.org/contracts/v4/reference/periphery/lens/StateView#getfeegrowthglobals)**
     * Returns `(uint256 feeGrowthGlobal0, uint256 feeGrowthGlobal1)`.
     * Useful for analytics around total fee accumulation in the pool.


### Note on `poolId` and `positionId`[​](https://docs.uniswap.org/contracts/v4/guides/state-view#note-on-poolid-and-positionid "Direct link to note-on-poolid-and-positionid")
  * In **Uniswap v4** , a `poolId` is typically a `bytes32` that is derived by calling `keccak256(abi.encode(poolKey))` where poolKey contains: 
    * currency0: The lower currency address of the pool
    * currency1: The higher currency address of the pool
    * fee: The pool LP fee (uint24)
    * tickSpacing: The tick spacing value (int24)
    * hooks: The hooks contract address
  * A `positionId` may also be a `bytes32` or other unique identifier that references a specific position.


# Security and Gas Considerations
  * **Offchain Reads** : Calls to `StateView` are purely read-only, so they cost no gas. This makes them ideal for frequently refreshing UI/analytics data.
  * **Onchain vs. Offchain** : Remember that if you need to integrate pool data into a live transaction, you must use `StateLibrary` within your smart contract.
  * **Edge Cases** : Always verify the returned data before using it in your application. Network or contract errors could lead to unexpected values.


# Conclusion
**StateView** is a powerful and efficient way to read Uniswap v4 pool data offchain. By separating onchain logic (using `StateLibrary`) and offchain reads (using `StateView`), Uniswap ensures the best developer experience for both contexts.
To recap:
  1. **Setup** : Use libraries like `viem` to connect to the Ethereum network.
  2. **Read** : Call `getSlot0`, `getLiquidity`, `getPositionInfo`, and other methods for crucial state data.
  3. **Handle Errors** : Implement basic checks for invalid `poolId` or connection failures.


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/12-state-view.mdx)
Was this helpful?
[PreviousPosition Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)[NextFlash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
On this page
  * [Comparing onchain and offchain Access](https://docs.uniswap.org/contracts/v4/guides/state-view#comparing-onchain-and-offchain-access)
  * [Setting Up With Viem](https://docs.uniswap.org/contracts/v4/guides/state-view#setting-up-with-viem)
    * [Handling Errors and Invalid Pool IDs](https://docs.uniswap.org/contracts/v4/guides/state-view#handling-errors-and-invalid-pool-ids)
  * [Getting Pool State](https://docs.uniswap.org/contracts/v4/guides/state-view#getting-pool-state)
  * [Getting Pool Liquidity](https://docs.uniswap.org/contracts/v4/guides/state-view#getting-pool-liquidity)
    * [Note on `poolId` and `positionId`](https://docs.uniswap.org/contracts/v4/guides/state-view#note-on-poolid-and-positionid)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/12-state-view.mdx)
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
