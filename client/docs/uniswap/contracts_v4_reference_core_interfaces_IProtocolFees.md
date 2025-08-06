# https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
            * [IERC20Minimal](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
            * [IERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims)
            * [IExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload)
            * [IExttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload)
            * [IHooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
            * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
            * [IProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
            * [IUnlockCallback](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IUnlockCallback)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * interfaces
  * [IProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)


On this page
# IProtocolFees
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/interfaces/IProtocolFees.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
Interface for all protocol-fee related functions in the pool manager
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#functions "Direct link to Functions")
### protocolFeesAccrued[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeesaccrued "Direct link to protocolFeesAccrued")
Given a currency address, returns the protocol fees accrued in that currency
```
functionprotocolFeesAccrued(Currency currency)externalviewreturns(uint256 amount);
```

**Parameters**
Name| Type| Description  
---|---|---  
`currency`| `Currency`| The currency to check  
**Returns**
Name| Type| Description  
---|---|---  
`amount`| `uint256`| The amount of protocol fees accrued in the currency  
### setProtocolFee[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#setprotocolfee "Direct link to setProtocolFee")
Sets the protocol fee for the given pool
```
functionsetProtocolFee(PoolKey memory key,uint24 newProtocolFee)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`key`| `PoolKey`| The key of the pool to set a protocol fee for  
`newProtocolFee`| `uint24`| The fee to set  
### setProtocolFeeController[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#setprotocolfeecontroller "Direct link to setProtocolFeeController")
Sets the protocol fee controller
```
functionsetProtocolFeeController(address controller)external;
```

**Parameters**
Name| Type| Description  
---|---|---  
`controller`| `address`| The new protocol fee controller  
### collectProtocolFees[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#collectprotocolfees "Direct link to collectProtocolFees")
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
### protocolFeeController[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeecontroller "Direct link to protocolFeeController")
Returns the current protocol fee controller address
```
functionprotocolFeeController()externalviewreturns(address);
```

**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `address`| address The current protocol fee controller address  
## Events[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#events "Direct link to Events")
### ProtocolFeeControllerUpdated[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeecontrollerupdated "Direct link to ProtocolFeeControllerUpdated")
Emitted when the protocol fee controller address is updated in setProtocolFeeController.
```
eventProtocolFeeControllerUpdated(addressindexed protocolFeeController);
```

### ProtocolFeeUpdated[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeeupdated "Direct link to ProtocolFeeUpdated")
Emitted when the protocol fee is updated for a pool.
```
eventProtocolFeeUpdated(PoolId indexed id,uint24 protocolFee);
```

## Errors[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#errors "Direct link to Errors")
### ProtocolFeeTooLarge[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeetoolarge "Direct link to ProtocolFeeTooLarge")
Thrown when protocol fee is set too high
```
error ProtocolFeeTooLarge(uint24 fee);
```

### InvalidCaller[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#invalidcaller "Direct link to InvalidCaller")
Thrown when collectProtocolFees or setProtocolFee is not called by the controller.
```
error InvalidCaller();
```

### ProtocolFeeCurrencySynced[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeecurrencysynced "Direct link to ProtocolFeeCurrencySynced")
Thrown when collectProtocolFees is attempted on a token that is synced.
```
error ProtocolFeeCurrencySynced();
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IProtocolFees.md)
Was this helpful?
[PreviousIPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)[NextIUnlockCallback](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IUnlockCallback)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#functions)
    * [protocolFeesAccrued](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeesaccrued)
    * [setProtocolFee](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#setprotocolfee)
    * [setProtocolFeeController](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#setprotocolfeecontroller)
    * [collectProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#collectprotocolfees)
    * [protocolFeeController](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeecontroller)
  * [Events](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#events)
    * [ProtocolFeeControllerUpdated](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeecontrollerupdated)
    * [ProtocolFeeUpdated](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeeupdated)
  * [Errors](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#errors)
    * [ProtocolFeeTooLarge](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeetoolarge)
    * [InvalidCaller](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#invalidcaller)
    * [ProtocolFeeCurrencySynced](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees#protocolfeecurrencysynced)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IProtocolFees.md)
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
