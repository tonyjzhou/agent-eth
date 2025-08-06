# https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3

[Skip to main content](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
        * [v4 vs v3](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/concepts/flash-accounting)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks)
        * [Subscribers](https://docs.uniswap.org/contracts/v4/concepts/subscribers)
        * [PoolManager](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
        * [Dynamic Fees](https://docs.uniswap.org/contracts/v4/concepts/dynamic-fees)
        * [Integrated Routing with UniswapX](https://docs.uniswap.org/contracts/v4/concepts/integrated-routing-uniswap-x)
        * [v4 Fee Structure Guide](https://docs.uniswap.org/contracts/v4/concepts/fees)
        * [Security](https://docs.uniswap.org/contracts/v4/concepts/security)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
    * [Permit2](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Concepts
  * [v4 vs v3](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)


On this page
While Uniswap v4's underlying concentrated liquidity is the same as Uniswap v3, there are some key differences in the architecture and accounting.
# Singleton Design
### Pool Creation[​](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3#pool-creation "Direct link to Pool Creation")
**v4** : The singleton contract facilitates the creation of a pool and also stores its state. This pattern reduces costs when creating a pool and doing multi-hop swaps. Because pools are _contract state_ and not entirely new _contracts_ themselves, pool creation is significantly cheaper.
**v3** : A factory contract is responsible for pool creation. The pool is a separate contract instance that manages its own state. Pool initialization is costly because contract creation is gas-intensive
### Flash Accounting[​](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3#flash-accounting "Direct link to Flash Accounting")
**v4** : The singleton uses _flash accounting_ , meaning a caller that unlocks the PoolManager is allowed to cause balance-changing operations (multiple swaps, multiple liquidity modifications, etc) and only needs to perform token transfers at the very end of the sequence.
**v3** : Because flash accounting is missing from v3, it is the responsibility of the integrating contract to perform token transfers, after each individual call, to each individual pool contract
# Liquidity Fee Accounting
**v4** : Accrued fees act like a credit when modifying liquidity. Increasing liquidity will convert the fee revenue to liquidity inside the position while decreasing liquidity will automatically require the withdrawal of unclaimed fee revenue.
An additional parameter _salt_ can be provided when creating liquidity. The _salt_ is used to distinguish positions of the same range on the same pool. This separation may be preferred to simplify fee accounting. If two users share the same range and state in `PoolManager`, integrating contracts must be careful in managing fees
**v3** : Liquidity positions of the same range and pool will share the same state. While believed to be more gas efficient at the time, integrating contracts will need to handle fee management since the state is shared on the core pool contract
# Native ETH
**v4** : Pool pairs support native tokens, in doing so ETH swappers and liquidity providers benefit from gas cost reductions from cheaper transfers and removal of additional wrapping costs.
**v3** : ETH needs to be wrapped first before being paired with other tokens. This results in higher gas costs because of wrapping and transferring a wrapped native token.
# Subscribers
Only v4: Owners can now set a subscriber for their positions. A subscriber contract will get notified every time the position's liquidity or owner changes. Subscribers enable staking / liquidity-mining, but users do not need to transfer their ERC-721 token.
**v3** : Staking in v3 requires users to transfer their ERC-721 token to a contract, putting the underlying assets at risk for malicious behavior.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/01-v4-vs-v3.mdx)
Was this helpful?
[PreviousDeployments](https://docs.uniswap.org/contracts/v4/deployments)[NextFlash Accounting](https://docs.uniswap.org/contracts/v4/concepts/flash-accounting)
On this page
  * [Pool Creation](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3#pool-creation)
  * [Flash Accounting](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3#flash-accounting)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/01-v4-vs-v3.mdx)
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
