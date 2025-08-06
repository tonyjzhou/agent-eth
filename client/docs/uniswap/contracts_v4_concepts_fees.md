# https://docs.uniswap.org/contracts/v4/concepts/fees

[Skip to main content](https://docs.uniswap.org/contracts/v4/concepts/fees#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/concepts/fees)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/fees)
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
      * [Quickstart](https://docs.uniswap.org/contracts/v4/concepts/fees)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/concepts/fees)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/fees)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/concepts/fees)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/concepts/fees)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/concepts/fees)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/concepts/fees)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/concepts/fees)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/concepts/fees)
    * [Permit2](https://docs.uniswap.org/contracts/v4/concepts/fees)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/concepts/fees)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/concepts/fees)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Concepts
  * [v4 Fee Structure Guide](https://docs.uniswap.org/contracts/v4/concepts/fees)


On this page
# v4 Fee Structure Guide
## Overview of Fee Types[​](https://docs.uniswap.org/contracts/v4/concepts/fees#overview-of-fee-types "Direct link to Overview of Fee Types")
In Uniswap v4, there are three main types of fees to understand:
  * **LP Fee** : Fees earned by liquidity providers
  * **Protocol Fee** : Fees collected by the protocol
  * **Swap Fee** : Total fee paid by swappers (calculated by applying protocol fee and LP fee sequentially)


## LP Fees[​](https://docs.uniswap.org/contracts/v4/concepts/fees#lp-fees "Direct link to LP Fees")
LP fees are set by the pool initializer at pool creation and may be static or dynamic.
**Fee Range:**
  * Maximum LP Fee: 100%
  * Minimum LP Fee: 0%
  * **Granularity** : Fees are set at pip-level precision


### Static LP Fees[​](https://docs.uniswap.org/contracts/v4/concepts/fees#static-lp-fees "Direct link to Static LP Fees")
  * **Immutable** once set during pool initialization
  * **Unlimited fee options** in v4 (major improvement from v3)
  * In **v3** , LP fee options were limited to: 0.01%, 0.05%, 0.30%, and 1.00%


### Dynamic LP Fees[​](https://docs.uniswap.org/contracts/v4/concepts/fees#dynamic-lp-fees "Direct link to Dynamic LP Fees")
Dynamic fees offer more flexibility and real-time adjustability:
  * A dynamic fee pool signals this capability by setting its LP fee to `0x800000` (where the first bit = 1)
  * **Only the pool's hook** can update the dynamic fee—no additional permissions required
  * A hook **cannot** update fees if the pool's fee is not set to `0x800000`


## Protocol Fees[​](https://docs.uniswap.org/contracts/v4/concepts/fees#protocol-fees "Direct link to Protocol Fees")
Protocol fees are configured **per pool** with the following characteristics:
  * Controlled by the **protocol fee controller** (set by the pool manager owner)
  * **Maximum protocol fee** : 0.1% (1,000 pips)
  * **Granularity** : Fees are set at pip-level precision (not basis points)
  * **Unit conversion** : 1 basis point = 100 pips
  * **Directional fees** : Separate fees can be set for: 
    * token0 → token1 swaps
    * token1 → token0 swaps


## Swap Fees[​](https://docs.uniswap.org/contracts/v4/concepts/fees#swap-fees "Direct link to Swap Fees")
### Key Change from v3 to v4[​](https://docs.uniswap.org/contracts/v4/concepts/fees#key-change-from-v3-to-v4 "Direct link to Key Change from v3 to v4")
**v3 behavior** : Swap fee = LP fee (protocol fee was a percentage taken from LP fees)
**v4 behavior** : Swap fee = effective total fee after applying both protocol and LP fees sequentially
### Application Order[​](https://docs.uniswap.org/contracts/v4/concepts/fees#application-order "Direct link to Application Order")
  1. **Protocol fee** applied first to the input amount
  2. **LP fee** applied second to the remaining input (after protocol fee deduction)


**Impact on LP Earnings:**
Note that this sequential application means introducing or increasing protocol fees will reduce LP earnings even if swap volume remains constant, since LPs now earn fees on a smaller base amount.
### Fee Cap[​](https://docs.uniswap.org/contracts/v4/concepts/fees#fee-cap "Direct link to Fee Cap")
  * **Total swap fee capped at 100%** of input amount
  * **Important** : If swap fee = 100%, exact output swaps become impossible (entire input consumed by fees)


### Fee Calculation Formula[​](https://docs.uniswap.org/contracts/v4/concepts/fees#fee-calculation-formula "Direct link to Fee Calculation Formula")
```
// Method 1: Sequential applicationuint256 swapFee = protocolFee +(lpFee *(1_000_000 - protocolFee))/1_000_000;(rounded up)// Method 2: Mathematically equivalentuint256 swapFee = protocolFee + lpFee -(protocolFee * lpFee)/1_000_000;
```

### Mathematical Derivation[​](https://docs.uniswap.org/contracts/v4/concepts/fees#mathematical-derivation "Direct link to Mathematical Derivation")
Starting with input amount:
```
amountIn
```

**Step 1** : Protocol fee takes:
```
amountIn × (protocolFee / 1_000_000)
```

**Step 2** : Remaining after protocol fee:
```
amountIn × (1 - protocolFee / 1_000_000)
```

**Step 3** : LP fee applies to remaining:
```
lpFee × (remaining amount)
```

**Final formula** :
```
swapFee = protocolFee + (lpFee × (1 - protocolFee / 1_000_000))
```

Which simplifies to:
```
swapFee = protocolFee + lpFee - (protocolFee × lpFee) / 1_000_000
```

## Example Calculation[​](https://docs.uniswap.org/contracts/v4/concepts/fees#example-calculation "Direct link to Example Calculation")
**Given:**
  * `protocolFee = 50` pips → 0.005%
  * `lpFee = 3000` pips → 0.30%


**Calculation:**
```
swapFee =50+3000-(50 × 3000)/1_000_000=50+3000-150/1_000_000=50+3000-0.15=3049.85 pips
```

**Result:** 3049.85 pips = **0.304985%** total swap fee
## Key Takeaways[​](https://docs.uniswap.org/contracts/v4/concepts/fees#key-takeaways "Direct link to Key Takeaways")
  * **Sequential application** : Protocol fees are deducted first, then LP fees apply to the remainder
  * **Dynamic flexibility** : v4 introduces unlimited static fee tiers and dynamic fee capabilities
  * **Directional control** : Protocol fees can differ by swap direction
  * **Fee interaction** : The combined effect is slightly less than simple addition due to sequential application


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/09-fees.mdx)
Was this helpful?
[PreviousIntegrated Routing with UniswapX](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)[NextSecurity](https://docs.uniswap.org/contracts/v4/concepts/security)
On this page
  * [Overview of Fee Types](https://docs.uniswap.org/contracts/v4/concepts/fees#overview-of-fee-types)
  * [LP Fees](https://docs.uniswap.org/contracts/v4/concepts/fees#lp-fees)
    * [Static LP Fees](https://docs.uniswap.org/contracts/v4/concepts/fees#static-lp-fees)
    * [Dynamic LP Fees](https://docs.uniswap.org/contracts/v4/concepts/fees#dynamic-lp-fees)
  * [Protocol Fees](https://docs.uniswap.org/contracts/v4/concepts/fees#protocol-fees)
  * [Swap Fees](https://docs.uniswap.org/contracts/v4/concepts/fees#swap-fees)
    * [Key Change from v3 to v4](https://docs.uniswap.org/contracts/v4/concepts/fees#key-change-from-v3-to-v4)
    * [Application Order](https://docs.uniswap.org/contracts/v4/concepts/fees#application-order)
    * [Fee Cap](https://docs.uniswap.org/contracts/v4/concepts/fees#fee-cap)
    * [Fee Calculation Formula](https://docs.uniswap.org/contracts/v4/concepts/fees#fee-calculation-formula)
    * [Mathematical Derivation](https://docs.uniswap.org/contracts/v4/concepts/fees#mathematical-derivation)
  * [Example Calculation](https://docs.uniswap.org/contracts/v4/concepts/fees#example-calculation)
  * [Key Takeaways](https://docs.uniswap.org/contracts/v4/concepts/fees#key-takeaways)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/09-fees.mdx)
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
