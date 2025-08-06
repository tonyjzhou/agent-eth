# https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [test](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [PositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
          * [PositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [UniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/UniswapV4DeployerCompetition)
          * [V4Router](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [base](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [lens](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [utils](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Periphery
  * [PositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)


On this page
# PositionManager
[Git Source](https://github.com/uniswap/v4-periphery/blob/ea2bf2e1ba6863bb809fc2ff791744f308c4a26d/src/PositionManager.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [IPositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IPositionManager), [ERC721Permit_v4](https://docs.uniswap.org/contracts/v4/reference/periphery/base/ERC721Permit_v4), [PoolInitializer_v4](https://docs.uniswap.org/contracts/v4/reference/periphery/base/PoolInitializer_v4), [Multicall_v4](https://docs.uniswap.org/contracts/v4/reference/periphery/base/Multicall_v4), [DeltaResolver](https://docs.uniswap.org/contracts/v4/reference/periphery/base/DeltaResolver), [ReentrancyLock](https://docs.uniswap.org/contracts/v4/reference/periphery/base/ReentrancyLock), [BaseActionsRouter](https://docs.uniswap.org/contracts/v4/reference/periphery/base/BaseActionsRouter), [Notifier](https://docs.uniswap.org/contracts/v4/reference/periphery/base/Notifier), [Permit2Forwarder](https://docs.uniswap.org/contracts/v4/reference/periphery/base/Permit2Forwarder), [NativeWrapper](https://docs.uniswap.org/contracts/v4/reference/periphery/base/NativeWrapper)
The PositionManager (PosM) contract is responsible for creating liquidity positions on v4. PosM mints and manages ERC721 tokens associated with each position.
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#state-variables "Direct link to State Variables")
### nextTokenId[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#nexttokenid "Direct link to nextTokenId")
Used to get the ID that will be used for the next minted liquidity position
_The ID of the next token that will be minted. Skips 0_
```
uint256public nextTokenId =1;
```

### tokenDescriptor[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#tokendescriptor "Direct link to tokenDescriptor")
```
IPositionDescriptor public immutable tokenDescriptor;
```

### positionInfo[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#positioninfo "Direct link to positionInfo")
```
mapping(uint256 tokenId => PositionInfo info)public positionInfo;
```

### poolKeys[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#poolkeys "Direct link to poolKeys")
```
mapping(bytes25 poolId => PoolKey poolKey)public poolKeys;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#constructor "Direct link to constructor")
```
constructor(  IPoolManager _poolManager,  IAllowanceTransfer _permit2,uint256 _unsubscribeGasLimit,  IPositionDescriptor _tokenDescriptor,  IWETH9 _weth9)BaseActionsRouter(_poolManager)Permit2Forwarder(_permit2)ERC721Permit_v4("Uniswap v4 Positions NFT","UNI-V4-POSM")Notifier(_unsubscribeGasLimit)NativeWrapper(_weth9);
```

### checkDeadline[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#checkdeadline "Direct link to checkDeadline")
Reverts if the deadline has passed
```
modifiercheckDeadline(uint256 deadline);
```

**Parameters**
Name| Type| Description  
---|---|---  
`deadline`| `uint256`| The timestamp at which the call is no longer valid, passed in by the caller  
### onlyIfApproved[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#onlyifapproved "Direct link to onlyIfApproved")
Reverts if the caller is not the owner or approved for the ERC721 token
_either msg.sender or msgSender() is passed in as the caller msgSender() should ONLY be used if this is called from within the unlockCallback, unless the codepath has reentrancy protection_
```
modifieronlyIfApproved(address caller,uint256 tokenId) override;
```

**Parameters**
Name| Type| Description  
---|---|---  
`caller`| `address`| The address of the caller  
`tokenId`| `uint256`| the unique identifier of the ERC721 token  
### onlyIfPoolManagerLocked[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#onlyifpoolmanagerlocked "Direct link to onlyIfPoolManagerLocked")
Enforces that the PoolManager is locked.
```
modifieronlyIfPoolManagerLocked() override;
```

### tokenURI[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#tokenuri "Direct link to tokenURI")
```
functiontokenURI(uint256 tokenId)publicview override returns(stringmemory);
```

### modifyLiquidities[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#modifyliquidities "Direct link to modifyLiquidities")
Unlocks Uniswap v4 PoolManager and batches actions for modifying liquidity
_This is the standard entrypoint for the PositionManager_
```
functionmodifyLiquidities(bytescalldata unlockData,uint256 deadline)externalpayable  isNotLockedcheckDeadline(deadline);
```

**Parameters**
Name| Type| Description  
---|---|---  
`unlockData`| `bytes`| is an encoding of actions, and parameters for those actions  
`deadline`| `uint256`| is the deadline for the batched actions to be executed  
### modifyLiquiditiesWithoutUnlock[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#modifyliquiditieswithoutunlock "Direct link to modifyLiquiditiesWithoutUnlock")
Batches actions for modifying liquidity without unlocking v4 PoolManager
_This must be called by a contract that has already unlocked the v4 PoolManager_
```
functionmodifyLiquiditiesWithoutUnlock(bytescalldata actions,bytes[]calldata params)externalpayable isNotLocked;
```

**Parameters**
Name| Type| Description  
---|---|---  
`actions`| `bytes`| the actions to perform  
`params`| `bytes[]`| the parameters to provide for the actions  
### msgSender[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#msgsender "Direct link to msgSender")
function that returns address considered executor of the actions
_The other context functions, _msgData and _msgValue, are not supported by this contract In many contracts this will be the address that calls the initial entry point that calls`_executeActions` `msg.sender` shouldn't be used, as this will be the v4 pool manager contract that calls `unlockCallback` If using ReentrancyLock.sol, this function can return _getLocker()_
```
functionmsgSender()publicview override returns(address);
```

### _handleAction[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_handleaction "Direct link to _handleAction")
```
function_handleAction(uint256 action,bytescalldata params)internal virtual override;
```

### _increase[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_increase "Direct link to _increase")
_Calling increase with 0 liquidity will credit the caller with any underlying fees of the position_
```
function_increase(uint256 tokenId,uint256 liquidity,uint128 amount0Max,uint128 amount1Max,bytescalldata hookData)internalonlyIfApproved(msgSender(), tokenId);
```

### _increaseFromDeltas[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_increasefromdeltas "Direct link to _increaseFromDeltas")
_The liquidity delta is derived from open deltas in the pool manager._
```
function_increaseFromDeltas(uint256 tokenId,uint128 amount0Max,uint128 amount1Max,bytescalldata hookData)internalonlyIfApproved(msgSender(), tokenId);
```

### _decrease[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_decrease "Direct link to _decrease")
_Calling decrease with 0 liquidity will credit the caller with any underlying fees of the position_
```
function_decrease(uint256 tokenId,uint256 liquidity,uint128 amount0Min,uint128 amount1Min,bytescalldata hookData)internalonlyIfApproved(msgSender(), tokenId);
```

### _mint[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_mint "Direct link to _mint")
```
function_mint(  PoolKey calldata poolKey,int24 tickLower,int24 tickUpper,uint256 liquidity,uint128 amount0Max,uint128 amount1Max,address owner,bytescalldata hookData)internal;
```

### _mintFromDeltas[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_mintfromdeltas "Direct link to _mintFromDeltas")
```
function_mintFromDeltas(  PoolKey calldata poolKey,int24 tickLower,int24 tickUpper,uint128 amount0Max,uint128 amount1Max,address owner,bytescalldata hookData)internal;
```

### _burn[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_burn "Direct link to _burn")
_this is overloaded with ERC721Permit_v4._burn_
```
function_burn(uint256 tokenId,uint128 amount0Min,uint128 amount1Min,bytescalldata hookData)internalonlyIfApproved(msgSender(), tokenId);
```

### _settlePair[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_settlepair "Direct link to _settlePair")
```
function_settlePair(Currency currency0, Currency currency1)internal;
```

### _takePair[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_takepair "Direct link to _takePair")
```
function_takePair(Currency currency0, Currency currency1,address recipient)internal;
```

### _close[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_close "Direct link to _close")
```
function_close(Currency currency)internal;
```

### _clearOrTake[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_clearortake "Direct link to _clearOrTake")
_integrators may elect to forfeit positive deltas with clear if the forfeit amount exceeds the user-specified max, the amount is taken instead if there is no credit, no call is made._
```
function_clearOrTake(Currency currency,uint256 amountMax)internal;
```

### _sweep[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_sweep "Direct link to _sweep")
Sweeps the entire contract balance of specified currency to the recipient
```
function_sweep(Currency currency,address to)internal;
```

### _modifyLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_modifyliquidity "Direct link to _modifyLiquidity")
_if there is a subscriber attached to the position, this function will notify the subscriber_
```
function_modifyLiquidity(  PositionInfo info,  PoolKey memory poolKey,int256 liquidityChange,bytes32 salt,bytescalldata hookData)internalreturns(BalanceDelta liquidityDelta, BalanceDelta feesAccrued);
```

### _pay[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_pay "Direct link to _pay")
```
function_pay(Currency currency,address payer,uint256 amount)internal override;
```

### _setSubscribed[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_setsubscribed "Direct link to _setSubscribed")
an internal helper used by Notifier
```
function_setSubscribed(uint256 tokenId)internal override;
```

### _setUnsubscribed[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_setunsubscribed "Direct link to _setUnsubscribed")
an internal helper used by Notifier
```
function_setUnsubscribed(uint256 tokenId)internal override;
```

### transferFrom[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#transferfrom "Direct link to transferFrom")
_overrides solmate transferFrom in case a notification to subscribers is needed_
_will revert if pool manager is locked_
```
functiontransferFrom(addressfrom,address to,uint256 id)public virtual override onlyIfPoolManagerLocked;
```

### getPoolAndPositionInfo[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#getpoolandpositioninfo "Direct link to getPoolAndPositionInfo")
Returns the pool key and position info of a position
```
functiongetPoolAndPositionInfo(uint256 tokenId)publicviewreturns(PoolKey memory poolKey, PositionInfo info);
```

**Parameters**
Name| Type| Description  
---|---|---  
`tokenId`| `uint256`| the ERC721 tokenId  
**Returns**
Name| Type| Description  
---|---|---  
`poolKey`| `PoolKey`| the pool key of the position  
`info`| `PositionInfo`| a uint256 packed value holding information about the position including the range (tickLower, tickUpper)  
### getPositionLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#getpositionliquidity "Direct link to getPositionLiquidity")
Returns the liquidity of a position
_this value can be processed as an amount0 and amount1 by using the LiquidityAmounts library_
```
functiongetPositionLiquidity(uint256 tokenId)externalviewreturns(uint128 liquidity);
```

**Parameters**
Name| Type| Description  
---|---|---  
`tokenId`| `uint256`| the ERC721 tokenId  
**Returns**
Name| Type| Description  
---|---|---  
`liquidity`| `uint128`| the position's liquidity, as a liquidityAmount  
### _getLiquidity[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_getliquidity "Direct link to _getLiquidity")
```
function_getLiquidity(uint256 tokenId, PoolKey memory poolKey,int24 tickLower,int24 tickUpper)internalviewreturns(uint128 liquidity);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/PositionManager.md)
Was this helpful?
[PreviousPositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)[NextUniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/UniswapV4DeployerCompetition)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#state-variables)
    * [nextTokenId](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#nexttokenid)
    * [tokenDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#tokendescriptor)
    * [positionInfo](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#positioninfo)
    * [poolKeys](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#poolkeys)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#constructor)
    * [checkDeadline](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#checkdeadline)
    * [onlyIfApproved](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#onlyifapproved)
    * [onlyIfPoolManagerLocked](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#onlyifpoolmanagerlocked)
    * [tokenURI](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#tokenuri)
    * [modifyLiquidities](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#modifyliquidities)
    * [modifyLiquiditiesWithoutUnlock](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#modifyliquiditieswithoutunlock)
    * [msgSender](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#msgsender)
    * [_handleAction](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_handleaction)
    * [_increase](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_increase)
    * [_increaseFromDeltas](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_increasefromdeltas)
    * [_decrease](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_decrease)
    * [_mint](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_mint)
    * [_mintFromDeltas](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_mintfromdeltas)
    * [_burn](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_burn)
    * [_settlePair](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_settlepair)
    * [_takePair](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_takepair)
    * [_close](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_close)
    * [_clearOrTake](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_clearortake)
    * [_sweep](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_sweep)
    * [_modifyLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_modifyliquidity)
    * [_pay](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_pay)
    * [_setSubscribed](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_setsubscribed)
    * [_setUnsubscribed](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_setunsubscribed)
    * [transferFrom](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#transferfrom)
    * [getPoolAndPositionInfo](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#getpoolandpositioninfo)
    * [getPositionLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#getpositionliquidity)
    * [_getLiquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager#_getliquidity)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/PositionManager.md)
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
