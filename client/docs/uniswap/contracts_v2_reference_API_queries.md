# https://docs.uniswap.org/contracts/v2/reference/API/queries

[Skip to main content](https://docs.uniswap.org/contracts/v2/reference/API/queries#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/queries)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/API/queries)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/API/queries)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/queries)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/queries)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/reference/API/queries)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/reference/API/queries)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/reference/API/queries)
    * [Permit2](https://docs.uniswap.org/contracts/v2/reference/API/queries)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/queries)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/reference/API/queries)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/reference/API/queries)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/queries)
          * [API Overview](https://docs.uniswap.org/contracts/v2/reference/API/overview)
          * [Entities](https://docs.uniswap.org/contracts/v2/reference/API/entities)
          * [Queries](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/API/queries)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/queries)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Technical Reference
  * API
  * [Queries](https://docs.uniswap.org/contracts/v2/reference/API/queries)


On this page
# Queries
The subgraph can be queried to retrieve important information about Uniswap, pairs, tokens, transactions, users, and more. This page will provide examples for common queries.
To try these queries and run your own visit the [subgraph sandbox](https://thegraph.com/explorer/subgraph/uniswap/uniswap-v2).
### Global Data[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#global-data "Direct link to Global Data")
To query global data you can pass in the Uniswap Factory address and select from available fields.
#### Global Stats[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#global-stats "Direct link to Global Stats")
All time volume in USD, total liquidity in USD, all time transaction count.
```
{ uniswapFactory(id: "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f"){  totalVolumeUSD  totalLiquidityUSD  txCount }}
```

#### Global Historical lookup[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#global-historical-lookup "Direct link to Global Historical lookup")
To get a snapshot of past state, use The Graph's block query feature and query at a previous block. See this post to get more information about [fetching block numbers from timestamps](https://blocklytics.org/blog/ethereum-blocks-subgraph-made-for-time-travel/). This can be used to calculate things like 24hr volume.
```
{ uniswapFactory(id: "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f", block: {number: 10291203}){  totalVolumeUSD  totalLiquidityUSD  txCount }}
```

### Pair Data[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#pair-data "Direct link to Pair Data")
#### Pair Overview[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#pair-overview "Direct link to Pair Overview")
Fetch a snapshot of the current state of the pair with common values. This example fetches the DAI/WETH pair.
```
{ pair(id: "0xa478c2975ab1ea89e8196811f51a7b7ade33eb11"){   token0 {    id    symbol    name    derivedETH   }   token1 {    id    symbol    name    derivedETH   }   reserve0   reserve1   reserveUSD   trackedReserveETH   token0Price   token1Price   volumeUSD   txCount }}
```

#### All pairs in Uniswap[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#all-pairs-in-uniswap "Direct link to All pairs in Uniswap")
The Graph limits entity return amounts to 1000 per query as of now. To get all pairs on Uniswap use a loop and graphql skip query to fetch multiple chunks of 1000 pairs. The query would look like this (where skip is some incrementing variable passed into your query).
```
{ query pairs($skip: Int!) {  pairs(first: 1000, skip: $skip) {   id  } }}
```

#### Most liquid pairs[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#most-liquid-pairs "Direct link to Most liquid pairs")
Order by liquidity to get the most liquid pairs in Uniswap.
```
{ pairs(first: 1000, orderBy: reserveUSD, orderDirection: desc) {  id }}
```

#### Recent Swaps within a Pair[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#recent-swaps-within-a-pair "Direct link to Recent Swaps within a Pair")
Get the last 100 swaps on a pair by fetching Swap events and passing in the pair address. You'll often want token information as well.
```
{swaps(orderBy: timestamp, orderDirection: desc, where: { pair: "0xa478c2975ab1ea89e8196811f51a7b7ade33eb11" }) {   pair {    token0 {     symbol    }    token1 {     symbol    }   }   amount0In   amount0Out   amount1In   amount1Out   amountUSD   to }}
```

#### Pair Daily Aggregated[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#pair-daily-aggregated "Direct link to Pair Daily Aggregated")
Day data is useful for building charts and historical views around entities. To get stats about a pair in daily buckets query for day entities bounded by timestamps. This query gets the first 100 days after the given unix timestamp on the DAI/WETH pair.
```
{ pairDayDatas(first: 100, orderBy: date, orderDirection: asc,  where: {   pairAddress: "0xa478c2975ab1ea89e8196811f51a7b7ade33eb11",   date_gt: 1592505859  } ) {   date   dailyVolumeToken0   dailyVolumeToken1   dailyVolumeUSD   reserveUSD }}
```

### Token Data[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#token-data "Direct link to Token Data")
Token data can be fetched using the token contract address as an ID. Token data is aggregated across all pairs the token is included in. Any token that is included in some pair in Uniswap can be queried.
#### Token Overview[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#token-overview "Direct link to Token Overview")
Get a snapshot of the current stats on a token in Uniswap. This query fetches current stats on DAI. The allPairs field gets the first 200 pairs DAI is included in sorted by liquidity in derived USD.
```
{ token(id: "0x6b175474e89094c44da98b954eedeac495271d0f"){  name  symbol  decimals  derivedETH  tradeVolumeUSD  totalLiquidity }}
```

#### All Tokens in Uniswap[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#all-tokens-in-uniswap "Direct link to All Tokens in Uniswap")
Similar to fetching all pairs (see above), you can query all tokens in Uniswap. Because The Graph service limits return size to 1000 entities use graphql skip query. (Note this query will not work in the graph sandbox and more resembles the structure of a query you'd pass to some graphql middleware like [Apollo](https://www.apollographql.com/)).
```
{ query tokens($skip: Int!) {  tokens(first: 1000, skip: $skip) {   id   name   symbol  } }}
```

#### Token Transactions[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#token-transactions "Direct link to Token Transactions")
To get transactions that include a token you'll need to first fetch an array of pairs that the token is included in (this can be done with the allPairs field on the Token entity.) Once you have an array of pairs the token is included in, filter on that in the transaction lookup.
This query fetches the latest 30 mints, swaps, and burns involving DAI. The allPairs array could look something like this where we include the DAI/WETH pair address and the DAI/USDC pair address.
```
allPairs = [ "0xa478c2975ab1ea89e8196811f51a7b7ade33eb11", "0xae461ca67b15dc8dc81ce7615e0320da1a9ab8d5"]
```

```
query($allPairs: [String!]) { mints(first: 30, where: { pair_in: $allPairs }, orderBy: timestamp, orderDirection: desc) {  transaction {   id   timestamp  }  to  liquidity  amount0  amount1  amountUSD } burns(first: 30, where: { pair_in: $allPairs }, orderBy: timestamp, orderDirection: desc) {  transaction {   id   timestamp  }  to  liquidity  amount0  amount1  amountUSD } swaps(first: 30, where: { pair_in: $allPairs }, orderBy: timestamp, orderDirection: desc) {  transaction {   id   timestamp  }  amount0In  amount0Out  amount1In  amount1Out  amountUSD  to }}
```

#### Token Daily Aggregated[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#token-daily-aggregated "Direct link to Token Daily Aggregated")
Like pair and global daily lookups, tokens have daily entities that can be queries as well. This query gets daily information for DAI. Note that you may want to sort in ascending order to receive your days from oldest to most recent in the return array.
```
{ tokenDayDatas(orderBy: date, orderDirection: asc, where: {  token: "0x6b175474e89094c44da98b954eedeac495271d0f" } ) {  id  date  priceUSD  totalLiquidityToken  totalLiquidityUSD  totalLiquidityETH  dailyVolumeETH  dailyVolumeToken  dailyVolumeUSD }}
```

### ETH Price[​](https://docs.uniswap.org/contracts/v2/reference/API/queries#eth-price "Direct link to ETH Price")
You can use the Bundle entity to query current USD price of ETH in Uniswap based on a weighted average of stablecoins.
```
{ bundle(id: "1" ) {  ethPrice }}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/API/03-queries.md)
Was this helpful?
[PreviousEntities](https://docs.uniswap.org/contracts/v2/reference/API/entities)[NextGovernance Reference](https://docs.uniswap.org/contracts/v2/reference/Governance/governance-reference)
On this page
  * [Global Data](https://docs.uniswap.org/contracts/v2/reference/API/queries#global-data)
  * [Pair Data](https://docs.uniswap.org/contracts/v2/reference/API/queries#pair-data)
  * [Token Data](https://docs.uniswap.org/contracts/v2/reference/API/queries#token-data)
  * [ETH Price](https://docs.uniswap.org/contracts/v2/reference/API/queries#eth-price)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/API/03-queries.md)
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
