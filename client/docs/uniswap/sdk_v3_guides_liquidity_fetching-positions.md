# https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions

[Skip to main content](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
        * [Position Management](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
          * [Liquidity Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
          * [Minting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting)
          * [Fetching Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
          * [Adding & Removing Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position)
          * [Collecting Fees](https://docs.uniswap.org/sdk/v3/guides/liquidity/liquidity-fees)
          * [Swapping and Adding Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/swap-and-add)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
    * [web3-react](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Guides
  * Pooling Liquidity
  * [Fetching Positions](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions)


On this page
# Fetching Positions
## Introduction[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#introduction "Direct link to Introduction")
This guide will cover how to create (or mint) a liquidity position on the Uniswap V3 protocol. Like the [Liquidity Position guide](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data) it doesn't have an accompanying example, nevertheless the concepts and functions used here can be found among the various examples that interact with liquidity positions.
info
If you need an introduction to liquidity positions, check out the [Liquidity Position guide](https://docs.uniswap.org/sdk/v3/guides/liquidity/position-data)
The [NonfungiblePositionManager Contract](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager) can be used to create Positions, as well as get information on **existing Positions**. In this guide, we will fetch **all Positions** an address has and fetch the **detailed Position Data** for those positions.
The guide will **cover** :
  1. Creating an ethersJS contract to interact with the NonfungiblePositionManager.
  2. Fetching all positions for an address.
  3. Fetching the position info for the positions.


At the end of the guide, given the inputs above, we should be able to mint a liquidity position with the press of a button and view the position on the UI of the web application.
For this guide, we do not need to use the Uniswap SDKs, we will only import the contract ABI for the NonfungiblePositionManager Contract from [`@uniswap/v3-periphery`](https://www.npmjs.com/package/@uniswap/v3-periphery).
## Connecting to the NFTPositionManager Contract[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#connecting-to-the-nftpositionmanager-contract "Direct link to Connecting to the NFTPositionManager Contract")
We use **ethersJS** to interact with the NonfungiblePositionManager Contract. Let's create an ethers Contract:
```
import{ ethers }from'ethers'importINONFUNGIBLE_POSITION_MANAGERfrom'@uniswap/v3-periphery/artifacts/contracts/NonfungiblePositionManager.sol/NonfungiblePositionManager.json'const provider =newethers.providers.JsonRpcProvider(rpcUrl)const nfpmContract =newethers.Contract(NONFUNGIBLE_POSITION_MANAGER_CONTRACT_ADDRESS,INONFUNGIBLE_POSITION_MANAGER.abi,  provider)
```

We get the Contract ABI from the 'v3-periphery` package and the contract address from [GitHub](https://github.com/Uniswap/v3-periphery/blob/main/deploys.md)
## Fetching the Position Ids[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#fetching-the-position-ids "Direct link to Fetching the Position Ids")
We want to fetch all Position Ids for our address. We first fetch the number of positions and then the ids by their indices.
We fetch the number of positions using the `balanceOf` read call:
```
const numPositions =await nfpmContract.balanceOf(address)
```

Next we iterate over the number of positions and fetch the ids:
```
const calls =[]for(let i =0; i < numPositions; i++){  calls.push(    nfpmContract.tokenOfOwnerByIndex(address, i))}const positionIds =awaitPromise.all(calls)
```

## Fetching the Position Info[​](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#fetching-the-position-info "Direct link to Fetching the Position Info")
Now that we have the ids of the Positions associated with our address, we can fetch the position info using the `positions` function.
The solidity function returns a lot of values describing the Position:
```
functionpositions(uint256 tokenId)externalviewreturns(uint96 nonce,address operator,address token0,address token1,uint24 fee,int24 tickLower,int24 tickUpper,uint128 liquidity,uint256 feeGrowthInside0LastX128,uint256 feeGrowthInside1LastX128,uint128 tokensOwed0,uint128 tokensOwed1)
```

In this example we only care about values needed to interact with positions, so we create an Interface `PositionInfo`:
```
interfacePositionInfo{ tickLower:number tickUpper:number liquidity:JSBI feeGrowthInside0LastX128:JSBI feeGrowthInside1LastX128:JSBI tokensOwed0:JSBI tokensOwed1:JSBI}
```

We fetch the Position data with `positions`:
```
const positionCalls =[]for(let id of positionIds){  positionCalls.push(    nfpmContract.positions(id))}const callResponses =awaitPromise.all(positionCalls)
```

Finally, we map the RPC response to our interface:
```
const positionInfos = callResponses.map((position)=>{return{    tickLower: position.tickLower,    tickUpper: position.tickUpper,    liquidity:JSBI.BigInt(position.liquidity),    feeGrowthInside0LastX128:JSBI.BigInt(position.feeGrowthInside0LastX128),    feeGrowthInside1LastX128:JSBI.BigInt(position.feeGrowthInside1LastX128),    tokensOwed0:JSBI.BigInt(position.tokensOwed0),    tokensOwed1:JSBI.BigInt(position.tokensOwed1),}})
```

We now have an array containing PositionInfo for all positions that our address holds.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/03-fetching-positions.md)
Was this helpful?
[PreviousMinting a Position](https://docs.uniswap.org/sdk/v3/guides/liquidity/minting)[NextAdding & Removing Liquidity](https://docs.uniswap.org/sdk/v3/guides/liquidity/modifying-position)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#introduction)
  * [Connecting to the NFTPositionManager Contract](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#connecting-to-the-nftpositionmanager-contract)
  * [Fetching the Position Ids](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#fetching-the-position-ids)
  * [Fetching the Position Info](https://docs.uniswap.org/sdk/v3/guides/liquidity/fetching-positions#fetching-the-position-info)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/liquidity/03-fetching-positions.md)
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
