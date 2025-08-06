# https://docs.uniswap.org/sdk/v3/guides/liquidity/minting

[Skip to main content](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#__docusaurus_skipToContent_fallback)
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
          * [Liquidity Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
          * [Minting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting)
          * [Fetching Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
          * [Adding & Removing Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position)
          * [Collecting Fees](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
          * [Swapping and Adding Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/advanced/introduction)
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
  * Pooling Liquidity
  * [Minting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting)


On this page
# Minting a Position
## Introduction[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#introduction "Direct link to Introduction")
This guide will cover how to create (or mint) a liquidity position on the Uniswap V3 protocol. It is based on the [minting a position code example](https://github.com/Uniswap/examples/tree/main/v3-sdk/minting-position), found in the Uniswap code examples [repository](https://github.com/Uniswap/examples). To run this example, check out the examples's [README](https://github.com/Uniswap/examples/blob/main/v3-sdk/minting-position/README.md) and follow the setup instructions.
info
If you need a briefer on the SDK and to learn more about how these guides connect to the examples repository, please visit our [background](https://docs.uniswap.org/sdk/v3/guides/background) page!
In the Uniswap V3 protocol, liquidity positions are represented using non-fungible tokens. In this guide we will use the `NonfungiblePositionManager` class to help us mint a liquidity position for the **USDC - DAI** pair. The inputs to our guide are the **two tokens** that we are pooling for, the **amount** of each token we are pooling for and the Pool **fee**.
The guide will **cover** :
  1. Giving approval to transfer our tokens
  2. Creating an instance of a `Pool`
  3. Calculating our `Position` from our input tokens
  4. Configuring and executing our minting transaction


At the end of the guide, given the inputs above, we should be able to mint a liquidity position with the press of a button and view the position on the UI of the web application.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v3-sdk`](https://www.npmjs.com/package/@uniswap/v3-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)
  * [`@uniswap/smart-order-router`](https://www.npmjs.com/package/@uniswap/smart-order-router)


The core code of this guide can be found in [`mintPosition()`](https://github.com/Uniswap/examples/blob/main/v3-sdk/minting-position/src/libs/positions.ts#L37)
## Giving approval to transfer our tokens[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#giving-approval-to-transfer-our-tokens "Direct link to Giving approval to transfer our tokens")
We want to use the `NonfungiblePositionManager` contract to create our liqudity position. In situations where a smart contract is transfering tokens on our behalf, we need to give it approval to do so. This is done by interacting with the Contract of the contract, considering ERC20 Tokens are smart contracts of their own.
Considering this, the first step to create our position is to give approval to the protocol's `NonfungiblePositionManager` to transfer our tokens:
```
const token0Approval =awaitgetTokenTransferApproval( token0Address, amount0)const token1Approval =awaitgetTokenTransferApproval( token1Address, amount1)
```

The logic to achieve that is wrapped in the `getTokenTransferApprovals` function. In short, since both **USDC** and **DAI** are ERC20 tokens, we setup a reference to their smart contracts and call the `approve` function:
```
import{ ethers, BigNumber }from'ethers'asyncfunctiongetTokenTransferApproval(address:string, amount: BigNumber){const provider =newethers.providers.JsonRpcProvider(rpcUrl)const tokenContract =newethers.Contract(    address,ERC20_ABI,    provider)return tokenContract.approve(NONFUNGIBLE_POSITION_MANAGER_CONTRACT_ADDRESS,    amount)}
```

We can get the Contract address for the NonfungiblePositionManager from [GitHub](https://github.com/Uniswap/v3-periphery/blob/main/deploys.md). For Ethereum mainnet or a local fork of mainnet, we see that the contract address is `0xC36442b4a4522E871399CD717aBDD847Ab11FE88`. In our example, this is defined in the [`constants.ts`](https://github.com/Uniswap/examples/blob/main/v3-sdk/minting-position/src/libs/constants.ts) file.
## Creating an instance of a `Pool`[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#creating-an-instance-of-a-pool "Direct link to creating-an-instance-of-a-pool")
Having approved the transfer of our tokens, we now need to get data about the pool for which we will provide liquidity, in order to instantiate a Pool class.
To start, we compute our Pool's address by using a helper function and passing in the unique identifiers of a Pool - the **two tokens** and the Pool **fee**. The **fee** input parameter represents the swap fee that is distributed to all in range liquidity at the time of the swap.
```
import{ computePoolAddress, FeeAmount }from'@uniswap/v3-sdk'import{ Token }from'@uniswap/sdk-core'const token0: Token =...const token1: Token =...const fee: FeeAmount =...constPOOL_FACTORY_CONTRACT_ADDRESS:string=...const currentPoolAddress =computePoolAddress({ factoryAddress:POOL_FACTORY_CONTRACT_ADDRESS, tokenA: token0, tokenB: token1, fee: poolFee,})
```

Again, we can get the factory contract address from [GitHub](https://github.com/Uniswap/v3-periphery/blob/main/deploys.md). For Ethereum mainnet, or a local fork of mainnet, it is `0x1F98431c8aD98523631AE4a59f267346ea31F984`. In our example, it is defined in [`constants.ts`](https://github.com/Uniswap/examples/blob/main/v3-sdk/minting-position/src/libs/constants.ts)
Then, we get the Pool's data by creating a reference to the Pool's smart contract and accessing its methods, very similar to what we did in the [Quoting guide](https://docs.uniswap.org/sdk/v3/guides/swaps/quoting#referencing-the-pool-contract-and-fetching-metadata):
```
import IUniswapV3PoolABI from'@uniswap/v3-core/artifacts/contracts/interfaces/IUniswapV3Pool.sol/IUniswapV3Pool.json'const poolContract =newethers.Contract( currentPoolAddress, IUniswapV3PoolABI.abi, provider)const[liquidity, slot0]=awaitPromise.all([  poolContract.liquidity(),  poolContract.slot0(),])
```

Having collected the required data, we can now create an instance of the `Pool` class:
```
import{ Pool }from'@uniswap/v3-sdk'const configuredPool =newPool( token0, token1, poolFee, slot0.sqrtPriceX96.toString(), liquidity.toString(), slot0.tick)
```

We need a Pool instance to create our Position as various parameters of liquidity positions depend on the state of the Pool where they are created. An example is the current price (named _sqrtPriceX96_ after the way it is encoded) to know the ratio of the two Tokens we need to send to the Pool.
Liquidity provided below the current Price will be provided in the first Token of the Pool, while liquidity provided above the current Price is made up by the second Token.
## Calculating our `Position` from our input tokens[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#calculating-our-position-from-our-input-tokens "Direct link to calculating-our-position-from-our-input-tokens")
Having created the instance of the `Pool` class, we can now use that to create an instance of a `Position` class, which represents the price range for a specific pool that LPs choose to provide in:
```
import{ Position }from'@uniswap/v3-sdk'import{ BigIntish }from'@uniswap/sdk-core'// The maximum token amounts we want to provide. BigIntish accepts number, string or JSBIconst amount0: BigIntish =...const amount1: BigIntish =...const position = Position.fromAmounts({ pool: configuredPool, tickLower:nearestUsableTick(configuredPool.tickCurrent, configuredPool.tickSpacing)-  configuredPool.tickSpacing *2, tickUpper:nearestUsableTick(configuredPool.tick, configuredPool.tickSpacing)+  configuredPool.tickSpacing *2, amount0: amount0, amount1: amount1, useFullPrecision:true,})
```

We use the `fromAmounts` static function of the `Position` class to create an instance of it, which uses the following parameters:
  * The **tickLower** and **tickUpper** parameters specify the price range at which to provide liquidity. This example calls **nearestUsableTick** to get the current useable tick and adjust the lower parameter to be below it by two **tickSpacing** and the upper to be above it by two tickSpacing. This guarantees that the provided liquidity is "in range", meaning it will be earning fees upon minting this position
  * **amount0** and **amount1** define the maximum amount of currency the liquidity position can use. In this example, we supply these from our configuration parameters.


Given those parameters, `fromAmounts` will attempt to calculate the maximum amount of liquidity we can supply.
## Configuring and executing our minting transaction[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#configuring-and-executing-our-minting-transaction "Direct link to Configuring and executing our minting transaction")
The Position instance is then passed as input to the `NonfungiblePositionManager`'s `addCallParameters` function. The function also requires an [`AddLiquidityOptions`](https://github.com/Uniswap/v3-sdk/blob/08a7c050cba00377843497030f502c05982b1c43/src/nonfungiblePositionManager.ts#L77) object as its second parameter. This is either of type [`MintOptions`](https://github.com/Uniswap/v3-sdk/blob/08a7c050cba00377843497030f502c05982b1c43/src/nonfungiblePositionManager.ts#L74) for minting a new position or [`IncreaseOptions`](https://github.com/Uniswap/v3-sdk/blob/08a7c050cba00377843497030f502c05982b1c43/src/nonfungiblePositionManager.ts#L75) for adding liquidity to an existing position. For this example, we're using a `MintOptions` to create our position.
```
import{ MintOptions, NonfungiblePositionManager }from'@uniswap/v3-sdk'import{ Percent }from'@uniswap/sdk-core'const mintOptions: MintOptions ={ recipient: address, deadline: Math.floor(Date.now()/1000)+60*20, slippageTolerance:newPercent(50,10_000),}// get calldata for minting a positionconst{ calldata, value }= NonfungiblePositionManager.addCallParameters( position, mintOptions)
```

The `MintOptions` interface requires three keys:
  * `recipient` defines the address of the Position owner, so in our case the address of our wallet.
  * `deadline` defines the latest point in time at which we want our transaction to be included in the blockchain.
  * `slippageTolerance` defines the maximum amount of **change of the ratio** of the Tokens we provide. The ratio can change if for example **trades** that change the price of the Pool are included before our transaction.


The `addCallParameters` function returns the calldata as well as the value required to execute the transaction:
```
const transaction ={ data: calldata, to:NONFUNGIBLE_POSITION_MANAGER_CONTRACT_ADDRESS, value: value, from: address, maxFeePerGas:MAX_FEE_PER_GAS, maxPriorityFeePerGas:MAX_PRIORITY_FEE_PER_GAS,}
```

We use our wallet to send the transaction. As it is a write call, we need to sign the transaction with a valid private key.
```
const wallet =newethers.Wallet(privateKey, provider)const txRes =await wallet.sendTransaction(transaction)
```

Write calls do not return the result of the transaction. If we want to read the result we would need to use for example `trace_transaction`. You can find an example of that in the [Range Order guide](https://docs.uniswap.org/sdk/v3/guides/advanced/range-orders). In this example, we don't need the result of the transaction.
The effect of the transaction is to mint a new Position NFT. We should see a new position with liquidity in our list of positions.
## Next Steps[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#next-steps "Direct link to Next Steps")
Once you have minted a position, our next guide [Adding and Removing Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position) will demonstrate how you can add and remove liquidity from that minted position!
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/02-minting-position.md)
Was this helpful?
[PreviousLiquidity Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)[NextFetching Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#introduction)
  * [Giving approval to transfer our tokens](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#giving-approval-to-transfer-our-tokens)
  * [Creating an instance of a `Pool`](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#creating-an-instance-of-a-pool)
  * [Calculating our `Position` from our input tokens](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#calculating-our-position-from-our-input-tokens)
  * [Configuring and executing our minting transaction](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#configuring-and-executing-our-minting-transaction)
  * [Next Steps](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/02-minting-position.md)
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
