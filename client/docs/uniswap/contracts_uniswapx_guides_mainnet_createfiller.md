# https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller

[Skip to main content](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
      * [Quickstart](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
      * [Technical Reference](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
    * [v3 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
    * [Smart Wallet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
      * [Overview](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [UniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
        * [Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
          * [Filling on Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
          * [Become a Quoter](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
        * [Arbitrum](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
        * [Unichain, Base](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
        * [Webhook Support](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)
    * [Universal Router](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
    * [Permit2](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
    * [v2 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
    * [v1 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * UniswapX
  * Guides
  * Mainnet
  * [Filling on Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)


On this page
# Integrating as a Filler
There are two components to integrating as a filler: defining a filler execution strategy and retrieving & executing discovered orders.
## 1. Defining a Filler Execution Strategy[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#1-defining-a-filler-execution-strategy "Direct link to 1. Defining a Filler Execution Strategy")
To execute a discovered order, a filler needs to call one of the `execute` methods ([source](https://github.com/Uniswap/UniswapX/blob/v1.1.0/src/reactors/BaseReactor.sol#L31)) of an Order Reactor, providing it with the orders to execute.
The simplest fill strategy is called `Direct Filler`, where the trade is executed directly against tokens held in the fillers address. To use this strategy, a filler can simply approve the order's output tokens to the reactor and call `execute` or `executeBatch` from their address. (see [source](https://github.com/Uniswap/UniswapX/blob/v1.1.0/src/reactors/BaseReactor.sol#L35)):
```
// Execute direct filler orderoutputToken.approve(reactor,type(uint256).max);reactor.execute(order);
```

More sophisticated fillers can implement arbitrarily complex strategies by deploying their own Executor contracts. This contract should implement the [IReactorCallback](https://github.com/Uniswap/UniswapX/blob/v1.1.0/src/interfaces/IReactorCallback.sol) interface, which takes in an order with input tokens and acquires the allotted number of output tokens for the caller. It must approve the output tokens to the reactor, which then transfers them to the order output recipients to settle the order. Executor contracts must call `reactor.executeWithCallback` or `reactor.executeBatchWithCallback`. They can also specify arbitrary callback data that will be passed into the `reactorCallback` call.
```
contractExecutor{functionexecute(Order calldata order,bytescalldata callbackData){  reactor.executeWithCallback(order, callbackData)}functionreactorCallback(ResolvedOrder[]calldata orders,bytescalldata callbackData){// implement strategy here}}// Execute custom fill strategyaddress executor =/* Address of deployed executor contract */;bytes fillData =/* Call data to be sent to your executor contract */;executor.execute(order, fillData);
```

For convenience, we’ve provided an [example Executor Contract](https://github.com/Uniswap/UniswapX/tree/v1.1.0/src/sample-executors) which demonstrates how a filler could implement a strategy that executes a UniswapX order against a Uniswap V3 pool. These contracts should be deployed to each chain that the filler would like to support.
## 2A. Retrieve & Execute Signed Dutch Orders[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#2a-retrieve--execute-signed-dutch-orders "Direct link to 2A. Retrieve & Execute Signed Dutch Orders")
All signed Dutch Orders created through the Uniswap UI will be available via the UniswapX Orders Endpoint. We have [swagger documentation](https://api.uniswap.org/v2/uniswapx/docs) but see below for a quick example curl.
```
GET https://api.uniswap.org/v2/orders?orderStatus=open&chainId=1&limit=1
```

As a lower latency alternative to polling the API, fillers can also apply to register a webhook and receive a feed of all open orders. See details for registering [here](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks).
It’s up to the individual filler to architect their own systems for finding and executing profitable orders, but the basic flow is as follows:
  1. Call `GET` on the `/orders` of the UniswapX Orders Endpoint as written above, to retrieve open signed orders. Dutch Orders are available on Mainnet (`chainId=1`) and Arbitrum (`chainId=42161`).
  2. Decode returned orders using the [UniswapX SDK](https://github.com/Uniswap/UniswapX-sdk/#parsing-orders).
  3. Determine which orders you would like to execute.
  4. Send a new transaction to the [execute](https://github.com/Uniswap/UniswapX/blob/a2025e3306312fc284a29daebdcabb88b50037c2/src/reactors/BaseReactor.sol#L29) or [executeBatch](https://github.com/Uniswap/UniswapX/blob/a2025e3306312fc284a29daebdcabb88b50037c2/src/reactors/BaseReactor.sol#L37) methods of the [Dutch Order Reactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/DutchOrderReactor.sol) specifying the signed orders you’d like to fill and the address of your executor contract.


## 2B. Retrieve & Execute Signed Limit Orders[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#2b-retrieve--execute-signed-limit-orders "Direct link to 2B. Retrieve & Execute Signed Limit Orders")
The process for retrieving and executing limit orders is the same as Dutch Orders above except that Limit Orders will be retrieved from the [Limit Orders Endpoint](https://api.uniswap.org/v2/limit-orders) (full API docs [here](https://api.uniswap.org/v2/uniswapx/docs)) and executed against the [Limit Order Reactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/LimitOrderReactor.sol). The process is:
  1. Call `GET` on the `/limit-orders` of the UniswapX Limit Orders Endpoint as written above, to retrieve open signed orders
  2. Decode returned orders using the [UniswapX SDK](https://github.com/Uniswap/UniswapX-sdk/#parsing-orders)
  3. Send a new transaction to the [execute](https://github.com/Uniswap/UniswapX/blob/a2025e3306312fc284a29daebdcabb88b50037c2/src/reactors/BaseReactor.sol#L29) or [executeBatch](https://github.com/Uniswap/UniswapX/blob/a2025e3306312fc284a29daebdcabb88b50037c2/src/reactors/BaseReactor.sol#L37) methods of the [Limit Order Reactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/LimitOrderReactor.sol) specifying the signed orders you’d like to fill and the address of your executor contract


For Dutch and Limit Orders, if the order is valid it will be competing against other fillers attempts to execute it in a gas auction. For this reason, we recommend submitting these transactions through a service like [Flashbots Protect](https://docs.flashbots.net/flashbots-protect/overview).
## Helpful Links[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#helpful-links "Direct link to Helpful Links")
  * [UniswapX Fillers - Announcements channel](https://t.me/uniswapx_fillers)
  * [UniswapX Fillers - Discussion](https://t.me/UniswapXdiscussion)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/guides/mainnet/createFiller.md)
Was this helpful?
[PreviousUniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)[NextBecome a Quoter](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
On this page
  * [1. Defining a Filler Execution Strategy](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#1-defining-a-filler-execution-strategy)
  * [2A. Retrieve & Execute Signed Dutch Orders](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#2a-retrieve--execute-signed-dutch-orders)
  * [2B. Retrieve & Execute Signed Limit Orders](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#2b-retrieve--execute-signed-limit-orders)
  * [Helpful Links](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller#helpful-links)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/guides/mainnet/createFiller.md)
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
