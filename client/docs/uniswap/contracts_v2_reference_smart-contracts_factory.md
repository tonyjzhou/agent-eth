# https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory

[Skip to main content](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#__docusaurus_skipToContent_fallback)
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
  * [Factory](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)


On this page
# Factory
## Code[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#code "Direct link to Code")
[`UniswapV2Factory.sol`](https://github.com/Uniswap/uniswap-v2-core/blob/master/contracts/UniswapV2Factory.sol)
# Address
`UniswapV2Factory` is deployed at `0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f` on the Ethereum [mainnet](https://etherscan.io/address/0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f), and the [Ropsten](https://ropsten.etherscan.io/address/0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f), [Rinkeby](https://rinkeby.etherscan.io/address/0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f), [Görli](https://goerli.etherscan.io/address/0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f), and [Kovan](https://kovan.etherscan.io/address/0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f) testnets. It was built from commit [8160750](https://github.com/Uniswap/uniswap-v2-core/tree/816075049f811f1b061bca81d5d040b96f4c07eb).
# Events
## PairCreated[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#paircreated "Direct link to PairCreated")
```
eventPairCreated(addressindexed token0,addressindexed token1,address pair,uint);
```

Emitted each time a pair is created via [createPair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#createpair).
  * `token0` is guaranteed to be strictly less than `token1` by sort order.
  * The final `uint` log value will be `1` for the first pair created, `2` for the second, etc. (see [allPairs](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#allpairs)/[getPair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#getpair)).


# Read-Only Functions
## getPair[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#getpair "Direct link to getPair")
```
functiongetPair(address tokenA,address tokenB)externalviewreturns(address pair);
```

Returns the address of the pair for `tokenA` and `tokenB`, if it has been created, else `address(0)` (`0x0000000000000000000000000000000000000000`).
  * `tokenA` and `tokenB` are interchangeable.
  * Pair addresses can also be calculated deterministically via the SDK.


## allPairs[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#allpairs "Direct link to allPairs")
```
functionallPairs(uint)externalviewreturns(address pair);
```

Returns the address of the `n`th pair (`0`-indexed) created through the factory, or `address(0)` (`0x0000000000000000000000000000000000000000`) if not enough pairs have been created yet.
  * Pass `0` for the address of the first pair created, `1` for the second, etc.


## allPairsLength[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#allpairslength "Direct link to allPairsLength")
```
functionallPairsLength()externalviewreturns(uint);
```

Returns the total number of pairs created through the factory so far.
## feeTo[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#feeto "Direct link to feeTo")
```
functionfeeTo()externalviewreturns(address);
```

See [Protocol Charge Calculation](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees).
## feeToSetter[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#feetosetter "Direct link to feeToSetter")
```
functionfeeToSetter()externalviewreturns(address);
```

The address allowed to change [feeTo](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#feeto).
# State-Changing Functions
## createPair[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#createpair "Direct link to createPair")
```
functioncreatePair(address tokenA,address tokenB)externalreturns(address pair);
```

Creates a pair for `tokenA` and `tokenB` if one doesn't exist already.
  * `tokenA` and `tokenB` are interchangeable.
  * Emits [PairCreated](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#paircreated).


# Interface
```
import'@uniswap/v2-core/contracts/interfaces/IUniswapV2Factory.sol';
```

```
pragmasolidity>=0.5.0;interfaceIUniswapV2Factory{eventPairCreated(addressindexed token0,addressindexed token1,address pair,uint);functiongetPair(address tokenA,address tokenB)externalviewreturns(address pair);functionallPairs(uint)externalviewreturns(address pair);functionallPairsLength()externalviewreturns(uint);functionfeeTo()externalviewreturns(address);functionfeeToSetter()externalviewreturns(address);functioncreatePair(address tokenA,address tokenB)externalreturns(address pair);}
```

# ABI
```
import IUniswapV2Factory from'@uniswap/v2-core/build/IUniswapV2Factory.json'
```

<https://unpkg.com/@uniswap/v2-core@1.0.0/build/IUniswapV2Factory.json>
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/01-factory.md)
Was this helpful?
[PreviousGovernance Reference](https://docs.uniswap.org/contracts/v2/reference/Governance/governance-reference)[NextPair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
On this page
  * [Code](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#code)
  * [PairCreated](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#paircreated)
  * [getPair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#getpair)
  * [allPairs](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#allpairs)
  * [allPairsLength](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#allpairslength)
  * [feeTo](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#feeto)
  * [feeToSetter](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#feetosetter)
  * [createPair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#createpair)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/01-factory.md)
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
