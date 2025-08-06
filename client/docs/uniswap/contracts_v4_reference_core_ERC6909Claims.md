# https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)


On this page
# ERC6909Claims
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/ERC6909Claims.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
ERC6909Claims inherits ERC6909 and implements an internal burnFrom function
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims#functions "Direct link to Functions")
### _burnFrom[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims#_burnfrom "Direct link to _burnFrom")
Burn `amount` tokens of token type `id` from `from`.
_if sender is not`from` they must be an operator or have sufficient allowance._
```
function_burnFrom(addressfrom,uint256 id,uint256 amount)internal;
```

**Parameters**
Name| Type| Description  
---|---|---  
`from`| `address`| The address to burn tokens from.  
`id`| `uint256`| The currency to burn.  
`amount`| `uint256`| The amount to burn.  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/ERC6909Claims.md)
Was this helpful?
[PreviousERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)[NextExtsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims#functions)
    * [_burnFrom](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims#_burnfrom)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/ERC6909Claims.md)
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
