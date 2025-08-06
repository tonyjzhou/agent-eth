# https://docs.uniswap.org/contracts/v4/reference/core/ERC6909

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)


On this page
# ERC6909
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/ERC6909.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [IERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims)
**Author:** Solmate (<https://github.com/transmissions11/solmate/blob/main/src/tokens/ERC6909.sol>)
Minimalist and gas efficient standard ERC6909 implementation.
_Copied from the commit at 4b47a19038b798b4a33d9749d25e570443520647_
_This contract has been modified from the implementation at the above link._
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#state-variables "Direct link to State Variables")
### isOperator[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#isoperator "Direct link to isOperator")
```
mapping(address owner =>mapping(address operator =>bool isOperator))public isOperator;
```

### balanceOf[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#balanceof "Direct link to balanceOf")
```
mapping(address owner =>mapping(uint256 id =>uint256 balance))public balanceOf;
```

### allowance[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#allowance "Direct link to allowance")
```
mapping(address owner =>mapping(address spender =>mapping(uint256 id =>uint256 amount)))public allowance;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#functions "Direct link to Functions")
### transfer[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#transfer "Direct link to transfer")
```
functiontransfer(address receiver,uint256 id,uint256 amount)public virtual returns(bool);
```

### transferFrom[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#transferfrom "Direct link to transferFrom")
```
functiontransferFrom(address sender,address receiver,uint256 id,uint256 amount)public virtual returns(bool);
```

### approve[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#approve "Direct link to approve")
```
functionapprove(address spender,uint256 id,uint256 amount)public virtual returns(bool);
```

### setOperator[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#setoperator "Direct link to setOperator")
```
functionsetOperator(address operator,bool approved)public virtual returns(bool);
```

### supportsInterface[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#supportsinterface "Direct link to supportsInterface")
```
functionsupportsInterface(bytes4 interfaceId)publicview virtual returns(bool);
```

### _mint[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#_mint "Direct link to _mint")
```
function_mint(address receiver,uint256 id,uint256 amount)internal virtual;
```

### _burn[​](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#_burn "Direct link to _burn")
```
function_burn(address sender,uint256 id,uint256 amount)internal virtual;
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/ERC6909.md)
Was this helpful?
[PreviousPoolKey Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)[NextERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#state-variables)
    * [isOperator](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#isoperator)
    * [balanceOf](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#balanceof)
    * [allowance](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#allowance)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#functions)
    * [transfer](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#transfer)
    * [transferFrom](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#transferfrom)
    * [approve](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#approve)
    * [setOperator](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#setoperator)
    * [supportsInterface](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#supportsinterface)
    * [_mint](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#_mint)
    * [_burn](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909#_burn)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/ERC6909.md)
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
