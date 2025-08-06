# https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
    * [Permit2](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
          * [Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
          * [Pools](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
          * [Oracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [API](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Governance](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Concepts
  * Core Concepts
  * [Flash Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)


On this page
Uniswap flash swaps allow you to withdraw up to the full reserves of any ERC20 token on Uniswap and execute arbitrary logic at no upfront cost, provided that by the end of the transaction you either:
  * pay for the withdrawn ERC20 tokens with the corresponding pair tokens
  * return the withdrawn ERC20 tokens along with a small fee


Flash swaps are incredibly useful because they obviate upfront capital requirements and unnecessary order-of-operations constraints for multi-step transactions involving Uniswap.
# Examples
## Capital Free Arbitrage[​](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#capital-free-arbitrage "Direct link to Capital Free Arbitrage")
One particularly interesting use case for flash swaps is capital-free arbitrage. It's well-known that an integral part of Uniswap's design is to create incentives for arbitrageurs to trade the Uniswap price to a "fair" market price. While game-theoretically sound, this strategy is accessible only to those with sufficient capital to take advantage of arbitrage opportunities. Flash swaps remove this barrier entirely, effectively democratizing arbitrage.
Imagine a scenario where the cost of buying 1 ETH on Uniswap is 200 DAI (which is calculated by calling `getAmountIn` with 1 ETH specified as an exact output), and on Oasis (or any other trading venue), 1 ETH buys 220 DAI. To anyone with 200 DAI available, this situation represents a risk-free profit of 20 DAI. Unfortunately, you may not have 200 DAI lying around. With flash swaps, however, this risk-free profit is available for anyone to take as long as they're able to pay gas fees.
### Withdrawing ETH from Uniswap[​](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#withdrawing-eth-from-uniswap "Direct link to Withdrawing ETH from Uniswap")
The first step is to _optimistically_ withdraw 1 ETH from Uniswap via a flash swap. This will serve as the capital that we use to execute our arbitrage. Note that in this scenario, we're assuming that:
  * 1 ETH is the pre-calculated profit-maximizing trade
  * The price has not changed on Uniswap or Oasis since our calculation


It may be the case that we'd like to calculate the profit-maximizing trade on-chain at the moment of execution, which is robust to price movements. This can be somewhat complex, depending on the strategy being executed. However, one common strategy is trading as profitably as possible _against a fixed external price_. (This price may be e.g., the average execution price of one or more orders on Oasis.) If the Uniswap market price is far enough above or below this external price, the following example contains code that calculates the amount to trade over Uniswap for maximum profit: [`ExampleSwapToPrice.sol`](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/examples/ExampleSwapToPrice.sol).
### Trade at External Venue[​](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#trade-at-external-venue "Direct link to Trade at External Venue")
Once we've obtained our temporary capital of 1 ETH from Uniswap, we now can trade this for 220 DAI on Oasis. Once we've received the DAI, we need to pay Uniswap back. We've mentioned that the amount required to cover 1 ETH is 200 DAI, calculated via `getAmountIn`. So, after sending 200 of the DAI back to the Uniswap pair, you're left with 20 DAI of profit!
## Instant Leverage[​](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#instant-leverage "Direct link to Instant Leverage")
Flash swaps can be used to improve the efficiency of levering up using lending protocols and Uniswap.
Consider Maker in its simplest form: a system which accepts ETH as collateral and allows DAI to be minted against it while ensuring that the value of the ETH never drops below 150% of the value of the DAI.
Say we use this system to deposit a principal amount of 3 ETH, and mint the maximum amount of DAI. At a price of 1 ETH / 200 DAI, we receive 400 DAI. In theory, we could lever this position up by selling the DAI for more ETH, depositing this ETH, minting the maximum amount of DAI (which would be less this time), and repeating until we've reached our desired leverage level.
It's quite simple to use Uniswap as a liquidity source for the DAI-to-ETH component of this process. However, looping through protocols in this way isn't particularly elegant, and can be gas-intensive.
Luckily, flash swaps enable us to withdraw the _full_ ETH amount upfront. If we wanted 2x leverage against our 3 ETH principal, we could simply request 3 ETH in a flash swap and deposit 6 ETH into Maker. This gives us the ability to mint 800 DAI. If we mint as much as we need to cover our flash swap (say 605), the remainder serves as a safety margin against price movements.
# Developer resources
  * To see how to integrate a flash swap in your smart contract read [Using Flash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps).


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/02-core-concepts/03-flash-swaps.md)
Was this helpful?
[PreviousPools](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)[NextOracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles)
On this page
  * [Capital Free Arbitrage](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#capital-free-arbitrage)
    * [Withdrawing ETH from Uniswap](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#withdrawing-eth-from-uniswap)
    * [Trade at External Venue](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#trade-at-external-venue)
  * [Instant Leverage](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps#instant-leverage)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/02-core-concepts/03-flash-swaps.md)
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
