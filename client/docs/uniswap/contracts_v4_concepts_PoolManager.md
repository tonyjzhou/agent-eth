# https://docs.uniswap.org/contracts/v4/concepts/PoolManager

[Skip to main content](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
        * [v4 vs v3](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/concepts/flash-accounting)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks)
        * [Subscribers](https://docs.uniswap.org/contracts/v4/concepts/subscribers)
        * [PoolManager](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
        * [Dynamic Fees](https://docs.uniswap.org/contracts/v4/concepts/dynamic-fees)
        * [Integrated Routing with UniswapX](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
        * [v4 Fee Structure Guide](https://docs.uniswap.org/contracts/v4/concepts/fees)
        * [Security](https://docs.uniswap.org/contracts/v4/concepts/security)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
    * [Permit2](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Concepts
  * [PoolManager](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)


On this page
In Uniswap v3, each liquidity pool was represented by a separate smart contract deployed through the Uniswapv3Factory contract. While this approach provided flexibility, it also led to increased gas costs for pool creation and multi-hop swaps.
Uniswap v4 addresses this issue by introducing the Singleton design pattern. The PoolManager contract now serves as a single entry point for all liquidity pools. Instead of deploying separate contracts for each pool, the pool state and logic are encapsulated within the PoolManager itself.
# Purpose
The primary purpose of the `PoolManager` is to:
  * Efficiently manage liquidity pools
  * Facilitate token swaps
  * Reduce gas costs compared to the factory-based approach in Uniswap v3
  * Enable extensibility through hooks


# Architecture
## Singleton Design[​](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#singleton-design "Direct link to Singleton Design")
  * Uniswap v4 uses a Singleton design pattern for the `PoolManager`
  * All pool state and logic are encapsulated within the `PoolManager` contract


## Locking Mechanism[​](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#locking-mechanism "Direct link to Locking Mechanism")
  * The `PoolManager` uses a locking mechanism to allow for _flash accounting_ (also known as deferred balance accounting)
  * When unlocked, the calling contract can perform various operations and zero-out outstanding balances before returning control to the `PoolManager` for final solvency checks


## Pool State[​](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#pool-state "Direct link to Pool State")
  * The `Pool.State` struct contains information such as: 
    * Current price
    * Liquidity
    * Tick bitmap
    * Fee growth
    * Position information


## Libraries[​](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#libraries "Direct link to Libraries")
  * The pool logic is implemented using Solidity libraries to keep the `PoolManager` contract modular and gas-efficient
  * These libraries are: 
    * `Pool`: Contains core pool functionality, such as swaps and liquidity management
    * `Hooks`: Handles the execution of hook functions
    * `Position`: Manages liquidity positions within a pool


# Core Functionality
## Pool Creation[​](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#pool-creation "Direct link to Pool Creation")
  * New pools are created by calling the `initialize` function on the `PoolManager`
  * The pool creator specifies the token pair, fee tier, tick spacing, and optional hook contract address
  * The `PoolManager` initializes the pool state and associates it with a unique `PoolId`


## Swaps[​](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#swaps "Direct link to Swaps")
  * Swaps are initiated through the `swap` function on the `PoolManager`, typically via a swap router contract
  * The `PoolManager` executes the following steps: 
    1. Checks if the pool is valid and initialized
    2. Executes the `beforeSwap` hook, if applicable
    3. Performs the actual swap, updating the pool state and charging fees
    4. Executes the `afterSwap` hook, if applicable
    5. Calculates the net token amounts owed to the user and the pool, represented by the `BalanceDelta` struct
  * Swaps utilize flash accounting, where tokens are moved into the `PoolManager`, and only the final output tokens are withdrawn


## Liquidity Management[​](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#liquidity-management "Direct link to Liquidity Management")
  * Liquidity providers can add or remove liquidity using the `modifyLiquidity` function on the `PoolManager`. However, you wouldn't call this directly from your application, you would call this from a periphery contract to handle the locking & unlocking a particular pool.
  * The `PoolManager` executes the following steps: 
    1. Checks if the pool is valid and initialized
    2. Determines if the modification is an addition or removal of liquidity
    3. Executes the appropriate `beforeAddLiquidity` or `beforeRemoveLiquidity` hook, if applicable
    4. Performs the actual liquidity modification and updates the pool state
    5. Emits the `ModifyLiquidity` event
    6. Executes the appropriate `afterAddLiquidity` or `afterRemoveLiquidity` hook, if applicable
    7. Calculates the balance delta and returns it to the caller


## Flash Accounting[​](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#flash-accounting "Direct link to Flash Accounting")
  * The `PoolManager` employs flash accounting to reduce gas costs and simplify multi-hop swaps
  * Tokens are moved into the `PoolManager` contract, and all subsequent actions are performed within the contract's context
  * Only the final output tokens are withdrawn from the `PoolManager` at the end of the transaction


# Transient Storage
  * The `PoolManager` utilizes transient storage (EIP-1153) to store temporary data during complex operations
  * Transient storage reduces gas costs by avoiding regular storage operations for data only needed within a single transaction


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/06-PoolManager.mdx)
Was this helpful?
[PreviousSubscribers](https://docs.uniswap.org/contracts/v4/concepts/subscribers)[NextDynamic Fees](https://docs.uniswap.org/contracts/v4/concepts/dynamic-fees)
On this page
  * [Singleton Design](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#singleton-design)
  * [Locking Mechanism](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#locking-mechanism)
  * [Pool State](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#pool-state)
  * [Libraries](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#libraries)
  * [Pool Creation](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#pool-creation)
  * [Swaps](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#swaps)
  * [Liquidity Management](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#liquidity-management)
  * [Flash Accounting](https://docs.uniswap.org/contracts/v4/concepts/PoolManager#flash-accounting)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/06-PoolManager.mdx)
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
