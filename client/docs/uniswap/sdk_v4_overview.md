# https://docs.uniswap.org/sdk/v4/overview

[Skip to main content](https://docs.uniswap.org/sdk/v4/overview#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/overview)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/overview)
        * [Swaps](https://docs.uniswap.org/sdk/v4/overview)
        * [Position Management](https://docs.uniswap.org/sdk/v4/overview)
        * [Advanced](https://docs.uniswap.org/sdk/v4/overview)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/overview)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/overview)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/overview)
    * [web3-react](https://docs.uniswap.org/sdk/v4/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)


On this page
# The Uniswap v4 SDK
> **Welcome to the v4 Uniswap SDK!**
The Uniswap v4 SDK provides abstractions to assist you with interacting with the Uniswap v4 smart contracts in a Typescript/Javascript environment (e.g. websites, node scripts). It makes use of the [**Core SDK**](https://docs.uniswap.org/sdk/core/overview) to gain access to abstractions that are common amongst the Uniswap SDKs. With the SDK, you can add/remove liquidity, collect fees like what you will usually do with v3 SDK, but more with the extra functionalities from hooks introduced in v4!
For complete documentation of the SDK's offerings, see the [**Technical Reference**](https://docs.uniswap.org/sdk/v4/reference/overview).
## Installation[​](https://docs.uniswap.org/sdk/v4/overview#installation "Direct link to Installation")
To interact with the v4 SDK we recommend installing through npm:
```
npm i --save @uniswap/v4-sdknpm i --save @uniswap/sdk-core
```

## What's Different in v4[​](https://docs.uniswap.org/sdk/v4/overview#whats-different-in-v4 "Direct link to What's Different in v4")
The Uniswap v4 SDK introduces some major changes that fundamentally alter how developers interact with Uniswap. Understanding these key differences is essential for successful v4 development.
## Key Changes[​](https://docs.uniswap.org/sdk/v4/overview#key-changes "Direct link to Key Changes")
### Universal Router Requirement for Swapping[​](https://docs.uniswap.org/sdk/v4/overview#universal-router-requirement-for-swapping "Direct link to Universal Router Requirement for Swapping")
**What changed** : v3 allowed direct calls to the v3 Swap Router contract. v4 requires all swapping operations to go through the Universal Router.
**Why** : v4's singleton PoolManager architecture and flash accounting system require a different interaction pattern. The v4Planner batches operations and encodes them for the Universal Router - you cannot make direct swap calls.
**Key differences** :
  * All swaps must be "planned" using v4Planner, even single swaps
  * Operations use new patterns: SETTLE (pay tokens) and TAKE (receive tokens)
  * Enables efficient multi-step operations and cross-protocol routes


**Impact** : This enables more efficient multi-step operations in a single transaction.
### StateView Contract Introduction[​](https://docs.uniswap.org/sdk/v4/overview#stateview-contract-introduction "Direct link to StateView Contract Introduction")
**Why it exists** : v4 uses a singleton PoolManager that tracks all pools in one contract, unlike v3's separate pool contracts.
**What it means** : The StateView contract wraps the PoolManager's state reading functions with a dedicated view-only interface. Instead of calling the PoolManager directly for state queries, you use StateView for cleaner, more organized access to pool data like slot0, tick info, liquidity, and position information.
**Impact** : Provides a dedicated, organized interface for off-chain clients to read pool state data.
### Position Fetching Changed[​](https://docs.uniswap.org/sdk/v4/overview#position-fetching-changed "Direct link to Position Fetching Changed")
**What changed** : v3 allowed easy enumeration of user positions on-chain. v4 provides no way to get all of a user's position IDs directly from the contracts. Additionally, position information is packed into a single uint256 value for efficiency, requiring decoding to extract individual fields like liquidity, fee growth, and tick ranges.
**Why it's different** : This design choice means position enumeration must happen off-chain through event indexing.
**Impact** : Applications must choose and implement indexing solutions to know which positions a user owns.
### Fee Collection Behavior Changed[​](https://docs.uniswap.org/sdk/v4/overview#fee-collection-behavior-changed "Direct link to Fee Collection Behavior Changed")
**What changed** : v3 had an explicit `collect()` function for fee collection. v4 has no standalone collect function - fees are automatically collected and distributed when you modify positions.
**New pattern** :
  * Fees automatically roll over when increasing/decreasing liquidity
  * To collect fees without modifying position size, you must modify the position with zero change (e.g., `modifyLiquidity(positionId, 0)`)
  * StateView contract must be used to query the fee growth inside in order to calculate the exact amount of fees owed


**Impact** : Fee collection logic must be redesigned around position modifications rather than explicit collect() calls.
### Quick Comparison[​](https://docs.uniswap.org/sdk/v4/overview#quick-comparison "Direct link to Quick Comparison")
Feature| v3| v4  
---|---|---  
Swapping| Direct router calls| Universal Router  
Pool State| Individual pool contracts| StateView contract  
Position Discovery| On-chain enumeration| Off-chain indexing  
Fee Collection| Explicit collect()| Automatic on modification  
### What This Means for Developers[​](https://docs.uniswap.org/sdk/v4/overview#what-this-means-for-developers "Direct link to What This Means for Developers")
### Migration Requirements[​](https://docs.uniswap.org/sdk/v4/overview#migration-requirements "Direct link to Migration Requirements")
  1. **Restructure all swaps** to use Universal Router with v4Planner
  2. **Build position indexing systems** using event logs and subgraphs for position discovery
  3. **Redesign fee collection logic** to use position modifications instead of explicit collect() calls
  4. **Implement StateView integration** for all pool state queries instead of direct PoolManager calls


### Development Impact[​](https://docs.uniswap.org/sdk/v4/overview#development-impact "Direct link to Development Impact")
  * **Universal Router** : All swaps must be batched, but enables complex multi-step operations
  * **Position tracking** : Requires additional infrastructure
  * **Fee collection** : Simpler in some cases (automatic), more complex in others (zero-change modifications)
  * **StateView** : Cleaner interface for state queries


## Learning Path[​](https://docs.uniswap.org/sdk/v4/overview#learning-path "Direct link to Learning Path")
To get started with v4 SDK development, follow these guides based on the key changes:
### 1. Swapping[​](https://docs.uniswap.org/sdk/v4/overview#1-swapping "Direct link to 1. Swapping")
Learn how to restructure swaps using Universal Router integration.
**Guides** :
  * [Getting a Quote](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
  * [Executing a Single-Hop Swap](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping)
  * [Executing Multi-Hop Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)


### 2. Position Management (Off-chain Indexing + Fee Collection)[​](https://docs.uniswap.org/sdk/v4/overview#2-position-management-off-chain-indexing--fee-collection "Direct link to 2. Position Management \(Off-chain Indexing + Fee Collection\)")
Understand position tracking systems and the new automatic fee collection patterns.
**Guides** :
  * [Minting a Position](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
  * [Fetching Positions](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
  * [Collecting Fees](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
  * [Adding and Removing Liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity)


### 3. Advanced Features (StateView + Pool Creation)[​](https://docs.uniswap.org/sdk/v4/overview#3-advanced-features-stateview--pool-creation "Direct link to 3. Advanced Features \(StateView + Pool Creation\)")
Explore efficient state queries and pool creation with the new architecture.
**Guides** :
  * [Fetching Pool Data](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
  * [Create Pool](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)


## Developer Links[​](https://docs.uniswap.org/sdk/v4/overview#developer-links "Direct link to Developer Links")
  * [**v4 SDK GitHub Repo**](https://github.com/Uniswap/sdks/tree/main/sdks/v4-sdk)
  * [**Core SDK GitHub Repo**](https://github.com/Uniswap/sdk-core)
  * [**v4 SDK NPM Package**](https://www.npmjs.com/package/@uniswap/v4-sdk)


[![Unit Tests](https://github.com/Uniswap/uniswap-v3-sdk/workflows/Unit%20Tests/badge.svg)](https://github.com/Uniswap/sdks/actions?query=workflow%3A%22%22Code+Quality+Checks%22%22++) [![Lint](https://github.com/Uniswap/uniswap-v3-sdk/workflows/Lint/badge.svg)](https://github.com/Uniswap/sdks/actions?query=workflow%3A%22%22Code+Quality+Checks%22%22++) [![npm version](https://img.shields.io/npm/v/@uniswap/v4-sdk/latest.svg)](https://www.npmjs.com/package/@uniswap/v4-sdk/v/latest) [![npm bundle size \(scoped version\)](https://img.shields.io/bundlephobia/minzip/@uniswap/v4-sdk/latest.svg)](https://bundlephobia.com/result?p=@uniswap/v4-sdk@latest) [![Discord](https://img.shields.io/badge/discord-join%20chat-blue.svg)](https://discord.com/channels/597638925346930701/607978109089611786)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/overview.md)
Was this helpful?
[PreviousInterfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)[NextOverview](https://docs.uniswap.org/sdk/v4/overview)
On this page
  * [Installation](https://docs.uniswap.org/sdk/v4/overview#installation)
  * [What's Different in v4](https://docs.uniswap.org/sdk/v4/overview#whats-different-in-v4)
  * [Key Changes](https://docs.uniswap.org/sdk/v4/overview#key-changes)
    * [Universal Router Requirement for Swapping](https://docs.uniswap.org/sdk/v4/overview#universal-router-requirement-for-swapping)
    * [StateView Contract Introduction](https://docs.uniswap.org/sdk/v4/overview#stateview-contract-introduction)
    * [Position Fetching Changed](https://docs.uniswap.org/sdk/v4/overview#position-fetching-changed)
    * [Fee Collection Behavior Changed](https://docs.uniswap.org/sdk/v4/overview#fee-collection-behavior-changed)
    * [Quick Comparison](https://docs.uniswap.org/sdk/v4/overview#quick-comparison)
    * [What This Means for Developers](https://docs.uniswap.org/sdk/v4/overview#what-this-means-for-developers)
    * [Migration Requirements](https://docs.uniswap.org/sdk/v4/overview#migration-requirements)
    * [Development Impact](https://docs.uniswap.org/sdk/v4/overview#development-impact)
  * [Learning Path](https://docs.uniswap.org/sdk/v4/overview#learning-path)
    * [1. Swapping](https://docs.uniswap.org/sdk/v4/overview#1-swapping)
    * [2. Position Management (Off-chain Indexing + Fee Collection)](https://docs.uniswap.org/sdk/v4/overview#2-position-management-off-chain-indexing--fee-collection)
    * [3. Advanced Features (StateView + Pool Creation)](https://docs.uniswap.org/sdk/v4/overview#3-advanced-features-stateview--pool-creation)
  * [Developer Links](https://docs.uniswap.org/sdk/v4/overview#developer-links)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/overview.md)
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
