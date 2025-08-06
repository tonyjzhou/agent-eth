# https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping

[Skip to main content](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#__docusaurus_skipToContent_fallback)
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
  * [Executing a Single-Hop Swap](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping)


On this page
# Executing a Single-Hop Swap
## Introduction[​](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#introduction "Direct link to Introduction")
This guide will build off our [quoting guide](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting) and show how to use a quote to construct and execute a trade on the Uniswap v4 protocol. In this example we will trade between two tokens: **ETH and USDC**.
The guide will cover:
  1. Setting up swap parameters and pool configuration
  2. Using Universal Router and executing a single-hop swap


At the end of this guide, you should be able to execute swaps between any two tokens using a single pool on Uniswap V4.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v4-sdk`](https://www.npmjs.com/package/@uniswap/v4-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)
  * [`@uniswap/universal-router-sdk`](https://www.npmjs.com/package/@uniswap/universal-router-sdk)


## Setting up Swap Configuration[​](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#setting-up-swap-configuration "Direct link to Setting up Swap Configuration")
First, let's define our swap configuration. We will use the same pool structure from the quoting guide:
```
import{ SwapExactInSingle }from'@uniswap/v4-sdk'import{USDC_TOKEN,ETH_TOKEN}from'./constants'exportconst CurrentConfig: SwapExactInSingle ={  poolKey:{    currency0:ETH_TOKEN.address,    currency1:USDC_TOKEN.address,    fee:500,    tickSpacing:10,    hooks:"0x0000000000000000000000000000000000000000",},  zeroForOne:true,  amountIn: ethers.utils.parseUnits('1',ETH_TOKEN.decimals).toString(),  amountOutMinimum:"minAmountOut",// Change according to the slippage desired  hookData:'0x00'}
```

Like the quoting guide, the pool used is defined by a pair of tokens in `constants.ts`. You can change these two tokens and the other pool parameters in the config as long as a pool actually exists for that configuration.
## Using Universal Router and executing a single-hop swap[​](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#using-universal-router-and-executing-a-single-hop-swap "Direct link to Using Universal Router and executing a single-hop swap")
The Universal Router is a flexible, gas-efficient contract designed to execute complex swap operations across various protocols, including Uniswap v4. It serves as an intermediary between users and the Uniswap v4 PoolManager, handling the intricacies of swap execution.
So, we construct an instance of an **ethers** `Contract` for the Universal Router contract in order to interact with it:
```
constUNIVERSAL_ROUTER_ADDRESS="0x66a9893cc07d91d95644aedd05d03f95e1dba8af"// Change the Universal Router address as per the chainconstUNIVERSAL_ROUTER_ABI=[{  inputs:[{ internalType:"bytes", name:"commands", type:"bytes"},{ internalType:"bytes[]", name:"inputs", type:"bytes[]"},{ internalType:"uint256", name:"deadline", type:"uint256"},],  name:"execute",  outputs:[],  stateMutability:"payable",  type:"function",},]const universalRouter =newethers.Contract(UNIVERSAL_ROUTER_ADDRESS,UNIVERSAL_ROUTER_ABI,  signer)
```

We can get the `UNIVERSAL_ROUTER_ADDRESS` for our chain from [Uniswap Deployments](https://docs.uniswap.org/contracts/v4/deployments).
A signer object can be created like this:
```
const provider =newethers.providers.JsonRpcProvider("RPC");const signer =newethers.Wallet("YOUR PRIVATE KEY", provider);
```

Now, let's implement the main function that handles the swap. When encoding a swap command for the Universal Router, we need to choose between two types of swaps:
  * Exact Input Swaps: Use this swap-type when you know the exact amount of tokens you want to swap in, and you're willing to accept any amount of output tokens above your minimum. This is common when you want to sell a specific amount of tokens.
  * Exact Output Swaps: Use this swap-type when you need a specific amount of output tokens, and you're willing to spend up to a maximum amount of input tokens. This is useful when you need to acquire a precise amount of tokens, for example, to repay a loan or meet a specific requirement.


We will be doing an Exact Input swap in this example.
```
import{ Actions, V4Planner }from'@uniswap/v4-sdk'import{ CommandType, RoutePlanner }from'@uniswap/universal-router-sdk'const v4Planner =newV4Planner()const routePlanner =newRoutePlanner()// Set deadline (1 hour from now)const deadline = Math.floor(Date.now()/1000)+3600v4Planner.addAction(Actions.SWAP_EXACT_IN_SINGLE,[CurrentConfig]);v4Planner.addAction(Actions.SETTLE_ALL,[CurrentConfig.poolKey.currency0, CurrentConfig.amountIn]);v4Planner.addAction(Actions.TAKE_ALL,[CurrentConfig.poolKey.currency1, CurrentConfig.amountOutMinimum]);const encodedActions = v4Planner.finalize()routePlanner.addCommand(CommandType.V4_SWAP,[v4Planner.actions, v4Planner.params])// Only needed for native ETH as input currency swapsconst txOptions:any={  value: CurrentConfig.amountIn}const tx =await universalRouter.execute(  routePlanner.commands,[encodedActions],  deadline,  txOptions)const receipt =await tx.wait()console.log('Swap completed! Transaction hash:', receipt.transactionHash)
```

The actions in the planner define the sequence of operations that will be performed in our v4 swap:
  * `SWAP_EXACT_IN_SINGLE`: This action specifies that we want to perform an exact input swap using a single pool.
  * `SETTLE_ALL`: This action ensures all input tokens involved in the swap are properly paid. This is part of v4's settlement pattern for handling token transfers.
  * `TAKE_ALL`: This final action collects all output tokens after the swap is complete.


The sequence of these actions is important as they define the complete flow of our swap operation from start to finish.
The `V4_SWAP` command tells the Universal Router that we want to perform a swap on a Uniswap v4 pool.
## Handling Token Approvals for ERC20 Swaps[​](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#handling-token-approvals-for-erc20-swaps "Direct link to Handling Token Approvals for ERC20 Swaps")
When swapping ERC20 tokens, we need to set up approvals through Permit2. So, we construct an instance of an **ethers** `Contract` for the Permit2 contract in order to interact with it:
```
const permit2Contract =newethers.Contract(PERMIT2_ADDRESS,PERMIT2_ABI,  signer)
```

Create a similar one for the ERC20 token contract. If enough allowances have not been provided or the deadline has expired, we first need to approve Permit2 as a spender on the ERC20 token and then approve the Universal Router on Permit2.
```
const tx1 =await erc20Contract.approve(PERMIT2_ADDRESS, ethers.constants.MaxUint256)const tx2 =await permit2Contract.approve( tokenAddress,UNIVERSAL_ROUTER_ADDRESS, ethers.BigNumber.from(2).pow(160).sub(1),// MAX_UINT160 deadline)
```

The rest of the swap process remains the same.
## Next Steps[​](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#next-steps "Direct link to Next Steps")
Now that you understand single-hop swaps, you might want to explore [multi-hop swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/03-multihop-swap.md) for trading between tokens without direct pools or enough liquidity.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/swaps/02-single-hop-swapping.md)
Was this helpful?
[PreviousGetting a Quote](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)[NextExecuting Multi-Hop Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/multi-hop-swapping)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#introduction)
  * [Setting up Swap Configuration](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#setting-up-swap-configuration)
  * [Using Universal Router and executing a single-hop swap](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#using-universal-router-and-executing-a-single-hop-swap)
  * [Handling Token Approvals for ERC20 Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#handling-token-approvals-for-erc20-swaps)
  * [Next Steps](https://docs.uniswap.org/sdk/v4/guides/swaps/single-hop-swapping#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/swaps/02-single-hop-swapping.md)
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
