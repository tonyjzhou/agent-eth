# https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload#__docusaurus_skipToContent_fallback)
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
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
            * [IERC20Minimal](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
            * [IERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims)
            * [IExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload)
            * [IExttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload)
            * [IHooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
            * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
            * [IProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
            * [IUnlockCallback](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IUnlockCallback)
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
  * interfaces
  * [IExttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload)


On this page
# IExttload
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/interfaces/IExttload.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
Interface for functions to access any transient storage slot in a contract
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload#functions "Direct link to Functions")
### exttload[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload#exttload "Direct link to exttload")
Called by external contracts to access transient storage of the contract
```
functionexttload(bytes32 slot)externalviewreturns(bytes32 value);
```

**Parameters**
Name| Type| Description  
---|---|---  
`slot`| `bytes32`| Key of slot to tload  
**Returns**
Name| Type| Description  
---|---|---  
`value`| `bytes32`| The value of the slot as bytes32  
### exttload[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload#exttload-1 "Direct link to exttload")
Called by external contracts to access sparse transient pool state
```
functionexttload(bytes32[]calldata slots)externalviewreturns(bytes32[]memory values);
```

**Parameters**
Name| Type| Description  
---|---|---  
`slots`| `bytes32[]`| List of slots to tload  
**Returns**
Name| Type| Description  
---|---|---  
`values`| `bytes32[]`| List of loaded values  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IExttload.md)
Was this helpful?
[PreviousIExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload)[NextIHooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload#functions)
    * [exttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload#exttload)
    * [exttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload#exttload-1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IExttload.md)
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
