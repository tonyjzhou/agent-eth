# https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#__docusaurus_skipToContent_fallback)
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
          * [lens](https://docs.uniswap.org/contracts/v4/reference/periphery/lens/StateView)
          * [libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/ActionConstants)
            * [ActionConstants](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/ActionConstants)
            * [Actions](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/Actions)
            * [AddressStringUtil](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/AddressStringUtil)
            * [BipsLibrary](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/BipsLibrary)
            * [CalldataDecoder](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/CalldataDecoder)
            * [CurrencyRatioSortOrder](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/CurrencyRatioSortOrder)
            * [Descriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/Descriptor)
            * [ERC721PermitHash](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/ERC721PermitHash)
            * [HexStrings](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/HexStrings)
            * [LiquidityAmounts](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/LiquidityAmounts)
            * [Locker](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/Locker)
            * [PathKey](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/PathKey)
            * [PositionConfig](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/PositionConfig)
            * [PositionConfigId](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/PositionConfigId)
            * [PositionInfo](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/PositionInfoLibrary)
            * [QuoterRevert](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert)
            * [SVG](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/SVG)
            * [SafeCurrencyMetadata](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/SafeCurrencyMetadata)
            * [SlippageCheck](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/SlippageCheck)
            * [VanityAddressLib](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/VanityAddressLib)
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
  * libraries
  * [QuoterRevert](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert)


On this page
# QuoterRevert
[Git Source](https://github.com/uniswap/v4-periphery/blob/ea2bf2e1ba6863bb809fc2ff791744f308c4a26d/src/libraries/QuoterRevert.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#functions "Direct link to Functions")
### revertQuote[​](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#revertquote "Direct link to revertQuote")
reverts, where the revert data is the provided bytes
_called when quoting, to record the quote amount in an error_
_QuoteSwap is used to differentiate this error from other errors thrown when simulating the swap_
```
functionrevertQuote(uint256 quoteAmount)internalpure;
```

### bubbleReason[​](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#bubblereason "Direct link to bubbleReason")
reverts using the revertData as the reason
_to bubble up both the valid QuoteSwap(amount) error, or an alternative error thrown during simulation_
```
functionbubbleReason(bytesmemory revertData)internalpure;
```

### parseQuoteAmount[​](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#parsequoteamount "Direct link to parseQuoteAmount")
validates whether a revert reason is a valid swap quote or not if valid, it decodes the quote to return. Otherwise it reverts.
```
functionparseQuoteAmount(bytesmemory reason)internalpurereturns(uint256 quoteAmount);
```

## Errors[​](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#errors "Direct link to Errors")
### UnexpectedRevertBytes[​](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#unexpectedrevertbytes "Direct link to UnexpectedRevertBytes")
error thrown when invalid revert bytes are thrown by the quote
```
error UnexpectedRevertBytes(bytes revertData);
```

### QuoteSwap[​](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#quoteswap "Direct link to QuoteSwap")
error thrown containing the quote as the data, to be caught and parsed later
```
error QuoteSwap(uint256 amount);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/libraries/QuoterRevert.md)
Was this helpful?
[PreviousPositionInfo](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/PositionInfoLibrary)[NextSVG](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/SVG)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#functions)
    * [revertQuote](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#revertquote)
    * [bubbleReason](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#bubblereason)
    * [parseQuoteAmount](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#parsequoteamount)
  * [Errors](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#errors)
    * [UnexpectedRevertBytes](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#unexpectedrevertbytes)
    * [QuoteSwap](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/QuoterRevert#quoteswap)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/libraries/QuoterRevert.md)
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
