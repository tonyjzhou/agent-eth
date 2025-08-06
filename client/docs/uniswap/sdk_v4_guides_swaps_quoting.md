# https://docs.uniswap.org/sdk/v4/guides/swaps/quoting

[Skip to main content](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/overview)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
          * [Getting a Quote](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
          * [Executing a Single-Hop Swap](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping)
          * [Executing Multi-Hop Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/overview)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/overview)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Guides
  * Swaps
  * [Getting a Quote](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)


On this page
# Getting a Quote
## Introduction[​](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting#introduction "Direct link to Introduction")
This guide will cover how to get the current quotes for any token pair on the Uniswap protocol.
In this example we will use `quoteExactInputSingle` to get a quote for the pair **ETH - USDC**. The inputs are **poolKey** , **zeroForOne** , **exactAmount** and **hookData**.
The guide will cover:
  1. Constructing the `PoolKey` and swap parameters
  2. Referencing the `Quoter` contract and getting a quote


At the end of the guide, we should be able to fetch the output for the given token pair and input amount.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v4-sdk`](https://www.npmjs.com/package/@uniswap/v4-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


## Constructing the PoolKey and Swap parameters[​](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting#constructing-the-poolkey-and-swap-parameters "Direct link to Constructing the PoolKey and Swap parameters")
We will first create an example configuration `CurrentConfig` in `config.ts`. For this example, we are using the [0.05% ETH - USDC pool](https://app.uniswap.org/explore/pools/ethereum/0x21c67e77068de97969ba93d4aab21826d33ca12bb9f565d8496e8fda8a82ca27) which has the format:
```
import{ SwapExactInSingle }from'@uniswap/v4-sdk'import{USDC_TOKEN,ETH_TOKEN}from'./constants'exportconst CurrentConfig: SwapExactInSingle ={  poolKey:{    currency0:ETH_TOKEN.address,    currency1:USDC_TOKEN.address,    fee:500,    tickSpacing:10,    hooks:"0x0000000000000000000000000000000000000000",},  zeroForOne:true,  amountIn: ethers.utils.parseUnits('1',ETH_TOKEN.decimals).toString(),  amountOutMinimum:"0",  hookData:'0x00'}
```

The pool used is defined by a pair of tokens in `constants.ts`. You can also change these two tokens and the other pool parameters in the config, just make sure a pool actually exists for your configuration. Check out the top pools on [Uniswap](https://app.uniswap.org/#/pools).
```
import{ Token, ChainId }from'@uniswap/sdk-core'constETH_TOKEN=newToken( ChainId.MAINNET,'0x0000000000000000000000000000000000000000',18,'ETH','Ether')constUSDC_TOKEN=newToken( ChainId.MAINNET,'0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48',6,'USDC','USDC')
```

## Referencing the Quoter contract and getting a quote[​](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting#referencing-the-quoter-contract-and-getting-a-quote "Direct link to Referencing the Quoter contract and getting a quote")
To get quotes for trades, Uniswap has deployed a **Quoter Contract**. We will use this contract to fetch the output amount we can expect for our trade, without actually executing the trade.
Now, we need to construct an instance of an **ethers** `Contract` for our Quoter contract in order to interact with it:
```
const quoterContract =newethers.Contract(QUOTER_CONTRACT_ADDRESS,QUOTER_ABI,// Import or define the ABI for Quoter contractnewethers.providers.JsonRpcProvider("RPC")// Provide the right RPC address for the chain)
```

We get the `QUOTE_CONTRACT_ADDRESS` for our chain from [Uniswap Deployments](https://docs.uniswap.org/contracts/v4/deployments).
We can now use our Quoter contract to obtain the quote.
In an ideal world, the quoter functions would be `view` functions, which would make them very easy to query on-chain with minimal gas costs. However, the Uniswap V4 Quoter contracts rely on state-changing calls designed to be reverted to return the desired data. This means calling the quoter will be very expensive and **should not be called on-chain.**
To get around this difficulty, we can use the `callStatic` method provided by the **ethers.js** `Contract` instances. This is a useful method that submits a state-changing transaction to an Ethereum node, but asks the node to simulate the state change, rather than to execute it. Our script can then return the result of the simulated state change:
```
const quotedAmountOut =await quoterContract.callStatic.quoteExactInputSingle({  poolKey: CurrentConfig.poolKey,  zeroForOne: CurrentConfig.zeroForOne,  exactAmount: CurrentConfig.amountIn,  hookData: CurrentConfig.hookData,})console.log(ethers.utils.formatUnits(quotedAmountOut.amountOut,USDC_TOKEN.decimals));
```

The result of the call is the number of output tokens you would receive for the quoted swap.
It should be noted that `quoteExactInputSingle` is only 1 of 4 different methods that the quoter offers:
  1. `quoteExactInputSingle` - given an input amount, produce a quote of the output amount for a swap on a single pool
  2. `quoteExactInput` - given an input amount, produce a quote for the output amount a swap over multiple pools
  3. `quoteExactOutputSingle` - given a desired output amount, produce a quote for the input amount on a swap over a single pool
  4. `quoteExactOutput` - given a desired output amount, produce a quote for the input amount in for a swap over multiple pools


If we want to trade two tokens that do not share a pool with each other, we will need to make swaps over multiple pools. This is where the `quoteExactInput` and `quoteExactOutput` methods come in.
For the `exactOutput` and `exactOutputSingle` methods, we need to keep in mind that a pool can not give us more than the amount of Tokens it holds. If we try to get a quote on an output of 100 ETH from a pool that only holds 50 ETH, the function call will fail.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/swaps/01-quoting.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/sdk/v4/overview)[NextExecuting a Single-Hop Swap](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting#introduction)
  * [Constructing the PoolKey and Swap parameters](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting#constructing-the-poolkey-and-swap-parameters)
  * [Referencing the Quoter contract and getting a quote](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting#referencing-the-quoter-contract-and-getting-a-quote)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/swaps/01-quoting.md)
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
