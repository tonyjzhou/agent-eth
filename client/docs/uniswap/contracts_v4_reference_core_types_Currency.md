# https://docs.uniswap.org/contracts/v4/reference/core/types/Currency

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#__docusaurus_skipToContent_fallback)
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
            * [BalanceDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta)
            * [BeforeSwapDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BeforeSwapDelta)
            * [Currency](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency)
            * [PoolId](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolId)
            * [PoolKey](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
            * [Slot0](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0)
            * [BalanceDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
            * [BeforeSwapDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/beforeswapdelta-guide)
            * [Currency Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
            * [PoolKey Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
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
  * Types
  * [Currency](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency)


On this page
# Currency
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/Currency.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
```
type Currency isaddress;
```

# CurrencyLibrary
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/Currency.sol)
_This library allows for transferring and holding native tokens and ERC20 tokens_
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#state-variables "Direct link to State Variables")
### ADDRESS_ZERO[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#address_zero "Direct link to ADDRESS_ZERO")
A constant to represent the native currency
```
Currency publicconstant ADDRESS_ZERO = Currency.wrap(address(0));
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#functions "Direct link to Functions")
### transfer[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#transfer "Direct link to transfer")
```
functiontransfer(Currency currency,address to,uint256 amount)internal;
```

### balanceOfSelf[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#balanceofself "Direct link to balanceOfSelf")
```
functionbalanceOfSelf(Currency currency)internalviewreturns(uint256);
```

### balanceOf[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#balanceof "Direct link to balanceOf")
```
functionbalanceOf(Currency currency,address owner)internalviewreturns(uint256);
```

### isAddressZero[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#isaddresszero "Direct link to isAddressZero")
```
functionisAddressZero(Currency currency)internalpurereturns(bool);
```

### toId[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#toid "Direct link to toId")
```
functiontoId(Currency currency)internalpurereturns(uint256);
```

### fromId[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#fromid "Direct link to fromId")
```
functionfromId(uint256 id)internalpurereturns(Currency);
```

## Errors[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#errors "Direct link to Errors")
### NativeTransferFailed[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#nativetransferfailed "Direct link to NativeTransferFailed")
Additional context for ERC-7751 wrapped error when a native transfer fails
```
error NativeTransferFailed();
```

### ERC20TransferFailed[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#erc20transferfailed "Direct link to ERC20TransferFailed")
Additional context for ERC-7751 wrapped error when an ERC20 transfer fails
```
error ERC20TransferFailed();
```

# greaterThanOrEqualTo
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/Currency.sol)
```
functiongreaterThanOrEqualTo(Currency currency, Currency other)purereturns(bool);
```

# lessThan
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/Currency.sol)
```
functionlessThan(Currency currency, Currency other)purereturns(bool);
```

# equals
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/Currency.sol)
```
functionequals(Currency currency, Currency other)purereturns(bool);
```

# greaterThan
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/Currency.sol)
```
functiongreaterThan(Currency currency, Currency other)purereturns(bool);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/Currency.md)
Was this helpful?
[PreviousBeforeSwapDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BeforeSwapDelta)[NextPoolId](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolId)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#state-variables)
    * [ADDRESS_ZERO](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#address_zero)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#functions)
    * [transfer](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#transfer)
    * [balanceOfSelf](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#balanceofself)
    * [balanceOf](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#balanceof)
    * [isAddressZero](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#isaddresszero)
    * [toId](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#toid)
    * [fromId](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#fromid)
  * [Errors](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#errors)
    * [NativeTransferFailed](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#nativetransferfailed)
    * [ERC20TransferFailed](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency#erc20transferfailed)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/Currency.md)
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
