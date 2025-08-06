# https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps#__docusaurus_skipToContent_fallback)
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
          * [Smart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
          * [Implement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
          * [Providing Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
          * [Building an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)
          * [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses)
          * [Supporting meta transactions](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/Governance/governance-reference)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Smart Contract Integration
  * [Flash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)


On this page
Flash swaps are an integral feature of Uniswap V2. In fact, under the hood, all swaps are actually flash swaps! This simply means that pair contracts send output tokens to the recipient _before_ enforcing that enough input tokens have been received. This is slightly atypical, as one might expect a pair to ensure it's received payment before delivery. However, because Ethereum transactions are _atomic_ , we can roll back the entire swap if it turns out that the contract hasn't received enough tokens to make itself whole by the end of the transaction.
To see how this all works, let's start by examining the interface of the `swap` function:
```
functionswap(uint amount0Out,uint amount1Out,address to,bytescalldata data);
```

For the sake of example, let's assume that we're dealing with a DAI/WETH pair, where DAI is `token0` and WETH is `token1`. `amount0Out` and `amount1Out` specify the amount of DAI and WETH that the `msg.sender` wants the pair to send to the `to` address (one of these amounts may be 0). At this point you may be wondering how the contract _receives_ tokens. For a typical (non-flash) swap, it's actually the responsibility of `msg.sender` to ensure that enough WETH or DAI has _already been sent_ to the pair before `swap` is called (in the context of trading, this is all handled neatly by a router contract). But when executing a flash swap, _tokens do not need to be sent to the contract before calling`swap`_. Instead, they must be sent from within a _callback function_ that the pair triggers on the `to` address.
# Triggering a Flash Swap
To differentiate between the "typical" trading case and the flash swap case, pairs use the `data` parameter. Specifically, if `data.length` equals 0, the contract assumes that payment has already been received, and simply transfers the tokens to the `to` address. But, if `data.length` is greater than 0, the contract transfers the tokens and then calls the following function on the `to` address:
```
functionuniswapV2Call(address sender,uint amount0,uint amount1,bytescalldata data);
```

The logic behind this identification strategy is simple: the vast majority of valid flash swap use cases involve interactions with external protocols. The best way to pass information dictating how these interactions happen (function arguments, safety parameters, addresses, etc.) is via the `data` parameter. It's expected that `data` will be `abi.decode`d from within `uniswapV2Call`. In the rare case where no data is required, callers should ensure that `data.length` equals 1 (i.e. encode a single junk byte as `bytes`), and then ignore this argument in `uniswapV2Call`.
Pairs call `uniswapV2Call` with the `sender` argument set to the `msg.sender` of the `swap`. `amount0` and `amount1` are simply `amount0Out` and `amount1Out`.
# Using uniswapV2Call
There are several conditions that should be checked in all `uniswapV2Call` functions:
```
functionuniswapV2Call(address sender,uint amount0,uint amount1,bytescalldata data){address token0 =IUniswapV2Pair(msg.sender).token0();// fetch the address of token0address token1 =IUniswapV2Pair(msg.sender).token1();// fetch the address of token1assert(msg.sender ==IUniswapV2Factory(factoryV2).getPair(token0, token1));// ensure that msg.sender is a V2 pair// rest of the function goes here!}
```

The first 2 lines simply fetch the token addresses from the pair, and the 3rd ensures that the `msg.sender` is an actual Uniswap V2 pair address.
# Repayment
At the end of `uniswapV2Call`, contracts must return enough tokens to the pair to make it whole. Specifically, this means that the product of the pair reserves after the swap, discounting all token amounts sent by 0.3% LP fee, must be greater than before.
## Multi-Token[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps#multi-token "Direct link to Multi-Token")
In the case where the token withdrawn is _not_ the token returned (i.e. DAI was requested in the flash swap, and WETH was returned, or vice versa), the fee simplifies to the simple swap case. This means that the standard `getAmountIn` pricing function should be used to calculate e.g., the amount of WETH that must be returned in exchange for the amount of DAI that was requested out.
This type of fee calculation gives a slight advantage to the caller, as the fee derived from repayment in a corresponding token will always be slightly less than the fee derived from a direct token repayment, as a result of the difference between the amount required to pay back a swap, versus the amount withdrawn and then directly returned. The approximate comparison of fees is ~ 30 bps for a swap fee vs. 30.09 bps for a direct repayment.
## Single-Token[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps#single-token "Direct link to Single-Token")
In the case where the token withdrawn is the _same_ as the token returned (i.e. DAI was requested in the flash swap, used, then returned, or vice versa with WETH), the following condition must be satisfied:
`DAIReservePre - DAIWithdrawn + (DAIReturned * .997) >= DAIReservePre`
It may be more intuitive to rewrite this formula in terms of a "fee" levied on the _withdrawn_ amount (despite the fact that Uniswap always levies fees on input amounts, in this case the _returned_ amount, here we can simplify to an effective fee on the _withdrawn_ amount). If we rearrange, the formula looks like:
`(DAIReturned * .997) - DAIWithdrawn >= 0`
`DAIReturned >= DAIWithdrawn / .997`
So, the effective fee on the withdrawn amount is `.003 / .997 ≈ 0.3009027%`.
# Resources
For further exploration of flash swaps, see the [whitepaper](https://docs.uniswap.org/whitepaper.pdf).
# Example
A fully functional example of flash swaps is available: [`ExampleFlashSwap.sol`](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/examples/ExampleFlashSwap.sol).
# Interface
```
import'@uniswap/v2-core/contracts/interfaces/IUniswapV2Callee.sol';
```

```
pragmasolidity>=0.5.0;interfaceIUniswapV2Callee{functionuniswapV2Call(address sender,uint amount0,uint amount1,bytescalldata data)external;}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/05-using-flash-swaps.md)
Was this helpful?
[PreviousBuilding an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)[NextPair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses)
On this page
  * [Multi-Token](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps#multi-token)
  * [Single-Token](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps#single-token)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/05-using-flash-swaps.md)
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
