# https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
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
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * Types
  * [PoolKey](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)


# PoolKey
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/PoolKey.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
Returns the key for identifying a pool
```
structPoolKey{  Currency currency0;  Currency currency1;uint24 fee;int24 tickSpacing;  IHooks hooks;}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/PoolKey.md)
Was this helpful?
[PreviousPoolId](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolId)[NextSlot0](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/PoolKey.md)
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
