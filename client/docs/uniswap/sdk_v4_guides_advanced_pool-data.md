# https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data

[Skip to main content](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#__docusaurus_skipToContent_fallback)
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
          * [Fetching Pool Data](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
          * [Create Pool](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
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
  * Advanced
  * [Fetching Pool Data](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)


On this page
# Fetching Pool Data
## Introduction[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#introduction "Direct link to Introduction")
In this example we will use **ethers JS** and **ethers-multicall** to construct a `Pool` object that we can use in the following guides.
This guide will **cover** :
  1. Computing the PoolId out of PoolKey
  2. Referencing the StateView contract and fetching metadata
  3. Fetching the positions of all initialized Ticks with multicall
  4. Fetching all ticks by their indices with a multicall
  5. Constructing the Pool object


At the end of the guide, we will have created a `Pool` Object that accurately represents the state of a v4 pool at the time we fetched it.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v4-sdk`](https://www.npmjs.com/package/@uniswap/v4-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


We will also use the `ethers-multicall` npm package:
  * [`ethers-multicall`](https://www.npmjs.com/package/ethers-multicall)


## Configuration[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#configuration "Direct link to Configuration")
We will first create an example configuration `CurrentConfig` in `config.ts`. It has the format:
```
exportconst CurrentConfig: ExampleConfig ={ env: Environment.MAINNET, rpc:{  local:'http://localhost:8545',  mainnet:'https://mainnet.infura.io/v3/YOUR_API_KEY',},... poolKey:{  currency0:USDC_TOKEN.address,  currency1:ETH_TOKEN.address,  fee:FEE_AMOUNT_LOW,  tickSpacing:TICK_SPACING_TEN,  hooks:EMPTY_HOOK,},}
```

The pool used is defined by a pair of tokens in `constants.ts`. You can also change these two tokens and the other pool parameters in the config, just make sure a pool actually exists for your configuration. Check out the top pools on [Uniswap](https://app.uniswap.org/explore/pools).
```
exportconstETH_TOKEN=newToken( SupportedChainId.MAINNET,'0x0000000000000000000000000000000000000000',18,'ETH','Ether')exportconstUSDC_TOKEN=newToken( SupportedChainId.MAINNET,'0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48',6,'USDC','USDC')
```

## Computing the PoolId out of PoolKey[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#computing-the-poolid-out-of-poolkey "Direct link to Computing the PoolId out of PoolKey")
In this example, we will construct the **USDC - ETH** Pool with **LOW** fees and without hooks. The SDK provides a method to compute the `PoolId` for this pool:
```
import{ Pool }from'@uniswap/v4-sdk';const{currency0, currency1, fee, tickSpacing, hooks}= CurrentConfig.poolKey;const poolId = Pool.getPoolId(currency0, currency1, fee, tickSpacing, hooks);
```

## Referencing the StateView contract and fetching metadata[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#referencing-the-stateview-contract-and-fetching-metadata "Direct link to Referencing the StateView contract and fetching metadata")
Now that we have the `PoolId` of a **USDC - ETH** Pool, we need to call [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view) contract to get the pool state. In v4 you need to use `StateLibrary` to read pool state, but offchain systems—such as frontends or analytics services—require a deployed contract with view functions. This is where `StateView` comes in. To construct the Contract we need to provide the address of the contract, its ABI and a provider connected to an RPC endpoint.
```
import{ ethers }from'ethers'constSTATE_VIEW_ADDRESS='0x7ffe42c4a5deea5b0fec41c94c136cf115597227';// Replace with actual StateView contract addressconstSTATE_VIEW_ABI=[...];// Import or define the ABI for StateView contractconst provider =getProvider()// Provide the right RPC address for the chainconst stateViewContract =newethers.Contract(STATE_VIEW_ADDRESS,STATE_VIEW_ABI,  provider)
```

We get the `STATE_VIEW_ADDRESS` for our chain from [Uniswap Deployments](https://docs.uniswap.org/contracts/v4/deployments). Once we have set up our reference to the contract, we can proceed to access its methods. To construct our offchain representation of the Pool, we need to fetch its liquidity, sqrtPrice, currently active tick and the full Tick data. We get the **liquidity** , **sqrtPrice** and **tick** directly from the blockchain by calling `getLiquidity()`and `getSlot0()` on the StateView contract:
```
const[slot0, liquidity]=awaitPromise.all([  stateViewContract.getSlot0(poolId,{   blockTag: blockNum,}),  stateViewContract.getLiquidity(poolId,{   blockTag: blockNum,}),])
```

The [getSlot0 function](https://docs.uniswap.org/contracts/v4/guides/state-view#getting-pool-state) represents the first (0th) storage slot of the pool and exposes multiple useful values in a single function:
  * `sqrtPriceX96`: The current pool price in Q64.96 fixed-point format.
  * `tick`: The current tick in which the pool is operating.
  * `protocolFee` and `lpFee`: Fee parameters for protocol and LP fee tiers.


For our use case, we only need the `sqrtPriceX96` and the currently active `tick`.
## Fetching all Ticks[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#fetching-all-ticks "Direct link to Fetching all Ticks")
v4 pools use ticks to [concentrate liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity) in price ranges and allow for better pricing of trades. Even though most Pools only have a couple of **initialized ticks** , it is possible that a pools liquidity is defined by thousands of **initialized ticks**. In that case, it can be very expensive or slow to get all of them with normal RPC calls.
If you are not familiar with the concept of ticks, check out the [`introduction`](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity#ticks).
To access tick data, we will use the `getTickInfo` function of the State View contract:
```
functiongetTickInfo(PoolId poolId,int24 tick)externalviewreturns(uint128 liquidityGross,int128 liquidityNet,uint256 feeGrowthOutside0X128,uint256 feeGrowthOutside1X128)
```

The `tick` parameter that we provide the function with is the **index** (memory position) of the Tick we are trying to fetch. To get the indices of all initialized Ticks of the Pool, we can calculate them from the **tickBitmaps**. To fetch a `tickBitmap` we use a `getTickBitmap` function of the State View contract:
```
functiongetTickBitmap(   PoolId poolId,int16 wordPosition)externalviewreturns(uint256 tickBitmap)
```

A pool stores lots of bitmaps, each of which contain the status of 256 Ticks. The parameter `int16 wordPosition` the function accepts is the position of the bitMap we want to fetch. We can calculate all the position of bitMaps (or words as they are sometimes called) from the `tickSpacing` of the Pool, which is in turn dependant on the Fee tier.
So to summarise we need 4 steps to fetch all initialized ticks:
  1. Calculate all bitMap positions from the tickSpacing of the Pool.
  2. Fetch all bitMaps using their positions.
  3. Calculate the memory positions of all Ticks from the bitMaps.
  4. Fetch all Ticks by their memory position.


We will use multicalls for the fetch calls.
## Multicall[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#multicall "Direct link to Multicall")
Multicall contracts **aggregate results** from multiple contract calls and therefore allow sending multiple contract calls in **one RPC request**. This can improve the **speed** of fetching large amounts of data significantly and ensures that the data fetched is all from the **same block**.
We will use the Multicall2 contract by MakerDAO. We use the `ethers-muticall` npm package to easily interact with the Contract.
## Calculating all bitMap positions[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#calculating-all-bitmap-positions "Direct link to Calculating all bitMap positions")
As mentioned, Uniswap v4 Pools store **bitmaps** , also called _words_ , that represent the state of **256 initializable ticks** at a time. The value at a bit of a word is 1 if the tick at this index is initialized and 0 if it isn't. We can calculate the positions of initialized ticks from the **words** of the Pool.
All ticks of Uniswap v4 pools are between the indices `-887272` and `887272`. We can calculate the minimum and maximum word from these indices and the Pool's tickSpacing:
```
functiontickToWord(tick:number):number{let compressed = Math.floor(tick / tickSpacing)if(tick <0&& tick % tickSpacing !==0){  compressed -=1}return compressed >>8}const minWord =tickToWord(-887272)const maxWord =tickToWord(887272)
```

Ticks can only be initialized at indices that are **divisible by the tickSpacing**. One word contains 256 ticks, so we can compress the ticks by right shifting 8 bit.
## Fetching bitMaps from their position[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#fetching-bitmaps-from-their-position "Direct link to Fetching bitMaps from their position")
Knowing the positions of words, we can now fetch them using multicall.
First we initialize our multicall providers and State View Contract:
```
import{ ethers }from'ethers'import{ Contract, Provider }from'ethers-multicall'const ethersProvider =newethers.providers.JsonRpcProvider("YOUR_RPC_URL")const multicallProvider =newProvider(ethersProvider)await multicallProvider.init()const stateViewContract =newContract(STATE_VIEW_ADDRESS,STATE_VIEW_ABI)
```

The `multicallProvider` creates the multicall request and sends it via the ethers Provider.
Next we loop through all possible word positions and add a `getTickBitmap` call for each:
```
let calls:any[]=[]let wordPosIndices:number[]=[]for(let i = minWord; i <= maxWord; i++){ wordPosIndices.push(i) calls.push(stateViewContract.getTickBitmap(poolId, i))}
```

We also keep track of the word position indices to be able to loop through them in the same order we added the calls to the array.
We use the `multicallProvider.all()` function to send a multicall and map the results:
```
const results: bigint[]=(await multicallProvider.all(calls)).map((ethersResponse)=>{returnBigInt(ethersResponse.toString())})
```

A great visualization of what the bitMaps look like can be found in the [Uniswap v3 development book](<https://uniswapv3book.com/docs/milestone_2/tick-bitmap-index/>](<https://uniswapv3book.com/milestone_2/tick-bitmap-index.html>):
![TickBitmap](https://docs.uniswap.org/assets/images/tickBitmap_cut-0657ffe617e53a11e38397e09300fe73.png)
We encourage anyone trying to get a deeper understanding of the Uniswap protocol to read the Book.
## Calculating the memory positions of all Ticks[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#calculating-the-memory-positions-of-all-ticks "Direct link to Calculating the memory positions of all Ticks")
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
## Fetching all Ticks by their indices[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#fetching-all-ticks-by-their-indices "Direct link to Fetching all Ticks by their indices")
We use the multicallProvider again to execute an aggregated read call for all tick indices. We create an array of call Promises again and use `.all()` to make our multicall:
```
const calls:any[]=[]for(const index of tickIndices){ calls.push(stateViewContract.getTickInfo(poolId, index))}const results =await multicallProvider.all(calls)
```

Again, the order of the results array is the same as the elements in **tickIndices**.
We are able to combine the **tickIndices** and **results** array to create an array of `Tick` objects:
```
const allTicks: Tick[]=[]for(let i =0; i < tickIndices.length; i++){const index = tickIndices[i]const ethersResponse = results[i]const tick =newTick({   index,   liquidityGross:JSBI.BigInt(ethersResponse.liquidityGross.toString()),   liquidityNet:JSBI.BigInt(ethersResponse.liquidityNet.toString()),})  allTicks.push(tick)}
```

We need to parse the response from our RPC provider to JSBI values that the v4-sdk can work with.
## Constructing the Pool[​](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#constructing-the-pool "Direct link to Constructing the Pool")
We have everything to construct our `Pool` now:
```
const usdcWethPool =newPool(USDC,WETH,  feeAmount,  slot0.sqrtPriceX96,  liquidity,  slot0.tick,  allTicks)
```

With this fully initialized Pool, we can make accurate offchain calculations.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/advanced/01-pool-data.md)
Was this helpful?
[PreviousAdding and Removing Liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity)[NextCreate Pool](https://docs.uniswap.org/sdk/v4/guides/advanced/create-pool)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#introduction)
  * [Configuration](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#configuration)
  * [Computing the PoolId out of PoolKey](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#computing-the-poolid-out-of-poolkey)
  * [Referencing the StateView contract and fetching metadata](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#referencing-the-stateview-contract-and-fetching-metadata)
  * [Fetching all Ticks](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#fetching-all-ticks)
  * [Multicall](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#multicall)
  * [Calculating all bitMap positions](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#calculating-all-bitmap-positions)
  * [Fetching bitMaps from their position](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#fetching-bitmaps-from-their-position)
  * [Calculating the memory positions of all Ticks](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#calculating-the-memory-positions-of-all-ticks)
  * [Fetching all Ticks by their indices](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#fetching-all-ticks-by-their-indices)
  * [Constructing the Pool](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data#constructing-the-pool)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/advanced/01-pool-data.md)
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
