# https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/periphery/base/BlockTimestamp)
          * [Base](https://docs.uniswap.org/contracts/v3/reference/periphery/base/BlockTimestamp)
          * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/IERC20Metadata)
          * [Lens](https://docs.uniswap.org/contracts/v3/reference/periphery/lens/Quoter)
          * [Libraries](https://docs.uniswap.org/contracts/v3/reference/periphery/libraries/Base64)
          * [Staker](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
            * [Uniswap V3 Staker Design](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)
            * [Uniswap V3 Staker Contract](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker)
            * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/interfaces/IUniswapV3Staker)
            * [Libraries](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/libraries/IncentiveId)
          * [Test](https://docs.uniswap.org/contracts/v3/reference/periphery/test/Base64Test)
          * [NonfungiblePositionManager](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager)
          * [NonfungibleTokenPositionDescriptor](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungibleTokenPositionDescriptor)
          * [SwapRouter](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter)
          * [V3Migrator](https://docs.uniswap.org/contracts/v3/reference/periphery/V3Migrator)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * Periphery
  * Staker
  * [Uniswap V3 Staker Contract](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker)


On this page
# Uniswap V3 Staker Contract
Below is the technical reference for the staker contract, [`UniswapV3Staker.sol`](https://github.com/Uniswap/uniswap-v3-staker/blob/main/contracts/UniswapV3Staker.sol). A technical guide for interacting with this staking contract will be released soon.
## Functions[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#functions "Direct link to Functions")
### stakes[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#stakes "Direct link to stakes")
```
functionstakes(uint256 tokenId,bytes32 incentiveId)publicview override returns(uint160 secondsPerLiquidityInsideInitialX128,uint128 liquidity)
```

Returns information about a staked liquidity NFT
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenId`| uint256| The ID of the staked token  
`incentiveId`| bytes32| The ID of the incentive for which the token is staked  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#return-values "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`secondsPerLiquidityInsideInitialX128`| uint160| secondsPerLiquidity represented as a UQ32.128  
`liquidity`| bytes32| The amount of liquidity in the NFT as of the last time the rewards were computed  
### constructor[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#constructor "Direct link to constructor")
```
functionconstructor(contractIUniswapV3Factory _factory,contractINonfungiblePositionManager _nonfungiblePositionManager,uint256 _maxIncentiveStartLeadTime,uint256 _maxIncentiveDuration)public
```

#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-1 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`_factory`| contract IUniswapV3Factory| the Uniswap V3 factory  
`_nonfungiblePositionManager`| contract INonfungiblePositionManager| the NFT position manager contract address  
`_maxIncentiveStartLeadTime`| uint256| the max duration of an incentive in seconds  
`_maxIncentiveDuration`| uint256| the max amount of seconds into the future the incentive startTime can be set  
### createIncentive[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#createincentive "Direct link to createIncentive")
```
functioncreateIncentive(structIUniswapV3Staker.IncentiveKey key,uint256 reward)external
```

Creates a new liquidity mining incentive program
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-2 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`key`| struct IUniswapV3Staker.IncentiveKey| Details of the incentive to create  
`reward`| uint256| The amount of reward tokens to be distributed  
### endIncentive[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#endincentive "Direct link to endIncentive")
```
functionendIncentive(structIUniswapV3Staker.IncentiveKey key)externalreturns(uint256 refund)
```

Ends an incentive after the incentive end time has passed and all stakes have been withdrawn
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-3 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`key`| struct IUniswapV3Staker.IncentiveKey| Details of the incentive to end  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#return-values-1 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`refund`| uint256| The remaining reward tokens when the incentive is ended  
### onERC721Received[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#onerc721received "Direct link to onERC721Received")
```
functiononERC721Received()externalreturns(bytes4)
```

Upon receiving a Uniswap V3 ERC721, creates the token deposit setting owner to `from`. Also stakes token in one or more incentives if properly formatted `data` has a length > 0.
Whenever an {IERC721} `tokenId` token is transferred to this contract via {IERC721-safeTransferFrom} by `operator` from `from`, this function is called. It must return its Solidity selector to confirm the token transfer. If any other value is returned or the interface is not implemented by the recipient, the transfer will be reverted. The selector can be obtained in Solidity with `IERC721.onERC721Received.selector`.
### transferDeposit[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#transferdeposit "Direct link to transferDeposit")
```
functiontransferDeposit(uint256 tokenId,address to)external
```

Transfers ownership of a deposit from the sender to the given recipient
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-4 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenId`| uint256| The ID of the token (and the deposit) to transfer  
`to`| address| The new owner of the deposit  
### withdrawToken[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#withdrawtoken "Direct link to withdrawToken")
```
functionwithdrawToken(uint256 tokenId,address to,bytes data)external
```

Withdraws a Uniswap V3 LP token `tokenId` from this contract to the recipient `to`
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-5 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenId`| uint256| The unique identifier of an Uniswap V3 LP token  
`to`| address| The address where the LP token will be sent  
`data`| bytes| An optional data array that will be passed along to the `to` address via the NFT safeTransferFrom  
### stakeToken[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#staketoken "Direct link to stakeToken")
```
functionstakeToken(structIUniswapV3Staker.IncentiveKey key,uint256 tokenId)external
```

Stakes a Uniswap V3 LP token
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-6 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`key`| struct IUniswapV3Staker.IncentiveKey| The key of the incentive for which to stake the NFT  
`tokenId`| uint256| The ID of the token to stake  
### unstakeToken[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#unstaketoken "Direct link to unstakeToken")
```
functionunstakeToken(structIUniswapV3Staker.IncentiveKey key,uint256 tokenId)external
```

Unstakes a Uniswap V3 LP token
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-7 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`key`| struct IUniswapV3Staker.IncentiveKey| The key of the incentive for which to unstake the NFT  
`tokenId`| uint256| The ID of the token to unstake  
### claimReward[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#claimreward "Direct link to claimReward")
```
functionclaimReward(contractIERC20Minimal rewardToken,address to,uint256 amountRequested)external override returns(uint256 reward)
```

Transfers `amountRequested` of accrued `rewardToken` rewards from the contract to the recipient `to`
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-8 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`rewardToken`| contract IERC20Minimal| The token being distributed as a reward  
`to`| address| The address where claimed rewards will be sent to  
`amountRequested`| uint256| The amount of reward tokens to claim. Claims entire reward amount if set to 0.  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#return-values-2 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`reward`| uint256| The amount of reward tokens claimed  
### getRewardInfo[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#getrewardinfo "Direct link to getRewardInfo")
```
functiongetRewardInfo(structIUniswapV3Staker.IncentiveKey key,uint256 tokenId)externalview override returns(uint256 reward,uint160 secondsInsideX128)
```

Calculates the reward amount that will be received for the given stake
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#parameters-9 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`key`| struct IUniswapV3Staker.IncentiveKey| The key of the incentive  
`tokenId`| uint256| The ID of the token  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#return-values-3 "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`reward`| uint256| The reward accrued to the NFT for the given incentive thus far  
`secondsInsideX128`| uint160| The seconds inside the tick range  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/staker/UniswapV3Staker.md)
Was this helpful?
[PreviousUniswap V3 Staker Design](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/Design)[NextIUniswapV3Staker](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/interfaces/IUniswapV3Staker)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#functions)
    * [stakes](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#stakes)
    * [constructor](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#constructor)
    * [createIncentive](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#createincentive)
    * [endIncentive](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#endincentive)
    * [onERC721Received](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#onerc721received)
    * [transferDeposit](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#transferdeposit)
    * [withdrawToken](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#withdrawtoken)
    * [stakeToken](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#staketoken)
    * [unstakeToken](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#unstaketoken)
    * [claimReward](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#claimreward)
    * [getRewardInfo](https://docs.uniswap.org/contracts/v3/reference/periphery/staker/UniswapV3Staker#getrewardinfo)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/periphery/staker/UniswapV3Staker.md)
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
