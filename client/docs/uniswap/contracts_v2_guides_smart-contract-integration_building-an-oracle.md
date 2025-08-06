# https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
    * [Permit2](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
          * [Smart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
          * [Implement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
          * [Providing Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
          * [Building an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)
          * [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses)
          * [Supporting meta transactions](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [API](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Governance](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Smart Contract Integration
  * [Building an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)


On this page
To build a price oracle on Uniswap V2, you must first understand the requirements for your use case. Once you understand the kind of price average you require, it is a matter of storing the cumulative price variable from the pair as often as necessary, and computing the average price using two or more observations of the cumulative price variables.
# Understanding requirements
To understand your requirements, you should first research the answer to the following questions:
  * Is data freshness important? I.e.: must the price average include the current price?
  * Are recent prices more important than historical prices? I.e.: is the current price given more weight than historical prices?


Note your answers for the following discussion.
# Oracle Strategies
## Fixed windows[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#fixed-windows "Direct link to Fixed windows")
In the case where data freshness is not important and recent prices are weighted equally with historical prices, it is enough to store the cumulative price once per period (e.g. once per 24 hours.)
Computing the average price over these data points gives you 'fixed windows', which can be updated after the lapse of each period. We wrote an example oracle of this kind [here](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/examples/ExampleOracleSimple.sol).
This example does not limit the maximum size of the fixed window, i.e. it only requires that the window size is greater than 1 period (e.g. 24 hours).
## Moving averages[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#moving-averages "Direct link to Moving averages")
In the case where data freshness is important, you can use a sliding window in which the cumulative price variable is measured more often than once per period.
There are at least [two kinds of moving averages](https://www.investopedia.com/terms/m/movingaverage.asp#types-of-moving-averages) that you can compute using the Uniswap cumulative price variable.
[Simple moving averages](https://www.investopedia.com/terms/s/sma.asp) give equal weight to each price measurement. We have built an example of a sliding window oracle [here](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/examples/ExampleSlidingWindowOracle.sol).
[Exponential moving averages](https://www.investopedia.com/terms/e/ema.asp) give more weight to the most recent price measurements. We do not yet have an example written for this type of oracle.
You may wish to use exponential moving averages where recent prices are more important than historical prices, e.g. in case of liquidations. However, note that putting more weight on recent prices makes the oracle cheaper to manipulate than weighting all price measurements equally.
## Computing average prices[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#computing-average-prices "Direct link to Computing average prices")
To compute the average price given two cumulative price observations, take the difference between the cumulative price at the beginning and end of the period, and divide by the elapsed time between them in seconds. This will produce a [fixed point unsigned Q112x112](https://en.wikipedia.org/wiki/Fixed-point_arithmetic#Notation) number that represents the price of one asset relative to the other. This number is represented as a `uint224` where the upper 112 bits represent the integer amount, and the lower 112 bits represent the fractional amount.
Pairs contain both `price0CumulativeLast` and `price1CumulativeLast`, which are ratios of reserves of `token1`/`token0` and `token0`/`token1` respectively. I.e. the price of `token0` is expressed in terms of `token1`/`token0`, while the price of `token1` is expressed in terms of `token0`/`token1`.
# Getting the latest cumulative price
If you wish to compute the average price between a historical price cumulative observation and the current cumulative price, you should use the cumulative price values from the current block. If the cumulative price has not been updated in the current block, e.g. because there has not been any liquidity event (`mint`/`burn`/`swap`) on the pair in the current block, you can compute the cumulative price counterfactually.
We provide a library for use in oracle contracts that has the method [`UniswapV2OracleLibrary#currentCumulativePrices`](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/libraries/UniswapV2OracleLibrary.sol#L16) for getting the cumulative price as of the current block. The current cumulative price returned by this method is computed _counterfactually_ , meaning it requires no call to the relative gas-expensive `#sync` method on the pair. It is correct regardless of whether a swap has already executed in the current block.
# Notes on overflow
The `UniswapV2Pair` cumulative price variables are designed to eventually overflow, i.e. `price0CumulativeLast` and `price1CumulativeLast` and `blockTimestampLast` will overflow through 0.
This should not pose an issue to your oracle design, as the price average computation is concerned with differences (i.e. subtraction) between two separate observations of a cumulative price variable. Subtracting between two cumulative price values will result in a number that fits within the range of `uint256` as long as the observations are made for periods of max `2^32` seconds, or ~136 years.
`blockTimestampLast` is stored only in a `uint32`. For the same reason as described above, the pair can save a storage slot, and many SSTORES over the life of the pair, by storing only `block.timestamp % uint32(-1)`. This is feasible because the pair is only concerned with the time that elapses between each liquidity event when updating the cumulative prices, which is always expected to be less than `2^32` seconds.
When computing time elapsed within your own oracle, you can simply store the `block.timestamp` of your observations as `uint256`, and avoid dealing with overflow math for computing the time elapsed between observations. This is how the [ExampleSlidingWindowOracle](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/examples/ExampleSlidingWindowOracle.sol) handles observation timestamps.
## Integrating the oracle[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#integrating-the-oracle "Direct link to Integrating the oracle")
To integrate an oracle into your contracts, you must ensure the oracle's observations of the cumulative price variable are kept up to date. As long as your oracle is up to date, you can depend on it to produce average prices. The process of keeping your oracle up to date is called 'maintenance'.
## Oracle maintenance[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#oracle-maintenance "Direct link to Oracle maintenance")
In order to measure average prices over a period, the oracle must have a way of referencing the cumulative price at the start and end of a period. The recommended way of doing this is by storing these prices in the oracle contract, and calling the oracle frequently enough to store the latest cumulative price.
Reliable oracle maintenance is a difficult task, and can become a point of failure in times of congestion. Instead, consider building this functionality directly into the critical calls of your own smart contracts, or incentivize oracle maintenance calls by other parties.
## No-maintenance option[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#no-maintenance-option "Direct link to No-maintenance option")
It is possible to avoid regularly storing this cumulative price at the start of the period by utilizing storage proofs. However, this approach has limitations, especially in regard to gas cost and maximum length of the time period over which the average price can be measured. If you wish to try this approach, you can follow [this repository by Keydonix](https://github.com/Keydonix/uniswap-oracle/).
Keydonix has developed a general purpose price feed oracle built on Uniswap v2 that supports arbitrary time windows (up to 256 blocks) and doesn't require any active maintenance.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/04-building-an-oracle.md)
Was this helpful?
[PreviousProviding Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)[NextFlash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)
On this page
  * [Fixed windows](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#fixed-windows)
  * [Moving averages](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#moving-averages)
  * [Computing average prices](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#computing-average-prices)
  * [Integrating the oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#integrating-the-oracle)
  * [Oracle maintenance](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#oracle-maintenance)
  * [No-maintenance option](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle#no-maintenance-option)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/04-building-an-oracle.md)
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
