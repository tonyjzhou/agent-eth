# https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library

[Skip to main content](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#__docusaurus_skipToContent_fallback)
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
  * [Library](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library)


On this page
# Library
## Code[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#code "Direct link to Code")
[`UniswapV2Library.sol`](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol)
# Internal Functions
## sortTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#sorttokens "Direct link to sortTokens")
```
functionsortTokens(address tokenA,address tokenB)internalpurereturns(address token0,address token1);
```

Sorts token addresses.
## pairFor[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#pairfor "Direct link to pairFor")
```
functionpairFor(address factory,address tokenA,address tokenB)internalpurereturns(address pair);
```

Calculates the address for a pair without making any external calls via the v2 SDK.
## getReserves[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getreserves "Direct link to getReserves")
```
functiongetReserves(address factory,address tokenA,address tokenB)internalviewreturns(uint reserveA,uint reserveB);
```

Calls [getReserves](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#getreserves) on the pair for the passed tokens, and returns the results sorted in the order that the parameters were passed in.
## quote[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#quote "Direct link to quote")
```
functionquote(uint amountA,uint reserveA,uint reserveB)internalpurereturns(uint amountB);
```

Given some asset amount and reserves, returns an amount of the other asset representing equivalent value.
  * Useful for calculating optimal token amounts before calling [mint](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#mint-1).


## getAmountOut[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountout "Direct link to getAmountOut")
```
functiongetAmountOut(uint amountIn,uint reserveIn,uint reserveOut)internalpurereturns(uint amountOut);
```

Given an _input_ asset amount, returns the maximum _output_ amount of the other asset (accounting for fees) given reserves.
  * Used in [getAmountsOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountsout).


## getAmountIn[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountin "Direct link to getAmountIn")
```
functiongetAmountIn(uint amountOut,uint reserveIn,uint reserveOut)internalpurereturns(uint amountIn);
```

Returns the minimum _input_ asset amount required to buy the given _output_ asset amount (accounting for fees) given reserves.
  * Used in [getAmountsIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountsin).


## getAmountsOut[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountsout "Direct link to getAmountsOut")
```
functiongetAmountsOut(uint amountIn,address[]memory path)internalviewreturns(uint[]memory amounts);
```

Given an _input_ asset amount and an array of token addresses, calculates all subsequent maximum _output_ token amounts by calling [getReserves](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getreserves) for each pair of token addresses in the path in turn, and using these to call [getAmountOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountout).
  * Useful for calculating optimal token amounts before calling [swap](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap-1).


## getAmountsIn[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountsin "Direct link to getAmountsIn")
```
functiongetAmountsIn(address factory,uint amountOut,address[]memory path)internalviewreturns(uint[]memory amounts);
```

Given an _output_ asset amount and an array of token addresses, calculates all preceding minimum _input_ token amounts by calling [getReserves](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getreserves) for each pair of token addresses in the path in turn, and using these to call [getAmountIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountin).
  * Useful for calculating optimal token amounts before calling [swap](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#swap-1).


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/04-library.md)
Was this helpful?
[PreviousPair (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20)[NextRouter01](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-01)
On this page
  * [Code](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#code)
  * [sortTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#sorttokens)
  * [pairFor](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#pairfor)
  * [getReserves](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getreserves)
  * [quote](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#quote)
  * [getAmountOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountout)
  * [getAmountIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountin)
  * [getAmountsOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountsout)
  * [getAmountsIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountsin)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/04-library.md)
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
