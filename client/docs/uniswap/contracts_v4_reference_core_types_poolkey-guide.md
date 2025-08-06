# https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
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
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * Types
  * [PoolKey Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)


`PoolKey` is a crucial struct in Uniswap V4 that uniquely identifies a liquidity pool. It encapsulates all the essential parameters that define a pool's characteristics.
# Structure
```
structPoolKey{  Currency currency0;  Currency currency1;uint24 fee;int24 tickSpacing;  IHooks hooks;}
```

# Fields
Field Name| Type| Description  
---|---|---  
currency0| Currency| The lower currency of the pool, sorted numerically  
currency1| Currency| The higher currency of the pool, sorted numerically  
fee| uint24| The pool swap fee, capped at 1,000,000. If the first bit is 1, the pool has a dynamic fee  
tickSpacing| int24| The spacing between ticks for the pool  
hooks| IHooks| The address of the hooks contract associated with the pool  
# Important Notes
  * The `currency0` and `currency1` fields are always sorted numerically, with `currency0` being the lower value. This ensures consistent pool identification regardless of the order in which tokens are provided.
  * The `fee` field can represent either a static fee or indicate that the pool uses a dynamic fee mechanism.
  * The `tickSpacing` field determines the granularity of price ranges that can be used for liquidity provision.
  * The `hooks` field is an interface of our Hooks that the PoolManager uses to call these functions


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/poolkey-guide.mdx)
Was this helpful?
[PreviousCurrency Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)[NextERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/poolkey-guide.mdx)
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
