# https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching

[Skip to main content](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
          * [Minting a position](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
          * [Fetching Positions](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
          * [Collecting Fee](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
          * [Adding and Removing Liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
    * [web3-react](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Guides
  * Position Management
  * [Fetching Positions](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)


On this page
# Fetching Positions
## Introduction[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#introduction "Direct link to Introduction")
This guide covers how to fetch and analyze liquidity positions in Uniswap v4 using the v4-sdk.
For this guide, the following Uniswap packages are used:
  * [`@uniswap/v4-sdk`](https://www.npmjs.com/package/@uniswap/v4-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


## Key Differences from v3[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#key-differences-from-v3 "Direct link to Key Differences from v3")
The v4 PositionManager does not implement ERC721Enumerable, so `tokenOfOwnerByIndex` is not available. This requires using the subgraph to discover position IDs. Additionally, v4 uses a packed data format for position information.
## Setup[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#setup "Direct link to Setup")
```
import{ createPublicClient, http, Address, zeroAddress }from'viem'import{ unichain }from'viem/chains'import request from'graphql-request'constPOSITION_MANAGER_ADDRESS='0x4529a01c7a0410167c5740c487a8de60232617bf'//unichainconst publicClient =createPublicClient({ chain: unichain, transport:http(),})
```

## Fetching Position IDs[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#fetching-position-ids "Direct link to Fetching Position IDs")
```
interfaceSubgraphPosition{ id:string tokenId:string owner:string}constGET_POSITIONS_QUERY=` query GetPositions($owner: String!) {  positions(where: { owner: $owner }) {   tokenId   owner   id  } }`constUNICHAIN_SUBGRAPH_URL='https://gateway.thegraph.com/api/subgraphs/id/EoCvJ5tyMLMJcTnLQwWpjAtPdn74PcrZgzfcT5bYxNBH'asyncfunctiongetPositionIds(owner: Address):Promise<bigint[]>{// You can explore queries at: https://thegraph.com/explorer/subgraphs/EoCvJ5tyMLMJcTnLQwWpjAtPdn74PcrZgzfcT5bYxNBH?view=Query&chain=arbitrum-oneconst headers ={  Authorization:'Bearer '+ process.env.GRAPH_KEY,// Get your API key from https://thegraph.com/studio/apikeys/}const response =awaitrequest<{ positions: SubgraphPosition[]}>(UNICHAIN_SUBGRAPH_URL,GET_POSITIONS_QUERY,{ owner: owner.toLowerCase()},  headers)return response.positions.map((p)=>BigInt(p.tokenId))}
```

## Decoding Packed Position Data[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#decoding-packed-position-data "Direct link to Decoding Packed Position Data")
v4 stores position information in a packed format. Here's how to decode it:
```
interfacePackedPositionInfo{getTickUpper():numbergetTickLower():numberhasSubscriber():boolean}functiondecodePositionInfo(value: bigint): PackedPositionInfo {return{getTickUpper:()=>{const raw =Number((value >>32n)&0xffffffn)return raw >=0x800000? raw -0x1000000: raw},getTickLower:()=>{const raw =Number((value >>8n)&0xffffffn)return raw >=0x800000? raw -0x1000000: raw},hasSubscriber:()=>(value &0xffn)!==0n,}}
```

## Position Details Interface[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#position-details-interface "Direct link to Position Details Interface")
```
interfacePositionDetails{ tokenId: bigint tickLower:number tickUpper:number liquidity: bigint poolKey:{  currency0: Address  currency1: Address  fee:number  tickSpacing:number  hooks: Address}}
```

## Contract ABI[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#contract-abi "Direct link to Contract ABI")
```
constPOSITION_MANAGER_ABI=[{  name:'getPoolAndPositionInfo',  type:'function',  inputs:[{ name:'tokenId', type:'uint256'}],  outputs:[{    name:'poolKey',    type:'tuple',    components:[{ name:'currency0', type:'address'},{ name:'currency1', type:'address'},{ name:'fee', type:'uint24'},{ name:'tickSpacing', type:'int24'},{ name:'hooks', type:'address'},],},{ name:'info', type:'uint256'},],},{  name:'getPositionLiquidity',  type:'function',  inputs:[{ name:'tokenId', type:'uint256'}],  outputs:[{ name:'liquidity', type:'uint128'}],},]asconst
```

## Fetching Position Details[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#fetching-position-details "Direct link to Fetching Position Details")
```
asyncfunctiongetPositionDetails(tokenId: bigint):Promise<PositionDetails>{// Get pool key and packed position info// Get pool key and packed position infoconst[poolKey, infoValue]=(await publicClient.readContract({  address:POSITION_MANAGER_ADDRESS,  abi:POSITION_MANAGER_ABI,  functionName:'getPoolAndPositionInfo',  args:[tokenId],}))asreadonly[{   currency0: Address   currency1: Address   fee:number   tickSpacing:number   hooks: Address},  bigint]// Get current liquidityconst liquidity =(await publicClient.readContract({  address:POSITION_MANAGER_ADDRESS,  abi:POSITION_MANAGER_ABI,  functionName:'getPositionLiquidity',  args:[tokenId],}))as bigint// Decode packed position infoconst positionInfo =decodePositionInfo(infoValue)return{  tokenId,  tickLower: positionInfo.getTickLower(),  tickUpper: positionInfo.getTickUpper(),  liquidity,  poolKey,}}
```

## Usage Example[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#usage-example "Direct link to Usage Example")
```
asyncfunctionfetchUserPositions(userAddress: Address){try{// Get position IDs from subgraphconst tokenIds =awaitgetPositionIds(userAddress)console.log(`Found ${tokenIds.length} positions on Unichain`)// Fetch details for each positionfor(const tokenId of tokenIds){const details =awaitgetPositionDetails(tokenId)console.log(`Position ${tokenId}:`)console.log(` Token0: ${details.poolKey.currency0}`)console.log(` Token1: ${details.poolKey.currency1}`)console.log(` Fee: ${details.poolKey.fee /10000}%`)console.log(` Range: ${details.tickLower} to ${details.tickUpper}`)console.log(` Liquidity: ${details.liquidity.toString()}`)console.log(` Hooks: ${details.poolKey.hooks}`)console.log('---')}}catch(error){console.error('Error:', error)}}// Example usagefetchUserPositions('0xYourAddress'as Address)
```

## Resources[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#resources "Direct link to Resources")
  * [Uniswap v4 SDK](https://github.com/Uniswap/sdks/tree/main/sdks/v4-sdk)
  * [Unichain Documentation](https://docs.unichain.org/)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/liquidity/02-fetching-positions.md)
Was this helpful?
[PreviousMinting a position](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)[NextCollecting Fee](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#introduction)
  * [Key Differences from v3](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#key-differences-from-v3)
  * [Setup](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#setup)
  * [Fetching Position IDs](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#fetching-position-ids)
  * [Decoding Packed Position Data](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#decoding-packed-position-data)
  * [Position Details Interface](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#position-details-interface)
  * [Contract ABI](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#contract-abi)
  * [Fetching Position Details](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#fetching-position-details)
  * [Usage Example](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#usage-example)
  * [Resources](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching#resources)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/liquidity/02-fetching-positions.md)
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
