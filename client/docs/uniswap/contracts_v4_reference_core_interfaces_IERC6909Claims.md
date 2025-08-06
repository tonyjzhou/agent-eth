# https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#__docusaurus_skipToContent_fallback)
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
            * [IERC20Minimal](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
            * [IERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims)
            * [IExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload)
            * [IExttload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExttload)
            * [IHooks](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IHooks)
            * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IPoolManager)
            * [IProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IProtocolFees)
            * [IUnlockCallback](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IUnlockCallback)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
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
  * Core
  * interfaces
  * [IERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims)


On this page
# IERC6909Claims
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/interfaces/external/IERC6909Claims.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
Interface for claims over a contract balance, wrapped as a ERC6909
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#functions "Direct link to Functions")
### balanceOf[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#balanceof "Direct link to balanceOf")
Owner balance of an id.
```
functionbalanceOf(address owner,uint256 id)externalviewreturns(uint256 amount);
```

**Parameters**
Name| Type| Description  
---|---|---  
`owner`| `address`| The address of the owner.  
`id`| `uint256`| The id of the token.  
**Returns**
Name| Type| Description  
---|---|---  
`amount`| `uint256`| The balance of the token.  
### allowance[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#allowance "Direct link to allowance")
Spender allowance of an id.
```
functionallowance(address owner,address spender,uint256 id)externalviewreturns(uint256 amount);
```

**Parameters**
Name| Type| Description  
---|---|---  
`owner`| `address`| The address of the owner.  
`spender`| `address`| The address of the spender.  
`id`| `uint256`| The id of the token.  
**Returns**
Name| Type| Description  
---|---|---  
`amount`| `uint256`| The allowance of the token.  
### isOperator[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#isoperator "Direct link to isOperator")
Checks if a spender is approved by an owner as an operator
```
functionisOperator(address owner,address spender)externalviewreturns(bool approved);
```

**Parameters**
Name| Type| Description  
---|---|---  
`owner`| `address`| The address of the owner.  
`spender`| `address`| The address of the spender.  
**Returns**
Name| Type| Description  
---|---|---  
`approved`| `bool`| The approval status.  
### transfer[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#transfer "Direct link to transfer")
Transfers an amount of an id from the caller to a receiver.
```
functiontransfer(address receiver,uint256 id,uint256 amount)externalreturns(bool);
```

**Parameters**
Name| Type| Description  
---|---|---  
`receiver`| `address`| The address of the receiver.  
`id`| `uint256`| The id of the token.  
`amount`| `uint256`| The amount of the token.  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bool`| bool True, always, unless the function reverts  
### transferFrom[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#transferfrom "Direct link to transferFrom")
Transfers an amount of an id from a sender to a receiver.
```
functiontransferFrom(address sender,address receiver,uint256 id,uint256 amount)externalreturns(bool);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The address of the sender.  
`receiver`| `address`| The address of the receiver.  
`id`| `uint256`| The id of the token.  
`amount`| `uint256`| The amount of the token.  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bool`| bool True, always, unless the function reverts  
### approve[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#approve "Direct link to approve")
Approves an amount of an id to a spender.
```
functionapprove(address spender,uint256 id,uint256 amount)externalreturns(bool);
```

**Parameters**
Name| Type| Description  
---|---|---  
`spender`| `address`| The address of the spender.  
`id`| `uint256`| The id of the token.  
`amount`| `uint256`| The amount of the token.  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bool`| bool True, always  
### setOperator[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#setoperator "Direct link to setOperator")
Sets or removes an operator for the caller.
```
functionsetOperator(address operator,bool approved)externalreturns(bool);
```

**Parameters**
Name| Type| Description  
---|---|---  
`operator`| `address`| The address of the operator.  
`approved`| `bool`| The approval status.  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bool`| bool True, always  
## Events[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#events "Direct link to Events")
### OperatorSet[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#operatorset "Direct link to OperatorSet")
```
eventOperatorSet(addressindexed owner,addressindexed operator,bool approved);
```

### Approval[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#approval "Direct link to Approval")
```
eventApproval(addressindexed owner,addressindexed spender,uint256indexed id,uint256 amount);
```

### Transfer[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#transfer-1 "Direct link to Transfer")
```
eventTransfer(address caller,addressindexedfrom,addressindexed to,uint256indexed id,uint256 amount);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IERC6909Claims.md)
Was this helpful?
[PreviousIERC20Minimal](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)[NextIExtsload](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IExtsload)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#functions)
    * [balanceOf](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#balanceof)
    * [allowance](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#allowance)
    * [isOperator](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#isoperator)
    * [transfer](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#transfer)
    * [transferFrom](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#transferfrom)
    * [approve](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#approve)
    * [setOperator](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#setoperator)
  * [Events](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#events)
    * [OperatorSet](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#operatorset)
    * [Approval](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#approval)
    * [Transfer](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims#transfer-1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IERC6909Claims.md)
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
