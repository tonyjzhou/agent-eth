# https://docs.uniswap.org/contracts/v4/concepts/subscribers

[Skip to main content](https://docs.uniswap.org/contracts/v4/concepts/subscribers#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/overview)
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
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Concepts
  * [Subscribers](https://docs.uniswap.org/contracts/v4/concepts/subscribers)


# Subscribers
Subscribers, new in Uniswap v4, allow for liquidity-position owners to opt-in to a contract that receives _notifications_. The new design is intended to support _liquidity mining_ , additional rewards given to in-range liquidity providers. Through notification logic, position owners do not need to risk their liquidity position and its underlying assets. In Uniswap v3, _liquidity mining_ was supported by fully transferring the liquidity position to an external contract; this old design would give the external contract full ownership and control of the liquidity position.
When a position owner _subscribes_ to a contract, the contract will receive notifcations when:
  * The position is initially subscribed
  * The position increases or decreases its liquidity
  * The position is transferred
  * The position is unsubscribed


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/05-subscribers.mdx)
Was this helpful?
[PreviousHooks](https://docs.uniswap.org/contracts/v4/concepts/hooks)[NextPoolManager](https://docs.uniswap.org/contracts/v4/concepts/PoolManager)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/05-subscribers.mdx)
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
