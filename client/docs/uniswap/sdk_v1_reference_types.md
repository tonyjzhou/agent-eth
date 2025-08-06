# https://docs.uniswap.org/sdk/v1/reference/types

[Skip to main content](https://docs.uniswap.org/sdk/v1/reference/types#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/overview)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/overview)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/overview)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
      * [Overview](https://docs.uniswap.org/sdk/v1/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/guides/getting-started)
        * [Getting Started](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Data](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Computation](https://docs.uniswap.org/sdk/v1/reference/computation)
        * [Format](https://docs.uniswap.org/sdk/v1/reference/format)
        * [Orchestration](https://docs.uniswap.org/sdk/v1/reference/orchestration)
        * [Transact](https://docs.uniswap.org/sdk/v1/reference/transact)
        * [Constants](https://docs.uniswap.org/sdk/v1/reference/constants)
        * [Types](https://docs.uniswap.org/sdk/v1/reference/types)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v1 SDK
  * Technical Reference
  * [Types](https://docs.uniswap.org/sdk/v1/reference/types)


# Types
Below is an exhaustive list of all the external types used in the SDK.
```
import BigNumber from'bignumber.js'import{ ethers }from'ethers'import{SUPPORTED_CHAIN_ID,TRADE_TYPE,TRADE_EXACT,FIXED_UNDERFLOW_BEHAVIOR}from'./constants'exporttypeBigNumberish= BigNumber | ethers.utils.BigNumber |string|number//// types for on-chain, submitted, and normalized dataexporttypeChainIdOrProvider=SUPPORTED_CHAIN_ID| ethers.providers.AsyncSendable | ethers.providers.Provider// type guard for ChainIdOrProviderexportfunctionisChainId(chainIdOrProvider: ChainIdOrProvider): chainIdOrProvider isSUPPORTED_CHAIN_ID{const chainId:SUPPORTED_CHAIN_ID= chainIdOrProvider asSUPPORTED_CHAIN_IDreturntypeof chainId ==='number'}// type guard for ChainIdOrProviderexportfunctionisLowLevelProvider( chainIdOrProvider: ChainIdOrProvider): chainIdOrProvider is ethers.providers.AsyncSendable {if(isChainId(chainIdOrProvider)){returnfalse}else{const provider: ethers.providers.AsyncSendable = chainIdOrProvider as ethers.providers.AsyncSendablereturn'send'in provider ||'sendAsync'in provider}}exportinterfaceToken{ chainId?:SUPPORTED_CHAIN_ID address?:string decimals:number}exportinterfaceTokenAmount{ token: Token amount: BigNumberish}exportinterfaceTokenAmountNormalized{ token: Token amount: BigNumber}exportinterfaceTokenReserves{ token: Token exchange?: Token ethReserve: TokenAmount tokenReserve: TokenAmount}exportinterfaceTokenReservesNormalized{ token: Token exchange?: Token ethReserve: TokenAmountNormalized tokenReserve: TokenAmountNormalized}exportinterfaceEthReserves{ token: Token}// type for input dataexporttypeOptionalReserves= TokenReserves | EthReserves |undefined// type guard for OptionalReservesexportfunctionareTokenReserves(reserves: OptionalReserves): reserves is TokenReserves {const tokenReserves: TokenReserves = reserves as TokenReservesreturn(  tokenReserves !==undefined&& tokenReserves.ethReserve !==undefined&& tokenReserves.tokenReserve !==undefined)}// type guard for OptionalReservesexportfunctionareETHReserves(reserves: OptionalReserves): reserves is EthReserves {const tokenReserves: TokenReserves = reserves as TokenReservesreturn(  tokenReserves !==undefined&& tokenReserves.ethReserve ===undefined&& tokenReserves.tokenReserve ===undefined)}// type for output dataexporttypeNormalizedReserves= TokenReservesNormalized | EthReserves// type guard for NormalizedReservesexportfunctionareTokenReservesNormalized(reserves: NormalizedReserves): reserves is TokenReservesNormalized {const tokenReservesNormalized: TokenReservesNormalized = reserves as TokenReservesNormalizedreturn tokenReservesNormalized.ethReserve !==undefined&& tokenReservesNormalized.tokenReserve !==undefined}//// types for computed dataexportinterfaceRate{ rate: BigNumber rateInverted: BigNumber}exportinterfaceMarketDetails{ tradeType:TRADE_TYPE inputReserves: NormalizedReserves outputReserves: NormalizedReserves marketRate: Rate}exportinterfaceTradeDetails{ marketDetailsPre: MarketDetails marketDetailsPost: MarketDetails tradeType:TRADE_TYPE tradeExact:TRADE_EXACT inputAmount: TokenAmountNormalized outputAmount: TokenAmountNormalized executionRate: Rate marketRateSlippage: BigNumber executionRateSlippage: BigNumber}exporttypeMethodArgument= BigNumber |number|stringexportinterfaceExecutionDetails{ exchangeAddress:string methodName:string methodId:string value: BigNumber methodArguments: MethodArgument[]}//// types for formatting dataexporttypeFlexibleFormat= BigNumber.Format |boolean// type guard for FlexibleFormatexportfunctionisFormat(flexibleFormat: FlexibleFormat): flexibleFormat is BigNumber.Format {const format: BigNumber.Format = flexibleFormat as BigNumber.Formatreturntypeof format !=='boolean'}exportinterfaceFormatSignificantOptions{ significantDigits:number roundingMode: BigNumber.RoundingMode forceIntegerSignificance:boolean format: FlexibleFormat}exportinterfaceFormatFixedOptions{ decimalPlaces:number roundingMode: BigNumber.RoundingMode dropTrailingZeros:boolean underflowBehavior:FIXED_UNDERFLOW_BEHAVIOR format: FlexibleFormat}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/08-types.md)
Was this helpful?
[PreviousConstants](https://docs.uniswap.org/sdk/v1/reference/constants)[NextOverview](https://docs.uniswap.org/api/subgraph/overview)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/08-types.md)
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
