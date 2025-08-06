# https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter

[Skip to main content](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
      * [Quickstart](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
      * [Technical Reference](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
    * [v3 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
    * [Smart Wallet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
      * [Overview](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [UniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
        * [Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
          * [Filling on Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
          * [Become a Quoter](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
        * [Arbitrum](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
        * [Unichain, Base](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
        * [Webhook Support](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)
    * [Universal Router](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
    * [Permit2](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
    * [v2 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
    * [v1 Protocol](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * UniswapX
  * Guides
  * Mainnet
  * [Become a Quoter](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter)


On this page
# Quoting on UniswapX
This guide provides step-by-step instructions for integrating as a Quoter on UniswapX. It is intended for experienced defi teams that have experience running similar systems on other protocols.
To ensure a smooth swapping experience for traders, the set of Quoters will be vetted by Uniswap Labs following UniswapX’s launch. There are plans to make the quoting system fully permissionless in the near future.
Once you've been approved to be a quoter by the Uniswap Labs team, follow the instructions below to complete your integration. If you have not been approved, please join the waitlist by filling out our [intake form](https://uniswap.typeform.com/to/UiPDKgY6).
# Getting Started as a Quoter
To participate as a quoter, you must host a service that adheres to the UniswapX RFQ API schema and responds to requests with quotes. The RFQ participant who submits the best quote for a given order will receive exclusive rights to fill it using their Executor during the _Exclusivity Period_ of the auction.
## Performance Expectations[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#performance-expectations "Direct link to Performance Expectations")
To ensure a smooth experience for swappers and a fair auction process for quoters, we will hold participants to the following performance standards:
  * _**500ms Response Time:**_ Your server must respond to a request for a quote within **500ms**. If you cannot provide a quote, respond with a 204 status code.


Consistent failure to meet this standard may result in suspension from the system.
## Handling Fades & the Circuit Breaker[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#handling-fades--the-circuit-breaker "Direct link to Handling Fades & the Circuit Breaker")
Quoters are expected to honor and execute the quotes they submit. If a quoter submits a winning quote but fails to fill the subsequent order, the "circuit breaker" will be triggered, temporarily disabling the quoter from receiving new requests.
  * _**Cooldown Time:**_ The cooldown period starts at 15 minutes for the first fade and increases exponentially for consecutive fades. More details on the cooldown calculation can be found in the [source code](https://github.com/Uniswap/uniswapx-parameterization-api/blob/bf87dcc0066fa21b72255f7155f5fbd04a518594/lib/cron/fade-rate-v2.ts#L215).


# RFQ API Integration Details
To successfully receive and respond to UniswapX RFQ Quotes, you must have a publicly accessible endpoint that handles incoming quote requests according to the following schema:
## Request Schema:[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#request-schema "Direct link to Request Schema:")
```
method:POSTcontent-type: application/jsondata:{requestId:"string uuid - a unique identifier for swapper's request",tokenInChainId:"number - the `tokenIn` chainId",tokenOutChainId:"number - the `tokenOut` chainId",swapper:"string address - The swapper’s EOA address that will sign the order",tokenIn:"string address - The ERC20 token that the swapper will provide",tokenOut:"string address - The ERC20 token that the swapper will receive",amount:"string number - If the trade type is exact input then this is amount of `tokenIn` the user wants to swap otherwise this is amount of tokenOut the user wants to receive",type:"number - This is either `EXACT_INPUT` or `EXACT_OUTPUT`",quoteId:"string uuid - a unique identifier for the quote an integrator is sending back"}
```

## Response Schema:[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#response-schema "Direct link to Response Schema:")
If you can fulfill the quote request, your server should respond with (status 200 - OK) and the following data:
```
{chainId:"number - the chainId for the quoted token",amountIn:"string number - If the request type is exact input then this field is `amount` from the quote request, otherwise this is the provided quote",amountOut:"string number - If the request type is exact output then this field is `amount` from the quote request, otherwise this is the provided quote",filler:"string address - The executor address that you would like to have last-look exclusivity for this order"{...The following fields should be echoed from the quote request...},requestId:"string uuid - a unique identifier for this quote request",swapper:"string address - The swapper’s EOA address that will sign the order",tokenIn:"string address - The ERC20 token that the swapper will provide",tokenOut:"string address - The ERC20 token that the swapper will receive",quoteId:"string uuid - a unique identifier for the quote an integrator is sending back"}
```

If you do not wish to respond to a quote request, you must return an empty response with status code `204`.
## Schema When Disabled Due to Circuit Breaker[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#schema-when-disabled-due-to-circuit-breaker "Direct link to Schema When Disabled Due to Circuit Breaker")
If you are an onboarded quoter who is currently disabled by the circuit breaker, your server will receive the following message on the same quote endpoint:
```
method:POSTcontent-type: application/jsondata:{blockUntilTimestamp:"timestamp - the timestamp that this quoter is disabled until"}
```

# Moving to Production
All new quoter instances will start by being onboarded to our [Beta environment](https://beta.api.uniswap.org/v2/uniswapx/docs). Here, they will need to demonstrate at least **5 valid Exclusive RFQ fills** to be moved to production. The Beta environment serves valid mainnet orders that should be filled against production contracts but does not receive traffic from production interfaces.
## Steps to Move to Production[​](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#steps-to-move-to-production "Direct link to Steps to Move to Production")
  1. **Provide their quote server URL** to your Uniswap Labs contact along with the contract address you’re using to fill. We recommend that this be the same quoting infrastructure that you plan to run in production.
  2. **(Optional) Provide notification webhook URL** to you Uniswap Labs contact. We’ll set up notifications of won orders to be served there. Alternatively, you can poll the [Beta /orders Endpoint](https://beta.api.uniswap.org/v2/uniswapx/docs) for won orders.
  3. **Begin sending quotes and orders to beta** via the [UniswapX CLI](https://github.com/Uniswap/uniswapx-tool?tab=readme-ov-file#simple-order-creation). Reach out to the Uniswap team to be added to the private github.
  4. **Send hashes of 5 filled transactions** that demonstrate that the integration was able to fill during the exclusive period; specifically before [decayStartTime](https://github.com/Uniswap/UniswapX/blob/abd7a0b080148fc42ef7c86536d14de714eec4c7/src/lib/ExclusiveDutchOrderLib.sol#L12)


The Uniswap Labs team will review the 5 transactions to confirm they were successful exclusive fills. Once they are confirmed, the quoters setup will be promoted to production and will start receiving traffic.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/guides/mainnet/becomeQuoter.md)
Was this helpful?
[PreviousFilling on Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)[NextFilling on Arbitrum](https://docs.uniswap.org/contracts/uniswapx/guides/arbitrum/arbitrumfiller)
On this page
  * [Performance Expectations](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#performance-expectations)
  * [Handling Fades & the Circuit Breaker](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#handling-fades--the-circuit-breaker)
  * [Request Schema:](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#request-schema)
  * [Response Schema:](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#response-schema)
  * [Schema When Disabled Due to Circuit Breaker](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#schema-when-disabled-due-to-circuit-breaker)
  * [Steps to Move to Production](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/becomequoter#steps-to-move-to-production)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/guides/mainnet/becomeQuoter.md)
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
