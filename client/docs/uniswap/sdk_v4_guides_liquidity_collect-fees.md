# https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees

[Skip to main content](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
          * [Minting a position](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
          * [Fetching Positions](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)
          * [Collecting Fee](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
          * [Adding and Removing Liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
    * [web3-react](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Guides
  * Position Management
  * [Collecting Fee](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees)


On this page
# Collecting Fee
## Introduction[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#introduction "Direct link to Introduction")
This guide will cover:
  1. **Setting up our fee collection** – Preparing to collect fees from a v4 position, including fetching position details, computing the `poolId`, using `StateView` to read fee growth data, and calculating the unclaimed fees off-chain.
  2. **Submitting our fee collection transaction** – Using the v4 SDK to create the transaction calldata (with `collectCallParameters`), executing the call (via a multicall on the PositionManager).


For this guide, the following Uniswap packages are used:
  * [`@uniswap/v4-sdk`](https://www.npmjs.com/package/@uniswap/v4-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)


## Fee Calculation Theory[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#fee-calculation-theory "Direct link to Fee Calculation Theory")
In Uniswap v4, fees are not stored directly. Instead, fees must be calculated using **differential calculation** from cumulative values called `feeGrowthInside`.
### feeGrowthInside Concept[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#feegrowthinside-concept "Direct link to feeGrowthInside Concept")
```
feeGrowthInside = Cumulative fees generated in pool ÷ Active liquidity at that time
```

### Unclaimed Fees (Currently Collectible Fees)[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#unclaimed-fees-currently-collectible-fees "Direct link to Unclaimed Fees \(Currently Collectible Fees\)")
```
constQ128=2n**128nunclaimedFees =((feeGrowthCurrent - feeGrowthLast)* liquidity)/Q128
```

**Compute unclaimed fees off-chain using the v4 formula:**
  * `feeGrowthInsideCurrentX128` (for token0 and token1): the total fee growth inside the range as of now.
  * `feeGrowthInsideLastX128` (for token0 and token1): the fee growth inside the range at the last time the position's state was updated (recorded in the position info).
  * `liquidity`: the amount of liquidity in the position.


**Implementation** :
```
functioncalculateUnclaimedFeesV4( liquidity: bigint, feeGrowthInside0Current: bigint, feeGrowthInside1Current: bigint, feeGrowthInside0Last: bigint, feeGrowthInside1Last: bigint): UnclaimedFees {constQ128=2n**128n// Overflow protection: return 0 if current is less than lastconst feeGrowthDelta0 =  feeGrowthInside0Current >= feeGrowthInside0Last ? feeGrowthInside0Current - feeGrowthInside0Last :0nconst feeGrowthDelta1 =  feeGrowthInside1Current >= feeGrowthInside1Last ? feeGrowthInside1Current - feeGrowthInside1Last :0nreturn{  token0Fees:(feeGrowthDelta0 * liquidity)/Q128,  token1Fees:(feeGrowthDelta1 * liquidity)/Q128,}}
```

### Lifetime Fees (Total Fees Since Position Creation)[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#lifetime-fees-total-fees-since-position-creation "Direct link to Lifetime Fees \(Total Fees Since Position Creation\)")
```
lifetimeFees =(feeGrowthCurrent * liquidity)/Q128
```

**Implementation** :
```
functioncalculateLifetimeFeesV4( liquidity: bigint, feeGrowthInside0Current: bigint, feeGrowthInside1Current: bigint): LifetimeFees {constQ128=2n**128nreturn{  token0LifetimeFees:(feeGrowthInside0Current * liquidity)/Q128,  token1LifetimeFees:(feeGrowthInside1Current * liquidity)/Q128,}}
```

### 3. Collected Fees Estimate[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#3-collected-fees-estimate "Direct link to 3. Collected Fees Estimate")
**Calculation basis** :
```
Total fees = Collected + Unclaimed∴ Collected = Total fees - Unclaimed
```

## v4 Architecture and Required Changes[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#v4-architecture-and-required-changes "Direct link to v4 Architecture and Required Changes")
### Fee Accrual and Credit Changes[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#fee-accrual-and-credit-changes "Direct link to Fee Accrual and Credit Changes")
**Fee Accrual and Credit:** Uniswap v4 changes how fee accrual is handled when modifying liquidity. In v3, adding or removing liquidity didn't automatically claim fees – you had to call a separate `collect` function to pull out accrued fees. In v4, **accrued fees act like a credit** that is automatically applied or required depending on liquidity changes. Increasing a position's liquidity will **roll any unclaimed fees into the position's liquidity** , and decreasing liquidity will **automatically withdraw** the proportional unclaimed fees for that position. This means that partially removing liquidity in v4 will force-claim the fees earned by that liquidity portion. However, if you want to claim fees without changing liquidity, you can perform a liquidity change of zero (as we'll do in this guide).
### Why StateView is Required[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#why-stateview-is-required "Direct link to Why StateView is Required")
In v4, all pools are managed by a single `PoolManager`, so direct access to pool contracts is not possible. Instead, data must be read through the `StateView` contract.
```
// v4 approach (required)await StateView.getPositionInfo(poolId, owner, tickLower, tickUpper, salt)
```

### salt[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#salt "Direct link to salt")
In v4, the same owner can have multiple positions in the same tick range. `salt` is used to identify them individually.
**Derive the salt for the position:** As noted, v4 positions include a `salt` to distinguish positions with identical range by the same owner. For positions created via the `PositionManager` (which holds ownership in the pool), the salt **is the NFT token ID, encoded as a 32-byte value**.
```
// Use tokenId as salt (PositionManager standard)const salt =`0x${tokenId.toString(16).padStart(64,'0')}`
```

## Code Implementation Flow[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#code-implementation-flow "Direct link to Code Implementation Flow")
### Phase 1: Position Information Retrieval[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#phase-1-position-information-retrieval "Direct link to Phase 1: Position Information Retrieval")
### Step 1: Position List Retrieval[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#step-1-position-list-retrieval "Direct link to Step 1: Position List Retrieval")
Retrieves the tokenIds for v4 positions owned by a specific address from a Subgraph.
### Step 2: Position Details Retrieval[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#step-2-position-details-retrieval "Direct link to Step 2: Position Details Retrieval")
```
asyncfunctiongetPositionDetails(tokenId: bigint):Promise<PositionDetails>{const[poolKey, infoValue]=await publicClient.readContract({  address:POSITION_MANAGER_ADDRESS,  functionName:'getPoolAndPositionInfo',  args:[tokenId],})// poolId calculationconst poolId = Pool.getPoolId(currency0, currency1, poolKey.fee, poolKey.tickSpacing, poolKey.hooks)}
```

### Step 3: Stored Fee State Retrieval[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#step-3-stored-fee-state-retrieval "Direct link to Step 3: Stored Fee State Retrieval")
```
asyncfunctiongetStoredPositionInfoV4(positionDetails, tokenId, owner){const salt =`0x${tokenId.toString(16).padStart(64,'0')}`const[liquidity, feeGrowthInside0Last, feeGrowthInside1Last]=await publicClient.readContract({  address:STATE_VIEW_ADDRESS,  functionName:'getPositionInfo',  args:[poolId,POSITION_MANAGER_ADDRESS, tickLower, tickUpper, salt],})}
```

### Step 4: Current Fee Growth Values Retrieval[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#step-4-current-fee-growth-values-retrieval "Direct link to Step 4: Current Fee Growth Values Retrieval")
**Read the current fee growth in the pool for the position's range:** To compute how much fees are unclaimed, we need the **current** fee growth inside the range and compare it to the last snapshot. We could manually fetch global fee growth and subtract out-of-range values, but `StateView` provides a convenience: [`getFeeGrowthInside(poolId, tickLower, tickUpper)`](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IStateView) will calculate the up-to-date fee growth inside that tick range for each token. This function reads the latest pool state (including global fee growth) and subtracts the parts outside the range. It accounts for any new trades that happened since the last snapshot.
```
asyncfunctiongetCurrentFeeGrowthV4(positionDetails){const[feeGrowthInside0X128, feeGrowthInside1X128]=await publicClient.readContract({  address:STATE_VIEW_ADDRESS,  functionName:'getFeeGrowthInside',  args:[poolId, tickLower, tickUpper],})}
```

### Phase 2: Submitting Our Fee Collection Transaction[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#phase-2-submitting-our-fee-collection-transaction "Direct link to Phase 2: Submitting Our Fee Collection Transaction")
Collecting fees in v4 is done via the `PositionManager` contract's `modifyLiquidities` function with a specific sequence of actions. We will use the Uniswap v4 SDK to construct the required calldata and then send the transaction.
### Build the fee-collection calldata with collectCallParameters[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#build-the-fee-collection-calldata-with-collectcallparameters "Direct link to Build the fee-collection calldata with collectCallParameters")
The Uniswap v4 SDK provides a helper `V4PositionManager.collectCallParameters(...)` that produces the calldata for the necessary multicall to collect fees. Under the hood, this will encode two actions: a `DECREASE_LIQUIDITY` with `liquidity = 0` (and min amounts = 0) and a `TAKE_PAIR` to sweep both tokens to a recipient. We need to supply the SDK with our position details and our desired options. First, create a `Position` object for the position (this requires the pool info and position info we fetched):
```
asyncfunctioncollectFeesViaMulticall(tokenId, userAddress){// Create Position object using pool and position parametersconst position =newPosition({  pool,  tickLower: positionDetails.tickLower,  tickUpper: positionDetails.tickUpper,  liquidity: positionDetails.liquidity.toString(),})// Specify collect optionsconst collectOptions ={  tokenId: tokenId,  recipient: userAddress,  slippageTolerance,  deadline,  hookData,}// Generate command with v4 SDKconst{ calldata, value }= V4PositionManager.collectCallParameters(position, collectOptions)// Execute with multicallconst txHash =await walletClient.writeContract({  account,  chain: unichain,  address:POSITION_MANAGER_ADDRESS,  abi:POSITION_MANAGER_ABI,  functionName:'multicall',  args:[[calldata]],  value:BigInt(value),})}
```

Let's break this down: we created a `Position` object using the pool and position parameters. We then specify `collectOptions` including the NFT `tokenId`, a `recipient` address (fees will be sent to this address), and a `deadline`. Because fee collection is not really subject to price slippage, we can set slippage tolerance to 0 and simply expect whatever fees are available. The SDK's `collectCallParameters` returns an object with `calldata` (the encoded bytes to send to the PositionManager) and `value` (the ETH value to send with the transaction, if needed). In our case, `value` will typically be `0` because we are not providing any additional ETH; we are only withdrawing. (The `value` would be non-zero if one of the actions required sending ETH to the contract, e.g. if adding liquidity to an ETH pair.)
**Under the hood:** The `calldata` produced encodes exactly two actions in `modifyLiquidities`: `Actions.DECREASE_LIQUIDITY` followed by `Actions.TAKE_PAIR`. The first action includes our `tokenId` and zeros for liquidity and min amounts, and the second action includes the two token currencies and the recipient address. Using a zero liquidity decrease is a trick to trigger the pool to calculate fees owed without actually changing the liquidity. The `TAKE_PAIR` then instructs the contract to transfer both token0 and token1 fee amounts out to us. (If our pool involved native ETH, one of the `Currency` entries in this param will be `Currency.wrap(0)` as shown, which signals the contract to send ETH. No manual WETH unwrap is needed – v4 handles it natively.)
### Phase 3: Verify the Fees Were Collected[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#phase-3-verify-the-fees-were-collected "Direct link to Phase 3: Verify the Fees Were Collected")
Once the transaction is mined, you'll want to confirm that the fees made it to the `recipient`. There are a few ways to verify:
### Check the Transaction Receipt Logs[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#check-the-transaction-receipt-logs "Direct link to Check the Transaction Receipt Logs")
For ERC-20 tokens, the fee amounts taken will appear as `Transfer` events from the pool or PositionManager contract to your address. Token contracts will emit these events when the PositionManager transfers the fees to you. You can parse the receipt for `Transfer` logs of `token0` and `token1`. The amounts in those events should match the fees we calculated (or be very close, allowing for rounding).
```
asyncfunctionverifyFeeCollection(receipt, userAddress, positionDetails, ethBalanceBefore){// Search for ERC-20 Transfer eventsconst transferSignature ='0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef'const erc20Transfers = receipt.logs.filter((log)=>    log.topics[0]=== transferSignature && log.topics[2]?.toLowerCase().includes(userAddress.slice(2).toLowerCase())).map((log)=>({   token: log.address as Address,   amount:BigInt(log.data),}))}
```

### Check Your Token Balances[​](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#check-your-token-balances "Direct link to Check Your Token Balances")
You can simply measuring the balance change in your wallet before vs. after the call. For example, read your token balances (and ETH balance) prior to calling, then after the transaction confirm the increases. Because v4 might auto-wrap or unwrap ETH, if one of the tokens was ETH you should check your ETH balance difference. In ETH pools, no ERC-20 transfer event will fire for the ETH – the ETH will be sent directly to you (as an internal transfer), which is why checking the balance or the transaction's internal traces is necessary to confirm the amount.
```
// Check native ETH balance changesconst hasNativeETH =isNativeETH(positionDetails.poolKey.currency0)if(hasNativeETH){const ethBalanceAfter =await publicClient.getBalance({ address: userAddress })const ethChange = ethBalanceAfter - ethBalanceBeforeif(ethChange >0n){  collectedFees.push({   token:'0x0000000000000000000000000000000000000000',   amount: ethChange,})}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/liquidity/03-collecting-fees.md)
Was this helpful?
[PreviousFetching Positions](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-fetching)[NextAdding and Removing Liquidity](https://docs.uniswap.org/sdk/v4/guides/liquidity/add-remove-liquidity)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#introduction)
  * [Fee Calculation Theory](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#fee-calculation-theory)
    * [feeGrowthInside Concept](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#feegrowthinside-concept)
    * [Unclaimed Fees (Currently Collectible Fees)](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#unclaimed-fees-currently-collectible-fees)
    * [Lifetime Fees (Total Fees Since Position Creation)](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#lifetime-fees-total-fees-since-position-creation)
    * [3. Collected Fees Estimate](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#3-collected-fees-estimate)
  * [v4 Architecture and Required Changes](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#v4-architecture-and-required-changes)
    * [Fee Accrual and Credit Changes](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#fee-accrual-and-credit-changes)
    * [Why StateView is Required](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#why-stateview-is-required)
    * [salt](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#salt)
  * [Code Implementation Flow](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#code-implementation-flow)
    * [Phase 1: Position Information Retrieval](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#phase-1-position-information-retrieval)
    * [Step 1: Position List Retrieval](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#step-1-position-list-retrieval)
    * [Step 2: Position Details Retrieval](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#step-2-position-details-retrieval)
    * [Step 3: Stored Fee State Retrieval](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#step-3-stored-fee-state-retrieval)
    * [Step 4: Current Fee Growth Values Retrieval](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#step-4-current-fee-growth-values-retrieval)
    * [Phase 2: Submitting Our Fee Collection Transaction](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#phase-2-submitting-our-fee-collection-transaction)
    * [Build the fee-collection calldata with collectCallParameters](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#build-the-fee-collection-calldata-with-collectcallparameters)
    * [Phase 3: Verify the Fees Were Collected](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#phase-3-verify-the-fees-were-collected)
    * [Check the Transaction Receipt Logs](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#check-the-transaction-receipt-logs)
    * [Check Your Token Balances](https://docs.uniswap.org/sdk/v4/guides/liquidity/collect-fees#check-your-token-balances)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/guides/liquidity/03-collecting-fees.md)
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
