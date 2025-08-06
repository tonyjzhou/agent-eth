# https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)


On this page
# ProtocolFees
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/ProtocolFees.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [IProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees), Owned
Contract handling the setting and accrual of protocol fees
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#state-variables "Direct link to State Variables")
### protocolFeesAccrued[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#protocolfeesaccrued "Direct link to protocolFeesAccrued")
Given a currency address, returns the protocol fees accrued in that currency
```
mapping(Currency currency =>uint256 amount)public protocolFeesAccrued;
```

### protocolFeeController[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#protocolfeecontroller "Direct link to protocolFeeController")
Returns the current protocol fee controller address
```
addresspublic protocolFeeController;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#constructor "Direct link to constructor")
```
constructor(address initialOwner)Owned(initialOwner);
```

### setProtocolFeeController[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#setprotocolfeecontroller "Direct link to setProtocolFeeController")
Sets the protocol fee controller
```
functionsetProtocolFeeController(address controller)external onlyOwner;
```

**Parameters**
Name| Type| Description  
---|---|---  
`controller`| `address`| The new protocol fee controller  
### setProtocolFee[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#setprotocolfee "Direct link to setProtocolFee")
Sets the protocol fee for the given pool
```
functionsetProtocolFee(PoolKey memory key,uint24 newProtocolFee)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The key of the pool to set a protocol fee for  
`newProtocolFee`| `uint24`| The fee to set  
### collectProtocolFees[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#collectprotocolfees "Direct link to collectProtocolFees")
Collects the protocol fees for a given recipient and currency, returning the amount collected
_This will revert if the contract is unlocked_
```
functioncollectProtocolFees(address recipient, Currency currency,uint256 amount)externalreturns(uint256 amountCollected);
```

**Parameters**
Name| Type| Description  
---|---|---  
`recipient`| `address`| The address to receive the protocol fees  
`currency`| `Currency`| The currency to withdraw  
`amount`| `uint256`| The amount of currency to withdraw  
**Returns**
Name| Type| Description  
---|---|---  
`amountCollected`| `uint256`| The amount of currency successfully withdrawn  
### _isUnlocked[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#_isunlocked "Direct link to _isUnlocked")
_abstract internal function to allow the ProtocolFees contract to access the lock_
```
function_isUnlocked()internal virtual returns(bool);
```

### _getPool[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#_getpool "Direct link to _getPool")
_abstract internal function to allow the ProtocolFees contract to access pool state_
_this is overridden in PoolManager.sol to give access to the _pools mapping_
```
function_getPool(PoolId id)internal virtual returns(Pool.State storage);
```

### _updateProtocolFees[​](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#_updateprotocolfees "Direct link to _updateProtocolFees")
```
function_updateProtocolFees(Currency currency,uint256 amount)internal;
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/ProtocolFees.md)
Was this helpful?
[PreviousPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)[NextIERC20Minimal](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#state-variables)
    * [protocolFeesAccrued](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#protocolfeesaccrued)
    * [protocolFeeController](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#protocolfeecontroller)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#constructor)
    * [setProtocolFeeController](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#setprotocolfeecontroller)
    * [setProtocolFee](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#setprotocolfee)
    * [collectProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#collectprotocolfees)
    * [_isUnlocked](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#_isunlocked)
    * [_getPool](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#_getpool)
    * [_updateProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees#_updateprotocolfees)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/ProtocolFees.md)
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
