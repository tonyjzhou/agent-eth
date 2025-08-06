# https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool

[Skip to main content](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
          * [Fetching Pool Data](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
          * [Create Pool](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
    * [web3-react](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Guides
  * Advanced
  * [Create Pool](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)


On this page
# Create Pool
## Introduction[​](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#introduction "Direct link to Introduction")
In this example we will use **ethers.js** and the **Uniswap v4 SDK** to create pools on Uniswap v4. Uniswap v4 is a popular destination for creating markets due to its:
  * Proven track record and battle-tested codebase
  * Concentrated liquidity, unlocks capital efficiency
  * Flexible pool design through dynamic fees and hooks
  * Gas-efficient architecture
  * Integrations with alternative trading venues


For more information, developers should see [Uniswap v4 Overview](https://docs.uniswap.org/contracts/v4/overview)
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v4-sdk`](https://www.npmjs.com/package/@uniswap/v4-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


## Configuration[​](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#configuration "Direct link to Configuration")
To initialize a Uniswap v4 Pool _without initial liquidity_ , developers should call [`PoolManager.initialize()`](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
Creating a pool without liquidity may be useful for "reserving" a pool for future use, when initial liquidity is not available, or when external market makers would provide the starting liquidity.
### Configure the Pool[​](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#configure-the-pool "Direct link to Configure the Pool")
We will first create an example configuration `CurrentConfig` in `config.ts`. It has the format:
```
exportconst CurrentConfig: ExampleConfig ={ env: Environment.MAINNET, rpc:{  local:'http://localhost:8545',  mainnet:'https://mainnet.infura.io/v3/YOUR_API_KEY',},... poolKey:{  currency0: currency0,  currency1: currency1,  fee: lpFee,  tickSpacing: tickSpacing,  hooks:HOOK_CONTRACT_ADDRESS,},}
```

> For native token pairs (Ether), use `ADDRESS_ZERO` as `currency0`
[PoolKey](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey) uniquely identifies a pool
  * _Currencies_ should be sorted, `uint160(currency0) < uint160(currency1)`
  * _lpFee_ is the fee expressed in pips, i.e. 3000 = 0.30%
  * _tickSpacing_ is the granularity of the pool. Lower values are more precise but may be more expensive to trade on
  * _hookContract_ is the address of the hook contract


A note on `tickSpacing`:
Lower tick spacing provides improved price precision; however, smaller tick spaces will cause swaps to cross ticks more often, incurring higher gas costs.
## Call `initialize` of Pool Manager contract[​](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#call-initialize-of-pool-manager-contract "Direct link to call-initialize-of-pool-manager-contract")
Now to initialize the `Pool` we need to call the `initialize` function of the Pool Manager Contract. To construct the Pool Manager Contract we need to provide the address of the contract, its ABI and a provider connected to an RPC endpoint.
```
import{ ethers }from'ethers'constPOOL_MANAGER_ADDRESS='0x000000000004444c5dc75cB358380D2e3dE08A90'// Replace with actual StateView contract addressconstPOOL_MANAGER_ABI=[...];// Import or define the ABI for PoolManager contractconst provider =getProvider()// Provide the right RPC address for the chainconst signer =newethers.Wallet(PRIVATE_KEY, provider)const poolManager =newethers.Contract(POOL_MANAGER_ADDRESS,POOL_MANAGER_ABI,  signer)
```

We get the `POOL_MANAGER_ADDRESS` for our chain from [Uniswap Deployments](https://docs.uniswap.org/contracts/v4/deployments).
Pools are initialized with a starting price
```
const result =await poolManager.initialize(  CurrentConfig.poolKey,  startingPrice)
```

  * the _startingPrice_ is expressed as sqrtPriceX96: `floor(sqrt(token1 / token0) * 2^96)`
    * i.e. `79228162514264337593543950336` is the starting price for a 1:1 pool


Now the pool is initialized and you can add liquidity to it.
## Important Note on Initial Liquidity[​](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#important-note-on-initial-liquidity "Direct link to Important Note on Initial Liquidity")
When creating a new pool, it's critical to understand that initializing a pool without liquidity can be dangerous. An empty pool's spot price is freely manipulatable since there is no liquidity to resist price movements.
This means that on the first liquidity provision, if proper slippage parameters are not set:
  1. Malicious actors can manipulate the price before the first position is minted
  2. The first position can be mispriced and have incorrect asset ratios


To safely add the first liquidity to a new pool:
  * Always use appropriate slippage parameters when minting the first position
  * Consider adding liquidity immediately after pool creation in the same transaction. Reference our [Mint Position guide](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting) for proper liquidity addition practices.


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/advanced/02-create-pool.md)
Was this helpful?
[PreviousFetching Pool Data](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)[NextOverview](https://docs.uniswap.org/sdk/v4/reference/overview)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#introduction)
  * [Configuration](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#configuration)
    * [Configure the Pool](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#configure-the-pool)
  * [Call `initialize` of Pool Manager contract](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#call-initialize-of-pool-manager-contract)
  * [Important Note on Initial Liquidity](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool#important-note-on-initial-liquidity)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/advanced/02-create-pool.md)
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
