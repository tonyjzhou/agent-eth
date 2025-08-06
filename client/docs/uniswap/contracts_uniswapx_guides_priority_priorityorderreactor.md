# https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor

[Skip to main content](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
      * [Quickstart](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
      * [Technical Reference](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
    * [v3 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
    * [Smart Wallet](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
      * [Overview](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [UniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
        * [Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
        * [Arbitrum](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
        * [Unichain, Base](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
          * [Priority Order Reactor](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
        * [Webhook Support](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)
    * [Universal Router](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
    * [Permit2](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
    * [v2 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
    * [v1 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * UniswapX
  * Guides
  * Unichain, Base
  * [Priority Order Reactor](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor)


On this page
# What is the Priority Order Reactor?
The [Priority Order Reactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/PriorityOrderReactor.sol) is a new, experimental UniswapX reactor built specifically for optimal execution chains that utilize Priority Gas Auctions (PGA) for ordering transactions. This new reactor type, which is based on research presented in [Priority is All you Need](https://www.paradigm.xyz/2024/06/priority-is-all-you-need), allows fillers to bid on orders during fulfillment through setting custom priority fees.
## Example Implementation[​](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#example-implementation "Direct link to Example Implementation")
Alice submits a PriorityOrder offering 1 ETH in exchange for a minimum of 1000 USDC. The fair market rate for the order is 1100 USDC, resulting in around 100 USDC in potential profit.
If we assume a filler has a desired margin of 10% of the total profit, the best price is 1090 USDC. This would be 900 bps of improvement. This filler would convert bps to mps (see below for details) to get 900 * 1000 = 900,000 mps of improvement. Thus they would set priorityFee of 900,000 wei on their fill transaction. Keep in mind that this is additional to the base fee.
# Important considerations
  * The PriorityOrderReactor is only meant to be used on chains which order transactions by priority fee.
  * We do not plan to run any preliminary auctions for the start price of these orders, rather we set a minimum price that each order must be executed at.
  * Each order is only executable after a certain block specified by the user. This block will be a few blocks in the future from when the order is made available through the UniswapX orders endpoint. To ensure the best UX for our users, Uniswap Labs has the ability to make the start block earlier by cosigning the order.
  * Only the fill transaction with the highest priority fee will win the order, all other transactions will revert on chain.
  * To minimize the gas used on reverting transactions, we revert early if the order is already filled or is not fillable yet.
  * For every wei of priority fee above a certain threshold (an optional value specified in the order), the user is owed 1 milli-bps more of their output token (or less of their input token).
  * Milli-bps (or MPS) are one-thousandth of a basis point.


## Retrieving and Executing Signed Orders[​](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#retrieving-and-executing-signed-orders "Direct link to Retrieving and Executing Signed Orders")
All signed Priority Orders created through the Uniswap UI will be available via the UniswapX Orders Endpoint. We have [swagger documentation](https://api.uniswap.org/v2/uniswapx/docs) but see below for a quick example curl.
```
GET https://api.uniswap.org/v2/orders?orderStatus=open&chainId=8453&orderType=Priority
```

As a lower latency alternative to polling the API, fillers can also apply to register a webhook and receive a feed of all open orders. See details for registering [here](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks).
After fetching orders, use the latest version of the [UniswapX SDK](https://github.com/Uniswap/sdks/tree/main/sdks/uniswapx-sdk#parsing-orders). Requires [2.1.0-beta.13](https://www.npmjs.com/package/@uniswap/uniswapx-sdk/v/2.1.0-beta.13) or later.
### Parsing an order[​](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#parsing-an-order "Direct link to Parsing an order")
```
import{ CosignedPriorityOrder, Order }from'@uniswap/uniswapx-sdk';const serializedOrder ='0x1111222233334444555500000000234300234...';const chainId =1;const order: Order = CosignedPriorityOrder.parse(serializedOrder, chainId);const orderData = order.info;const orderHash = order.hash();
```

The existing `UniswapXOrderQuoter` can also be used to quote priority orders, however, you must use an JsonRpcProvider which supports block overrides. Without block overrides, the SDK quoter cannot validate the entire order as the block number is checked first in the contract.
## Executing an order[​](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#executing-an-order "Direct link to Executing an order")
The [PriorityOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/PriorityOrderReactor.sol) shares the same interface as all other existing UniswapX reactors. Orders are executed against the `execute` and `executeBatch` functions, and optionally a callback is available via `executeWithCallback` and `executeBatchWithCallback`.
# Deployment addresses
The PriorityOrderReactor is deployed on the following chains:
Chain| Source| Address  
---|---|---  
Unichain| [PriorityOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/PriorityOrderReactor.sol)| [0x00000006021a6Bce796be7ba509BBBA71e956e37](https://uniscan.xyz/address/0x00000006021a6Bce796be7ba509BBBA71e956e37)  
Base| [PriorityOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/PriorityOrderReactor.sol)| [0x000000001Ec5656dcdB24D90DFa42742738De729](https://basescan.org/address/0x000000001Ec5656dcdB24D90DFa42742738De729)  
# Timeline
The Priority Order Reactor has been live on Base since August 2024 and is now live on Unichain. Join the [UniswapX Fillers Channel](https://t.me/UniswapXdiscussion) for more details.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/guides/priority/priorityorderreactor.md)
Was this helpful?
[PreviousFilling on Arbitrum](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)[NextWebhook Support](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)
On this page
  * [Example Implementation](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#example-implementation)
  * [Retrieving and Executing Signed Orders](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#retrieving-and-executing-signed-orders)
    * [Parsing an order](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#parsing-an-order)
  * [Executing an order](https://docs.uniswap.org/contracts/uniswapx/guides/priority/priorityorderreactor#executing-an-order)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/guides/priority/priorityorderreactor.md)
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
