# https://docs.uniswap.org/sdk/v3/guides/swaps/quoting

[Skip to main content](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
        * [Position Management](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
          * [Getting a Quote](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
          * [Executing a Trade](https://docs.uniswap.org/sdk/v3/guides/swaps/trading)
          * [Routing a Swap](https://docs.uniswap.org/sdk/v3/guides/swaps/routing)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
    * [web3-react](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Guides
  * Swaps
  * [Getting a Quote](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)


On this page
# Getting a Quote
## Introduction[​](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#introduction "Direct link to Introduction")
This guide will cover how to get the current quotes for any token pair on the Uniswap protocol. It is based on the [Quoting code example](https://github.com/Uniswap/examples/tree/main/v3-sdk/quoting), found in the Uniswap code examples [repository](https://github.com/Uniswap/examples). To run this example, check out the examples's [README](https://github.com/Uniswap/examples/blob/main/v3-sdk/quoting/README.md) and follow the setup instructions.
info
If you need a briefer on the SDK and to learn more about how these guides connect to the examples repository, please visit our [background](https://docs.uniswap.org/sdk/v3/guides/background) page!
In this example we will use `quoteExactInputSingle` to get a quote for the pair **USDC - WETH**. The inputs are the **token in** , the **token out** , the **amount in** and the **fee**.
The **fee** input parameter represents the swap fee that is distributed to all in-range liquidity at the time of the swap. It is one of the identifiers of a Pool, along with **tokenIn** and **tokenOut**.
The guide will **cover** :
  1. Computing the Pool's deployment address
  2. Referencing the Pool contract and fetching metadata
  3. Referencing the Quoter contract and getting a quote


At the end of the guide, we should be able to fetch a quote for the given input token pair and the input token amount with the press of a button on the web application.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v3-sdk`](https://www.npmjs.com/package/@uniswap/v3-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


The core code of this guide can be found in [`quote.ts`](https://github.com/Uniswap/examples/blob/main/v3-sdk/quoting/src/libs/quote.ts)
## Example configuration[​](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#example-configuration "Direct link to Example configuration")
We will use the example configuration `CurrentConfig` in most code snippets of this guide. It has the format:
```
import{ Token }from'@uniswap/sdk-core'interfaceExampleConfig{ rpc:{  local:string  mainnet:string} tokens:{in: Token  amountIn:number  out: Token  poolFee:number}}exportconst CurrentConfig: ExampleConfig ={...}
```

The default config of the example uses a local fork of mainnet. If you haven't already, check out our [local development guide](https://docs.uniswap.org/sdk/v3/guides/local-development). To change the rpc endpoint or the Pool used, edit the [`Currentconfig`](https://github.com/Uniswap/examples/blob/main/v3-sdk/quoting/src/config.ts#L21). To connect to mainnet directly, set the `mainnet` field in the config:
```
exportconst CurrentConfig: ExampleConfig ={ rpc:{  local:'http://localhost:8545',  mainnet:'https://mainnet.infura.io/v3/0ac57a06f2994538829c14745750d721',}, tokens:{in:USDC_TOKEN,  amountIn:1000,  out:WETH_TOKEN,  poolFee: FeeAmount.MEDIUM,},}
```

The pool used is defined by a pair of tokens in [`constants.ts`](https://github.com/Uniswap/examples/blob/main/v3-sdk/quoting/src/libs/constants.ts#L14). You can also change these two tokens and the fee of the pool in the config, just make sure a Pool actually exists for your configuration. Check out the top pools on [Uniswap info](https://info.uniswap.org/#/pools).
## Computing the Pool's deployment address[​](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#computing-the-pools-deployment-address "Direct link to Computing the Pool's deployment address")
To interact with the **USDC - WETH** Pool contract, we first need to compute its deployment address. If you haven't worked directly with smart contracts yet, check out this [guide](https://docs.alchemy.com/docs/smart-contract-basics) from Alchemy. The SDK provides a utility method for that:
```
import{ computePoolAddress }from'@uniswap/v3-sdk'const currentPoolAddress =computePoolAddress({ factoryAddress:POOL_FACTORY_CONTRACT_ADDRESS, tokenA: CurrentConfig.tokens.in, tokenB: CurrentConfig.tokens.out, fee: CurrentConfig.tokens.poolFee,})
```

Since each _Uniswap V3 Pool_ is uniquely identified by 3 characteristics (token in, token out, fee), we use those in combination with the address of the _PoolFactory_ contract to compute the address of the **USDC - ETH** Pool. These parameters have already been defined in our [constants.ts](https://github.com/Uniswap/examples/blob/main/v3-sdk/quoting/src/libs/constants.ts#L14) file:
```
constWETH_TOKEN=newToken(1,'0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2',18,'WETH','Wrapped Ether')constUSDC_TOKEN=newToken(1,'0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48',6,'USDC','USD//C')
```

These constants are used in the `config.ts` file, as mentioned in the Introduction.
We can find the Pool Factory Contract address for our chain [here](https://docs.uniswap.org/contracts/v3/reference/Deployments.md).
## Referencing the Pool contract and fetching metadata[​](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#referencing-the-pool-contract-and-fetching-metadata "Direct link to Referencing the Pool contract and fetching metadata")
Now that we have the deployment address of the **USDC - ETH** Pool, we can construct an instance of an **ethers** `Contract` to interact with it:
```
import{ ethers }from'ethers'const provider =newethers.providers.JsonRpcProvider(rpcUrl)const poolContract =newethers.Contract( currentPoolAddress, IUniswapV3PoolABI.abi, provider)
```

To construct the _Contract_ we need to provide the address of the contract, its ABI and the provider that will carry out the RPC call for us. We get access to the contract's ABI through the [@uniswap/v3-core](https://www.npmjs.com/package/@uniswap/v3-core) package, which holds the core smart contracts of the Uniswap V3 protocol:
```
import IUniswapV3PoolABI from'@uniswap/v3-core/artifacts/contracts/interfaces/IUniswapV3Pool.sol/IUniswapV3Pool.json'
```

Having constructed our reference to the contract, we can now access its methods through our provider. We use a batch `Promise` call. This approach queries state data concurrently, rather than sequentially, to minimize the chance of fetching out of sync data that may be returned if sequential queries are executed over the span of two blocks:
```
const[token0, token1, fee, liquidity, slot0]=awaitPromise.all([ poolContract.token0(), poolContract.token1(), poolContract.fee(), poolContract.liquidity(), poolContract.slot0(),])
```

The return values of these methods will become inputs to the quote fetching function. The `token0` and `token1` variables are the addresses of the tokens in the Pool and should not be mistaken for `Token` objects from the sdk. For the full code, check out [`getPoolConstants()`](https://github.com/Uniswap/examples/blob/main/v3-sdk/quoting/src/libs/quote.ts#L35) in `quote.ts`.
note
In this example, the metadata we fetch is already present in our inputs. This guide fetches this information first in order to show how to fetch any metadata, which will be expanded on in future guides.
## Referencing the Quoter contract and getting a quote[​](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#referencing-the-quoter-contract-and-getting-a-quote "Direct link to Referencing the Quoter contract and getting a quote")
To get quotes for trades, Uniswap has deployed a **Quoter Contract**. We will use this contract to fetch the output amount we can expect for our trade, without actually executing the trade. Check out the full code for the following snippets in [quote.ts](https://github.com/Uniswap/examples/blob/main/v3-sdk/quoting/src/libs/quote.ts)
Like we did for the Pool contract, we need to construct an instance of an **ethers** `Contract` for our Quoter contract in order to interact with it:
```
const quoterContract =newethers.Contract(QUOTER_CONTRACT_ADDRESS, Quoter.abi,getProvider())
```

We get access to the contract's ABI through the [@uniswap/v3-periphery](https://www.npmjs.com/package/@uniswap/v3-periphery) package, which holds the periphery smart contracts of the Uniswap V3 protocol:
```
import Quoter from'@uniswap/v3-periphery/artifacts/contracts/lens/Quoter.sol/Quoter.json'
```

We get the QUOTE_CONTRACT_ADDRESS for our chain from [GitHub](https://github.com/Uniswap/v3-periphery/blob/main/deploys.md).
We can now use our Quoter contract to obtain the quote.
In an ideal world, the quoter functions would be `view` functions, which would make them very easy to query on-chain with minimal gas costs. However, the Uniswap V3 Quoter contracts rely on state-changing calls designed to be reverted to return the desired data. This means calling the quoter will be very expensive and should not be called on-chain.
To get around this difficulty, we can use the `callStatic` method provided by the **ethers.js** `Contract` instances. This is a useful method that submits a state-changing transaction to an Ethereum node, but asks the node to simulate the state change, rather than to execute it. Our script can then return the result of the simulated state change:
```
const quotedAmountOut =await quoterContract.callStatic.quoteExactInputSingle( token0, token1, fee,fromReadableAmount(  CurrentConfig.tokens.amountIn,  CurrentConfig.tokens.in.decimals).toString(),0)
```

The `fromReadableAmount()` function creates the amount of the smallest unit of a token from the full unit amount and the decimals.
The result of the call is the number of output tokens you'd receive for the quoted swap.
It should be noted that `quoteExactInputSingle` is only 1 of 4 different methods that the quoter offers:
  1. `quoteExactInputSingle` - given the amount you want to swap, produces a quote for the amount out for a swap of a single pool
  2. `quoteExactInput` - given the amount you want to swap, produces a quote for the amount out for a swap over multiple pools
  3. `quoteExactOutputSingle` - given the amount you want to get out, produces a quote for the amount in for a swap over a single pool
  4. `quoteExactOutput` - given the amount you want to get out, produces a quote for the amount in for a swap over multiple pools


If we want to trade two tokens that do not share a pool with each other, we will need to make swaps over multiple pools. This is where the `quoteExactInput` and `quoteExactOutput` methods come in. We will dive deeper into routing in the [routing guide](https://docs.uniswap.org/sdk/v3/guides/swaps/routing).
For the `exactOutput` and `exactOutputSingle` methods, we need to keep in mind that a pool can not give us more than the amount of Tokens it holds. If we try to get a quote on an output of 100 WETH from a Pool that only holds 50 WETH, the function call will fail.
## Next Steps[​](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#next-steps "Direct link to Next Steps")
Now that you're able to make a quote, check out our next guide on [trading](https://docs.uniswap.org/sdk/v3/guides/swaps/trading) using this quote!
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/swaps/01-quoting.md)
Was this helpful?
[PreviousWeb3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)[NextExecuting a Trade](https://docs.uniswap.org/sdk/v3/guides/swaps/trading)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#introduction)
  * [Example configuration](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#example-configuration)
  * [Computing the Pool's deployment address](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#computing-the-pools-deployment-address)
  * [Referencing the Pool contract and fetching metadata](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#referencing-the-pool-contract-and-fetching-metadata)
  * [Referencing the Quoter contract and getting a quote](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#referencing-the-quoter-contract-and-getting-a-quote)
  * [Next Steps](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/swaps/01-quoting.md)
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
