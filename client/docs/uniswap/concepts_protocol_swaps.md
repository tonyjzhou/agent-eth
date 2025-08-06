# https://docs.uniswap.org/concepts/protocol/swaps

[Skip to main content](https://docs.uniswap.org/concepts/protocol/swaps#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/protocol/swaps)
      * [Hooks](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Swaps](https://docs.uniswap.org/concepts/protocol/swaps)
      * [Fees](https://docs.uniswap.org/concepts/protocol/fees)
      * [Oracle](https://docs.uniswap.org/concepts/protocol/oracle)
      * [Range Orders](https://docs.uniswap.org/concepts/protocol/range-orders)
      * [Concentrated Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)
      * [Token Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)
    * [Governance](https://docs.uniswap.org/concepts/protocol/swaps)
    * [Research](https://docs.uniswap.org/concepts/research)
    * [Resources](https://docs.uniswap.org/concepts/resources)
    * [Glossary](https://docs.uniswap.org/concepts/glossary)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * Protocol Concepts
  * [Swaps](https://docs.uniswap.org/concepts/protocol/swaps)


On this page
# Swaps
## Introduction[​](https://docs.uniswap.org/concepts/protocol/swaps#introduction "Direct link to Introduction")
Swaps are the most common way of interacting with the Uniswap protocol. For end-users, swapping is straightforward: a user selects an ERC-20 token that they own and a token they would like to trade it for. Executing a swap sells the currently owned tokens for the proportional[1](https://docs.uniswap.org/concepts/protocol/swaps#user-content-fn-1) amount of the tokens desired, minus the swap fee, which is awarded to liquidity providers[2](https://docs.uniswap.org/concepts/protocol/swaps#user-content-fn-2). Swapping with the Uniswap protocol is a permissionless process.
> note: Using web interfaces (websites) to swap via the Uniswap protocol can introduce additional permission structures, and may result in different execution behavior compared to using the Uniswap protocol directly. To learn more about the differences between the protocol and a web interface, see What is Uniswap.
Swaps using the Uniswap protocol are different from traditional order book trades in that they are not executed against discrete orders on a first-in-first-out basis — rather, swaps execute against a passive pool of liquidity, with liquidity providers earning fees proportional to their capital committed
## Price Impact[​](https://docs.uniswap.org/concepts/protocol/swaps#price-impact "Direct link to Price Impact")
In a traditional order-book market, a sizeable market-buy order may deplete the available liquidity of a prior limit-sell and continue to execute against a subsequent limit-sell order at a higher price. The result is the final execution price of the order is somewhere in between the two limit-sell prices against which the order was filled.
Price impact affects the execution price of a swap similarly but is a result of a different dynamic. When using an automated market maker, the relative value of one asset in terms of the other continuously shifts during the execution of a swap, leaving the final execution price somewhere between where the relative price started - and ended.
This dynamic affects every swap using the Uniswap protocol, as it is an inextricable part of AMM design.
As the amount of liquidity available at different price points can vary, the price impact for a given swap size will change relative to the amount of liquidity available at any given point in price space. The greater the liquidity available at a given price, the lower the price impact for a given swap size. The lesser the liquidity available, the higher the price impact.
Approximate[3](https://docs.uniswap.org/concepts/protocol/swaps#user-content-fn-3) price impact is anticipated in real-time via the Uniswap interface, and warnings appear if unusually high price impact will occur during a swap. Anyone executing a swap will have the ability to assess the circumstances of price impact when needed.
## Slippage[​](https://docs.uniswap.org/concepts/protocol/swaps#slippage "Direct link to Slippage")
The other relevant detail to consider when approaching swaps with the Uniswap protocol is slippage. Slippage is the term we use to describe alterations to a given price that could occur while a submitted transaction is pending.
When transactions are submitted to Ethereum, their order of execution is established by the amount of "gas" offered as a fee for executing each transaction. The higher the fee offered, the faster the transaction is executed. The transactions with a lower gas fee will remain pending for an indeterminate amount of time. During this time, the price environment in which the transaction will eventually be executed will change, as other swaps will be taking place.
Slippage tolerances establish a margin of change acceptable to the user beyond price impact. As long as the execution price is within the slippage range, e.g., %1, the transaction will be executed. If the execution price ends up outside of the accepted slippage range, the transaction will fail, and the swap will not occur.
A comparable situation in a traditional market would be a market-buy order executed after a delay. One can know the expected price of a market-buy order when submitted, but much can change in the time between submission and execution.
## Safety Checks[​](https://docs.uniswap.org/concepts/protocol/swaps#safety-checks "Direct link to Safety Checks")
Price impact and slippage can both change while a transaction is pending, which is why we have built numerous safety checks into the Uniswap protocol to protect end-users from drastic changes in the execution environment of their swap. Some of the most commonly encountered safety checks:
  * **Expired** : A transaction error that occurs if a swap is pending longer than a predetermined deadline. The deadline is a point in time after which the swap will be canceled to protect against unusually long pending periods and the changes in price that typically accompany the passage of time.
  * **INSUFFICIENT_OUTPUT_AMOUNT** : When a user submits a swap, the Uniswap interface will send an estimate of how much of the purchased token the user should expect to receive. If the anticipated output amount of a swap does not match the estimate within a certain margin of error (the slippage tolerance), the swap will be canceled. This attempts to protect the user from any drastic and unfavorable price changes while their transaction is pending.


## Footnotes[​](https://docs.uniswap.org/concepts/protocol/swaps#footnote-label "Direct link to Footnotes")
  1. Proportional in this instance takes into account many factors, including the relative price of one token in terms of the other, slippage, price impact, and other factors related to the open and adversarial nature of Ethereum. [↩](https://docs.uniswap.org/concepts/protocol/swaps#user-content-fnref-1)
  2. For information about liquidity provision, see the liquidity user guide [↩](https://docs.uniswap.org/concepts/protocol/swaps#user-content-fnref-2)
  3. The Uniswap interface informs the user about the circumstances of their swap, but it is not guaranteed. [↩](https://docs.uniswap.org/concepts/protocol/swaps#user-content-fnref-3)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/swaps.md)
Was this helpful?
[PreviousHooks](https://docs.uniswap.org/concepts/protocol/hooks)[NextFees](https://docs.uniswap.org/concepts/protocol/fees)
On this page
  * [Introduction](https://docs.uniswap.org/concepts/protocol/swaps#introduction)
  * [Price Impact](https://docs.uniswap.org/concepts/protocol/swaps#price-impact)
  * [Slippage](https://docs.uniswap.org/concepts/protocol/swaps#slippage)
  * [Safety Checks](https://docs.uniswap.org/concepts/protocol/swaps#safety-checks)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/swaps.md)
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
