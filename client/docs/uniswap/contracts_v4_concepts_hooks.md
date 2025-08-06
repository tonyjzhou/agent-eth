# https://docs.uniswap.org/contracts/v4/concepts/hooks

[Skip to main content](https://docs.uniswap.org/contracts/v4/concepts/hooks#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/concepts/hooks)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/hooks)
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
      * [Quickstart](https://docs.uniswap.org/contracts/v4/concepts/hooks)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/concepts/hooks)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/concepts/hooks)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/concepts/hooks)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/concepts/hooks)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/concepts/hooks)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/concepts/hooks)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/concepts/hooks)
    * [Permit2](https://docs.uniswap.org/contracts/v4/concepts/hooks)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/concepts/hooks)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/concepts/hooks)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Concepts
  * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks)


On this page
# Hooks
Uniswap v4 introduces Hooks, a system that allows developers to customize and extend the behavior of liquidity pools.
Hooks are external smart contracts that can be attached to individual pools. Every pool can have one hook but a hook can serve an infinite amount of pools to intercept and modify the execution flow at specific points during pool-related actions.
## Key Concepts[​](https://docs.uniswap.org/contracts/v4/concepts/hooks#key-concepts "Direct link to Key Concepts")
### Pool-Specific Hooks[​](https://docs.uniswap.org/contracts/v4/concepts/hooks#pool-specific-hooks "Direct link to Pool-Specific Hooks")
  * Each liquidity pool in Uniswap v4 can have its own hook contract attached to it. Hooks are optional for Uniswap v4 pools.
  * The hook contract is specified when creating a new pool in the `PoolManager.initialize` function.
  * Having pool-specific hooks allows for fine-grained control and customization of individual pools.


## Core Hook Functions[​](https://docs.uniswap.org/contracts/v4/concepts/hooks#core-hook-functions "Direct link to Core Hook Functions")
Uniswap v4 provides a set of core hook functions that can be implemented by developers. Developers do not have to implement every hook, you can mix&match them to whatever your liking is. You can use one or all of them!
  * Hook contracts specify the permissions that determine which hook functions they implement, which is encoded in the address of the contract.
  * The `PoolManager` uses these permissions to determine which hook functions to call for a given pool based on its Key.


### Initialize Hooks[​](https://docs.uniswap.org/contracts/v4/concepts/hooks#initialize-hooks "Direct link to Initialize Hooks")
  * `beforeInitialize`: Called before a new pool is initialized.
  * `afterInitialize`: Called after a new pool is initialized.
  * These hooks allow developers to perform custom actions or validations during pool initialization, but these hooks can only be invoked once.


### Liquidity Modification Hooks[​](https://docs.uniswap.org/contracts/v4/concepts/hooks#liquidity-modification-hooks "Direct link to Liquidity Modification Hooks")
The liquidity modification hooks are extremely granular for security purposes.
  * `beforeAddLiquidity`: Called before liquidity is added to a pool.
  * `afterAddLiquidity`: Called after liquidity is added to a pool.
  * `beforeRemoveLiquidity`: Called before liquidity is removed from a pool.
  * `afterRemoveLiquidity`: Called after liquidity is removed from a pool.


### Swap Hooks[​](https://docs.uniswap.org/contracts/v4/concepts/hooks#swap-hooks "Direct link to Swap Hooks")
  * `beforeSwap`: Called before a swap is executed in a pool.
  * `afterSwap`: Called after a swap is executed in a pool.


### Donate Hooks[​](https://docs.uniswap.org/contracts/v4/concepts/hooks#donate-hooks "Direct link to Donate Hooks")
  * `beforeDonate`: Called before a donation is made to a pool.
  * `afterDonate`: Called after a donation is made to a pool.
  * Donate hooks provide a way to customize the behavior of token donations to liquidity providers.


## Innovation and Potential[​](https://docs.uniswap.org/contracts/v4/concepts/hooks#innovation-and-potential "Direct link to Innovation and Potential")
The introduction of hooks in Uniswap v4 opens up a world of possibilities for developers to innovate and build new DeFi protocols. Some potential use cases include:
  * Customized AMMs with different pricing curves than xy = k.
  * Yield farming and liquidity mining protocols that incentivize liquidity provision.
  * Derivative and synthetic asset platforms built on top of Uniswap v4 liquidity.
  * Lending hooks integrated with Uniswap v4 pools.


As a hook developer you can easily bootstrap the codebase of an entirely new DeFi protocol through hook designs, which subsequently drives down your audit costs and allows you to develop faster. However, it's important to note that just because you made a hook, that does not mean you will get liquidity routed to your hook from the Uniswap frontend.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/04-hooks.mdx)
Was this helpful?
[PreviousERC-6909](https://docs.uniswap.org/contracts/v4/concepts/erc6909)[NextSubscribers](https://docs.uniswap.org/contracts/v4/concepts/subscribers)
On this page
  * [Key Concepts](https://docs.uniswap.org/contracts/v4/concepts/hooks#key-concepts)
    * [Pool-Specific Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks#pool-specific-hooks)
  * [Core Hook Functions](https://docs.uniswap.org/contracts/v4/concepts/hooks#core-hook-functions)
    * [Initialize Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks#initialize-hooks)
    * [Liquidity Modification Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks#liquidity-modification-hooks)
    * [Swap Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks#swap-hooks)
    * [Donate Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks#donate-hooks)
  * [Innovation and Potential](https://docs.uniswap.org/contracts/v4/concepts/hooks#innovation-and-potential)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/04-hooks.mdx)
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
