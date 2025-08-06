# https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
    * [Permit2](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
          * [Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
          * [Pools](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
          * [Oracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
      * [Guides](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [API](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Governance](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Concepts
  * Core Concepts
  * [Pools](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/pools)


![](https://docs.uniswap.org/assets/images/anatomy-d22fb7ab46013a1195f086ee672468c7.jpg)
# Introduction
Each Uniswap liquidity pool is a trading venue for a pair of ERC20 tokens. When a pool contract is created, its balances of each token are 0; in order for the pool to begin facilitating trades, someone must seed it with an initial deposit of each token. This first liquidity provider is the one who sets the initial price of the pool. They are incentivized to deposit an equal _value_ of both tokens into the pool. To see why, consider the case where the first liquidity provider deposits tokens at a ratio different from the current market rate. This immediately creates a profitable arbitrage opportunity, which is likely to be taken by an external party.
When other liquidity providers add to an existing pool, they must deposit pair tokens proportional to the current price. If they don’t, the liquidity they added is at risk of being arbitraged as well. If they believe the current price is not correct, they may arbitrage it to the level they desire, and add liquidity at that price.
# Pool tokens
![](https://docs.uniswap.org/assets/images/lp-c0b1b03ef921f1325971fa8ab6e9a4f1.jpg)
Whenever liquidity is deposited into a pool, unique tokens known as _liquidity tokens_ are minted and sent to the provider's address. These tokens represent a given liquidity provider's contribution to a pool. The proportion of the pool's liquidity provided determines the number of liquidity tokens the provider receives. If the provider is minting a new pool, the number of liquidity tokens they will receive will equal sqrt(x * y), where x and y represent the amount of each token provided.
Whenever a trade occurs, a 0.3% fee is charged to the transaction sender. This fee is distributed _pro-rata_ to all LPs in the pool upon completion of the trade.
To retrieve the underlying liquidity, plus any fees accrued, liquidity providers must "burn" their liquidity tokens, effectively exchanging them for their portion of the liquidity pool, plus the proportional fee allocation.
As liquidity tokens are themselves tradable assets, liquidity providers may sell, transfer, or otherwise use their liquidity tokens in any way they see fit.
> Learn more with advanced topics:
  * [Understanding Returns](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/understanding-returns)
  * [Fees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)


# Why pools?
Uniswap is unique in that it doesn’t use an order book to derive the price of an asset or to match buyers and sellers of tokens. Instead, Uniswap uses what are called Liquidity Pools.
Liquidity is typically represented by discrete orders placed by individuals onto a centrally operated order book. A participant looking to provide liquidity or make markets must actively manage their orders, continuously updating them in response to the activity of others in the marketplace.
While order books are foundational to finance and work great for certain usecases, they suffer from a few important limitations that are especially magnified when applied to a decentralized or blockchain-native setting. Order books require intermediary infrastructure to host the orderbook and match orders. This creates points of control and adds additional layers of complexity. They also require active participation and management from market makers who usually use sophisticated infrastructure and algorithms, limiting participation to advanced traders. Order books were invented in a world with relatively few assets being traded, so it is not surprising they aren't ideal for an ecosystem where anyone can create their own token, and those tokens usually have low liquidity. In sum, with the infrastructural trade-offs presented by a platform like Ethereum, order books are not the native architecture for implementing a liquidity protocol on a blockchain.
Uniswap focuses on the strengths of Ethereum to reimagine token swaps from first principles.
A blockchain-native liquidity protocol should take advantage of the trusted code execution environment, the autonomous and perpetually running virtual machine, and an open, permissionless, and inclusive access model that produces an exponentially growing ecosystem of virtual assets.
It is important to reiterate that a Pool is just a smart contract, operated by users calling functions on it. Swapping tokens is calling `swap` on a Pool contract instance, while providing liquidity is calling `deposit`.
Just how end-users can interact with the Uniswap protocol through the Interface (which in turn interacts with the underlying contracts), developers can interact directly with the smart contracts and integrate Uniswap functionality into their own applications without relying on intermediaries or needing permission.
# Developer resources
  * To see how to pool tokens in a smart contract read [Providing Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity).


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/02-core-concepts/02-pools.md)
Was this helpful?
[PreviousSwaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)[NextFlash Swaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/flash-swaps)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/02-core-concepts/02-pools.md)
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
