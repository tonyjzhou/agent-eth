# https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
            * [BalanceDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta)
            * [BeforeSwapDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BeforeSwapDelta)
            * [Currency](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency)
            * [PoolId](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolId)
            * [PoolKey](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
            * [Slot0](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0)
            * [BalanceDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
            * [BeforeSwapDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/beforeswapdelta-guide)
            * [Currency Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
            * [PoolKey Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * Types
  * [BalanceDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)


On this page
`BalanceDelta` is a type used in Uniswap V4 to represent the balance changes of two tokens (token0 and token1). It tightly packs the two values in a single 256 bits. It is designed to efficiently store and manipulate these balance deltas, with the upper 128 bits representing the change in token0 (`amount0`) and the lower 128 bits representing the change in token1 (`amount1`).
# Purpose
The main purpose of `BalanceDelta` is to keep track of the net balance changes in the two tokens of a pool after various operations such as swaps, liquidity modifications, and interactions with hooks. It provides a compact and efficient way to store and update these balance deltas throughout the execution flow of the pool.
In the context of hooks, `BalanceDelta` is used to ensure that the net balance change for each token is zero after the hook's functionality is executed. This is important for maintaining the integrity of the pool's balances and ensuring that the hooks do not introduce any unexpected or unauthorized balance changes.
# Type Definition
```
type BalanceDelta isint256;
```

# Using Directives
```
using{add as+, sub as-, eq as==, neq as!=}for BalanceDelta global;usingBalanceDeltaLibraryfor BalanceDelta global;usingSafeCastforint256;
```

These using directives enable arithmetic operations, equality comparisons, and library functions to be used directly on `BalanceDelta` values.
# Functions
## toBalanceDelta[​](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#tobalancedelta "Direct link to toBalanceDelta")
```
functiontoBalanceDelta(int128 amount0,int128 amount1)purereturns(BalanceDelta balanceDelta);
```

Creates a `BalanceDelta` value from two `int128` values representing `amount0` and `amount1`.
Param Name| Type| Description  
---|---|---  
amount0| int128| The amount for the first token  
amount1| int128| The amount for the second token  
Returns the created `BalanceDelta` value.
## add[​](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#add "Direct link to add")
```
functionadd(BalanceDelta a, BalanceDelta b)purereturns(BalanceDelta);
```

Adds two `BalanceDelta` values.
Param Name| Type| Description  
---|---|---  
a| BalanceDelta| The first `BalanceDelta` value  
b| BalanceDelta| The second `BalanceDelta` value  
Returns the sum of the two `BalanceDelta` values.
## sub[​](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#sub "Direct link to sub")
```
functionsub(BalanceDelta a, BalanceDelta b)purereturns(BalanceDelta);
```

Subtracts one `BalanceDelta` value from another.
Param Name| Type| Description  
---|---|---  
a| BalanceDelta| The first `BalanceDelta` value  
b| BalanceDelta| The second `BalanceDelta` value  
Returns the difference of the two `BalanceDelta` values.
## eq[​](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#eq "Direct link to eq")
```
functioneq(BalanceDelta a, BalanceDelta b)purereturns(bool);
```

Checks if two `BalanceDelta` values are equal.
Param Name| Type| Description  
---|---|---  
a| BalanceDelta| The first `BalanceDelta` value  
b| BalanceDelta| The second `BalanceDelta` value  
Returns `true` if the values are equal, `false` otherwise.
## neq[​](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#neq "Direct link to neq")
```
functionneq(BalanceDelta a, BalanceDelta b)purereturns(bool);
```

Checks if two `BalanceDelta` values are not equal.
Param Name| Type| Description  
---|---|---  
a| BalanceDelta| The first `BalanceDelta` value  
b| BalanceDelta| The second `BalanceDelta` value  
Returns `true` if the values are not equal, `false` otherwise.
# Library Functions
## amount0[​](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#amount0 "Direct link to amount0")
```
functionamount0(BalanceDelta balanceDelta)internalpurereturns(int128 _amount0);
```

Extracts the `amount0` value from a `BalanceDelta`.
Param Name| Type| Description  
---|---|---  
balanceDelta| BalanceDelta| The `BalanceDelta` value  
Returns the extracted `amount0` value as an `int128`.
## amount1[​](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#amount1 "Direct link to amount1")
```
functionamount1(BalanceDelta balanceDelta)internalpurereturns(int128 _amount1);
```

Extracts the `amount1` value from a `BalanceDelta`.
Param Name| Type| Description  
---|---|---  
balanceDelta| BalanceDelta| The `BalanceDelta` value  
Returns the extracted `amount1` value as an `int128`.
# Usage in Hooks
When a hook is called during a swap or liquidity modification, it can perform custom logic and interact with the pool's balances. However, to maintain the correctness of the pool's state, the hook must ensure that any balance changes it introduces are properly accounted for and net to zero at the end of its execution. The BalanceDelta is forwarded to the `afterSwap` & `afterAddliquidity`, `afterRemoveLiquidity` hooks.
# Usage in the Pool Library
In the `Pool` library, `BalanceDelta` is used extensively to track balance changes during various operations such as swaps, liquidity modifications, and donations. The library functions `swap`, `modifyLiquidity`, and `donate` all return `BalanceDelta` values representing the net balance changes resulting from these operations.
The `Pool` library uses `BalanceDelta` to efficiently update and manage the pool's balances, ensuring that the net balance changes are accurately accounted for and that the pool remains in a consistent state.
By leveraging the compact representation and efficient arithmetic operations provided by `BalanceDelta`, the `Pool` library can perform complex balance calculations and updates in a gas-optimized manner, reducing the overall cost of executing pool-related operations.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/balancedelta-guide.mdx)
Was this helpful?
[PreviousSlot0](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0)[NextBeforeSwapDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/beforeswapdelta-guide)
On this page
  * [toBalanceDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#tobalancedelta)
  * [add](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#add)
  * [sub](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#sub)
  * [eq](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#eq)
  * [neq](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#neq)
  * [amount0](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#amount0)
  * [amount1](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide#amount1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/balancedelta-guide.mdx)
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
