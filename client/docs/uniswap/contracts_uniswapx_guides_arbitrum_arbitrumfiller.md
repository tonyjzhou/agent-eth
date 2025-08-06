# https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller

[Skip to main content](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
      * [Quickstart](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
      * [Technical Reference](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
    * [v3 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
    * [Smart Wallet](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
      * [Overview](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [UniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
        * [Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
        * [Arbitrum](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
          * [Filling on Arbitrum](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
        * [Unichain, Base](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
        * [Webhook Support](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)
    * [Universal Router](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
    * [Permit2](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
    * [v2 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
    * [v1 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * UniswapX
  * Guides
  * Arbitrum
  * [Filling on Arbitrum](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)


On this page
# Arbitrum Pilot Overview
Starting June 19 2024, the Uniswap team began running a portion of trades on Arbitrum through UniswapX. Unlike UniswapX on Mainnet, these orders will have **no RFQ portion and thus no exclusivity** during the pilot.
Filling on Arbitrum, however, follows the same two steps as filling on Mainnet:
  1. Retrieving signed orders
  2. Filling orders


## Retrieving Signed Orders[​](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#retrieving-signed-orders "Direct link to Retrieving Signed Orders")
All signed Dutch Orders on Arbitrum, created through the Uniswap UI will be available via the UniswapX Orders Endpoint. We have [swagger documentation](https://api.uniswap.org/v2/uniswapx/docs) but see below for a quick example curl.
```
GET https://api.uniswap.org/v2/orders?orderStatus=open&chainId=42161&limit=1000
```

Use the [UniswapX SDK](https://github.com/Uniswap/sdks/tree/main/sdks/uniswapx-sdk) to parse the `encodedOrder` field returned from endpoint. Each one of these `Orders` represents a fillable user trader.
As a lower latency alternative to polling the API, fillers can also apply to register a webhook and receive a feed of all open orders. See details for registering [here](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)
## Filling Orders[​](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#filling-orders "Direct link to Filling Orders")
To execute a discovered order, a filler needs to call the [execute](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/BaseReactor.sol#L31) method of the Reactor specified in the retrieved `encodedOrder` body. Currently the Reactor used by the Uniswap interface is located at:
[0xB274d5F4b833b61B340b654d600A864fB604a87c](https://arbiscan.io/address/0xb274d5f4b833b61b340b654d600a864fb604a87c)
Always confirm the address from the retrieved order before submitting.
The simplest fill strategy is called `Direct Filler`, where the trade is executed directly against tokens held in the fillers address. To use this strategy, a filler can simply approve the order's output tokens to the reactor and call `execute` or `executeBatch` from their address. (see [source](https://github.com/Uniswap/UniswapX/blob/v2.0.0-deploy/src/reactors/BaseReactor.sol)):
```
// Execute direct filler orderoutputToken.approve(reactor,type(uint256).max);reactor.execute(order);
```

More sophisticated fillers can implement arbitrarily complex strategies by deploying their own Executor contracts. This contract should implement the [IReactorCallback](https://github.com/Uniswap/UniswapX/blob/v2.0.0-deploy/src/interfaces/IReactorCallback.sol) interface, which takes in an order with input tokens and acquires the allotted number of output tokens for the caller. It must approve the output tokens to the reactor, which then transfers them to the order output recipients to settle the order. Executor contracts must call `reactor.executeWithCallback` or `reactor.executeBatchWithCallback`. They can also specify arbitrary callback data that will be passed into the `reactorCallback` call.
```
contractExecutor{functionexecute(Order calldata order,bytescalldata callbackData){  reactor.executeWithCallback(order, callbackData)}functionreactorCallback(ResolvedOrder[]calldata orders,bytescalldata callbackData){// implement strategy here}}// Execute custom fill strategyaddress executor =/* Address of deployed executor contract */;bytes fillData =/* Call data to be sent to your executor contract */;executor.execute(order, fillData);
```

For convenience, we’ve provided an [example Executor Contract](https://github.com/Uniswap/UniswapX/blob/v2.0.0-deploy/src/sample-executors/SwapRouter02Executor.sol) which demonstrates how a filler could implement a strategy that executes a UniswapX order against a Uniswap V3 pool. These contracts should be deployed to each chain that the filler would like to support.
## Order Types[​](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#order-types "Direct link to Order Types")
On Arbitrum, both DutchV2 and DutchV3 order types are supported. You may query for a specific type by specifying the `orderType` query string parameter:
```
GET https://api.uniswap.org/v2/orders?orderStatus=open&chainId=42161&limit=1000&orderType={Dutch_V2 | Dutch_V3}
```

The main difference between the two order types is that DutchV2 orders use a time-based decay while DutchV3 use a block-based decay. Because the `block.timestamp` of the EVM does not allow for millisecond-level granularity, all decay in DutchV2 must happen in seconds which does not take advantage of Arbitrum's 250 ms block frequency. For this reason, we will migrate Arbitrum to DutchV3 orders over time.
### Order Type References[​](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#order-type-references "Direct link to Order Type References")
OrderType| Contract Address| Reactor Specification| Example Filler Implementation  
---|---|---|---  
DutchV3| `0xB274d5F4b833b61B340b654d600A864fB604a87c`| [V3DutchOrderReactor.sol](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/V3DutchOrderReactor.sol)| [dutchv3_strategy.rs](https://github.com/Uniswap/uniswapx-artemis/blob/main/src/strategies/dutchv3_strategy.rs)  
DutchV2 (deprecated April 15, 2025)| `0x1bd1aAdc9E230626C44a139d7E70d842749351eb`| [V2DutchOrderReactor.sol](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/V2DutchOrderReactor.sol)| [uniswapx_strategy.rs](https://github.com/Uniswap/uniswapx-artemis/blob/main/src/strategies/uniswapx_strategy.rs)  
# Get in touch
  * To keep up to date, join our [Announcements Channel](https://t.me/uniswapx_fillers)
  * To ask questions and discuss, join our [Fillers Group](https://t.me/UniswapXdiscussion)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/guides/arbitrum/arbitrumfiller.md)
Was this helpful?
[PreviousBecome a Quoter](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)[NextPriority Order Reactor](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
On this page
  * [Retrieving Signed Orders](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#retrieving-signed-orders)
  * [Filling Orders](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#filling-orders)
  * [Order Types](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#order-types)
    * [Order Type References](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller#order-type-references)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/guides/arbitrum/arbitrumfiller.md)
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
