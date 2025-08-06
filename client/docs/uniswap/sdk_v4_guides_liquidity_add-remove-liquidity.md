# https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity

[Skip to main content](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#__docusaurus_skipToContent_fallback)
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
          * [Minting a position](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
          * [Fetching Positions](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
          * [Collecting Fee](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
          * [Adding and Removing Liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity)
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
  * Position Management
  * [Adding and Removing Liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity)


On this page
# Adding and Removing Liquidity
## Introduction[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#introduction "Direct link to Introduction")
This guide will cover:
  1. **Setting up liquidity operations** – Preparing to add/remove liquidity from v4 positions, including fetching position details, handling native ETH vs ERC20 tokens, and configuring Permit2 for ERC20 token approvals.
  2. **Adding liquidity to existing positions** – Using the v4 SDK to increase liquidity with `addCallParameters`, handling native ETH positions, and executing transactions via PositionManager multicall.
  3. **Removing liquidity from positions** – Using `removeCallParameters` to decrease or fully exit positions, handling proportional withdrawals, and token collection.


For this guide, the following Uniswap packages are used:
  * [`@uniswap/v4-sdk`](https://www.npmjs.com/package/@uniswap/v4-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


## v4 Architecture and Key Changes[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#v4-architecture-and-key-changes "Direct link to v4 Architecture and Key Changes")
### Native ETH Handling[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#native-eth-handling "Direct link to Native ETH Handling")
Unlike v3, Uniswap v4 has native support for ETH without wrapping to WETH. This requires special handling in the SDK:
```
// ✅ Correct: Using Ether.onChain() for native ETHconst token0 = Ether.onChain(chainId)
```

### Position Manager Multicall[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#position-manager-multicall "Direct link to Position Manager Multicall")
All v4 position operations use the `PositionManager` contract's `multicall` function with encoded action sequences:
```
const{ calldata, value }= V4PositionManager.addCallParameters(position, options)await walletClient.writeContract({ address:POSITION_MANAGER_ADDRESS, functionName:'multicall', args:[[calldata]], value:BigInt(value),})
```

## Adding Liquidity to Existing Positions[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#adding-liquidity-to-existing-positions "Direct link to Adding Liquidity to Existing Positions")
### Theory: IncreaseLiquidityOptions[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#theory-increaseliquidityoptions "Direct link to Theory: IncreaseLiquidityOptions")
When adding liquidity to existing positions, we use `IncreaseLiquidityOptions` which combines:
  * `CommonOptions`: slippage, deadline, hookData
  * `ModifyPositionSpecificOptions`: tokenId
  * `CommonAddLiquidityOptions`: useNative, batchPermit


### Step 1: Fetch Position Details[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-1-fetch-position-details "Direct link to Step 1: Fetch Position Details")
```
interfacePositionDetails{ tokenId: bigint tickLower:number tickUpper:number liquidity: bigint poolKey:{  currency0: Address  currency1: Address  fee:number  tickSpacing:number  hooks: Address} token0: Currency // Can be Ether or Token token1: Token // Always Token in current implementation currentTick:number sqrtPriceX96:string poolLiquidity:string}asyncfunctiongetPositionDetails(tokenId: bigint):Promise<PositionDetails>{// Fetch position info from PositionManagerconst[poolKey, infoValue]=await publicClient.readContract({  address:POSITION_MANAGER_ADDRESS,  abi:POSITION_MANAGER_ABI,  functionName:'getPoolAndPositionInfo',  args:[tokenId],})// Create proper Currency instanceslet token0: Currencyif(isNativeETH(poolKey.currency0)){  token0 = Ether.onChain(chainId)}else{const decimals0 =awaitfetchTokenDecimals(poolKey.currency0)const symbol0 =awaitgetTokenSymbol(poolKey.currency0)  token0 =newToken(chainId, poolKey.currency0, decimals0, symbol0)}const token1 =newToken(chainId, poolKey.currency1, decimals1, symbol1)return{  tokenId,  tickLower: infoValue.tickLower,  tickUpper: infoValue.tickUpper,  liquidity: infoValue.liquidity,  poolKey,  token0,  token1,// ... other fields}}
```

### Step 2: Configure Permit2 (Recommended)[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-2-configure-permit2-recommended "Direct link to Step 2: Configure Permit2 \(Recommended\)")
```
constPERMIT2_TYPES={ PermitDetails:[{ name:'token', type:'address'},{ name:'amount', type:'uint160'},{ name:'expiration', type:'uint48'},{ name:'nonce', type:'uint48'},], PermitBatch:[{ name:'details', type:'PermitDetails[]'},{ name:'spender', type:'address'},{ name:'sigDeadline', type:'uint256'},],}asyncfunctionconfigurePermit2(positionDetails: EnhancedPositionDetails, deadline:number){const permitDetails =[]// Add token1 (always ERC20)const[,, nonce1]=await publicClient.readContract({  address:PERMIT2_ADDRESS,  abi:PERMIT2_ABI,  functionName:'allowance',  args:[userAddress, positionDetails.token1.address,POSITION_MANAGER_ADDRESS],}) permitDetails.push({  token: positionDetails.token1.address,  amount:(2n**160n-1n).toString(),  expiration: deadline.toString(),  nonce: nonce1.toString(),})// Add token0 only if it's not native ETHif(!positionDetails.token0.isNative){const[,, nonce0]=await publicClient.readContract({   address:PERMIT2_ADDRESS,   abi:PERMIT2_ABI,   functionName:'allowance',   args:[userAddress,(positionDetails.token0 as Token).address,POSITION_MANAGER_ADDRESS],})  permitDetails.push({   token:(positionDetails.token0 as Token).address,   amount:(2n**160n-1n).toString(),   expiration: deadline.toString(),   nonce: nonce0.toString(),})}const permitData ={  details: permitDetails,  spender:POSITION_MANAGER_ADDRESS,  sigDeadline: deadline.toString(),}// Sign Permit2 dataconst signature =await walletClient.signTypedData({  account,  domain:{   name:'Permit2',   chainId,   verifyingContract:PERMIT2_ADDRESS,},  types:PERMIT2_TYPES,  primaryType:'PermitBatch',  message: permitData,})return{  owner: userAddress,  permitBatch: permitData,  signature,}}
```

### Step 3: Create Position and Add Liquidity[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-3-create-position-and-add-liquidity "Direct link to Step 3: Create Position and Add Liquidity")
```
asyncfunctionaddLiquidityToPosition( positionDetails: EnhancedPositionDetails, amount0:string, amount1:string, slippageTolerance:number=0.05){// Create Pool instanceconst pool =newPool(  positionDetails.token0,  positionDetails.token1,  positionDetails.poolKey.fee,  positionDetails.poolKey.tickSpacing,  positionDetails.poolKey.hooks,  positionDetails.sqrtPriceX96,  positionDetails.poolLiquidity,  positionDetails.currentTick)// Create currency amountsconst amount0Currency = CurrencyAmount.fromRawAmount(positionDetails.token0, amount0)const amount1Currency = CurrencyAmount.fromRawAmount(positionDetails.token1, amount1)// Create Position from amountsconst position = Position.fromAmounts({  pool,  tickLower: positionDetails.tickLower,  tickUpper: positionDetails.tickUpper,  amount0: amount0Currency.quotient,  amount1: amount1Currency.quotient,  useFullPrecision:true,})// Configure optionsconst slippagePct =newPercent(Math.floor(slippageTolerance *100),10_000)const deadline = Math.floor(Date.now()/1000)+1200// 20 minutesconst addOptions: AddLiquidityOptions ={// CommonOptions  slippageTolerance: slippagePct,  deadline: deadline.toString(),  hookData:'0x',// ModifyPositionSpecificOptions  tokenId: positionDetails.tokenId.toString(),// CommonAddLiquidityOptions...(positionDetails.token0.isNative &&{ useNative: Ether.onChain(chainId)}),  batchPermit:awaitconfigurePermit2(positionDetails, deadline),}// Generate calldata and executeconst{ calldata, value }= V4PositionManager.addCallParameters(position, addOptions)const txHash =await walletClient.writeContract({  account,  address:POSITION_MANAGER_ADDRESS,  chain: unichain,  abi:POSITION_MANAGER_ABI,  functionName:'multicall',  args:[[calldata]],  value:BigInt(value.toString()),})return{ txHash, addedAmounts:{ amount0, amount1 }}}
```

## Removing Liquidity from Positions[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#removing-liquidity-from-positions "Direct link to Removing Liquidity from Positions")
### Theory: RemoveLiquidityOptions[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#theory-removeliquidityoptions "Direct link to Theory: RemoveLiquidityOptions")
When removing liquidity, we use `RemoveLiquidityOptions` which includes:
  * `CommonOptions`: slippage, deadline, hookData
  * `ModifyPositionSpecificOptions`: tokenId
  * `RemoveLiquiditySpecificOptions`: liquidityPercentage, burnToken, permit


### Step 1: Calculate Liquidity to Remove[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-1-calculate-liquidity-to-remove "Direct link to Step 1: Calculate Liquidity to Remove")
```
functioncalculateLiquidityToRemove( currentLiquidity: bigint, percentageToRemove:number// 0.25 = 25%, 1.0 = 100%):{ liquidityToRemove: bigint liquidityPercentage: Percent}{const liquidityToRemove =(currentLiquidity *BigInt(Math.floor(percentageToRemove *10000)))/10000nconst liquidityPercentage =newPercent(Math.floor(percentageToRemove *100),100)return{ liquidityToRemove, liquidityPercentage }}
```

### Step 2: Remove Liquidity Implementation[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-2-remove-liquidity-implementation "Direct link to Step 2: Remove Liquidity Implementation")
```
asyncfunctionremoveLiquidityFromPosition( positionDetails: EnhancedPositionDetails, percentageToRemove:number,// 0.25 = 25%, 1.0 = 100% slippageTolerance:number=0.05, burnTokenIfEmpty:boolean=false){const{ liquidityToRemove, liquidityPercentage }=calculateLiquidityToRemove(  positionDetails.liquidity,  percentageToRemove)// Create Pool instanceconst pool =newPool(  positionDetails.token0,  positionDetails.token1,  positionDetails.poolKey.fee,  positionDetails.poolKey.tickSpacing,  positionDetails.poolKey.hooks,  positionDetails.sqrtPriceX96,  positionDetails.poolLiquidity,  positionDetails.currentTick)// Create Position instance with current liquidityconst position =newPosition({  pool,  tickLower: positionDetails.tickLower,  tickUpper: positionDetails.tickUpper,  liquidity: positionDetails.liquidity.toString(),})// Configure remove optionsconst slippagePct =newPercent(Math.floor(slippageTolerance *100),10_000)const deadline = Math.floor(Date.now()/1000)+1200const removeOptions: RemoveLiquidityOptions ={// CommonOptions  slippageTolerance: slippagePct,  deadline: deadline.toString(),  hookData:'0x',// ModifyPositionSpecificOptions  tokenId: positionDetails.tokenId.toString(),// RemoveLiquiditySpecificOptions  liquidityPercentage,  burnToken: burnTokenIfEmpty && percentageToRemove ===1.0,// permit: optional NFT permit if transaction sender doesn't own the NFT}// Generate calldata and executeconst{ calldata, value }= V4PositionManager.removeCallParameters(position, removeOptions)const txHash =await walletClient.writeContract({  account,  address:POSITION_MANAGER_ADDRESS,  chain: unichain,  abi:POSITION_MANAGER_ABI,  functionName:'multicall',  args:[[calldata]],  value:BigInt(value.toString()),})return{  txHash,  removedLiquidity: liquidityToRemove,  percentageRemoved: percentageToRemove,  tokenBurned: burnTokenIfEmpty && percentageToRemove ===1.0,}}
```

## Complete Example: Add/Remove Workflow[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#complete-example-addremove-workflow "Direct link to Complete Example: Add/Remove Workflow")
```
asyncfunctioncompleteAddRemoveWorkflow(){const tokenId =123456n// 1. Fetch position detailsconst positionDetails =awaitgetPositionDetails(tokenId)console.log(`Position: ${positionDetails.token0.symbol}/${positionDetails.token1.symbol}`)// 2. Add liquidityconst addResult =awaitaddLiquidityToPosition(  positionDetails,'1000000000000000',// 0.001 ETH'1000000',// 1 USDC0.05// 5% slippage)console.log(`Added liquidity: ${addResult.txHash}`)// 3. Wait and verifyawaitnewPromise((resolve)=>setTimeout(resolve,5000))const updatedPosition =awaitgetPositionDetails(tokenId)// 4. Remove 50% of liquidityconst removeResult =awaitremoveLiquidityFromPosition(  updatedPosition,0.5,// 50%0.05,// 5% slippagefalse// don't burn token)console.log(`Removed 50% liquidity: ${removeResult.txHash}`)return{ addResult, removeResult }}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/liquidity/04-add-remove-liquidity.md)
Was this helpful?
[PreviousCollecting Fee](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)[NextFetching Pool Data](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#introduction)
  * [v4 Architecture and Key Changes](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#v4-architecture-and-key-changes)
    * [Native ETH Handling](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#native-eth-handling)
    * [Position Manager Multicall](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#position-manager-multicall)
  * [Adding Liquidity to Existing Positions](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#adding-liquidity-to-existing-positions)
    * [Theory: IncreaseLiquidityOptions](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#theory-increaseliquidityoptions)
    * [Step 1: Fetch Position Details](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-1-fetch-position-details)
    * [Step 2: Configure Permit2 (Recommended)](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-2-configure-permit2-recommended)
    * [Step 3: Create Position and Add Liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-3-create-position-and-add-liquidity)
  * [Removing Liquidity from Positions](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#removing-liquidity-from-positions)
    * [Theory: RemoveLiquidityOptions](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#theory-removeliquidityoptions)
    * [Step 1: Calculate Liquidity to Remove](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-1-calculate-liquidity-to-remove)
    * [Step 2: Remove Liquidity Implementation](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#step-2-remove-liquidity-implementation)
  * [Complete Example: Add/Remove Workflow](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity#complete-example-addremove-workflow)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/liquidity/04-add-remove-liquidity.md)
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
