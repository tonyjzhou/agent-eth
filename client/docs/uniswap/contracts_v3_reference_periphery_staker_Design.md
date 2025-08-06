# https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
          * [Base](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
          * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
          * [Lens](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
          * [Libraries](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
          * [Staker](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
            * [Uniswap V3 Staker Design](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
            * [Uniswap V3 Staker Contract](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker)
            * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
            * [Libraries](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
          * [Test](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
          * [NonfungiblePositionManager](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager)
          * [NonfungibleTokenPositionDescriptor](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungibleTokenPositionDescriptor)
          * [SwapRouter](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter)
          * [V3Migrator](https://docs.uniswap.org/contracts/v3/reference/periphery/V3Migrator)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
    * [Permit2](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * Periphery
  * Staker
  * [Uniswap V3 Staker Design](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)


On this page
# Uniswap V3 Staker Design
The liquidity mining staker design is comprised of one canonical position staking contract, Staker. The technical reference for this contract is [here](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker) and the source code is [here](https://github.com/Uniswap/uniswap-v3-staker).
## Data Structures[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#data-structures "Direct link to Data Structures")
```
structIncentive{uint128 totalRewardUnclaimed;uint128 numberOfStakes;uint160 totalSecondsClaimedX128;}structDeposit{address owner;uint96 numberOfStakes;}structStake{uint160 secondsPerLiquidityInsideInitialX128;uint128 liquidity;}structIncentiveKey{    IERC20Minimal rewardToken;    IUniswapV3Pool pool;uint256 startTime;uint256 endTime;address refundee;}
```

State:
```
IUniswapV3Factory public immutable factory;INonfungiblePositionManager public immutable nonfungiblePositionManager;/// @dev bytes32 refers to the return value of IncentiveId.computemapping(bytes32=> Incentive)public incentives;/// @dev deposits[tokenId] => Depositmapping(uint256=> Deposit)public deposits;/// @dev stakes[tokenId][incentiveHash] => Stakemapping(uint256=>mapping(bytes32=> Stake))public stakes;/// @dev rewards[rewardToken][msg.sender] => uint256mapping(address=>mapping(address=>uint256))public rewards;
```

Params:
```
structCreateIncentiveParams{address rewardToken;address pool;uint256 startTime;uint256 endTime;uint128 totalReward;}structEndIncentiveParams{address creator;address rewardToken;address pool;uint256 startTime;uint256 endTime;}
```

## Incentives[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#incentives "Direct link to Incentives")
### `createIncentive(CreateIncentiveParams memory params)`[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#createincentivecreateincentiveparams-memory-params "Direct link to createincentivecreateincentiveparams-memory-params")
`createIncentive` creates a liquidity mining incentive program. The key used to look up an Incentive is the hash of its immutable properties.
**Check:**
  * Incentive with these params does not already exist
  * Timestamps: `params.endTime >= params.startTime`, `params.startTime >= block.timestamp`
  * Incentive with this ID does not already exist.


**Effects:**
  * Sets `incentives[key] = Incentive(totalRewardUnclaimed=totalReward, totalSecondsClaimedX128=0, rewardToken=rewardToken)`


**Interaction:**
  * Transfers `params.totalReward` from `msg.sender` to self. 
    * Make sure there is a check here and it fails if the transfer fails
  * Emits `IncentiveCreated`


### `endIncentive(EndIncentiveParams memory params)`[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#endincentiveendincentiveparams-memory-params "Direct link to endincentiveendincentiveparams-memory-params")
`endIncentive` can be called by anyone to end an Incentive after the `endTime` has passed, transferring `totalRewardUnclaimed` of `rewardToken` back to `refundee`.
**Check:**
  * `block.timestamp > params.endTime`
  * Incentive exists (`incentive.totalRewardUnclaimed != 0`)


**Effects:**
  * deletes `incentives[key]` (This is a new change)


**Interactions:**
  * safeTransfers `totalRewardUnclaimed` of `rewardToken` to the incentive creator `msg.sender`
  * emits `IncentiveEnded`


## Deposit/Withdraw Token[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#depositwithdraw-token "Direct link to Deposit/Withdraw Token")
**Interactions**
  * `nonfungiblePositionManager.safeTransferFrom(sender, this, tokenId)`
    * This transfer triggers the onERC721Received hook


### `onERC721Received(address, address from, uint256 tokenId, bytes calldata data)`[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#onerc721receivedaddress-address-from-uint256-tokenid-bytes-calldata-data "Direct link to onerc721receivedaddress-address-from-uint256-tokenid-bytes-calldata-data")
**Check:**
  * Make sure sender is univ3 nft


**Effects:**
  * Creates a deposit for the token setting deposit `owner` to `from`. 
    * Setting `owner` to `from` ensures that the owner of the token also owns the deposit. Approved addresses and operators may first transfer the token to themselves before depositing for deposit ownership.
  * If `data.length>0`, stakes the token in one or more incentives


### `withdrawToken(uint256 tokenId, address to, bytes memory data)`[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#withdrawtokenuint256-tokenid-address-to-bytes-memory-data "Direct link to withdrawtokenuint256-tokenid-address-to-bytes-memory-data")
**Checks**
  * Check that a Deposit exists for the token and that `msg.sender` is the `owner` on that Deposit.
  * Check that `numberOfStakes` on that Deposit is 0.


**Effects**
  * Delete the Deposit `delete deposits[tokenId]`.


**Interactions**
  * `safeTransferFrom` the token to `to` with `data`.
  * emit `DepositTransferred(token, deposit.owner, address(0))`


## Stake/Unstake/Rewards[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#stakeunstakerewards "Direct link to Stake/Unstake/Rewards")
### `stakeToken`[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#staketoken "Direct link to staketoken")
**Check:**
  * `deposits[params.tokenId].owner == msg.sender`
  * Make sure incentive actually exists and has reward that could be claimed (incentive.rewardToken != address(0)) 
    * Check if this check can check totalRewardUnclaimed instead
  * Make sure token not already staked


### `claimReward`[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#claimreward "Direct link to claimreward")
**Interactions**
  * `msg.sender` to withdraw all of their reward balance in a specific token to a specified `to` address.
  * emit RewardClaimed(to, reward)


### `unstakeToken`[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#unstaketoken "Direct link to unstaketoken")
To unstake an NFT, you call `unstakeToken`, which takes all the same arguments as `stake`.
**Checks**
  * It checks that you are the owner of the Deposit
  * It checks that there exists a `Stake` for the provided key (with exists=true).


**Effects**
  * Deletes the Stake.
  * Decrements `numberOfStakes` for the Deposit by 1.
  * `totalRewardsUnclaimed` is decremented by `reward`.
  * `totalSecondsClaimed` is incremented by `seconds`.
  * Increments `rewards[rewardToken][msg.sender]` by the amount given by `getRewardInfo`.


### `getRewardInfo`[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#getrewardinfo "Direct link to getrewardinfo")
  * It computes `secondsInsideX128` (the total liquidity seconds for which rewards are owed) for a given Stake, by:
    * using`snapshotCumulativesInside` from the Uniswap v3 core contract to get `secondsPerLiquidityInRangeX128`, and subtracting `secondsPerLiquidityInRangeInitialX128`.
    * Multiplying that by `stake.liquidity` to get the total seconds accrued by the liquidity in that period
  * Note that X128 means it's a `UQ32X128`.
  * It computes `totalSecondsUnclaimed` by taking `max(endTime, block.timestamp) - startTime`, casting it as a Q128, and subtracting `totalSecondsClaimedX128`.
  * It computes `rewardRate` for the Incentive casting `incentive.totalRewardUnclaimed` as a Q128, then dividing it by `totalSecondsUnclaimedX128`.
  * `reward` is then calculated as `secondsInsideX128` times the `rewardRate`, scaled down to a regular uint128.


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/staker/Design.md)
Was this helpful?
[PreviousWeightedOracleLibrary](https://docs.uniswap.org/contracts/v3/reference/periphery/libraries/WeightedOracleLibrary)[NextUniswap V3 Staker Contract](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker)
On this page
  * [Data Structures](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#data-structures)
  * [Incentives](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#incentives)
    * [`createIncentive(CreateIncentiveParams memory params)`](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#createincentivecreateincentiveparams-memory-params)
    * [`endIncentive(EndIncentiveParams memory params)`](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#endincentiveendincentiveparams-memory-params)
  * [Deposit/Withdraw Token](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#depositwithdraw-token)
    * [`onERC721Received(address, address from, uint256 tokenId, bytes calldata data)`](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#onerc721receivedaddress-address-from-uint256-tokenid-bytes-calldata-data)
    * [`withdrawToken(uint256 tokenId, address to, bytes memory data)`](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#withdrawtokenuint256-tokenid-address-to-bytes-memory-data)
  * [Stake/Unstake/Rewards](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#stakeunstakerewards)
    * [`stakeToken`](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#staketoken)
    * [`claimReward`](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#claimreward)
    * [`unstakeToken`](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#unstaketoken)
    * [`getRewardInfo`](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design#getrewardinfo)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/staker/Design.md)
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
