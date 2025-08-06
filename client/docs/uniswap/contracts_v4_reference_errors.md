# https://docs.uniswap.org/contracts/v4/reference/errors/

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/errors/#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/errors/)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/errors/)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/errors/)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/errors/)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/errors/)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/errors/)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/errors/)
          * [test](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/errors/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)


On this page
# Custom Error Selectors
These are custom error selectors for Uniswap v4 contracts.
## IPoolManager.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#ipoolmanagersol "Direct link to IPoolManager.sol")
Error Selector| Hex Value  
---|---  
`IPoolManager.CurrencyNotSettled.selector`| `0x5212cba1`  
`IPoolManager.PoolNotInitialized.selector`| `0x486aa307`  
`IPoolManager.AlreadyUnlocked.selector`| `0x5090d6c6`  
`IPoolManager.ManagerLocked.selector`| `0x54e3ca0d`  
`IPoolManager.TickSpacingTooLarge.selector`| `0xb02b5dc2`  
`IPoolManager.TickSpacingTooSmall.selector`| `0x16fe7696`  
`IPoolManager.CurrenciesOutOfOrderOrEqual.selector`| `0xeaa6c6eb`  
`IPoolManager.UnauthorizedDynamicLPFeeUpdate.selector`| `0x30d21641`  
`IPoolManager.SwapAmountCannotBeZero.selector`| `0xbe8b8507`  
`IPoolManager.NonZeroNativeValue.selector`| `0x19d245cf`  
## Hooks.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#hookssol "Direct link to Hooks.sol")
Error Selector| Hex Value  
---|---  
`Hooks.HookAddressNotValid.selector`| `0xe65af6a0`  
`Hooks.InvalidHookResponse.selector`| `0x1e048e1d`  
`Hooks.FailedHookCall.selector`| `0x36bc48c5`  
`Hooks.HookDeltaExceedsSwapAmount.selector`| `0xfa0b71d6`  
## Pool.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#poolsol "Direct link to Pool.sol")
Error Selector| Hex Value  
---|---  
`Pool.TicksMisordered.selector`| `0xc4433ed5`  
`Pool.TickLowerOutOfBounds.selector`| `0xd5e2f7ab`  
`Pool.TickUpperOutOfBounds.selector`| `0x1ad777f8`  
`Pool.TickLiquidityOverflow.selector`| `0xb8e3c385`  
`Pool.TickNotInitialized.selector`| `0x82a774d3`  
`Pool.PoolAlreadyInitialized.selector`| `0x7983c051`  
`Pool.PoolNotInitialized.selector`| `0x486aa307`  
`Pool.PriceLimitAlreadyExceeded.selector`| `0x7c9c6e8f`  
`Pool.PriceLimitOutOfBounds.selector`| `0x9e4d7cc7`  
`Pool.NoLiquidityToReceiveFees.selector`| `0xa74f97ab`  
`Pool.InvalidFeeForExactOut.selector`| `0x96206246`  
## IProtocolFees.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#iprotocolfeessol "Direct link to IProtocolFees.sol")
Error Selector| Hex Value  
---|---  
`IProtocolFees.ProtocolFeeCannotBeFetched.selector`| `0x1ee49702`  
`IProtocolFees.InvalidProtocolFee.selector`| `0xba97f838`  
`IProtocolFees.InvalidCaller.selector`| `0x48f5c3ed`  
## LPFeeLibrary.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#lpfeelibrarysol "Direct link to LPFeeLibrary.sol")
Error Selector| Hex Value  
---|---  
`LPFeeLibrary.FeeTooLarge.selector`| `0xfc5bee12`  
## Position.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#positionsol "Direct link to Position.sol")
Error Selector| Hex Value  
---|---  
`Position.CannotUpdateEmptyPosition.selector`| `0xaefeb924`  
## Reserves.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#reservessol "Direct link to Reserves.sol")
Error Selector| Hex Value  
---|---  
`Reserves.ReservesMustBeSynced.selector`| `0x8774be48`  
## SqrtPriceMath.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#sqrtpricemathsol "Direct link to SqrtPriceMath.sol")
Error Selector| Hex Value  
---|---  
`SqrtPriceMath.InvalidPriceOrLiquidity.selector`| `0x4f2461b8`  
`SqrtPriceMath.InvalidPrice.selector`| `0x00bfc921`  
`SqrtPriceMath.NotEnoughLiquidity.selector`| `0x4323a555`  
`SqrtPriceMath.PriceOverflow.selector`| `0xf5c787f1`  
## TickBitmap.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#tickbitmapsol "Direct link to TickBitmap.sol")
Error Selector| Hex Value  
---|---  
`TickBitmap.TickMisaligned.selector`| `0xd4d8f3e6`  
## TickMath.sol[​](https://docs.uniswap.org/contracts/v4/reference/errors/#tickmathsol "Direct link to TickMath.sol")
Error Selector| Hex Value  
---|---  
`TickMath.InvalidTick.selector`| `0xce8ef7fc`  
`TickMath.InvalidSqrtPrice.selector`| `0x31efafe8`  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/errors/errors.mdx)
Was this helpful?
[PreviousAccess msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)[NextBitMath](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
On this page
  * [IPoolManager.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#ipoolmanagersol)
  * [Hooks.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#hookssol)
  * [Pool.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#poolsol)
  * [IProtocolFees.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#iprotocolfeessol)
  * [LPFeeLibrary.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#lpfeelibrarysol)
  * [Position.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#positionsol)
  * [Reserves.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#reservessol)
  * [SqrtPriceMath.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#sqrtpricemathsol)
  * [TickBitmap.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#tickbitmapsol)
  * [TickMath.sol](https://docs.uniswap.org/contracts/v4/reference/errors/#tickmathsol)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/errors/errors.mdx)
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
