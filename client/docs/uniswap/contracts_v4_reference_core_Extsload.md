# https://docs.uniswap.org/contracts/v4/reference/core/Extsload

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)


On this page
# Extsload
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/Extsload.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [IExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload)
Enables public storage access for efficient state retrieval by external contracts. <https://eips.ethereum.org/EIPS/eip-2330#rationale>
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#functions "Direct link to Functions")
### extsload[​](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#extsload-1 "Direct link to extsload")
Called by external contracts to access granular pool state
```
functionextsload(bytes32 slot)externalviewreturns(bytes32);
```

**Parameters**
Name| Type| Description  
---|---|---  
`slot`| `bytes32`| Key of slot to sload  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes32`| value The value of the slot as bytes32  
### extsload[​](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#extsload-2 "Direct link to extsload")
Called by external contracts to access granular pool state
```
functionextsload(bytes32 startSlot,uint256 nSlots)externalviewreturns(bytes32[]memory);
```

**Parameters**
Name| Type| Description  
---|---|---  
`startSlot`| `bytes32`|   
`nSlots`| `uint256`|   
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes32[]`| value The value of the slot as bytes32  
### extsload[​](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#extsload-3 "Direct link to extsload")
Called by external contracts to access granular pool state
```
functionextsload(bytes32[]calldata slots)externalviewreturns(bytes32[]memory);
```

**Parameters**
Name| Type| Description  
---|---|---  
`slots`| `bytes32[]`|   
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bytes32[]`| value The value of the slot as bytes32  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/Extsload.md)
Was this helpful?
[PreviousERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)[NextExttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#functions)
    * [extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#extsload-1)
    * [extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#extsload-2)
    * [extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload#extsload-3)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/Extsload.md)
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
