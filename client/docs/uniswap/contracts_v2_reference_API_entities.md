# https://docs.uniswap.org/contracts/v2/reference/API/entities

[Skip to main content](https://docs.uniswap.org/contracts/v2/reference/API/entities#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/entities)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/API/entities)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/API/entities)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/entities)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/entities)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/reference/API/entities)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/reference/API/entities)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/reference/API/entities)
    * [Permit2](https://docs.uniswap.org/contracts/v2/reference/API/entities)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/entities)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/reference/API/entities)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/reference/API/entities)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/entities)
          * [API Overview](https://docs.uniswap.org/contracts/v2/reference/API/overview)
          * [Entities](https://docs.uniswap.org/contracts/v2/reference/API/entities)
          * [Queries](https://docs.uniswap.org/contracts/v2/reference/API/queries)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/API/entities)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/API/entities)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/reference/API/entities)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Technical Reference
  * API
  * [Entities](https://docs.uniswap.org/contracts/v2/reference/API/entities)


On this page
# Entities
Entities define the schema of the subgraph, and represent the data that can be queried. Within each entity are sets of fields that store useful information related to the entity. Below is a list of the available entities within the Uniswap Subgraph, and descriptions for the available fields.
To see an interactive sandbox of all entities see the [Graph Explorer](https://thegraph.com/explorer/subgraph/uniswap/uniswap-v2).
Each entity is defined with a value type, which will always be a base AssemblyScript type, or a custom type provided by The Graph's custom TypeScript library. For more information on value types see [here](https://thegraph.com/docs/en/developing/assemblyscript-api/#api-reference).
### Uniswap Factory[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#uniswap-factory "Direct link to Uniswap Factory")
The Uniswap Factory entity is responsible for storing aggregate information across all Uniswap pairs. It can be used to view stats about total liquidity, volume, amount of pairs and more. There is only one UniswapFactory entity in the subgraph.
Field Name| Value Type| Description  
---|---|---  
id| ID| factory address  
pairCount| Int| amount of pairs created by the Uniswap factory  
totalVolumeUSD| BigDecimal| all time USD volume across all pairs (USD is derived)  
totalVolumeETH| BigDecimal| all time volume in ETH across all pairs (ETH is derived)  
totalLiquidityUSD| BigDecimal| total liquidity across all pairs stored as a derived USD amount  
totalLiquidityETH| BigDecimal| total liquidity across all pairs stored as a derived ETH amount  
txCount| BigInt| all time amount of transactions across all pairs  
### Token[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#token "Direct link to Token")
Stores aggregated information for a specific token across all pairs that token is included in.
Field Name| Value Type| Description  
---|---|---  
id| ID| token address  
symbol| String| token symbol  
name| String| token name  
decimals| BigInt| token decimals  
tradeVolume| BigDecimal| amount of token traded all time across all pairs  
tradeVolumeUSD| BigDecimal| amount of token in USD traded all time across pairs (only for tokens with liquidity above minimum threshold)  
untrackedVolumeUSD| BigDecimal| amount of token in USD traded all time across pairs (no minimum liquidity threshold)  
txCount| BigInt| amount of transactions all time in pairs including token  
totalLiquidity| BigDecimal| total amount of token provided as liquidity across all pairs  
derivedETH| BigDecimal| ETH per token  
### Pair[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#pair "Direct link to Pair")
Information about a pair. Includes references to each token within the pair, volume information, liquidity information, and more. The pair entity mirrors the pair smart contract, and also contains aggregated information about use.
Field Name| Value Type| Description  
---|---|---  
id| ID| pair contract address  
factory| UniswapFactory| reference to Uniswap factory entity  
token0| Token| reference to token0 as stored in pair contract  
token1| Token| reference to token1 as stored in pair contract  
reserve0| BigDecimal| reserve of token0  
reserve1| BigDecimal| reserve of token1  
totalSupply| BigDecimal| total supply of liquidity token distributed to LPs  
reserveETH| BigDecimal| total liquidity in pair stored as an amount of ETH  
reserveUSD| BigDecimal| total liquidity amount in pair stored as an amount of USD  
trackedReserveETH| BigDecimal| total liquidity with only tracked amount (see tracked amounts)  
token0Price| BigDecimal| token0 per token1  
token1Price| BigDecimal| token1 per token0  
volumeToken0| BigDecimal| amount of token0 swapped on this pair  
volumeToken1| BigDecimal| amount of token1 swapped on this pair  
volumeUSD| BigDecimal| total amount swapped all time in this pair stored in USD (only tracked if USD liquidity is above minimum threshold)  
untrackedVolumeUSD| BigDecimal| total amount swapped all time in this pair stored in USD, no minimum liquidity threshold  
txCount| BigInt| all time amount of transactions on this pair  
createdAtTimestamp| BigInt| timestamp contract was created  
createdAtBlockNumber| BigInt| Ethereum block contract was created  
liquidityPositions| [LiquidityPosition]| array of liquidity providers, used as a reference to LP entities  
### User[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#user "Direct link to User")
A user entity is created for any address that provides liquidity to a pool on Uniswap. This entity can be used to track open positions for users. LiquidyPosition entities can be referenced to get specific data about each position.
Field Name| Value Type| Description  
---|---|---  
id| ID| user address  
liquidityPositions| [LiquidityPosition]| array of all liquidity positions user has open  
usdSwapped| BigDecimal| total USD value swapped  
### LiquidityPositiion[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#liquiditypositiion "Direct link to LiquidityPositiion")
This entity is used to store data about a user's liquidity position. This information, along with information from the pair itself can be used to provide position sizes, token deposits, and more.
Field Name| Value Type| Description  
---|---|---  
id| ID| user address and pair address concatenated with a dash  
user| User| reference to user  
pair| Pair| reference to the pair liquidity is being provided on  
liquidityTokenBalance| BigDecimal| amount of LP tokens minted for this position  
### Transaction[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#transaction "Direct link to Transaction")
Transaction entities are created for each Ethereum transaction that contains an interaction within Uniswap contracts. This subgraph tracks Mint, Burn, and Swap events on the Uniswap core contracts. Each transaction contains 3 arrays, and at least one of these arrays has a length of 1.
Field Name| Value Type| Description  
---|---|---  
id| ID| Ethereum transaction hash  
blockNumber| BigInt| block transaction was mined in  
timestamp| BigInt| timestamp for transaction  
mints| [Mint]| array of Mint events within the transaction, 0 or greater  
burns| [Burn]| array of Burn events within transaction, 0 or greater  
swaps| [Swap]| array of Swap events within transaction, 0 or greater  
### Mint[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#mint "Direct link to Mint")
Mint entities are created for every emitted Mint event on the Uniswap core contracts. The Mint entity stores key data about the event like token amounts, who sent the transaction, who received the liquidity, and more. This entity can be used to track liquidity provisions on pairs.
Field Name| Value Type| Description  
---|---|---  
id| ID| Transaction hash plus index in the transaction mint array  
transaction| Transaction| reference to the transaction Mint was included in  
timestamp| BigInt| timestamp of Mint, used to sort recent liquidity provisions  
pair| Pair| reference to pair  
to| Bytes| recipient of liquidity tokens  
liquidity| BigDecimal| amount of liquidity tokens minted  
sender| Bytes| address that initiated the liquidity provision  
amount0| BigDecimal| amount of token0 provided  
amount1| BigDecimal| amount of token1 provided  
logIndex| BigInt| index in the transaction event was emitted  
amountUSD| BigDecimal| derived USD value of token0 amount plus token1 amount  
feeTo| Bytes| address of fee recipient (if fee is on)  
feeLiquidity| BigDecimal| amount of liquidity sent to fee recipient (if fee is on)  
### Burn[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#burn "Direct link to Burn")
Burn entities are created for every emitted Burn event on the Uniswap core contracts. The Burn entity stores key data about the event like token amounts, who burned LP tokens, who received tokens, and more. This entity can be used to track liquidity removals on pairs.
Field Name| Value Type| Description  
---|---|---  
id| ID| Transaction hash plus index in the transaction burn array  
transaction| Transaction| reference to the transaction Burn was included in  
timestamp| BigInt| timestamp of Burn, used to sort recent liquidity removals  
pair| Pair| reference to pair  
to| Bytes| recipient of tokens  
liquidity| BigDecimal| amount of liquidity tokens burned  
sender| Bytes| address that initiated the liquidity removal  
amount0| BigDecimal| amount of token0 removed  
amount1| BigDecimal| amount of token1 removed  
logIndex| BigInt| index in the transaction event was emitted  
amountUSD| BigDecimal| derived USD value of token0 amount plus token1 amount  
feeTo| Bytes| address of fee recipient (if fee is on)  
feeLiquidity| BigDecimal| amount of tokens sent to fee recipient (if fee is on)  
### Swap[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#swap "Direct link to Swap")
Swap entities are created for each token swap within a pair. The Swap entity can be used to get things like swap size (in tokens and USD), sender, recipient and more. See the Swap overview page for more information on amounts.
Field Name| Value Type| Description  
---|---|---  
id| ID| transaction hash plus index in Transaction swap array  
transaction| Transaction| reference to transaction swap was included in  
timestamp| BigInt| timestamp of swap, used for sorted lookups  
pair| Pair| reference to pair  
sender| Bytes| address that initiated the swap  
amount0In| BigDecimal| amount of token0 sold  
amount1In| BigDecimal| amount of token1 sold  
amount0Out| BigDecimal| amount of token0 received  
amount1Out| BigDecimal| amount of token1 received  
to| Bytes| recipient of output tokens  
logIndex| BigInt| event index within transaction  
amountUSD| BigDecimal| derived amount of tokens sold in USD  
### Bundle[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#bundle "Direct link to Bundle")
The Bundle is used as a global store of derived ETH price in USD. Because there is no guaranteed common base token across pairs, a global reference of USD price is useful for deriving other USD values. The Bundle entity stores an updated weighted average of ETH<->Stablecoin pair prices. This provides a strong estimate for the USD price of ETH that can be used in other places in the subgraph.
Field Name| Value Type| Description  
---|---|---  
id| ID| constant 1  
ethPrice| BigDecimal| derived price of ETH in USD based on stablecoin pairs  
## Historical Entities[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#historical-entities "Direct link to Historical Entities")
The subgraph tracks aggregated information grouped by days to provide insights to daily activity on Uniswap. While [time travel queries](https://blocklytics.org/blog/ethereum-blocks-subgraph-made-for-time-travel/) can be used for direct comparison against values in the past, it is much more expensive to query grouped data. For this reason the subgraph tracks information grouped in daily buckets, using timestamps provided by contract events. These entities can be used to query things like total volume on a given day, price of a token on a given day, etc.
For each DayData type, a new entity is created each day.
### UniswapDayData[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#uniswapdaydata "Direct link to UniswapDayData")
Tracks data across all pairs aggregated into a daily bucket.
Field Name| Value Type| Description  
---|---|---  
id| ID| unix timestamp for start of day / 86400 giving a unique day index  
date| Int| unix timestamp for start of day  
dailyVolumeETH| BigDecimal| total volume across all pairs on this day, stored as a derived amount of ETH  
dailyVolumeUSD| BigDecimal| total volume across all pairs on this day, stored as a derived amount of USD  
totalVolumeETH| BigDecimal| all time volume across all pairs in ETH up to and including this day  
totalLiquidityETH| BigDecimal| total liquidity across all pairs in ETH up to and including this day  
totalVolumeUSD| BigDecimal| all time volume across all pairs in USD up to and including this day  
totalLiquidityUSD| BigDecimal| total liquidity across all pairs in USD up to and including this day  
maxStored| Int| reference used to store most liquid tokens, used for historical liquidity charts  
mostLiquidTokens| [TokenDayData!]| tokens with most liquidity in Uniswap  
txCount| BigInt| number of transactions throughout this day  
### Pair Day Data[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#pair-day-data "Direct link to Pair Day Data")
Tracks pair data across each day.
Field Name| Value Type| Description  
---|---|---  
id| ID| pair contract address and day id (day start timestamp in unix / 86400) concatenated with a dash  
date| Int| unix timestamp for start of day  
pairAddress| Bytes| address for pair contract  
token0| Token| reference to token0  
token1| Token| reference to token1  
reserve0| BigDecimal| reserve of token0 (updated during each transaction on pair)  
reserve1| BigDecimal| reserve of token1 (updated during each transaction on pair)  
totalSupply| BigDecimal| total supply of liquidity token distributed to LPs  
reserveUSD| BigDecimal| reserve of token0 plus token1 stored as a derived USD amount  
dailyVolumeToken0| BigDecimal| total amount of token0 swapped throughout day  
dailyVolumeToken1| BigDecimal| total amount of token1 swapped throughout day  
dailyVolumeUSD| BigDecimal| total volume within pair throughout day  
dailyTxns| BigInt| amount of transactions on pair throughout day  
### TokenDayData[​](https://docs.uniswap.org/contracts/v2/reference/API/entities#tokendaydata "Direct link to TokenDayData")
Tracks token data aggregated across all pairs that include token.
Field Name| Value Type| Description  
---|---|---  
id| ID| token address and day id (day start timestamp in unix / 86400) concatenated with a dash  
date| Int| unix timestamp for start of day  
token| Token| reference to token entity  
dailyVolumeToken| BigDecimal| amount of token swapped across all pairs throughout day  
dailyVolumeETH| BigDecimal| amount of token swapped across all pairs throughout day stored as a derived amount of ETH  
dailyVolumeUSD| BigDecimal| amount of token swapped across all pairs throughout day stored as a derived amount of USD  
dailyTxns| BigInt| amount of transactions with this token across all pairs  
totalLiquidityToken| BigDecimal| token amount of token deposited across all pairs  
totalLiquidityETH| BigDecimal| token amount of token deposited across all pairs stored as amount of ETH  
totalLiquidityUSD| BigDecimal| token amount of token deposited across all pairs stored as amount of USD  
priceUSD| BigDecimal| price of token in derived USD  
maxStored| Int| amount of token deposited in pair with highest token liquidity - used only as a reference for storing most liquid pairs for this token  
mostLiquidPairs| [PairDayData]| pairs with most liquidity for this token  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/API/02-entities.md)
Was this helpful?
[PreviousAPI Overview](https://docs.uniswap.org/contracts/v2/reference/API/overview)[NextQueries](https://docs.uniswap.org/contracts/v2/reference/API/queries)
On this page
  * [Uniswap Factory](https://docs.uniswap.org/contracts/v2/reference/API/entities#uniswap-factory)
  * [Token](https://docs.uniswap.org/contracts/v2/reference/API/entities#token)
  * [Pair](https://docs.uniswap.org/contracts/v2/reference/API/entities#pair)
  * [User](https://docs.uniswap.org/contracts/v2/reference/API/entities#user)
  * [LiquidityPositiion](https://docs.uniswap.org/contracts/v2/reference/API/entities#liquiditypositiion)
  * [Transaction](https://docs.uniswap.org/contracts/v2/reference/API/entities#transaction)
  * [Mint](https://docs.uniswap.org/contracts/v2/reference/API/entities#mint)
  * [Burn](https://docs.uniswap.org/contracts/v2/reference/API/entities#burn)
  * [Swap](https://docs.uniswap.org/contracts/v2/reference/API/entities#swap)
  * [Bundle](https://docs.uniswap.org/contracts/v2/reference/API/entities#bundle)
  * [Historical Entities](https://docs.uniswap.org/contracts/v2/reference/API/entities#historical-entities)
    * [UniswapDayData](https://docs.uniswap.org/contracts/v2/reference/API/entities#uniswapdaydata)
    * [Pair Day Data](https://docs.uniswap.org/contracts/v2/reference/API/entities#pair-day-data)
    * [TokenDayData](https://docs.uniswap.org/contracts/v2/reference/API/entities#tokendaydata)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/API/02-entities.md)
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
