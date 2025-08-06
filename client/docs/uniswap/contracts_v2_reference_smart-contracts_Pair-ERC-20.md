# https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20

[Skip to main content](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#__docusaurus_skipToContent_fallback)
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
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/Governance/governance-reference)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
          * [Factory](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
          * [Pair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
          * [Pair (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20)
          * [Library](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library)
          * [Router01](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-01)
          * [Router02](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
          * [Common Errors](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/common-errors)
          * [V2 Deployment Addresses](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/v2-deployments)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Technical Reference
  * Smart Contracts
  * [Pair (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20)


On this page
This documentation covers ERC-20 functionality for denominating pool tokens. For Uniswap-specific functionality, see [Pair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair).
# Code
[`UniswapV2ERC20.sol`](https://github.com/Uniswap/uniswap-v2-core/blob/master/contracts/UniswapV2ERC20.sol)
# Events
## Approval[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#approval "Direct link to Approval")
```
eventApproval(addressindexed owner,addressindexed spender,uint value);
```

Emitted each time an approval occurs via [approve](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#approve) or [permit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#permit).
## Transfer[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transfer "Direct link to Transfer")
```
eventTransfer(addressindexedfrom,addressindexed to,uint value);
```

Emitted each time a transfer occurs via [transfer](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transfer-1), [transferFrom](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transferfrom), [mint](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint-1), or [burn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#burn-1).
# Read-Only Functions
## name[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#name "Direct link to name")
```
functionname()externalpurereturns(stringmemory);
```

Returns `Uniswap V2` for all pairs.
## symbol[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#symbol "Direct link to symbol")
```
functionsymbol()externalpurereturns(stringmemory);
```

Returns `UNI-V2` for all pairs.
## decimals[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#decimals "Direct link to decimals")
```
functiondecimals()externalpurereturns(uint8);
```

Returns `18` for all pairs.
## totalSupply[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#totalsupply "Direct link to totalSupply")
```
functiontotalSupply()externalviewreturns(uint);
```

Returns the total amount of pool tokens for a pair.
## balanceOf[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#balanceof "Direct link to balanceOf")
```
functionbalanceOf(address owner)externalviewreturns(uint);
```

Returns the amount of pool tokens owned by an address.
## allowance[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#allowance "Direct link to allowance")
```
functionallowance(address owner,address spender)externalviewreturns(uint);
```

Returns the amount of liquidity tokens owned by an address that a spender is allowed to transfer via [transferFrom](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transferfrom).
## DOMAIN_SEPARATOR[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#domain_separator "Direct link to DOMAIN_SEPARATOR")
```
functionDOMAIN_SEPARATOR()externalviewreturns(bytes32);
```

Returns a domain separator for use in [permit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#permit).
## PERMIT_TYPEHASH[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#permit_typehash "Direct link to PERMIT_TYPEHASH")
```
functionPERMIT_TYPEHASH()externalviewreturns(bytes32);
```

Returns a typehash for use in [permit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#permit).
## nonces[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#nonces "Direct link to nonces")
```
functionnonces(address owner)externalviewreturns(uint);
```

Returns the current nonce for an address for use in [permit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#permit).
# State-Changing Functions
## approve[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#approve "Direct link to approve")
```
functionapprove(address spender,uint value)externalreturns(bool);
```

Lets `msg.sender` set their allowance for a spender.
  * Emits [Approval](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#approval).


## transfer[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transfer-1 "Direct link to transfer")
```
functiontransfer(address to,uint value)externalreturns(bool);
```

Lets `msg.sender` send pool tokens to an address.
  * Emits [Transfer](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transfer).


## transferFrom[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transferfrom "Direct link to transferFrom")
```
functiontransferFrom(addressfrom,address to,uint value)externalreturns(bool);
```

Sends pool tokens from one address to another.
  * Requires approval.
  * Emits [Transfer](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transfer).


## permit[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#permit "Direct link to permit")
```
functionpermit(address owner,address spender,uint value,uint deadline,uint8 v,bytes32 r,bytes32 s)external;
```

Sets the allowance for a spender where approval is granted via a signature.
  * See [Using Permit](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions).
  * Emits [Approval](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#approval).


# Interface
```
import'@uniswap/v2-core/contracts/interfaces/IUniswapV2ERC20.sol';
```

```
pragmasolidity>=0.5.0;interfaceIUniswapV2ERC20{eventApproval(addressindexed owner,addressindexed spender,uint value);eventTransfer(addressindexedfrom,addressindexed to,uint value);functionname()externalpurereturns(stringmemory);functionsymbol()externalpurereturns(stringmemory);functiondecimals()externalpurereturns(uint8);functiontotalSupply()externalviewreturns(uint);functionbalanceOf(address owner)externalviewreturns(uint);functionallowance(address owner,address spender)externalviewreturns(uint);functionapprove(address spender,uint value)externalreturns(bool);functiontransfer(address to,uint value)externalreturns(bool);functiontransferFrom(addressfrom,address to,uint value)externalreturns(bool);functionDOMAIN_SEPARATOR()externalviewreturns(bytes32);functionPERMIT_TYPEHASH()externalpurereturns(bytes32);functionnonces(address owner)externalviewreturns(uint);functionpermit(address owner,address spender,uint value,uint deadline,uint8 v,bytes32 r,bytes32 s)external;}
```

# ABI
```
import IUniswapV2ERC20 from'@uniswap/v2-core/build/IUniswapV2ERC20.json'
```

<https://unpkg.com/@uniswap/v2-core@1.0.0/build/IUniswapV2ERC20.json>
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/03-pair-erc-20.md)
Was this helpful?
[PreviousPair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)[NextLibrary](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library)
On this page
  * [Approval](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#approval)
  * [Transfer](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transfer)
  * [name](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#name)
  * [symbol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#symbol)
  * [decimals](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#decimals)
  * [totalSupply](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#totalsupply)
  * [balanceOf](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#balanceof)
  * [allowance](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#allowance)
  * [DOMAIN_SEPARATOR](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#domain_separator)
  * [PERMIT_TYPEHASH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#permit_typehash)
  * [nonces](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#nonces)
  * [approve](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#approve)
  * [transfer](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transfer-1)
  * [transferFrom](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#transferfrom)
  * [permit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20#permit)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/03-pair-erc-20.md)
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
