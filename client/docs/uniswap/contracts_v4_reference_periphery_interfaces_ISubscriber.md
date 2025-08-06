# https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#__docusaurus_skipToContent_fallback)
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
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
          * [PositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
          * [PositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [UniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/UniswapV4DeployerCompetition)
          * [V4Router](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [base](https://docs.uniswap.org/contracts/v4/reference/periphery/base/BaseActionsRouter)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IEIP712_v4)
            * [IEIP712_v4](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IEIP712_v4)
            * [IERC721Permit_v4](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IERC721Permit_v4)
            * [IImmutableState](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IImmutableState)
            * [IMulticall_v4](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IMulticall_v4)
            * [INotifier](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/INotifier)
            * [IPermit2Forwarder](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IPermit2Forwarder)
            * [IPoolInitializer_v4](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IPoolInitializer_v4)
            * [IPositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IPositionDescriptor)
            * [IPositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IPositionManager)
            * [IStateView](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IStateView)
            * [ISubscriber](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber)
            * [IUniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IUniswapV4DeployerCompetition)
            * [IUnorderedNonce](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IUnorderedNonce)
            * [IV4Quoter](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IV4Quoter)
            * [IV4Router](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IV4Router)
            * [IWETH9](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IWETH9)
          * [lens](https://docs.uniswap.org/contracts/v4/reference/periphery/lens/StateView)
          * [libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/ActionConstants)
          * [utils](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
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
  * v4 Protocol
  * Technical Reference
  * Periphery
  * interfaces
  * [ISubscriber](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber)


On this page
# ISubscriber
[Git Source](https://github.com/uniswap/v4-periphery/blob/ea2bf2e1ba6863bb809fc2ff791744f308c4a26d/src/interfaces/ISubscriber.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
Interface that a Subscriber contract should implement to receive updates from the v4 position manager
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#functions "Direct link to Functions")
### notifySubscribe[​](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#notifysubscribe "Direct link to notifySubscribe")
Called when a position subscribes to this subscriber contract
```
functionnotifySubscribe(uint256 tokenId,bytesmemory data)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`tokenId`| `uint256`| the token ID of the position  
`data`| `bytes`| additional data passed in by the caller  
### notifyUnsubscribe[​](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#notifyunsubscribe "Direct link to notifyUnsubscribe")
Called when a position unsubscribes from the subscriber
_This call's gas is capped at`unsubscribeGasLimit` (set at deployment)_
_Because of EIP-150, solidity may only allocate 63/64 of gasleft()_
```
functionnotifyUnsubscribe(uint256 tokenId)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`tokenId`| `uint256`| the token ID of the position  
### notifyBurn[​](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#notifyburn "Direct link to notifyBurn")
Called when a position is burned
```
functionnotifyBurn(uint256 tokenId,address owner, PositionInfo info,uint256 liquidity, BalanceDelta feesAccrued)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`tokenId`| `uint256`| the token ID of the position  
`owner`| `address`| the current owner of the tokenId  
`info`| `PositionInfo`| information about the position  
`liquidity`| `uint256`| the amount of liquidity decreased in the position, may be 0  
`feesAccrued`| `BalanceDelta`| the fees accrued by the position if liquidity was decreased  
### notifyModifyLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#notifymodifyliquidity "Direct link to notifyModifyLiquidity")
Called when a position modifies its liquidity or collects fees
_Note that feesAccrued can be artificially inflated by a malicious user Pools with a single liquidity position can inflate feeGrowthGlobal (and consequently feesAccrued) by donating to themselves; atomically donating and collecting fees within the same unlockCallback may further inflate feeGrowthGlobal/feesAccrued_
```
functionnotifyModifyLiquidity(uint256 tokenId,int256 liquidityChange, BalanceDelta feesAccrued)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`tokenId`| `uint256`| the token ID of the position  
`liquidityChange`| `int256`| the change in liquidity on the underlying position  
`feesAccrued`| `BalanceDelta`| the fees to be collected from the position as a result of the modifyLiquidity call  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/interfaces/ISubscriber.md)
Was this helpful?
[PreviousIStateView](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IStateView)[NextIUniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IUniswapV4DeployerCompetition)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#functions)
    * [notifySubscribe](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#notifysubscribe)
    * [notifyUnsubscribe](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#notifyunsubscribe)
    * [notifyBurn](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#notifyburn)
    * [notifyModifyLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/ISubscriber#notifymodifyliquidity)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/interfaces/ISubscriber.md)
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
