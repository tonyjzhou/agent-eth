# https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees

[Skip to main content](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
        * [Position Management](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
          * [Liquidity Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
          * [Minting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting)
          * [Fetching Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
          * [Adding & Removing Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position)
          * [Collecting Fees](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
          * [Swapping and Adding Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
    * [web3-react](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Guides
  * Pooling Liquidity
  * [Collecting Fees](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)


On this page
# Collecting Fees
## Introduction[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#introduction "Direct link to Introduction")
This guide will cover how to collect fees from a liquidity position on the Uniswap V3 protocol. It is based on the [collecting fees code example](https://github.com/Uniswap/examples/tree/main/v3-sdk/collecting-fees), found in the Uniswap code examples [repository](https://github.com/Uniswap/examples). To run this example, check out the examples's [README](https://github.com/Uniswap/examples/blob/main/v3-sdk/collecting-fees/README.md) and follow the setup instructions.
info
If you need a briefer on the SDK and to learn more about how these guides connect to the examples repository, please visit our [background](https://docs.uniswap.org/sdk/v3/guides/background) page!
In the Uniswap V3 protocol, liquidity positions are represented using non-fungible tokens. In this guide we will use the `NonfungiblePositionManager` class to help us mint a liquidity position for the **USDC - DAI** pair. We will then attempt to collect any fees that the position has accrued from those trading against our provisioned liquidity. The inputs to our guide are the **two tokens** that we are pooling for, the **amount** of each token we are pooling for, the Pool **fee** and the **max amount of accrued fees** we want to collect for each token.
The guide will **cover** :
  1. Setting up our fee collection
  2. Submitting our fee collection transaction


At the end of the guide, given the inputs above, we should be able to collect the accrued fees (if any) of a minted position with the press of a button and see the change reflected in our position and the balance of our tokens.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v3-sdk`](https://www.npmjs.com/package/@uniswap/v3-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


The core code of this guide can be found in [`collectFees()`](https://github.com/Uniswap/examples/blob/main/v3-sdk/collecting-fees/src/libs/liquidity.ts#L35).
note
This guide assumes you are familiar with our [Minting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/01-minting-position.md) guide. A minted position is required to add or remove liquidity from, so the buttons will be disabled until a position is minted.
Also note that we do not need to give approval to the `NonfungiblePositionManager` to transfer our tokens as we will have already done that when minting our position.
## Setting up our fee collection[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#setting-up-our-fee-collection "Direct link to Setting up our fee collection")
All of the fee collecting logic can be found in the [`collectFees`](https://github.com/Uniswap/examples/blob/be67e7df220b0a270c9d18bbaab529e017213adf/v3-sdk/collecting-fees/src/example/Example.tsx#L24) function. Notice how the **Collect Fees** button is disabled until a position is minted. This happens because there will be no fees to collect unless there is a position whose liquidity has been traded against.
To start, we fetch the position from the NonfungiblePositionManager Contract to get the fees we are owed:
```
import{ ethers }from'ethers'importJSBIfrom'jsbi'...const nfpmContract =newethers.Contract(NONFUNGIBLE_POSITION_MANAGER_ADDRESS, provider)const position = nfpmContract.positions(positionId)
```

Next, we construct an options object of type [`CollectOptions`](https://github.com/Uniswap/v3-sdk/blob/08a7c050cba00377843497030f502c05982b1c43/src/nonfungiblePositionManager.ts#L105) that holds the data about the fees we want to collect:
```
import{ CurrencyAmount }from'@uniswap/sdk-core'const collectOptions: CollectOptions ={ tokenId: positionId, expectedCurrencyOwed0: CurrencyAmount.fromRawAmount(  CurrentConfig.tokens.token0,JSBI.BigInt(position.tokensOwed0)), expectedCurrencyOwed1: CurrencyAmount.fromRawAmount(  CurrentConfig.tokens.token1,JSBI.BigInt(position.tokensOwed1)), recipient: address,}
```

Read more about fetching position info [here](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data#fetching-positions).
Similar to the other functions exposed by the `NonfungiblePositionManager`, we pass the `tokenId` and the `recipient` of the fees, which in this case is our function's input position id and our wallet's address.
The other two `CurrencyAmount` parameters (`expectedCurrencyOwed0` and `expectedCurrencyOwed1`) define the **maximum** amount of currency we expect to get collect through accrued fees of each token in the pool. We set these through our guide's configuration.
In a real world scenario, we can fetch the amount of fees that are owed to the Position through the `positions()` function of the NonfungiblePositionManager Contract. We fetch the position info like in this code snippet taken from the [Fetching Positions guide](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions):
```
const positionInfos = callResponses.map((position)=>{return{    tickLower: position.tickLower,    tickUpper: position.tickUpper,    liquidity:JSBI.BigInt(position.liquidity),    feeGrowthInside0LastX128:JSBI.BigInt(position.feeGrowthInside0LastX128),    feeGrowthInside1LastX128:JSBI.BigInt(position.feeGrowthInside1LastX128),    tokensOwed0:JSBI.BigInt(position.tokensOwed0),    tokensOwed1:JSBI.BigInt(position.tokensOwed1),}})
```

The `tokensOwed0` and `tokensOwed1` values are the fees owed.
In this example, we have the values hardcoded in the [`config.ts`](https://github.com/Uniswap/examples/blob/main/v3-sdk/collecting-fees/src/config.ts) file.
## Submitting our fee collection transaction[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#submitting-our-fee-collection-transaction "Direct link to Submitting our fee collection transaction")
Next, we get the call parameters for collecting our fees from our `NonfungiblePositionManager` using the constructed `CollectOptions`:
```
const{ calldata, value }= NonfungiblePositionManager.collectCallParameters(collectOptions)
```

The function above returns the calldata and value required to construct the transaction for collecting accrued fees. Now that we have both the calldata and value we needed for the transaction, we can build and execute the it:
```
const transaction ={ data: calldata, to:NONFUNGIBLE_POSITION_MANAGER_CONTRACT_ADDRESS, value: value, from: address, maxFeePerGas:MAX_FEE_PER_GAS, maxPriorityFeePerGas:MAX_PRIORITY_FEE_PER_GAS,}const txRes =await wallet.sendTransaction(transaction)
```

After pressing the button, if someone has traded against our position, we should be able to note how the balance of USDC and DAI increases as we collect fees.
## Next Steps[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#next-steps "Direct link to Next Steps")
The previous guides detail all the atomic steps needed to create and manage positions. However, these approaches may not use all of your desired currency. To ensure you are using your full funds while minimizing gas prices, check out our guide on [Swapping and Adding Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add) in a single transaction!
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/05-collecting-fees.md)
Was this helpful?
[PreviousAdding & Removing Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position)[NextSwapping and Adding Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#introduction)
  * [Setting up our fee collection](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#setting-up-our-fee-collection)
  * [Submitting our fee collection transaction](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#submitting-our-fee-collection-transaction)
  * [Next Steps](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/05-collecting-fees.md)
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
