# https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)


On this page
# NoDelegateCall
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/NoDelegateCall.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
Base contract that provides a modifier for preventing delegatecall to methods in a child contract
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#state-variables "Direct link to State Variables")
### original[​](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#original "Direct link to original")
_The original address of this contract_
```
addressprivate immutable original;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#constructor "Direct link to constructor")
```
constructor();
```

### checkNotDelegateCall[​](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#checknotdelegatecall "Direct link to checkNotDelegateCall")
_Private method is used instead of inlining into modifier because modifiers are copied into each method, and the use of immutable means the address bytes are copied in every place the modifier is used._
```
functioncheckNotDelegateCall()privateview;
```

### noDelegateCall[​](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#nodelegatecall-1 "Direct link to noDelegateCall")
Prevents delegatecall into the modified method
```
modifiernoDelegateCall();
```

## Errors[​](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#errors "Direct link to Errors")
### DelegateCallNotAllowed[​](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#delegatecallnotallowed "Direct link to DelegateCallNotAllowed")
```
error DelegateCallNotAllowed();
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/NoDelegateCall.md)
Was this helpful?
[PreviousIPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)[NextPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#state-variables)
    * [original](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#original)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#constructor)
    * [checkNotDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#checknotdelegatecall)
    * [noDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#nodelegatecall-1)
  * [Errors](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#errors)
    * [DelegateCallNotAllowed](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall#delegatecallnotallowed)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/NoDelegateCall.md)
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
