# https://docs.uniswap.org/concepts/protocol/hooks

[Skip to main content](https://docs.uniswap.org/concepts/protocol/hooks#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Hooks](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Swaps](https://docs.uniswap.org/concepts/protocol/swaps)
      * [Fees](https://docs.uniswap.org/concepts/protocol/fees)
      * [Oracle](https://docs.uniswap.org/concepts/protocol/oracle)
      * [Range Orders](https://docs.uniswap.org/concepts/protocol/range-orders)
      * [Concentrated Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)
      * [Token Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)
    * [Governance](https://docs.uniswap.org/concepts/governance/overview)
    * [Research](https://docs.uniswap.org/concepts/research)
    * [Resources](https://docs.uniswap.org/concepts/resources)
    * [Glossary](https://docs.uniswap.org/concepts/glossary)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * Protocol Concepts
  * [Hooks](https://docs.uniswap.org/concepts/protocol/hooks)


On this page
# Hooks
## Introduction[​](https://docs.uniswap.org/concepts/protocol/hooks#introduction "Direct link to Introduction")
Uniswap v4 inherits all of the capital efficiency gains of Uniswap v3 while introducing major architectural improvements.
The key innovations are the Hook System and Singleton Architecture, which together enable unprecedented protocol customization and gas optimization.
## Hooks[​](https://docs.uniswap.org/concepts/protocol/hooks#hooks "Direct link to Hooks")
Hooks allow developers to customize and extend the behavior of liquidity pools. They are external smart contracts that can be attached to individual pools to intercept and modify the execution flow at specific points during pool-related actions.
The logic is executed before and/or after major operations such as pool creation, liquidity addition and removal, swapping, and donations.
Through these hook functions, developers can build sophisticated features like custom AMMs with different pricing curves, yield farming protocols, advanced trading features including limit orders, dynamic fee strategies, and custom oracle implementations. Each pool can have one hook (though a hook can serve multiple pools), hooks are optional and specified during pool creation, and developers can implement any combination of hook functions based on their needs.
## Singleton Architecture[​](https://docs.uniswap.org/concepts/protocol/hooks#singleton-architecture "Direct link to Singleton Architecture")
The hook system in v4 is built on top of a revolutionary architectural change known as the singleton design. Unlike previous versions where each pool was a separate smart contract, v4 manages all pools through a single contract called the [PoolManager](https://docs.uniswap.org/contracts/v4/concepts/PoolManager). This architectural innovation brings several key improvements:
  * **Efficient Pool Creation** : Pools are created as state updates rather than contract deployments, significantly reducing gas costs
  * **Gas Optimization** : Multi-hop swaps and complex operations are streamlined through a single contract
  * **Flash Accounting** : Token balances are tracked internally and settled at the end of transactions, minimizing transfers
  * **Native ETH Support** : Direct ETH trading without the need to wrap to WETH, improving user experience


These core features are just the beginning of what's possible with Uniswap v4.
To explore all features including flash accounting, native ETH support, dynamic fees, and custom accounting, check out the [v4 whitepaper](https://uniswap.org/whitepaper-v4.pdf).
For technical implementations and detailed guides, visit the [v4 technical documentation](https://docs.uniswap.org/contracts/v4/overview).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/hooks.md)
Was this helpful?
[PreviousThe Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)[NextSwaps](https://docs.uniswap.org/concepts/protocol/swaps)
On this page
  * [Introduction](https://docs.uniswap.org/concepts/protocol/hooks#introduction)
  * [Hooks](https://docs.uniswap.org/concepts/protocol/hooks#hooks)
  * [Singleton Architecture](https://docs.uniswap.org/concepts/protocol/hooks#singleton-architecture)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/hooks.md)
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
