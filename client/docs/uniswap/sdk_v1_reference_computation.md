# https://docs.uniswap.org/sdk/v1/reference/computation

[Skip to main content](https://docs.uniswap.org/sdk/v1/reference/computation#__docusaurus_skipToContent_fallback)
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
  * [Computation](https://docs.uniswap.org/sdk/v1/reference/computation)


On this page
# getMarketDetails
This function computes market details for the passed reserves data. Markets are defined as ETH<>ERC20, ERC20<>ETH, or ERC20<>ERC20 pairs, where the first currency is the input and the second is the output. Reserves must be specified for both the input and output currency.
  * In the case of ETH, `undefined` should be passed as the reserves data. [`getTokenReserves`](https://docs.uniswap.org/sdk/1.0.0/reference/data/#getttokenreserves) formatted ERC20 reserves, or the requisite data can be fetched manually and passed in.
  * Rates are calculated to 18 decimal places of precision.


## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/computation#function-signature "Direct link to Function Signature")
```
exportfunctiongetMarketDetails( optionalReservesInput: OptionalReserves, optionalReservesOutput: OptionalReserves): MarketDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/computation#input-parameters "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
optionalReservesInput| `OptionalReserves`| Reserves data for the input currency.  
optionalReservesOutput| `OptionalReserves`| Reserves data for the output currency.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/computation#example-usage "Direct link to Example Usage")
```
const reserves: ChainIdOrProvider =awaitgetTokenReserves(tokenAddress)const marketDetails: MarketDetails =getMarketDetails(undefined, reserves)// ETH<>ERC20/*{ // market type tradeType: 'ETH_TO_TOKEN', // dummy ETH reserves inputReserves: {  token: {   chainId: 1,   address: 'ETH',   decimals: 18  } }, // normalized token reserves outputReserves: <NormalizedReserves>, // market rate calculated to 18 decimals of precision marketRate: {  rate: <BigNumber>,    // x output / 1 input  rateInverted: <BigNumber> // x input / 1 output }}*/
```

# getTradeDetails
This function computes trade details for the passed market data.
-This function throws an error if the passed _tradeAmount is greater than the amount of ETH/tokens in the relevant Uniswap exchange.
  * Trade amounts must be passed in non-decimal form (where e.g. 1 ETH is represented as 1000000000000000000 wei).


## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/computation#function-signature-1 "Direct link to Function Signature")
```
exportfunctiongetTradeDetails( tradeExact:TRADE_EXACT, _tradeAmount: BigNumberish, marketDetails: MarketDetails): TradeDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/computation#input-parameters-1 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
tradeExact| `TRADE_EXACT`| Whether either the input or the output currency is the exact amount.  
_tradeAmount| `BigNumberish`| The amount to buy/sell (of the output/input currency, depending on tradeExact)  
marketDetails| `MarketDetails`| Market details.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/computation#example-usage-1 "Direct link to Example Usage")
```
const _purchaseAmount: BigNumber =newBigNumber('2.5')const _decimals:number=18const tradeAmount: BigNumber = _purchaseAmount.multipliedBy(10** _decimals)const marketDetails: MarketDetails =getMarketDetails(undefined, reserves)// ETH<>ERC20// buy exactly 2.5 of an 18 decimal ERC20 with ETHconst tradeDetails: TradeDetails =getTradeDetails(TRADE_EXACT.OUTPUT, tradeAmount, marketDetails)/*{ marketDetailsPre: <MarketDetails>, marketDetailsPost: <MarketDetails>, tradeType: 'ETH_TO_TOKEN', tradeExact: 'OUTPUT', inputAmount: {  token: <Token>,  amount: <BigNumber> }, outputAmount: {  token: <Token>,  amount: <BigNumber> }, // execution rate calculated to 18 decimals of precision executionRate: {  rate: <BigNumber>     // x output / 1 input  rateInverted: <BigNumber> // x input / 1 output }, // slippage between the pre- and post-trade market rates, in basis points, calculated to 18 decimals of precision marketRateSlippage: <BigNumber>, // slippage between the execution and pre-trade market rate, in basis points, calculated to 18 decimals of precision executionRateSlippage: <BigNumber>}*/
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/03-computation.md)
Was this helpful?
[PreviousData](https://docs.uniswap.org/sdk/v1/reference/data)[NextFormat](https://docs.uniswap.org/sdk/v1/reference/format)
On this page
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/computation#function-signature)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/computation#input-parameters)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/computation#example-usage)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/computation#function-signature-1)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/computation#input-parameters-1)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/computation#example-usage-1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/03-computation.md)
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
