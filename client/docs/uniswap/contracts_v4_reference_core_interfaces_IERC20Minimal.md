# https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#__docusaurus_skipToContent_fallback)
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
  * [IERC20Minimal](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)


On this page
# IERC20Minimal
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/interfaces/external/IERC20Minimal.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
Contains a subset of the full ERC20 interface that is used in Uniswap V3
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#functions "Direct link to Functions")
### balanceOf[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#balanceof "Direct link to balanceOf")
Returns an account's balance in the token
```
functionbalanceOf(address account)externalviewreturns(uint256);
```

**Parameters**
Name| Type| Description  
---|---|---  
`account`| `address`| The account for which to look up the number of tokens it has, i.e. its balance  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `uint256`| The number of tokens held by the account  
### transfer[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#transfer "Direct link to transfer")
Transfers the amount of token from the `msg.sender` to the recipient
```
functiontransfer(address recipient,uint256 amount)externalreturns(bool);
```

**Parameters**
Name| Type| Description  
---|---|---  
`recipient`| `address`| The account that will receive the amount transferred  
`amount`| `uint256`| The number of tokens to send from the sender to the recipient  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bool`| Returns true for a successful transfer, false for an unsuccessful transfer  
### allowance[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#allowance "Direct link to allowance")
Returns the current allowance given to a spender by an owner
```
functionallowance(address owner,address spender)externalviewreturns(uint256);
```

**Parameters**
Name| Type| Description  
---|---|---  
`owner`| `address`| The account of the token owner  
`spender`| `address`| The account of the token spender  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `uint256`| The current allowance granted by `owner` to `spender`  
### approve[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#approve "Direct link to approve")
Sets the allowance of a spender from the `msg.sender` to the value `amount`
```
functionapprove(address spender,uint256 amount)externalreturns(bool);
```

**Parameters**
Name| Type| Description  
---|---|---  
`spender`| `address`| The account which will be allowed to spend a given amount of the owners tokens  
`amount`| `uint256`| The amount of tokens allowed to be used by `spender`  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bool`| Returns true for a successful approval, false for unsuccessful  
### transferFrom[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#transferfrom "Direct link to transferFrom")
Transfers `amount` tokens from `sender` to `recipient` up to the allowance given to the `msg.sender`
```
functiontransferFrom(address sender,address recipient,uint256 amount)externalreturns(bool);
```

**Parameters**
Name| Type| Description  
---|---|---  
`sender`| `address`| The account from which the transfer will be initiated  
`recipient`| `address`| The recipient of the transfer  
`amount`| `uint256`| The amount of the transfer  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bool`| Returns true for a successful transfer, false for unsuccessful  
## Events[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#events "Direct link to Events")
### Transfer[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#transfer-1 "Direct link to Transfer")
Event emitted when tokens are transferred from one address to another, either via `#transfer` or `#transferFrom`.
```
eventTransfer(addressindexedfrom,addressindexed to,uint256 value);
```

**Parameters**
Name| Type| Description  
---|---|---  
`from`| `address`| The account from which the tokens were sent, i.e. the balance decreased  
`to`| `address`| The account to which the tokens were sent, i.e. the balance increased  
`value`| `uint256`| The amount of tokens that were transferred  
### Approval[​](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#approval "Direct link to Approval")
Event emitted when the approval amount for the spender of a given owner's tokens changes.
```
eventApproval(addressindexed owner,addressindexed spender,uint256 value);
```

**Parameters**
Name| Type| Description  
---|---|---  
`owner`| `address`| The account that approved spending of its tokens  
`spender`| `address`| The account for which the spending allowance was modified  
`value`| `uint256`| The new allowance from the owner to the spender  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IERC20Minimal.md)
Was this helpful?
[PreviousProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)[NextIERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC6909Claims)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#functions)
    * [balanceOf](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#balanceof)
    * [transfer](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#transfer)
    * [allowance](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#allowance)
    * [approve](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#approve)
    * [transferFrom](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#transferfrom)
  * [Events](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#events)
    * [Transfer](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#transfer-1)
    * [Approval](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal#approval)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/interfaces/IERC20Minimal.md)
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
