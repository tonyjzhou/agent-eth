# https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data

[Skip to main content](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#__docusaurus_skipToContent_fallback)
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
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/overview)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/overview)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/advanced/introduction)
          * [Introduction](https://docs.uniswap.org/sdk/v3/guides/advanced/introduction)
          * [Fetching Pool Data](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data)
          * [Active Liquidity](https://docs.uniswap.org/sdk/v3/guides/advanced/active-liquidity)
          * [Uniswap as a Price Oracle](https://docs.uniswap.org/sdk/v3/guides/advanced/price-oracle)
          * [Range Orders](https://docs.uniswap.org/sdk/v3/guides/advanced/range-orders)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/reference/overview)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Guides
  * Advanced
  * [Fetching Pool Data](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data)


On this page
# Fetching Pool Data
## Introduction[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#introduction "Direct link to Introduction")
This guide will cover how to initialize a Pool with full tick data to allow offchain calculations. It is based on the [Fetching Pool data example](https://github.com/Uniswap/examples/tree/main/v3-sdk/pool-data), found in the Uniswap code examples [repository](https://github.com/Uniswap/examples). To run this example, check out the guide's [README](https://github.com/Uniswap/examples/blob/main/v3-sdk/pool-data/README.md) and follow the setup instructions.
info
If you need a briefer on the SDK and to learn more about how these guides connect to the examples repository, please visit our [background](https://docs.uniswap.org/sdk/v3/guides/background) page!
In this example we will use **ethers JS** and **ethers-multicall** to construct a `Pool` object that we can use in the following guides.
This guide will **cover** :
  1. Computing the Pool's address
  2. Referencing the Pool contract and fetching metadata
  3. Fetching the positions of all initialized Ticks with multicall
  4. Fetching all ticks by their indices with a multicall
  5. Constructing the Pool object


At the end of the guide, we will have created a `Pool` Object that accurately represents the state of a V3 pool at the time we fetched it.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v3-sdk`](https://www.npmjs.com/package/@uniswap/v3-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


We will also use the `ethers-multicall` npm package:
  * [`ethers-multicall`](https://www.npmjs.com/package/ethers-multicall)


The core code of this guide can be found in [`fetcher.ts`](https://github.com/Uniswap/examples/tree/main/v3-sdk/multicall/src/libs/fetcher.ts)
## Configuration[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#configuration "Direct link to Configuration")
The example accompanying this guide can be configured in the [`config.ts`](https://github.com/Uniswap/examples/tree/main/v3-sdk/multicall/src/config.ts) file. The default configuration defines the rpc endpoint and the pool that is used for this guide:
```
exportconst CurrentConfig: ExampleConfig ={ env: Environment.MAINNET, rpc:{  local:'http://localhost:8545',  mainnet:'https://mainnet.infura.io/v3/0ac57a06f2994538829c14745750d721',},... pool:{  token0:USDC_TOKEN,  token1:WETH_TOKEN,  fee: FeeAmount.MEDIUM,},}
```

FeeAmount.MEDIUM means that the pool has a swap fee of **0.3%**. The `USDC_TOKEN` and `WETH_TOKEN` are defined in the [`constants.ts`](https://github.com/Uniswap/examples/tree/main/v3-sdk/multicall/src/libs/constants.ts) file:
```
exportconstWETH_TOKEN=newToken( SupportedChainId.MAINNET,'0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2',18,'WETH','Wrapped Ether')exportconstUSDC_TOKEN=newToken( SupportedChainId.MAINNET,'0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48',6,'USDC','USD//C')
```

## Computing the Pool's deployment address[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#computing-the-pools-deployment-address "Direct link to Computing the Pool's deployment address")
In this example, we will construct the **USDC - WETH** Pool with **MEDIUM** fees. The SDK provides a method to compute the address:
```
import{ Pool }from'@uniswap/v3-sdk'import{ CurrentConfig }from'../config.ts'const poolAddress = Pool.getAddress(  CurrentConfig.pool.token0,  CurrentConfig.pool.token1,  CurrentConfig.pool.fee)
```

Uniswap V3 allows 4 different Fee tiers when deploying a pool, so multiple pools can exist for each pair of tokens.
## Creating a Pool Contract instance and fetching metadata[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#creating-a-pool-contract-instance-and-fetching-metadata "Direct link to Creating a Pool Contract instance and fetching metadata")
Now that we have the address of a **USDC - ETH** Pool, we can construct an instance of an **ethers** `Contract` to interact with it. To construct the Contract we need to provide the address of the contract, its ABI and a provider connected to an [RPC endpoint](https://www.chainnodes.org/docs). We get access to the contract's ABI through the `@uniswap/v3-core` package, which holds the core smart contracts of the Uniswap V3 protocol:
```
import{ ethers }from'ethers'import IUniswapV3PoolABI from'@uniswap/v3-core/artifacts/contracts/interfaces/IUniswapV3Pool.sol/IUniswapV3Pool.json'const provider =getProvider()const poolContract =newethers.Contract(  poolAddress,  IUniswapV3PoolABI.abi,  provider)
```

The `getProvider()` function returns an `ethers.providers.JsonRpcProvider` with either the local or mainnet rpc url that we defined, depending on the Environment that we set in `config.ts`.
Once we have set up our reference to the contract, we can proceed to access its methods. To construct our offchain representation of the Pool Contract, we need to fetch its liquidity, sqrtPrice, currently active tick and the full Tick data. We get the **liquidity** , **sqrtPrice** and **tick** directly from the blockchain by calling `liquidity()`and `slot0()` on the Pool contract:
```
const[liquidity, slot0]=awaitPromise.all([  poolContract.liquidity(),  poolContract.slot0(),])
```

The [slot0 function](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/pool/IUniswapV3PoolState#slot0) represents the first (0th) storage slot of the pool and exposes multiple useful values in a single function:
```
functionslot0()externalviewreturns(uint160 sqrtPriceX96,int24 tick,uint16 observationIndex,uint16 observationCardinality,uint16 observationCardinalityNext,uint8 feeProtocol,bool unlocked)
```

For our use case, we only need the `sqrtPriceX96` and the currently active `tick`.
## Fetching all Ticks[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#fetching-all-ticks "Direct link to Fetching all Ticks")
V3 pools use ticks to [concentrate liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity) in price ranges and allow for better pricing of trades. Even though most Pools only have a couple of **initialized ticks** , it is possible that a pools liquidity is defined by thousands of **initialized ticks**. In that case, it can be very expensive or slow to get all of them with normal RPC calls.
If you are not familiar with the concept of ticks, check out the [`introduction`](https://docs.uniswap.org/sdk/v3/guides/advanced/introduction).
To access tick data, we will use the `ticks` function of the V3 Pool contract:
```
functionticks(int24 tick)externalviewreturns(uint128 liquidityGross,int128 liquidityNet,uint256 feeGrowthOutside0X128,uint256 feeGrowthOutside1X128,int56 tickCumulativeOutside,uint160 secondsPerLiquidityOutsideX128,uint32 secondsOutside,bool initialized)
```

The `tick` parameter that we provide the function with is the **index** (memory position) of the Tick we are trying to fetch. To get the indices of all initialized Ticks of the Pool, we can calculate them from the **tickBitmaps**. To fetch a `tickBitmap` function of the V3 Pool:
```
functiontickBitmap(int16 wordPosition)externalviewreturns(uint256)
```

A pool stores lots of bitmaps, each of which contain the status of 256 Ticks. The parameter `int16 wordPosition` the function accepts is the position of the bitMap we want to fetch. We can calculate all the position of bitMaps (or words as they are sometimes called) from the `tickSpacing` of the Pool, which is in turn dependant on the Fee tier.
So to summarise we need 4 steps to fetch all initialized ticks:
  1. Calculate all bitMap positions from the tickSpacing of the Pool.
  2. Fetch all bitMaps using their positions.
  3. Calculate the memory positions of all Ticks from the bitMaps.
  4. Fetch all Ticks by their memory position.


We will use multicalls for the fetch calls.
## Multicall[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#multicall "Direct link to Multicall")
Multicall contracts **aggregate results** from multiple contract calls and therefore allow sending multiple contract calls in **one RPC request**. This can improve the **speed** of fetching large amounts of data significantly and ensures that the data fetched is all from the **same block**.
We will use the Multicall2 contract by MakerDAO. We use the `ethers-muticall` npm package to easily interact with the Contract.
## Calculating all bitMap positions[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#calculating-all-bitmap-positions "Direct link to Calculating all bitMap positions")
As mentioned, Uniswap V3 Pools store **bitmaps** , also called _words_ , that represent the state of **256 initializable ticks** at a time. The value at a bit of a word is 1 if the tick at this index is initialized and 0 if it isn't. We can calculate the positions of initialized ticks from the **words** of the Pool.
All ticks of Uniswap V3 pools are between the indices `-887272` and `887272`. We can calculate the minimum and maximum word from these indices and the Pool's tickSpacing:
```
functiontickToWord(tick:number):number{let compressed = Math.floor(tick / tickSpacing)if(tick <0&& tick % tickSpacing !==0){  compressed -=1}return compressed >>8}const minWord =tickToWord(-887272)const maxWord =tickToWord(887272)
```

Ticks can only be initialized at indices that are **divisible by the tickSpacing**. One word contains 256 ticks, so we can compress the ticks by right shifting 8 bit.
## Fetching bitMaps from their position[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#fetching-bitmaps-from-their-position "Direct link to Fetching bitMaps from their position")
Knowing the positions of words in the Pool contract, we can now fetch them from the Pool using multicall and the `tickBitmap` read call.
First we initialize our multicall providers and Pool Contract:
```
import{ ethers }from'ethers'import{ Contract, Provider }from'ethers-multicall'const ethersProvider =newethers.providers.JsonRpcProvider("...rpcUrl")const multicallProvider =newProvider(ethersProvider)await multicallProvider.init()const poolContract =newContract(poolAddress, IUniswapV3PoolABI.abi)
```

The `multicallProvider` creates the multicall request and sends it via the ethers Provider.
Next we loop through all possible word positions and add a `tickBitmap` call for each:
```
let calls:any[]=[]let wordPosIndices:number[]=[]for(let i = minWord; i <= maxWord; i++){ wordPosIndices.push(i) calls.push(poolContract.tickBitmap(i))}
```

We also keep track of the word position indices to be able to loop through them in the same order we added the calls to the array.
We use the `multicallProvider.all()` function to send a multicall and map the results:
```
const results: bigint[]=(await multicallProvider.all(calls)).map((ethersResponse)=>{returnBigInt(ethersResponse.toString())})
```

A great visualization of what the bitMaps look like can be found in the [Uniswap V3 development book](https://uniswapv3book.com/docs/milestone_2/tick-bitmap-index/):
![TickBitmap](https://docs.uniswap.org/assets/images/tickBitmap_cut-0657ffe617e53a11e38397e09300fe73.png)
We encourage anyone trying to get a deeper understanding of the Uniswap protocol to read the Uniswap V3 Book.
## Calculating the memory positions of all Ticks[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#calculating-the-memory-positions-of-all-ticks "Direct link to Calculating the memory positions of all Ticks")
Now that we fetched all **bitMaps** , we check which ticks are initialized and calculate the **tick position** from the **word index** and the **tickSpacing** of the pool.
We check if a tick is **initialized** inside the word by shifting a bit by the index we are looking at and performing a bitwise AND operation:
```
const bit =1nconst initialized =(bitmap &(bit <<BigInt(i)))!==0n
```

If the tick is **initialized** , we revert the compression from tick to word we made earlier by multiplying the word index with 256, which is the same as left shifting by 8 bit, adding the position we are currently at, and multiplying with the tickSpacing:
```
const tickIndex =(ind *256+ i)* tickSpacing
```

The whole loop looks like this:
```
const tickIndices:number[]=[]for(let j =0; j < wordPosIndices.length; j++){const ind = wordPosIndices[j]const bitmap = results[j]if(bitmap !==0n){for(let i =0; i <256; i++){const bit =1nconst initialized =(bitmap &(bit <<BigInt(i)))!==0nif(initialized){const tickIndex =(ind *256+ i)* tickSpacing     tickIndices.push(tickIndex)}}}}
```

We now have an array containing the indices of all initialized Ticks.
## Fetching all Ticks by their indices[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#fetching-all-ticks-by-their-indices "Direct link to Fetching all Ticks by their indices")
We use the multicallProvider again to execute an aggregated read call for all tick indices. We create an array of call Promises again and use `.all()` to make our multicall:
```
const calls:any[]=[]for(const index of tickIndices){ calls.push(poolContract.ticks(index))}const results =await multicallProvider.all(calls)
```

Again, the order of the results array is the same as the elements in **tickIndices**.
We are able to combine the **tickIndices** and **results** array to create an array of `Tick` objects:
```
const allTicks: Tick[]=[]for(let i =0; i < tickIndices.length; i++){const index = tickIndices[i]const ethersResponse = results[i]const tick =newTick({   index,   liquidityGross:JSBI.BigInt(ethersResponse.liquidityGross.toString()),   liquidityNet:JSBI.BigInt(ethersResponse.liquidityNet.toString()),})  allTicks.push(tick)}
```

We need to parse the response from our RPC provider to JSBI values that the v3-sdk can work with.
## Constructing the Pool[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#constructing-the-pool "Direct link to Constructing the Pool")
We have everything to construct our `Pool` now:
```
const usdcWethPool =newPool(USDC,WETH,  feeAmount,  slot0.sqrtPriceX96,  liquidity,  slot0.tick,  allTicks)
```

With this fully initialized Pool, we can make accurate offchain calculations.
## Next Steps[​](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#next-steps "Direct link to Next Steps")
Now that you are familiar with fetching Pool data, continue your journey with the [next example](https://docs.uniswap.org/sdk/v3/guides/advanced/active-liquidity) on visualizing the Liquidity density of a pool.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/advanced/02-pool-data.md)
Was this helpful?
[PreviousIntroduction](https://docs.uniswap.org/sdk/v3/guides/advanced/introduction)[NextActive Liquidity](https://docs.uniswap.org/sdk/v3/guides/advanced/active-liquidity)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#introduction)
  * [Configuration](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#configuration)
  * [Computing the Pool's deployment address](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#computing-the-pools-deployment-address)
  * [Creating a Pool Contract instance and fetching metadata](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#creating-a-pool-contract-instance-and-fetching-metadata)
  * [Fetching all Ticks](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#fetching-all-ticks)
  * [Multicall](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#multicall)
  * [Calculating all bitMap positions](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#calculating-all-bitmap-positions)
  * [Fetching bitMaps from their position](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#fetching-bitmaps-from-their-position)
  * [Calculating the memory positions of all Ticks](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#calculating-the-memory-positions-of-all-ticks)
  * [Fetching all Ticks by their indices](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#fetching-all-ticks-by-their-indices)
  * [Constructing the Pool](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#constructing-the-pool)
  * [Next Steps](https://docs.uniswap.org/sdk/v3/guides/advanced/pool-data#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/advanced/02-pool-data.md)
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
