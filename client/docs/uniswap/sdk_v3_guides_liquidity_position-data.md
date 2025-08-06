# https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data

[Skip to main content](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
        * [Position Management](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
          * [Liquidity Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
          * [Minting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting)
          * [Fetching Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
          * [Adding & Removing Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position)
          * [Collecting Fees](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
          * [Swapping and Adding Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
    * [web3-react](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Guides
  * Pooling Liquidity
  * [Liquidity Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)


On this page
# Liquidity Positions
## Introduction[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#introduction "Direct link to Introduction")
This guide will introduce us to **liquidity positions** in Uniswap V3 and present the `v3-sdk` classes and Contracts used to interact with the protocol. The concepts and code snippets showcased here can be found across the **Pooling Liquidity** examples in the Uniswap code examples [repository](https://github.com/Uniswap/examples).
In this guide, we will take a look at the [Position](https://docs.uniswap.org/sdk/v3/reference/classes/Position) and [NonfungiblePositionManager](https://docs.uniswap.org/sdk/v3/reference/classes/NonfungiblePositionManager) classes, as well as the [NonfungiblePositionManager Contract](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager).
At the end of the guide, we should be familiar with the most important classes used to interact with liquidity positions. We should also understand how to fetch positions from the **NonfungiblePositionManager Contract**.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v3-sdk`](https://www.npmjs.com/package/@uniswap/v3-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)
  * [`@uniswap/v3-periphery`](https://www.npmjs.com/package/@uniswap/v3-periphery)


The code mentioned in this guide can be found across the [minting Position](https://github.com/Uniswap/examples/blob/main/v3-sdk/minting-position/src), [collecting Fees](https://github.com/Uniswap/examples/blob/main/v3-sdk/collecting-fees/src), [modifying positions](https://github.com/Uniswap/examples/blob/d34a53412dbf905802da2249391788a225719bb8/v3-sdk/modifying-position/src) and [swap and add liquidity](https://github.com/Uniswap/examples/blob/main/v3-sdk/swap-and-add-liquidity/src) examples.
## Prerequisites[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#prerequisites "Direct link to Prerequisites")
To understand what Positions are, we need to understand some underlying concepts of the Uniswap protocol.
Consider checking out the [Concepts section](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity) as well as the [Uniswap Book](https://uniswapv3book.com/docs/introduction/uniswap-v3/).
### Concentrated liquidity[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#concentrated-liquidity "Direct link to Concentrated liquidity")
Uniswap V3 Pools use concentrated liquidity to allow a denser concentration of liquidity at specific prices. Compared to the full range liquidity model Uniswap V2 uses, this allows traders to make larger trades with less price impact. Liquidity providers can choose a specific price range in which they want their liquidity to be used by trades.
To achieve this, Uniswap V3 Pools discriminate the price range with **Ticks**.
### Ticks[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#ticks "Direct link to Ticks")
Ticks are the boundaries between discrete price ranges. A change of 1 Tick always represents a price change of 0.01% from the current price. Uniswap V3 Pools can have different `tickSpacings`, a constant that describes which ticks can be used by the Pool. Only ticks at indices that are divisible by the tickSpacing can be initialized. This value is dependant on the fee of the Pool, Pools with higher fees have higher tickSpacing.
For example, a Pool with **HIGH** fee (1%) has a tickSpacing of 200, meaning the price difference between initializable Ticks is:
$$1.0001^200 = 1.0202$$ or $$2.02$$%
### Liquidity Positions[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#liquidity-positions "Direct link to Liquidity Positions")
When someone provides liquidity to a Pool, they create a **Liquidity Position**. This position is defined by the amount of liquidity provided and the start tick and the end tick, or price range, of the Position.
Because V3 Pools allow users to choose any price range in which they want to provide liquidity, it is possible to create positions that do not contain the current Price of the Pool. In this case, the liquidity provider will pay only one type of Token into the Pool, creating a **single side liquidity position**.
To learn more about how Ticks and Liquidity positions work, consider reading the [whitepaper](https://uniswap.org/whitepaper-v3.pdf) or the other resources mentioned above.
Now that we have a rough understanding of liquidity positions in Uniswap V3, let's look at the correspondent classes the SDK offers us.
## Position class[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#position-class "Direct link to Position class")
The **sdk** provides a [`Position`](https://github.com/Uniswap/v3-sdk/blob/main/src/entities/position.ts) class used to create local representations of an onchain position. It is used to create the calldata for onchain calls to mint or modify an onchain position.
There are four ways to construct a position.
Directly with the [constructor](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/position.ts#L40):
```
import{ Pool, Position }from'@uniswap/v3-sdk'importJSBIfrom'jsbi'const pool =newPool(...)const tickLower:number=-100const tickUpper:number=200const liquidity:JSBI=JSBI.BigInt('1000000000000000000')const position =newPosition({  pool,  liquidity,  tickLower,  tickUpper})
```

Using the [`fromAmounts()`](https://github.com/Uniswap/v3-sdk/blob/08a7c05/src/entities/position.ts#L312) function:
```
import{ BigIntish }from'@uniswap/sdk-core'const pool =newPool(...)const tickLower:number=-100const tickUpper:number=200const amount0: BigIntish ='1000000000000000000'const amount1: BigIntish =JSBI.BigInt('1000000000000000000')const useFullPrecision:boolean=trueconst position = Position.fromAmounts({  pool,  tickLower,  tickUpper,  amount0,  amount1,  useFullPrecision})
```

Or using the [`fromAmount0()`](https://github.com/Uniswap/v3-sdk/blob/08a7c050cba00377843497030f502c05982b1c43/src/entities/position.ts#L354) or [`fromAmount1()`](https://github.com/Uniswap/v3-sdk/blob/08a7c050cba00377843497030f502c05982b1c43/src/entities/position.ts#L378) functions:
```
import{ BigIntish }from'@uniswap/sdk-core'...const pool =newPool(...)const tickLower:number=-200const tickUpper:number=100const amount0: BigIntish ='1000000000000000000'const useFullPrecision:boolean=trueconst singleSidePositionToken0 = Position.fromAmount0({  pool,  tickLower,  tickUpper,  amount0,  useFullPrecision})const amount1: BigIntish =100000000const singleSidePositionToken1 = Position.fromAmount1({  pool,  tickLower,  tickUpper,  amount1,  useFullPrecision})
```

These last two functions calculate a position at the given tick range given the amount of `token0` or `token1`. The amount of the second token is calculated from the ratio of the tokens inside the tick range and the amount of token one.
A create transaction would then fail if the wallet doesn't hold enough `token1` or the Contract is not given the necessary **Transfer Approval**.
All of these functions take an Object with **named values** as a call parameter. The amount and liquidity values are of type `BigIntish` which accepts `number`, `string` and `JSBI`.
The values of `tickLower` and `tickUpper` must match **initializable ticks** of the Pool.
## NonfungiblePositionManager[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#nonfungiblepositionmanager "Direct link to NonfungiblePositionManager")
The `NonfungiblePositionManager` class is mainly used to create calldata for functions on the **NonfungiblePositionManager Contract**.
We will look at the **sdk** class and write functions on the Contract in this section.
### Creating a Position[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#creating-a-position "Direct link to Creating a Position")
To create a position on a Pool, the [`mint`](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager#mint) function is called on the Contract. The **sdk** class provides the `addCallParameters` function to create the calldata for the transaction:
```
import{ MintOptions, NonfungiblePositionManager }from'@uniswap/v3-sdk'const mintOptions: MintOptions ={ recipient: address, deadline: Math.floor(Date.now()/1000)+60*20, slippageTolerance:newPercent(50,10_000),}// get calldata for minting a positionconst{ calldata, value }= NonfungiblePositionManager.addCallParameters( positionToMint, mintOptions)
```

This call creates a position if it doesn't exist, but can also be used to increase an existing position. Take a look at the [Mint Position guide](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting) and [Modify Position guide](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position) to learn more.
### Decreasing and Increasing a Position[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#decreasing-and-increasing-a-position "Direct link to Decreasing and Increasing a Position")
To decrease or increase the liquidity of a Position, the `decreaseLiquidity` or `increaseLiquidity` functions are called on the Contract. To increase, `addCallParameters` is used as mentioned above, to decrease we use `removeCallParameters`:
```
const{ calldata, value }= NonfungiblePositionManager.removeCallParameters( currentPosition, removeLiquidityOptions)
```

Take a look at the [Modify Positions guide](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position) to learn how to create the `currentPosition` and `removeLiquidityOptions` parameters.
### Collecting Fees[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#collecting-fees "Direct link to Collecting Fees")
To collect fees accrued, the `collect` function is called on the Contract. The **sdk class** provides the `collectCallParameters` function to create the calldata for that:
```
const{ calldata, value }= NonfungiblePositionManager.collectCallParameters(collectOptions)
```

## Next steps[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#next-steps "Direct link to Next steps")
Now that you are familiar with the most important classes and Contract to interact with Liquidity Positions, continue with the next guide on [Minting Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/01-position-data.md)
Was this helpful?
[PreviousRouting a Swap](https://docs.uniswap.org/sdk/v3/guides/swaps/routing)[NextMinting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#introduction)
  * [Prerequisites](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#prerequisites)
    * [Concentrated liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#concentrated-liquidity)
    * [Ticks](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#ticks)
    * [Liquidity Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#liquidity-positions)
  * [Position class](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#position-class)
  * [NonfungiblePositionManager](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#nonfungiblepositionmanager)
    * [Creating a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#creating-a-position)
    * [Decreasing and Increasing a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#decreasing-and-increasing-a-position)
    * [Collecting Fees](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#collecting-fees)
  * [Next steps](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/01-position-data.md)
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
