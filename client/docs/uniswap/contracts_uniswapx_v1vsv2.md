# https://docs.uniswap.org/contracts/uniswapx/v1vsv2

[Skip to main content](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Quickstart](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Technical Reference](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
    * [v3 Protocol](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
    * [Smart Wallet](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Overview](https://docs.uniswap.org/contracts/uniswapx/overview)
      * [UniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
      * [Guides](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
        * [Mainnet](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
        * [Arbitrum](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
        * [Unichain, Base](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
        * [Webhook Support](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)
    * [Universal Router](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
    * [Permit2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
    * [v2 Protocol](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
    * [v1 Protocol](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * UniswapX
  * [UniswapX V2](https://docs.uniswap.org/contracts/uniswapx/v1vsv2)


On this page
# **UniswapX V2 Overview**
UniswapX V2 improves price execution and minimizes quoter gaming by redesigning the flow of order quote information. The primary goal is to make trading on UniswapX more efficient and more reliable.
## **V1 Overview**[​](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#v1-overview "Direct link to v1-overview")
In V1, the RFQ flow operates entirely pre-signature. Users request a swap quote, which is parameterized using an RFQ quote. If they accept the quote, they sign the order. This approach is straightforward but causes quoters in the network to be uncertain about when a quote request will convert into a signed order. This uncertainty leads to occasional unfilled orders and degraded performance.
![image](https://docs.uniswap.org/assets/images/v1-flow-c19ebe2f784563f62c47cab02cd10cfe.png)
## **V2 Overview**[​](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#v2-overview "Direct link to v2-overview")
V2 rearchitects the flow such that the time between a quoter submitting their quote and being required to fill the order is minimized. This enforces a near 100% fill rate and causes most orders to fill almost instantly.
To achieve this, the quoting flow is divided into two phases:
  1. **Indicative Quotes** : Provided pre-signature when the swapper is exploring quotes but has not committed to trading. Quoters can participate here without being held fully accountable.
  2. **Hard Quotes** : Provided post-signature when the swapper has committed to trading. Quoters are held fully accountable for these quotes.


Since hard quotes are provided post-signature, they are instantly executable by fillers removing the risk of market movements causing the order to fail.
To prevent gaming, quoters cannot distinguish between indicative and hard quotes. As a result, they must always assume all quotes are hard and provide competitive prices.
## V2 Flow[​](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#v2-flow "Direct link to V2 Flow")
![image](https://docs.uniswap.org/assets/images/v2-flow-1c31af9e8e1c805e48a4ee0f1b51ab32.png)
  1. The user requests a quote from the interface.
  2. The Uniswap Backend (URA) runs an indicative RFQ auction, soliciting quotes for the best price. (**Note:** Quoters do not know whether the RFQ is indicative or hard, forcing them to always provide competitive prices.)
  3. The quotes from the RFQ process parameterize an [initial quote](https://github.com/Uniswap/UniswapX/blob/33fa564cfaa6d58f6e3fcf7e7988cb5fc1c61de7/src/lib/V2DutchOrderLib.sol#L31) order shown to the user.
  4. The user signs the order and submits it for execution.
  5. The Uniswap Backend (GPA) runs a second “hard” RFQ process, soliciting new quotes.
  6. Quoters return their best prices again. 
     * If the price is within or improves on the user’s signed parameters, it is finalized and added to the order’s [CosignerData](https://github.com/Uniswap/UniswapX/blob/33fa564cfaa6d58f6e3fcf7e7988cb5fc1c61de7/src/lib/V2DutchOrderLib.sol#L20).
     * If the price moves outside the signed parameters, the order fails, and the user must try again.
  7. The finalized order containing CosignerData is posted to the filler network for execution.
  8. The winning filler executes the order.


## Why is a cosigner needed?[​](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#why-is-a-cosigner-needed "Direct link to Why is a cosigner needed?")
In UniswapX V2, users commit to a range of prices when signing their order. Without safeguards, a malicious auctioneer could provide users the worst price within their range.
![image](https://docs.uniswap.org/assets/images/cosigner-38a7de719736b3c6a9eabb0f05091c7e.png)
The cosigner field allows users to designate an auctioneer they trust to run the auction fairly, ensuring the best executable price within the signed parameters. Currently, the trading API sets the cosigner to Uniswap Labs, though this could be updated in the future.
# Current Status
UniswapX V2 is currently the default version of the protocol running on Mainnet across Uniswap's interfaces.
# Smart Contracts
  * [V2DutchOrderReactor](https://github.com/Uniswap/UniswapX/blob/main/src/reactors/V2DutchOrderReactor.sol)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/02-v1-vs-v2.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/contracts/uniswapx/overview)[NextFilling on Mainnet](https://docs.uniswap.org/contracts/uniswapx/guides/mainnet/createfiller)
On this page
  * [**V1 Overview**](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#v1-overview)
  * [**V2 Overview**](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#v2-overview)
  * [V2 Flow](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#v2-flow)
  * [Why is a cosigner needed?](https://docs.uniswap.org/contracts/uniswapx/v1vsv2#why-is-a-cosigner-needed)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/uniswapx/02-v1-vs-v2.md)
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
