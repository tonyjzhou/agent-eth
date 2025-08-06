# https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping

[Skip to main content](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
          * [Getting a Quote](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
          * [Executing a Single-Hop Swap](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping)
          * [Executing Multi-Hop Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
    * [web3-react](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Guides
  * Swaps
  * [Executing Multi-Hop Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)


On this page
# Executing Multi-Hop Swaps
## Introduction[​](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#introduction "Direct link to Introduction")
This guide demonstrates how to execute multi-hop swaps on Uniswap V4, allowing you to trade between tokens that might not share a direct pool. Multi-hop swaps route through multiple pools to achieve the desired token exchange, often providing better pricing than attempting direct swaps through less liquid pools.
Building on our [single-hop swap guide](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping), this guide will show you how to construct routing paths and execute them efficiently.
The guide will cover:
  1. Constructing swap paths through multiple pools
  2. Executing the multi-hop swap


At the end of this guide, you should be able to execute swaps between any two tokens using optimal routing through multiple pools.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v4-sdk`](https://www.npmjs.com/package/@uniswap/v4-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)
  * [`@uniswap/universal-router-sdk`](https://www.npmjs.com/package/@uniswap/universal-router-sdk)


## Constructing swap paths through multiple pools[​](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#constructing-swap-paths-through-multiple-pools "Direct link to Constructing swap paths through multiple pools")
Let's first define a multi-hop swap configuration. In this example, we'll swap ETH → USDC → USDT. The configuration follows closely from the quoting and single-hop swapping guides.
```
import{ SwapExactIn, PoolKey }from'@uniswap/v4-sdk'import{ETH_TOKEN,USDC_TOKEN,USDT_TOKEN}from'./constants'constETH_USDC_POOL_KEY: PoolKey ={  currency0:ETH_TOKEN.address,  currency1:USDC_TOKEN.address,  fee:3000,  tickSpacing:60,  hooks:"0x0000000000000000000000000000000000000000",};constUSDC_USDT_POOL_KEY: PoolKey ={  currency0:USDC_TOKEN.address,  currency1:USDT_TOKEN.address,  fee:10,  tickSpacing:1,  hooks:"0x0000000000000000000000000000000000000000",};exportconst CurrentConfig: SwapExactIn ={  currencyIn:ETH_TOKEN.address,  path:encodeMultihopExactInPath([ETH_USDC_POOL_KEY,USDC_USDT_POOL_KEY],ETH_TOKEN.address),  amountIn: ethers.utils.parseUnits('1',ETH_TOKEN.decimals).toString(),  amountOutMinimum:"minAmountOut",// Change according to the slippage desired}
```

Uniswap V4 uses a specific format for encoding multi-hop paths. Each hop in the path requires:
```
typePathKey={  intermediateCurrency:string;  fee:number;  tickSpacing:number;  hooks:string;  hookData:string;};
```

We can encode the path using a function like:
```
exportfunctionencodeMultihopExactInPath( poolKeys: PoolKey[], currencyIn:string): PathKey[]{const pathKeys: PathKey[]=[]let currentCurrencyIn = currencyInfor(let i =0; i < poolKeys.length; i++){// Determine the output currency for this hopconst currencyOut = currentCurrencyIn === poolKeys[i].currency0? poolKeys[i].currency1: poolKeys[i].currency0// Create path key for this hopconst pathKey: PathKey ={   intermediateCurrency: currencyOut,   fee: poolKeys[i].fee,   tickSpacing: poolKeys[i].tickSpacing,   hooks: poolKeys[i].hooks,   hookData:'0x'}  pathKeys.push(pathKey)  currentCurrencyIn = currencyOut // Output becomes input for next hop}return pathKeys}
```

## Executing the multi-hop swap[​](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#executing-the-multi-hop-swap "Direct link to Executing the multi-hop swap")
We'll use the same contract addresses and ABIs from the single-hop guide and construct the **ethers** `Contract` for them:
```
constUNIVERSAL_ROUTER_ADDRESS="0xf70536B3bcC1bD1a972dc186A2cf84cC6da6Be5D"constPERMIT2_ADDRESS="0x000000000022D473030F116dDEE9F6B43aC78BA3"// ABIs remain the same as in single-hop guideconstUNIVERSAL_ROUTER_ABI=[/* ... */]constERC20_ABI=[/* ... */]constPERMIT2_ABI=[/* ... */]
```

The main function for executing multi-hop swaps is very similar to the single-hop guide as well. The only difference is that the first action to the Universal Router is `SWAP_EXACT_IN` instead of `SWAP_EXACT_IN_SINGLE`.
```
import{ Actions, V4Planner }from'@uniswap/v4-sdk'import{ CommandType, RoutePlanner }from'@uniswap/universal-router-sdk'const v4Planner =newV4Planner()const routePlanner =newRoutePlanner()const deadline = Math.floor(Date.now()/1000)+3600v4Planner.addAction(Actions.SWAP_EXACT_IN,[CurrentConfig]);v4Planner.addAction(Actions.SETTLE_ALL,[ETH_USDC_POOL_KEY.currency0, CurrentConfig.amountIn]);v4Planner.addAction(Actions.TAKE_ALL,[USDC_USDT_POOL_KEY.currency1, CurrentConfig.amountOutMinimum]);const encodedActions = v4Planner.finalize()routePlanner.addCommand(CommandType.V4_SWAP,[v4Planner.actions, v4Planner.params])// Only needed for native ETH as input currency swapsconst txOptions:any={  value: CurrentConfig.amountIn}const tx =await universalRouter.execute( routePlanner.commands,[encodedActions], deadline, txOptions)const receipt =await tx.wait()console.log('Multi-hop swap completed! Transaction hash:', receipt.transactionHash)
```

The token approvals for ERC20 token swaps remain the same as the [single-hop swapping guide](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping).
## Next Steps[​](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#next-steps "Direct link to Next Steps")
Now that you're familiar with trading, consider checking out our next guides on [pooling liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/01-pool-data.md) to Uniswap!
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/swaps/03-multi-hop-swapping.md)
Was this helpful?
[PreviousExecuting a Single-Hop Swap](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping)[NextMinting a position](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#introduction)
  * [Constructing swap paths through multiple pools](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#constructing-swap-paths-through-multiple-pools)
  * [Executing the multi-hop swap](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#executing-the-multi-hop-swap)
  * [Next Steps](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/swaps/03-multi-hop-swapping.md)
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
