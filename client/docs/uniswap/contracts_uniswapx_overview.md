# https://docs.uniswap.org/contracts/uniswapx/overview

[Skip to main content](https://docs.uniswap.org/contracts/uniswapx/overview#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [Quickstart](https://docs.uniswap.org/contracts/uniswapx/overview)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/uniswapx/overview)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/uniswapx/overview)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [v3 Protocol](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [Overview](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [UniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/overview)
        * [Mainnet](https://docs.uniswap.org/contracts/uniswapx/overview)
        * [Arbitrum](https://docs.uniswap.org/contracts/uniswapx/overview)
        * [Unichain, Base](https://docs.uniswap.org/contracts/uniswapx/overview)
        * [Webhook Support](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)
    * [Universal Router](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Permit2](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/uniswapx/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * UniswapX
  * [Overview](https://docs.uniswap.org/contracts/uniswapx/overview)


On this page
# UniswapX
UniswapX is a new permissionless, open source (GPL), auction-based routing protocol for trading across AMMs and other liquidity sources.
UniswapX improves swapping in several ways:
  * Better prices by aggregating liquidity sources
  * Gas-free swapping
  * Protection against MEV (Maximal Extractable Value)
  * No cost for failed transactions
  * And in the coming months, UniswapX will expand to gas-free cross-chain swaps.


Swappers generate signed orders which specify the intents of their swap, and fillers compete using arbitrary filling strategies to satisfy these orders.
# Trading on UniswapX
To trade using UniswapX, swappers create a new type of order called an Exclusive Dutch Order which specifies the maximum and minimum outputs they are willing to receive in a trade over a certain time period.
They then sign a message that uses Permit2 to allow the transfer of tokens to complete the trade as long as the number of tokens sent and received matched what is specified in the decay curve. These Signed Order messages are broadcast publicly and available to be executed by anyone who wants to be a “filler”.
## Fillers on UniswapX[​](https://docs.uniswap.org/contracts/uniswapx/overview#fillers-on-uniswapx "Direct link to Fillers on UniswapX")
UniswapX introduces a new participant in the Uniswap ecosystem, the _Filler_. These agents pickup signed orders from swappers and compete to execute them using any source of liquidity they have access to.
Anyone can fill orders on UniswapX, get started by reading our [Filler Integration Guide](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller).
## Parametizing UniswapX Orders on Mainnet[​](https://docs.uniswap.org/contracts/uniswapx/overview#parametizing-uniswapx-orders-on-mainnet "Direct link to Parametizing UniswapX Orders on Mainnet")
The UniswapX protocol on Mainnet does not explicitly parameterize the pricing of orders like the Exclusive Dutch Order, rather order parameterization is left to be configured by the order constructor.
In the current Uniswap Labs interface implementation of UniswapX, some fillers may choose to help parameterize orders on Mainnet by participating as quoters. These fillers can _only_ win a quote if they guarantee improved swapper execution over Uniswap v3 or v2 liquidity pools. Fillers who win a quote will receive execution priority for a limited period of time to fill orders they submitted wining quotes for.
To ensure a smooth swapping experience for traders, the set of Quoters will be vetted by Uniswap Labs following UniswapX’s launch, with plans to make the quoting system fully permissionless in the near future.
If you are interested in participating as a Quoter, please reach out here.
# UniswapX Protocol Architecture
![UniswapX](https://docs.uniswap.org/assets/images/UniswapX-60eee10e73f923436d8858b6fdded078.png)
### Reactors[​](https://docs.uniswap.org/contracts/uniswapx/overview#reactors "Direct link to Reactors")
Order Reactors _settle_ UniswapX orders. They are responsible for validating orders of a specific type, resolving them into inputs and outputs, and executing them against the filler's strategy, and verifying that the order was successfully fulfilled.
Reactors process orders using the following steps:
  * Validate the order
  * Resolve the order into inputs and outputs
  * Pull input tokens from the swapper to the fillContract using permit2 `permitWitnessTransferFrom` with the order as witness
  * Call `reactorCallback` on the fillContract
  * Verify that the output tokens were received by the output recipients


Reactors implement the [IReactor](https://github.com/Uniswap/UniswapX/blob/main/src/interfaces/IReactor.sol) interface which abstracts the specifics of the order specification. This allows for different reactor implementations with different order formats to be used with the same interface, allowing for shared infrastructure and easy extension by fillers.
Current reactor implementations:
  * [V2DutchOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/V2DutchOrderReactor.sol): The latest version of the Dutch Order Reactor, that settle v2 linear decay dutch orders.
  * [LimitOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/LimitOrderReactor.sol): A reactor that settles simple static limit orders
  * [DutchOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/DutchOrderReactor.sol): A reactor that settles linear-decay dutch orders
  * [ExclusiveDutchOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/ExclusiveDutchOrderReactor.sol): A reactor that settles linear-decay dutch orders with a period of exclusivity before decay begins
  * [PriorityOrderReactor.sol](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/PriorityOrderReactor.sol): A reactor that settles orders which implement priority taxes


### Fill Contracts[​](https://docs.uniswap.org/contracts/uniswapx/overview#fill-contracts "Direct link to Fill Contracts")
Order fillContracts _fill_ UniswapX orders. They specify the filler's strategy for fulfilling orders and are called by the reactor with `reactorCallback`.
Some sample fillContract implementations are provided in this repository:
  * [SwapRouter02Executor](https://github.com/Uniswap/UniswapX/blob/main/src/sample-executors/SwapRouter02Executor.sol): A fillContract that uses UniswapV2 and UniswapV3 via the SwapRouter02 router


### Direct Fill[​](https://docs.uniswap.org/contracts/uniswapx/overview#direct-fill "Direct link to Direct Fill")
If a filler wants to fill orders using funds on-hand rather than a fillContract, they can do so gas efficiently using the `directFill` macro by specifying `address(1)` as the fillContract. This will pull tokens from the filler using `msg.sender` to satisfy the order outputs.
# Whitepaper
More details on the UniswapX protocol are available in the [UniswapX Whitepaper](https://uniswap.org/whitepaper-uniswapx.pdf).
# Deployment Addresses
Contract| Address| Source  
---|---|---  
V2 Dutch Order Reactor| [0x00000011f84b9aa48e5f8aa8b9897600006289be](https://etherscan.io/address/0x00000011f84b9aa48e5f8aa8b9897600006289be)| [V2DutchOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/V2DutchOrderReactor.sol)  
V1 Exclusive Dutch Order Reactor| [0x6000da47483062A0D734Ba3dc7576Ce6A0B645C4](https://etherscan.io/address/0x6000da47483062A0D734Ba3dc7576Ce6A0B645C4)| [ExclusiveDutchOrderReactor](https://github.com/Uniswap/UniswapX/blob/v1.0.0/src/reactors/ExclusiveDutchOrderReactor.sol)  
OrderQuoter| [0x54539967a06Fc0E3C3ED0ee320Eb67362D13C5fF](https://etherscan.io/address/0x54539967a06Fc0E3C3ED0ee320Eb67362D13C5fF)| [OrderQuoter](https://github.com/Uniswap/UniswapX/blob/v1.0.0/src/OrderQuoter.sol)  
Permit2| [0x000000000022D473030F116dDEE9F6B43aC78BA3](https://etherscan.io/address/0x000000000022D473030F116dDEE9F6B43aC78BA3)| [Permit2](https://github.com/Uniswap/permit2)  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/01-overview.md)
Was this helpful?
[PreviousTechnical Reference](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)[NextUniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
On this page
  * [Fillers on UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview#fillers-on-uniswapx)
  * [Parametizing UniswapX Orders on Mainnet](https://docs.uniswap.org/contracts/uniswapx/overview#parametizing-uniswapx-orders-on-mainnet)
    * [Reactors](https://docs.uniswap.org/contracts/uniswapx/overview#reactors)
    * [Fill Contracts](https://docs.uniswap.org/contracts/uniswapx/overview#fill-contracts)
    * [Direct Fill](https://docs.uniswap.org/contracts/uniswapx/overview#direct-fill)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/01-overview.md)
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
