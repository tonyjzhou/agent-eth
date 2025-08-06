# https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add

[Skip to main content](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Position Management](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
          * [Liquidity Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
          * [Minting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting)
          * [Fetching Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
          * [Adding & Removing Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position)
          * [Collecting Fees](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
          * [Swapping and Adding Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
    * [web3-react](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Guides
  * Pooling Liquidity
  * [Swapping and Adding Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)


On this page
# Swapping and Adding Liquidity
## Introduction[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#introduction "Direct link to Introduction")
This guide will cover how to execute a swap-and-add operation in a single atomic transaction. It is based on the [swap-and-add example](https://github.com/Uniswap/examples/tree/main/v3-sdk/swap-and-add-liquidity), found in the Uniswap code examples [repository](https://github.com/Uniswap/examples). To run this example, check out the examples's [README](https://github.com/Uniswap/examples/tree/main/v3-sdk/swap-and-add-liquidity) and follow the setup instructions.
info
If you need a briefer on the SDK and to learn more about how these guides connect to the examples repository, please visit our [background](https://docs.uniswap.org/sdk/v3/guides/background) page!
When adding liquidity to a Uniswap v3 pool, you must provide two assets in a particular ratio. In many cases, your contract or the user's wallet hold a different ratio of those two assets. In order to deposit 100% of your assets, you must first swap your assets to the optimal ratio and then add liquidity.
However, the swap may shift the balance of the pool and thus change the optimal ratio. To avoid that, we can execute this swap-and-add liquidity operation in an atomic fashion, using a router. The inputs to our guide are the **two tokens** that we are pooling for, the **amount** of each token we are pooling for, the **amount** of each token to swap-and-add, and the Pool **fee**.
The guide will **cover** :
  1. Setup a router instance
  2. Configuring our ratio calculation
  3. Calculating our currency ratio
  4. Constructing and executing our swap-and-add transaction


At the end of the guide, given the inputs above, we should be able swap-and-add liquidity using 100% of the input assets with the press of a button and see the change reflected in our position and the balance of our tokens.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v3-sdk`](https://www.npmjs.com/package/@uniswap/v3-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)
  * [`@uniswap/smart-order-router`](https://www.npmjs.com/package/@uniswap/smart-order-router)


The core code of this guide can be found in [`swapAndAddLiquidity()`](https://github.com/Uniswap/examples/blob/main/v3-sdk/swap-and-add-liquidity/src/libs/liquidity.ts#L48).
note
This guide assumes you are familiar with our [Minting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/01-minting-position.md) guide. A minted position is required to add or remove liquidity from, so the buttons will be disabled until a position is minted.
Also note that we do not need to give approval to the `NonfungiblePositionManager` to transfer our tokens as we will have already done that when minting our position.
## Setup a router instance[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#setup-a-router-instance "Direct link to Setup a router instance")
The first step is to approve the `SwapRouter` smart contract to spend our tokens for us in order for us to add liquidity to our position:
```
const tokenInApproval =awaitgetTokenTransferApproval( token0,V3_SWAP_ROUTER_ADDRESS)const tokenOutApproval =awaitgetTokenTransferApproval( token1,V3_SWAP_ROUTER_ADDRESS)
```

We described the `getTokenTransferApproval` function [here](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#giving-approval-to-transfer-our-tokens).
Then we can setup our router, the [`AlphaRouter`](https://github.com/Uniswap/smart-order-router/blob/97c1bb7cb64b22ebf3509acda8de60c0445cf250/src/routers/alpha-router/alpha-router.ts#L333), which is part of the [smart-order-router package](https://www.npmjs.com/package/@uniswap/smart-order-router). The router requires a `chainId` and a `provider` to be initialized. Note that routing is not supported for local forks, so we will use a mainnet provider even when swapping on a local fork:
```
import{ ethers }from'ethers'import{ AlphaRouter }from'@uniswap/smart-order-router'const provider =newethers.providers.JsonRpcProvider(rpcUrl)const router =newAlphaRouter({ chainId:1, provider })
```

For a more detailed example, check out our [routing guide](https://docs.uniswap.org/sdk/v3/guides/trading/03-routing.md).
## Configuring our ratio calculation[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#configuring-our-ratio-calculation "Direct link to Configuring our ratio calculation")
Having created the router, we now need to construct the parameters required to make a call to its `routeToRatio` function, which will ensure the ratio of currency used matches the pool's required ratio to add our total liquidity. This will require the following parameters:
The first two parameters are the currency amounts we use as input to the `routeToRatio` algorithm:
```
import{ CurrencyAmount }from'@uniswap/sdk-core'const token0CurrencyAmount = CurrencyAmount.fromRawAmount( token0,fromReadableAmount(  token0AmountToAdd,  token0.decimals))const token1CurrencyAmount = CurrencyAmount.fromRawAmount( token1,fromReadableAmount(  token1AmountToAdd,  token1.decimals))
```

Next, we will create a placeholder position with a liquidity of `1` since liquidity is still unknown and will be set inside the call to `routeToRatio`:
```
import{ Pool, Position, nearestUsableTick }from'@uniswap/v3-sdk'const placeholderPosition =newPosition{  pool,  liquidity:1,  tickLower:nearestUsableTick(pool.tickCurrent, pool.tickSpacing)-   pool.tickSpacing *2,  tickUpper:nearestUsableTick(pool.tickCurrent, pool.tickSpacing)+   poolInfo.tickSpacing *2}
```

We then need to create an instance of `SwapAndAddConfig` which will set additional configuration parameters for the `routeToRatio` algorithm:
  * `ratioErrorTolerance` determines the margin of error the resulting ratio can have from the optimal ratio.
  * `maxIterations` determines the maximum times the algorithm will iterate to find a ratio within error tolerance. If max iterations is exceeded, an error is returned. The benefit of running the algorithm more times is that we have more chances to find a route, but more iterations will longer to execute. We've used a default of 6 in our example.


```
import{ Fraction }from'@uniswap/sdk-core'import{ SwapAndAddConfig }from'@uniswap/smart-order-router'const swapAndAddConfig: SwapAndAddConfig ={ ratioErrorTolerance:newFraction(1,100), maxIterations:6,}
```

Finally, we will create an instance of `SwapAndAddOptions` to configure which position we are adding liquidity to and our defined swapping parameters in two different objects:
  * **`swapConfig`**configures the`recipient` of leftover dust from swap, `slippageTolerance` and a `deadline` for the swap.
  * **`addLiquidityOptions`**must contain a`tokenId` to add to an existing position


```
import{ SwapAndAddOptions }from'@uniswap/smart-order-router'const swapAndAddOptions: SwapAndAddOptions ={ swapOptions:{  type: SwapType.SWAP_ROUTER_02,  recipient: address,  slippageTolerance:newPercent(50,10_000),  deadline: Math.floor(Date.now()/1000)+60*20,}, addLiquidityOptions:{  tokenId: positionId,},}
```

## Calculating our currency ratio[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#calculating-our-currency-ratio "Direct link to Calculating our currency ratio")
Having constructed all the parameters we need to call `routeToRatio`, we can now make the call to the function:
```
import{ SwapToRatioResponse }from'@uniswap/smart-order-router'const routeToRatioResponse: SwapToRatioResponse =await router.routeToRatio( token0CurrencyAmount, token1CurrencyAmount, currentPosition, swapAndAddConfig, swapAndAddOptions)
```

The return type of the function call is [SwapToRatioResponse](https://github.com/Uniswap/smart-order-router/blob/97c1bb7cb64b22ebf3509acda8de60c0445cf250/src/routers/router.ts#L121). If a route was found successfully, this object will have two fields: the status (success) and the `SwapToRatioRoute` object. We check to make sure that both of those conditions hold true before we construct and submit the transaction:
```
import{ SwapToRatioStatus }from'@uniswap/smart-order-router'if(!routeToRatioResponse || routeToRatioResponse.status !== SwapToRatioStatus.SUCCESS){// Handle Failed Transaction}
```

In case a route was not found, we return from the function a `Failed` state for the transaction.
## Constructing and executing our swap-and-add transaction[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#constructing-and-executing-our-swap-and-add-transaction "Direct link to Constructing and executing our swap-and-add transaction")
After making sure that a route was successfully found, we can now construct and send the transaction. The response (`SwapToRatioRoute`) will have the properties we need to construct our transaction object:
```
import{ SwapToRatioRoute }from'@uniswap/smart-order-router'const route: SwapToRatioRoute = routeToRatioResponse.resultconst transaction ={ data: route.methodParameters?.calldata, to:V3_SWAP_ROUTER_ADDRESS, value: route.methodParameters?.value, from: address,}const txRes =await wallet.sendTransaction(transaction)
```

If the transaction was successful, our swap-and-add will be completed! We should see our input token balances decrease and our position balance should be increased accordingly.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/06-swap-and-add-liquidity.md)
Was this helpful?
[PreviousCollecting Fees](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)[NextIntroduction](https://docs.uniswap.org/sdk/v3/guides/advanced/introduction)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#introduction)
  * [Setup a router instance](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#setup-a-router-instance)
  * [Configuring our ratio calculation](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#configuring-our-ratio-calculation)
  * [Calculating our currency ratio](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#calculating-our-currency-ratio)
  * [Constructing and executing our swap-and-add transaction](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add#constructing-and-executing-our-swap-and-add-transaction)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/06-swap-and-add-liquidity.md)
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
