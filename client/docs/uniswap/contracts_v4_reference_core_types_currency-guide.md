# https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
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
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Core
  * Types
  * [Currency Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)


On this page
# Currency Guide
`Currency` is a custom type that represents either native currency (ETH) or ERC20 tokens.
## Type Definition[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#type-definition "Direct link to Type Definition")
```
type Currency isaddress;
```

## Global Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#global-functions "Direct link to Global Functions")
### equals[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#equals "Direct link to equals")
```
functionequals(Currency currency, Currency other)purereturns(bool)
```

Checks if two `Currency` values are equal.
Param Name| Type| Description  
---|---|---  
currency| Currency| The first Currency value  
other| Currency| The second Currency value  
Returns `true` if the Currency values are equal, `false` otherwise.
### greaterThan[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#greaterthan "Direct link to greaterThan")
```
functiongreaterThan(Currency currency, Currency other)purereturns(bool)
```

Compares two `Currency` values based on their underlying addresses.
Param Name| Type| Description  
---|---|---  
currency| Currency| The first Currency value  
other| Currency| The second Currency value  
Returns `true` if the underlying address of `currency` is numerically greater than the underlying address of `other`, `false` otherwise.
Note: This comparison is based on the numerical value of the addresses and does not imply any inherent ordering or value relationship between different currencies. It's primarily used for consistent ordering in data structures.
### lessThan[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#lessthan "Direct link to lessThan")
```
functionlessThan(Currency currency, Currency other)purereturns(bool)
```

Compares two `Currency` values based on their underlying addresses.
Param Name| Type| Description  
---|---|---  
currency| Currency| The first Currency value  
other| Currency| The second Currency value  
Returns `true` if the underlying address of `currency` is numerically less than the underlying address of `other`, `false` otherwise.
Note: As with `greaterThan`, this comparison is based on address values and does not imply any inherent ordering or value relationship between currencies.
### greaterThanOrEqualTo[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#greaterthanorequalto "Direct link to greaterThanOrEqualTo")
```
functiongreaterThanOrEqualTo(Currency currency, Currency other)purereturns(bool)
```

Checks if one `Currency` value is greater than or equal to another, based on their underlying addresses.
Param Name| Type| Description  
---|---|---  
currency| Currency| The first Currency value  
other| Currency| The second Currency value  
Returns `true` if the underlying address of `currency` is numerically greater than or equal to the underlying address of `other`, `false` otherwise.
# CurrencyLibrary
The `CurrencyLibrary` provides utility functions for handling both native currency (ETH) and ERC20 tokens.
## Constants[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#constants "Direct link to Constants")
```
Currency publicconstant NATIVE = Currency.wrap(address(0));
```

`NATIVE` represents the native currency (ETH). It is defined as a `Currency` with the underlying address of `address(0)`.
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#functions "Direct link to Functions")
### transfer[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#transfer "Direct link to transfer")
```
functiontransfer(Currency currency,address to,uint256 amount)internal
```

Transfers `amount` of `currency` to the `to` address.
Param Name| Type| Description  
---|---|---  
currency| Currency| The currency to transfer  
to| address| The recipient address  
amount| uint256| The amount of currency to transfer  
### balanceOfSelf[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#balanceofself "Direct link to balanceOfSelf")
```
functionbalanceOfSelf(Currency currency)internalviewreturns(uint256)
```

Returns the balance of `currency` held by the contract itself.
Param Name| Type| Description  
---|---|---  
currency| Currency| The currency to check  
Returns the balance of the specified currency.
### balanceOf[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#balanceof "Direct link to balanceOf")
```
functionbalanceOf(Currency currency,address owner)internalviewreturns(uint256)
```

Returns the balance of `currency` held by the `owner` address.
Param Name| Type| Description  
---|---|---  
currency| Currency| The currency to check  
owner| address| The address to check  
Returns the balance of the specified currency for the given address.
### isNative[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#isnative "Direct link to isNative")
```
functionisNative(Currency currency)internalpurereturns(bool)
```

Checks if the given `currency` is the native currency (ETH).
Param Name| Type| Description  
---|---|---  
currency| Currency| The currency to check  
Returns `true` if the currency is native (ETH), `false` otherwise.
### toId[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#toid "Direct link to toId")
```
functiontoId(Currency currency)internalpurereturns(uint256)
```

Converts a `Currency` to its corresponding ID.
Param Name| Type| Description  
---|---|---  
currency| Currency| The currency to convert  
Returns the ID of the currency.
### fromId[​](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#fromid "Direct link to fromId")
```
functionfromId(uint256 id)internalpurereturns(Currency)
```

Converts an ID to its corresponding `Currency`.
Param Name| Type| Description  
---|---|---  
id| uint256| The ID to convert  
Returns the Currency corresponding to the given ID.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/currency-guide.mdx)
Was this helpful?
[PreviousBeforeSwapDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/beforeswapdelta-guide)[NextPoolKey Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
On this page
  * [Type Definition](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#type-definition)
  * [Global Functions](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#global-functions)
    * [equals](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#equals)
    * [greaterThan](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#greaterthan)
    * [lessThan](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#lessthan)
    * [greaterThanOrEqualTo](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#greaterthanorequalto)
  * [Constants](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#constants)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#functions)
    * [transfer](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#transfer)
    * [balanceOfSelf](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#balanceofself)
    * [balanceOf](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#balanceof)
    * [isNative](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#isnative)
    * [toId](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#toid)
    * [fromId](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide#fromid)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/currency-guide.mdx)
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
