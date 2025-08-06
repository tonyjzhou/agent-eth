# https://docs.uniswap.org/contracts/v4/guides/read-pool-state

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#__docusaurus_skipToContent_fallback)
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
  * [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)


On this page
# Reading Pool State
## Introduction[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#introduction "Direct link to Introduction")
Unlike previous versions, v4 uses a different approach for storing and accessing pool data, which requires understanding the use of [`StateLibrary`](https://docs.uniswap.org/contracts/v4/reference/core/libraries/StateLibrary) and [`extsload`](https://docs.uniswap.org/contracts/v4/reference/core/Extsload).
## Understanding the PoolManager Architecture[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#understanding-the-poolmanager-architecture "Direct link to Understanding the PoolManager Architecture")
### The Singleton Design[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#the-singleton-design "Direct link to The Singleton Design")
In Uniswap v4, all pools are managed by a single `PoolManager` contract, unlike v3 where each pool was a separate contract. This design offers simplified management since all pools are now accessible through a single contract.
This approach significantly reduces deployment costs, simplifies protocol upgrades, and enables more efficient cross-pool interactions. It also allows for easier implementation of new features across all pools simultaneously.
### Pools as Library Calls[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#pools-as-library-calls "Direct link to Pools as Library Calls")
In v4, pools are stored as complex structs, with Solidity libraries handling state changes. The `PoolManager` contract uses these libraries to perform operations on the pool state:
```
contractPoolManager{usingPoolfor Pool.State;mapping(PoolId => Pool.State)internal pools;functionswap(PoolId id,...)external{    pools[id].swap(...);// Library call}}
```

This design allows all AMM logic to be encapsulated within the `PoolManager` contract.
## Reading Pool State in v4[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#reading-pool-state-in-v4 "Direct link to Reading Pool State in v4")
In Uniswap v4, reading pool state involves a few key concepts and mechanisms that differ from previous versions. At the core of this new structure is a complex mapping within the PoolManager contract:
```
mapping(PoolId id => Pool.State)internal _pools;
```

This mapping represents a fundamental shift in pool data storage:
  1. Each pool is identified by a unique `PoolId`.
  2. The `Pool.State` is a struct that contains all the state variables for a single pool.
  3. This struct itself contains several nested mappings and complex data structures.


For example, the `Pool.State` struct might look something like this (simplified for illustration):
```
structState{uint160 sqrtPriceX96;int24 tick;uint128 liquidity;uint256 feeGrowthGlobal0X128;uint256 feeGrowthGlobal1X128;mapping(int24=> TickInfo) ticks;mapping(bytes32=> Position.Info) positions;// ... other fields}
```

This complex structure allows for efficient storage of multiple pools and their associated data within a single contract. However, it also means that traditional getter functions would be inefficient or impractical for accessing this data, especially for nested mappings like `ticks` and `positions`.
To address this, Uniswap V4 introduces the StateLibrary and the concept of using `extsload` for reading pool state. These mechanisms provide efficient ways to access the data stored in this complex structure.
### The StateLibrary and `extsload`[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#the-statelibrary-and-extsload "Direct link to the-statelibrary-and-extsload")
```
abstract contractExtsloadis IExtsload {/// @inheritdoc IExtsloadfunctionextsload(bytes32 slot)externalviewreturns(bytes32){assembly("memory-safe"){mstore(0,sload(slot))return(0,0x20)}}// [...]}
```

The `StateLibrary` is a crucial component in Uniswap v4 for reading pool state. It utilizes the `extsload` function, which is an external wrapper for the `SLOAD` opcode. This allows for efficient reading of arbitrary storage slots.
**How`extsload` works:**
  * It takes a storage slot as input.
  * It reads the value stored in that slot directly, using `SLOAD`, from the contract's storage.
  * It returns the value as a `bytes32`.


This method is more gas-efficient than traditional getter functions, especially when reading multiple storage slots.
Moreover, using `extsload` instead of hand-written Solidity view functions lowers the contract bytecode size. This optimization is particularly important for Uniswap v4, as the core contracts are nearly at Ethereum's contract size limit.
### TransientStateLibrary and `exttload`[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#transientstatelibrary-and-exttload "Direct link to transientstatelibrary-and-exttload")
```
abstract contractExttloadis IExttload {/// @inheritdoc IExttloadfunctionexttload(bytes32 slot)externalviewreturns(bytes32){assembly("memory-safe"){mstore(0,tload(slot))return(0,0x20)}}// [...]}
```

While `StateLibrary` deals with persistent storage, [`TransientStateLibrary`](https://docs.uniswap.org/contracts/v4/reference/core/libraries/transient-state-library) is used for handling transient storage. Transient storage, introduced in EIP-1153, is a way to store data that is only needed for the duration of a transaction, making it ideal for temporary data.
It uses the [`exttload`](https://docs.uniswap.org/contracts/v4/reference/core/Exttload) function, which is similar to `extsload`, but for transient storage; it is an external wrapper for the `TLOAD` opcode.
## Implementing a `PoolStateReader` Contract[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#implementing-a-poolstatereader-contract "Direct link to implementing-a-poolstatereader-contract")
Let's create a `PoolStateReader` contract that showcases different methods for reading pool state. For each function, we'll explain its purpose, how it works, and provide an example use case.
```
// SPDX-License-Identifier: MITpragmasolidity0.8.26;import{IPoolManager}from"v4-core/interfaces/IPoolManager.sol";import{PoolKey}from"v4-core/types/PoolKey.sol";import{PoolId, PoolIdLibrary}from"v4-core/types/PoolId.sol";contractPoolStateReader{usingPoolIdLibraryfor PoolKey;  IPoolManager public immutable poolManager;constructor(IPoolManager _poolManager){    poolManager = _poolManager;}// Functions will be implemented here}
```

Before we start, we need to import `StateLibrary` from the libraries available in v4-core.
```
import{StateLibrary}from"v4-core/libraries/StateLibrary.sol";
```

Let's focus on this important line that we should add:
```
usingStateLibraryfor IPoolManager;
```

This line is crucial for our PoolStateReader contract because it allows us to call StateLibrary functions as if they were methods of the IPoolManager interface, like for instance now we will be able to do `poolManager.getSlot0()`.
Now we’re up for breaking down each wrapper function that we're going to be adding to our helper contract, explain the purpose of the pool manager function to read the state, and provide use cases to make sure we understand its utility:
### `getSlot0()`[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#getslot0 "Direct link to getslot0")
```
functiongetPoolState(PoolKey calldata key)externalviewreturns(uint160 sqrtPriceX96,int24 tick,uint24 protocolFee,uint24 lpFee){return poolManager.getSlot0(key.toId());}
```

**Explanation:**
This function retrieves the current state of the pool, including its price, tick, and fee settings. It uses the `getSlot0()` function from `StateLibrary`, which efficiently reads these values from a single storage slot.
  * `sqrtPriceX96`: The current price, encoded as a square root and scaled by 2^96. This encoding allows for efficient price calculations in the Uniswap algorithm.
  * `tick`: The current tick, representing the quantized price. Ticks are used to efficiently track and update liquidity positions.
  * `protocolFee`: The current protocol fee, represented in hundredths of a bip (i.e., units of 0.0001%).
  * `lpFee`: The current liquidity provider fee, also represented in hundredths of a bip.


**Use Case:**
This function is essential for any application that needs to know the current state of a Uniswap v4 pool. For example:
  * A price oracle could use this to get the current price of the pool.
  * A trading bot could use this to determine if a trade is profitable given the current price and fees.
  * A liquidity management system could use the `tick` to decide where to place new liquidity.


### `getLiquidity()`[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#getliquidity "Direct link to getliquidity")
```
functiongetPoolLiquidity(PoolKey calldata key)externalviewreturns(uint128 liquidity){return poolManager.getLiquidity(key.toId());}
```

**Explanation:**
This function retrieves the current total liquidity in the pool. Liquidity in Uniswap v4 represents the amount of assets available for trading within the current price range.
**Use Case:**
Understanding the total liquidity is crucial for several scenarios:
  * Assessing the depth of the market and potential slippage for large trades.
  * Monitoring the depth and growth of a pool over time.


### `getPositionInfo`[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#getpositioninfo "Direct link to getpositioninfo")
```
functiongetPositionInfo(  PoolKey calldata key,address owner,int24 tickLower,int24 tickUpper,bytes32 salt)externalviewreturns(uint128 liquidity,uint256 feeGrowthInside0LastX128,uint256 feeGrowthInside1LastX128){return poolManager.getPositionInfo(key.toId(), owner, tickLower, tickUpper,bytes32(salt));}
```

**Explanation:**
This function retrieves information about a specific liquidity position. It returns:
  * `liquidity`: The amount of liquidity in the position.
  * `feeGrowthInside0LastX128` and `feeGrowthInside1LastX128`: The last recorded cumulative fees earned per unit of liquidity for each token.


**Use Case:**
This function is crucial for applications that need to manage or analyze individual liquidity positions:
  * A liquidity management dashboard could use this to display a user's current positions and earned fees.
  * An automated liquidity provision system could use this to decide when to rebalance or compound rewards.
  * An analytics tool could use this to calculate the performance of different liquidity provision strategies.


### `getFeeGrowthGlobal`[​](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#getfeegrowthglobal "Direct link to getfeegrowthglobal")
```
functiongetFeeGrowthGlobal(PoolKey calldata key)externalviewreturns(uint256 feeGrowthGlobal0X128,uint256 feeGrowthGlobal1X128){return poolManager.getFeeGrowthGlobal(key.toId());}
```

**Explanation:**
This function retrieves the global fee growth for both tokens in the pool. These values represent the cumulative fees per unit of liquidity since the pool's inception.
**Use Case:**
Global fee growth is essential for several advanced operations:
  * Calculating the fees earned by a position that has been held for a long time.
  * Analyzing the overall fee generation of a pool over its lifetime.
  * Comparing the performance of different pools or fee tiers.


For additional reference, see [`StateLibrary`](https://docs.uniswap.org/contracts/v4/reference/core/libraries/StateLibrary) and [`Extsload`](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/07-read-pool-state.mdx)
Was this helpful?
[PreviousUnlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)[NextCustom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
On this page
  * [Introduction](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#introduction)
  * [Understanding the PoolManager Architecture](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#understanding-the-poolmanager-architecture)
    * [The Singleton Design](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#the-singleton-design)
    * [Pools as Library Calls](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#pools-as-library-calls)
  * [Reading Pool State in v4](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#reading-pool-state-in-v4)
    * [The StateLibrary and `extsload`](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#the-statelibrary-and-extsload)
    * [TransientStateLibrary and `exttload`](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#transientstatelibrary-and-exttload)
  * [Implementing a `PoolStateReader` Contract](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#implementing-a-poolstatereader-contract)
    * [`getSlot0()`](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#getslot0)
    * [`getLiquidity()`](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#getliquidity)
    * [`getPositionInfo`](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#getpositioninfo)
    * [`getFeeGrowthGlobal`](https://docs.uniswap.org/contracts/v4/guides/read-pool-state#getfeegrowthglobal)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/07-read-pool-state.mdx)
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
