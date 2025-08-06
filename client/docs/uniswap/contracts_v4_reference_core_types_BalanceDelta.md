# https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#__docusaurus_skipToContent_fallback)
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
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta)
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
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
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
  * Technical Reference
  * Core
  * Types
  * [BalanceDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta)


On this page
# BalanceDelta
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/BalanceDelta.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
_Two`int128` values packed into a single `int256` where the upper 128 bits represent the amount0 and the lower 128 bits represent the amount1._
```
type BalanceDelta isint256;
```

# BalanceDeltaLibrary
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/BalanceDelta.sol)
Library for getting the amount0 and amount1 deltas from the BalanceDelta type
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#state-variables "Direct link to State Variables")
### ZERO_DELTA[​](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#zero_delta "Direct link to ZERO_DELTA")
A BalanceDelta of 0
```
BalanceDelta publicconstant ZERO_DELTA = BalanceDelta.wrap(0);
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#functions "Direct link to Functions")
### amount0[​](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#amount0 "Direct link to amount0")
```
functionamount0(BalanceDelta balanceDelta)internalpurereturns(int128 _amount0);
```

### amount1[​](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#amount1 "Direct link to amount1")
```
functionamount1(BalanceDelta balanceDelta)internalpurereturns(int128 _amount1);
```

# sub
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/BalanceDelta.sol)
```
functionsub(BalanceDelta a, BalanceDelta b)purereturns(BalanceDelta);
```

# toBalanceDelta
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/BalanceDelta.sol)
```
functiontoBalanceDelta(int128 _amount0,int128 _amount1)purereturns(BalanceDelta balanceDelta);
```

# eq
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/BalanceDelta.sol)
```
functioneq(BalanceDelta a, BalanceDelta b)purereturns(bool);
```

# add
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/BalanceDelta.sol)
```
functionadd(BalanceDelta a, BalanceDelta b)purereturns(BalanceDelta);
```

# neq
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/BalanceDelta.sol)
```
functionneq(BalanceDelta a, BalanceDelta b)purereturns(bool);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/BalanceDelta.md)
Was this helpful?
[PreviousTransientStateLibrary](https://docs.uniswap.org/contracts/v4/reference/core/libraries/transient-state-library)[NextBeforeSwapDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BeforeSwapDelta)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#state-variables)
    * [ZERO_DELTA](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#zero_delta)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#functions)
    * [amount0](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#amount0)
    * [amount1](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta#amount1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/BalanceDelta.md)
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
