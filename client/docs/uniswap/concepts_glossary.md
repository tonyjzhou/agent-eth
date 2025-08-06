# https://docs.uniswap.org/concepts/glossary

[Skip to main content](https://docs.uniswap.org/concepts/glossary#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/glossary)
    * [Governance](https://docs.uniswap.org/concepts/glossary)
    * [Research](https://docs.uniswap.org/concepts/research)
    * [Resources](https://docs.uniswap.org/concepts/resources)
    * [Glossary](https://docs.uniswap.org/concepts/glossary)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Glossary](https://docs.uniswap.org/concepts/glossary)


On this page
# Glossary
## Automated Market Maker[​](https://docs.uniswap.org/concepts/glossary#automated-market-maker "Direct link to Automated Market Maker")
An automated market maker is a smart contract on Ethereum that holds liquidity reserves. Users can trade against these reserves at prices determined by a fixed formula. Anyone may contribute liquidity to these smart contracts, earning pro-rata trading fees in return.
## Asset[​](https://docs.uniswap.org/concepts/glossary#asset "Direct link to Asset")
While a digital asset can take many forms, the Uniswap Protocol supports ERC-20 token pairs, and represents a position in the form of an NFT (ERC-721).
## Concentrated Liquidity[​](https://docs.uniswap.org/concepts/glossary#concentrated-liquidity "Direct link to Concentrated Liquidity")
Liquidity that is allocated within a determined price range.
## Constant Product Formula[​](https://docs.uniswap.org/concepts/glossary#constant-product-formula "Direct link to Constant Product Formula")
The automated market making algorithm used by Uniswap. In v1 and v2, this was x*y=k.
## Core[​](https://docs.uniswap.org/concepts/glossary#core "Direct link to Core")
Smart contracts that are considered foundational, and are essential for Uniswap to exist. Upgrading to a new version of core would require deploying an entirely new set of smart contracts on Ethereum and would be considered a new version of the Uniswap Protocol.
## ERC20[​](https://docs.uniswap.org/concepts/glossary#erc20 "Direct link to ERC20")
ERC20 tokens are fungible tokens on Ethereum. Uniswap supports all standard ERC20 implementations.
## Factory[​](https://docs.uniswap.org/concepts/glossary#factory "Direct link to Factory")
A smart contract that deploys a unique smart contract for any ERC20/ERC20 trading pair.
## Flash Swap[​](https://docs.uniswap.org/concepts/glossary#flash-swap "Direct link to Flash Swap")
A trade that uses the tokens purchased before paying for them.
## Invariant[​](https://docs.uniswap.org/concepts/glossary#invariant "Direct link to Invariant")
The “k” value in the constant product formula X*Y=K
## Liquidity Provider / "LP"[​](https://docs.uniswap.org/concepts/glossary#liquidity-provider--lp "Direct link to Liquidity Provider / "LP"")
A liquidity provider is someone who deposits ERC20 tokens into a given liquidity pool. Liquidity providers take on price risk and are compensated with trading fees.
## Liquidity[​](https://docs.uniswap.org/concepts/glossary#liquidity "Direct link to Liquidity")
Digital assets that are stored in a Uniswap pool contract, and are able to be traded against by traders.
## Mid Price[​](https://docs.uniswap.org/concepts/glossary#mid-price "Direct link to Mid Price")
The price between the available buy and sell prices. In Uniswap v1 and v2, this is the ratio of the two ERC20 token reserves. In V3, this is the ratio of the two ERC20 token reserves available within the current active tick.
## Observation[​](https://docs.uniswap.org/concepts/glossary#observation "Direct link to Observation")
An instance of historical price and liquidity data of a given pair.
## Pair[​](https://docs.uniswap.org/concepts/glossary#pair "Direct link to Pair")
A smart contract deployed from a Uniswap v1 or v2 factory contract that enables trading between two ERC20 tokens. Pair contracts are now called Pools in v3.
## Periphery[​](https://docs.uniswap.org/concepts/glossary#periphery "Direct link to Periphery")
External smart contracts that are useful, but not required for Uniswap to exist. New periphery contracts can always be deployed without migrating liquidity.
## Pool[​](https://docs.uniswap.org/concepts/glossary#pool "Direct link to Pool")
A contract deployed by the V3 factory that pairs two ERC-20 assets. Different pools may have different fees despite containing the same token pair. Pools were previously called Pairs before the introduction of multiple fee options.
## Position[​](https://docs.uniswap.org/concepts/glossary#position "Direct link to Position")
An instance of liquidity defined by upper and lower tick. And the amount of liquidity contained therein.
## Price Impact[​](https://docs.uniswap.org/concepts/glossary#price-impact "Direct link to Price Impact")
The difference between the mid-price and the execution price **caused by your trade size relative to the pool’s liquidity**. This is an expected result of the constant product formula in AMMs.
## Protocol Fees[​](https://docs.uniswap.org/concepts/glossary#protocol-fees "Direct link to Protocol Fees")
Fees that are rewarded to the protocol itself, rather than to liquidity providers.
## Range[​](https://docs.uniswap.org/concepts/glossary#range "Direct link to Range")
Any interval between two ticks of any distance.
## Range Order[​](https://docs.uniswap.org/concepts/glossary#range-order "Direct link to Range Order")
An approximation of a limit order, in which a single asset is provided as liquidity across a specified range, and is continuously swapped to the destination address as the spot price crosses the range.
## Reserves[​](https://docs.uniswap.org/concepts/glossary#reserves "Direct link to Reserves")
The liquidity available within a pair. This was more commonly referenced before concentrated liquidity was introduced.
## Slippage[​](https://docs.uniswap.org/concepts/glossary#slippage "Direct link to Slippage")
The total difference between the expected price at the time of submitting a transaction and the actual execution price, which may include price impact and other market movements that occur before the transaction is mined.
## Spot Price[​](https://docs.uniswap.org/concepts/glossary#spot-price "Direct link to Spot Price")
The current price of a token relative to another within a given pair.
## Swap Fees[​](https://docs.uniswap.org/concepts/glossary#swap-fees "Direct link to Swap Fees")
The fees collected upon swapping which are rewarded to liquidity providers.
## Tick Interval[​](https://docs.uniswap.org/concepts/glossary#tick-interval "Direct link to Tick Interval")
The price space between two nearest ticks.
## Tick[​](https://docs.uniswap.org/concepts/glossary#tick "Direct link to Tick")
The boundaries between discrete areas in price space.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/glossary.md)
Was this helpful?
[PreviousResources](https://docs.uniswap.org/concepts/resources)[NextOverview](https://docs.uniswap.org/contracts/v4/overview)
On this page
  * [Automated Market Maker](https://docs.uniswap.org/concepts/glossary#automated-market-maker)
  * [Asset](https://docs.uniswap.org/concepts/glossary#asset)
  * [Concentrated Liquidity](https://docs.uniswap.org/concepts/glossary#concentrated-liquidity)
  * [Constant Product Formula](https://docs.uniswap.org/concepts/glossary#constant-product-formula)
  * [Core](https://docs.uniswap.org/concepts/glossary#core)
  * [ERC20](https://docs.uniswap.org/concepts/glossary#erc20)
  * [Factory](https://docs.uniswap.org/concepts/glossary#factory)
  * [Flash Swap](https://docs.uniswap.org/concepts/glossary#flash-swap)
  * [Invariant](https://docs.uniswap.org/concepts/glossary#invariant)
  * [Liquidity Provider / "LP"](https://docs.uniswap.org/concepts/glossary#liquidity-provider--lp)
  * [Liquidity](https://docs.uniswap.org/concepts/glossary#liquidity)
  * [Mid Price](https://docs.uniswap.org/concepts/glossary#mid-price)
  * [Observation](https://docs.uniswap.org/concepts/glossary#observation)
  * [Pair](https://docs.uniswap.org/concepts/glossary#pair)
  * [Periphery](https://docs.uniswap.org/concepts/glossary#periphery)
  * [Pool](https://docs.uniswap.org/concepts/glossary#pool)
  * [Position](https://docs.uniswap.org/concepts/glossary#position)
  * [Price Impact](https://docs.uniswap.org/concepts/glossary#price-impact)
  * [Protocol Fees](https://docs.uniswap.org/concepts/glossary#protocol-fees)
  * [Range](https://docs.uniswap.org/concepts/glossary#range)
  * [Range Order](https://docs.uniswap.org/concepts/glossary#range-order)
  * [Reserves](https://docs.uniswap.org/concepts/glossary#reserves)
  * [Slippage](https://docs.uniswap.org/concepts/glossary#slippage)
  * [Spot Price](https://docs.uniswap.org/concepts/glossary#spot-price)
  * [Swap Fees](https://docs.uniswap.org/concepts/glossary#swap-fees)
  * [Tick Interval](https://docs.uniswap.org/concepts/glossary#tick-interval)
  * [Tick](https://docs.uniswap.org/concepts/glossary#tick)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/glossary.md)
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
