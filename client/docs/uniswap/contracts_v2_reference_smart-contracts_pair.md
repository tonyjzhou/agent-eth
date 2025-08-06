# https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair

[Skip to main content](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
    * [Permit2](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [API](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
          * [Factory](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
          * [Pair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
          * [Pair (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20)
          * [Library](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library)
          * [Router01](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-01)
          * [Router02](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
          * [Common Errors](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/common-errors)
          * [V2 Deployment Addresses](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/v2-deployments)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Technical Reference
  * Smart Contracts
  * [Pair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)


On this page
This documentation covers Uniswap-specific functionality. For ERC-20 functionality, see [Pair (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair-erc-20).
# Code
[`UniswapV2Pair.sol`](https://github.com/Uniswap/uniswap-v2-core/blob/master/contracts/UniswapV2Pair.sol)
# Address
See [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses).
# Events
## Mint[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint "Direct link to Mint")
```
eventMint(addressindexed sender,uint amount0,uint amount1);
```

Emitted each time liquidity tokens are created via [mint](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint-1).
## Burn[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#burn "Direct link to Burn")
```
eventBurn(addressindexed sender,uint amount0,uint amount1,addressindexed to);
```

Emitted each time liquidity tokens are destroyed via [burn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#burn-1).
## Swap[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap "Direct link to Swap")
```
eventSwap(addressindexed sender,uint amount0In,uint amount1In,uint amount0Out,uint amount1Out,addressindexed to);
```

Emitted each time a swap occurs via [swap](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap-1).
## Sync[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync "Direct link to Sync")
```
eventSync(uint112 reserve0,uint112 reserve1);
```

Emitted each time reserves are updated via [mint](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint-1), [burn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#burn-1), [swap](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap-1), or [sync](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync-1).
# Read-Only Functions
## MINIMUM_LIQUIDITY[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#minimum_liquidity "Direct link to MINIMUM_LIQUIDITY")
```
functionMINIMUM_LIQUIDITY()externalpurereturns(uint);
```

Returns `1000` for all pairs. See [Minimum Liquidity](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts#minimum-liquidity).
## factory[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#factory "Direct link to factory")
```
functionfactory()externalviewreturns(address);
```

Returns the [factory address](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#address).
## token0[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#token0 "Direct link to token0")
```
functiontoken0()externalviewreturns(address);
```

Returns the address of the pair token with the lower sort order.
## token1[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#token1 "Direct link to token1")
```
functiontoken1()externalviewreturns(address);
```

Returns the address of the pair token with the higher sort order.
## getReserves[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#getreserves "Direct link to getReserves")
```
functiongetReserves()externalviewreturns(uint112 reserve0,uint112 reserve1,uint32 blockTimestampLast);
```

Returns the reserves of token0 and token1 used to price trades and distribute liquidity. See [Pricing](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing). Also returns the `block.timestamp` (mod `2**32`) of the last block during which an interaction occured for the pair.
## price0CumulativeLast[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#price0cumulativelast "Direct link to price0CumulativeLast")
```
functionprice0CumulativeLast()externalviewreturns(uint);
```

See [Oracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles).
## price1CumulativeLast[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#price1cumulativelast "Direct link to price1CumulativeLast")
```
functionprice1CumulativeLast()externalviewreturns(uint);
```

See [Oracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles).
## kLast[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#klast "Direct link to kLast")
```
functionkLast()externalviewreturns(uint);
```

Returns the product of the reserves as of the most recent liquidity event. See [Protocol Charge Calculation](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees#protocol-charge-calculation).
# State-Changing Functions
## mint[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint-1 "Direct link to mint")
```
functionmint(address to)externalreturns(uint liquidity);
```

Creates pool tokens.
  * Emits [Mint](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint), [Sync](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync), [Transfer](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair-erc-20#transfer).


## burn[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#burn-1 "Direct link to burn")
```
functionburn(address to)externalreturns(uint amount0,uint amount1);
```

Destroys pool tokens.
  * Emits [Burn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#burn), [Sync](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync), [Transfer](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair-erc-20#transfer).


## swap[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap-1 "Direct link to swap")
```
functionswap(uint amount0Out,uint amount1Out,address to,bytescalldata data)external;
```

Swaps tokens. For regular swaps, `data.length` must be `0`. Also see [Flash Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps).
  * Emits [Swap](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap), [Sync](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync).


## skim[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#skim "Direct link to skim")
```
functionskim(address to)external;
```

See the [whitepaper](https://docs.uniswap.org/whitepaper.pdf).
## sync[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync-1 "Direct link to sync")
```
functionsync()external;
```

See the [whitepaper](https://docs.uniswap.org/whitepaper.pdf).
  * Emits [Sync](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync).


# Interface
```
import'@uniswap/v2-core/contracts/interfaces/IUniswapV2Pair.sol';
```

```
pragmasolidity>=0.5.0;interfaceIUniswapV2Pair{eventApproval(addressindexed owner,addressindexed spender,uint value);eventTransfer(addressindexedfrom,addressindexed to,uint value);functionname()externalpurereturns(stringmemory);functionsymbol()externalpurereturns(stringmemory);functiondecimals()externalpurereturns(uint8);functiontotalSupply()externalviewreturns(uint);functionbalanceOf(address owner)externalviewreturns(uint);functionallowance(address owner,address spender)externalviewreturns(uint);functionapprove(address spender,uint value)externalreturns(bool);functiontransfer(address to,uint value)externalreturns(bool);functiontransferFrom(addressfrom,address to,uint value)externalreturns(bool);functionDOMAIN_SEPARATOR()externalviewreturns(bytes32);functionPERMIT_TYPEHASH()externalpurereturns(bytes32);functionnonces(address owner)externalviewreturns(uint);functionpermit(address owner,address spender,uint value,uint deadline,uint8 v,bytes32 r,bytes32 s)external;eventMint(addressindexed sender,uint amount0,uint amount1);eventBurn(addressindexed sender,uint amount0,uint amount1,addressindexed to);eventSwap(addressindexed sender,uint amount0In,uint amount1In,uint amount0Out,uint amount1Out,addressindexed to);eventSync(uint112 reserve0,uint112 reserve1);functionMINIMUM_LIQUIDITY()externalpurereturns(uint);functionfactory()externalviewreturns(address);functiontoken0()externalviewreturns(address);functiontoken1()externalviewreturns(address);functiongetReserves()externalviewreturns(uint112 reserve0,uint112 reserve1,uint32 blockTimestampLast);functionprice0CumulativeLast()externalviewreturns(uint);functionprice1CumulativeLast()externalviewreturns(uint);functionkLast()externalviewreturns(uint);functionmint(address to)externalreturns(uint liquidity);functionburn(address to)externalreturns(uint amount0,uint amount1);functionswap(uint amount0Out,uint amount1Out,address to,bytescalldata data)external;functionskim(address to)external;functionsync()external;}
```

# ABI
```
import IUniswapV2Pair from'@uniswap/v2-core/build/IUniswapV2Pair.json'
```

<https://unpkg.com/@uniswap/v2-core@1.0.0/build/IUniswapV2Pair.json>
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/02-pair.md)
Was this helpful?
[PreviousFactory](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)[NextPair (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20)
On this page
  * [Mint](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint)
  * [Burn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#burn)
  * [Swap](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap)
  * [Sync](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync)
  * [MINIMUM_LIQUIDITY](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#minimum_liquidity)
  * [factory](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#factory)
  * [token0](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#token0)
  * [token1](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#token1)
  * [getReserves](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#getreserves)
  * [price0CumulativeLast](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#price0cumulativelast)
  * [price1CumulativeLast](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#price1cumulativelast)
  * [kLast](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#klast)
  * [mint](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint-1)
  * [burn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#burn-1)
  * [swap](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap-1)
  * [skim](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#skim)
  * [sync](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#sync-1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/02-pair.md)
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
