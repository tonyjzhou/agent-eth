# https://docs.uniswap.org/concepts/protocol/oracle

[Skip to main content](https://docs.uniswap.org/concepts/protocol/oracle#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Hooks](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Swaps](https://docs.uniswap.org/concepts/protocol/swaps)
      * [Fees](https://docs.uniswap.org/concepts/protocol/fees)
      * [Oracle](https://docs.uniswap.org/concepts/protocol/oracle)
      * [Range Orders](https://docs.uniswap.org/concepts/protocol/range-orders)
      * [Concentrated Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)
      * [Token Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)
    * [Governance](https://docs.uniswap.org/concepts/governance/overview)
    * [Research](https://docs.uniswap.org/concepts/research)
    * [Resources](https://docs.uniswap.org/concepts/resources)
    * [Glossary](https://docs.uniswap.org/concepts/glossary)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * Protocol Concepts
  * [Oracle](https://docs.uniswap.org/concepts/protocol/oracle)


On this page
# Oracle
note
Uniswap v4 does not include built-in oracle functionality. If you're unfamiliar with oracles, check out the Ethereum Foundation's [oracle overview](https://ethereum.org/en/developers/docs/oracles/).
All Uniswap v3 pools can serve as oracles, offering access to historical price and liquidity data. This capability unlocks a wide range of on-chain use cases.
Historical data is stored as an array of observations. At first, each pool tracks only a single observation, overwriting it as blocks elapse. This limits how far into the past users may access data. However, any party willing to pay the transaction fees may [increase the number of tracked observations](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#increaseobservationcardinalitynext) (up to a maximum of `65535`), expanding the period of data availability to ~9 days or more.
Storing price and liquidity history directly in the pool contract substantially reduces the potential for logical errors on the part of the calling contract, and reduces integration costs by eliminating the need to store historical values. Additionally, the v3 oracle's considerable maximum length makes oracle price manipulation significantly more difficult, as the calling contract may cheaply construct a time-weighted average over any arbitrary range inside of (or fully encompassing) the length of the oracle array.
## Observations[​](https://docs.uniswap.org/concepts/protocol/oracle#observations "Direct link to Observations")
`Observation`s take the following form:
```
structObservation{// the block timestamp of the observationuint32 blockTimestamp;// the tick accumulator, i.e. tick * time elapsed since the pool was first initializedint56 tickCumulative;// the seconds per liquidity, i.e. seconds elapsed / max(1, liquidity) since the pool was first initializeduint160 secondsPerLiquidityCumulativeX128;// whether or not the observation is initializedbool initialized;}
```

`Observation`s may be retrieved via the [`observations`](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/pool/IUniswapV3PoolState#observations) method on v3 pools. However, this is _not_ the recommended way to consume oracle data. Instead, prefer [`observe`](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool#observe):
```
functionobserve(uint32[]calldata secondsAgos)externalviewreturns(int56[]memory tickCumulatives,uint160[]memory secondsPerLiquidityCumulativeX128s);
```

Each time `observe` is called, the caller must specify an array containing any number of seconds ago, denoting the times to return observations from. Note that each of the given times must be more recent (or as old as) the oldest stored observation. Note: if the times don't correspond exactly to a block in which an observation was written, a counterfactual observation will be constructed, removing the need for the caller to interpolate manually. This is one of the primary reasons to use `observe` over `observations`.
Note that because the oracle is only updated at most once every block, calling `observe` with a `secondsAgo` value of `0` will return the most recently written observation, which can only be as recent as the beginning of the current block (or older).
## Tick Accumulator[​](https://docs.uniswap.org/concepts/protocol/oracle#tick-accumulator "Direct link to Tick Accumulator")
The tick accumulator stores the cumulative sum of the active tick at the time of the observation. The tick accumulator value increases monotonically and grows by the value of the current tick - per second.
To derive the arithmetic mean tick over an interval, the caller needs to retrieve two observations, one after the other, take the delta of the two values, and divide by the time elapsed between them. Calculating a TWAP from the tick accumulator is also covered in the [**whitepaper**](https://uniswap.org/whitepaper-v3.pdf). Note that using an arithmetic mean tick to derive a price corresponds to a _geometric_ mean price.
See [OracleLibrary](https://github.com/Uniswap/uniswap-v3-periphery/blob/main/contracts/libraries/OracleLibrary.sol) for an example of how to use the tick accumulator.
## Liquidity Accumulator[​](https://docs.uniswap.org/concepts/protocol/oracle#liquidity-accumulator "Direct link to Liquidity Accumulator")
The liquidity accumulator stores the value of seconds / in-range liquidity at the time of the observation. The liquidity accumulator value increases monotonically and grows by the value of seconds / in-range liquidity - per second.
To derive the harmonic mean liquidity over an interval, the caller needs to retrieve two observations, one after the other, take the delta of the two values, then divide the time elapsed by this value. Calculating TWAL is addressed in finer detail in the [**whitepaper**](https://uniswap.org/whitepaper-v3.pdf).
note
The in-range liquidity accumulator should be used with care. Because the current tick and the current in-range liquidity can be entirely uncorrelated, there are scenarios in which taking the arithmetic mean tick and the harmonic mean liquidity over the same interval in a pool can inaccurately characterize this pool relative to another. For example, if the current tick on pool A is 0 for 5 seconds, and 100 for 5 seconds, the tick accumulator will be 50. If over this same interval, the in-range liquidity was 5000 and 50, the harmonic mean liquidity will be ~99. Compare this to pool B (composed of the same assets) where the tick was 50 and the in-range liquidity was ~99 for 10 seconds. The accumulator values will be identical, but the underlying behavior is of course quite different.
## Deriving Price From A Tick[​](https://docs.uniswap.org/concepts/protocol/oracle#deriving-price-from-a-tick "Direct link to Deriving Price From A Tick")
> When we use "active tick" or otherwise to refer to the current tick of a pool, we mean the lower tick boundary that is closest to the current price.
When a pool is created, each token is assigned to either `token0` or `token1` based on the contract address of the tokens in the pair. Whether or not a token is `token0` or `token1` is meaningless; it is only used to maintain a fixed assignment for the purpose of relative valuation and general logic in the pool contract.
Deriving an asset price from the current tick is achievable due to the fixed expression across the pool contract of token0 in terms of token1.
An example of finding the price of WETH in a WETH / USDC pool, where WETH is `token0` and USDC is `token1`:
You have an oracle reading that shows a return of `tickCumulative` as [`70_000`, `1_070_000`], with an elapsed time between the observations of 10 seconds.
We can derive the average tick over this interval by taking the difference in accumulator values (`1_070_000` - `70_000` = `1_000_000`), and dividing by the time elapsed (`1_000_000` / `10` = `100_000`).
With a tick reading of `100_000`, we can find the value of `token1` (USDC) in terms of `token0` (WETH) by using the current tick as `i` in the formula `p(i) = 1.0001**i` (see 6.1 in the [whitepaper](https://uniswap.org/whitepaper-v3.pdf)).
`1.0001**100_000 ≅ 22015.5` USDC / WETH
Ticks are signed integers and can be expressed as a negative number, so for any circumstances where `token0` is of a lower value than `token1`, a negative tick value will be returned by `tickCumulative` and a relative value of `< 0` will be returned by a calculation of `token0` in terms of `token1`.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/oracle.md)
Was this helpful?
[PreviousFees](https://docs.uniswap.org/concepts/protocol/fees)[NextRange Orders](https://docs.uniswap.org/concepts/protocol/range-orders)
On this page
  * [Observations](https://docs.uniswap.org/concepts/protocol/oracle#observations)
  * [Tick Accumulator](https://docs.uniswap.org/concepts/protocol/oracle#tick-accumulator)
  * [Liquidity Accumulator](https://docs.uniswap.org/concepts/protocol/oracle#liquidity-accumulator)
  * [Deriving Price From A Tick](https://docs.uniswap.org/concepts/protocol/oracle#deriving-price-from-a-tick)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/oracle.md)
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
