# https://docs.uniswap.org/contracts/v4/overview

[Skip to main content](https://docs.uniswap.org/contracts/v4/overview#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
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


On this page
# Uniswap v4
Uniswap v4 inherits all of the capital efficiency gains of Uniswap v3, but provides flexibility via _hooks_ and gas optimizations across the entire lifecycle.
For additional information, see the [Uniswap v4 whitepaper](https://app.uniswap.org/whitepaper-v4.pdf)
## Hooks[​](https://docs.uniswap.org/contracts/v4/overview#hooks "Direct link to Hooks")
Developers can attach solidity logic to the _swap lifecycle_ through Hooks. The logic is executed before and/or after major operations such as pool creation, liquidity addition and removal, swapping, and donations. Hooks are deployed contracts, and are called by the Uniswap v4 PoolManager, for permissionless execution.
The flexibility of hooks can enable:
  * Limit orders
  * Custom oracles
  * Fee management
  * Automated liquidity management


## Dynamic Fees[​](https://docs.uniswap.org/contracts/v4/overview#dynamic-fees "Direct link to Dynamic Fees")
Uniswap v4 supports dynamic fees, allowing pools to adjust their fees up or down. While other AMMs may have hard-coded logic for dynamic fees, v4 provides no opinionated calculation of the fee. The frequency of _liquidity fee_ updates is also flexible and determined by the developer. Fee updates can occur on every swap, every block, or on an arbitrary schedule (weekly, monthly, yearly, etc).
Dynamic fees open up the design space for fee optimization, value redistribution, and research.
## Singleton Design[​](https://docs.uniswap.org/contracts/v4/overview#singleton-design "Direct link to Singleton Design")
Architecturally, all pool state and operations are managed by a single contract -- `PoolManager.sol`. The singleton design provides major gas savings. For example, creating a pool is now a state update instead of the deployment of a new contract. Swapping through multiple pools no longer requires transferring tokens for intermediate pools.
## Flash Accounting[​](https://docs.uniswap.org/contracts/v4/overview#flash-accounting "Direct link to Flash Accounting")
By leveraging EIP-1153 Transient Storage, v4 provides an optimization referred to as _flash accounting_. Swapping, liquidity modification, and donations incur _balance changes_ , i.e. tokens to be sent in and tokens to be taken out. With _flash accounting_ these balance changes are efficiently recorded in transient storage and netted against each other. This system allows users to only pay the final balance change, without the need for resolving intermediate balance changes.
## Native ETH[​](https://docs.uniswap.org/contracts/v4/overview#native-eth "Direct link to Native ETH")
Uniswap v4 supports native token assets (Ether), without the need to wrap/unwrap the native token to Wrapped Ether (WETH9).
## Custom Accounting[​](https://docs.uniswap.org/contracts/v4/overview#custom-accounting "Direct link to Custom Accounting")
The flexibility of custom accounting allows developers to alter token amounts for swaps and liquidity modifications. The feature opens up the design space for hooks to charge fees or forgo the underlying concentrated liquidity model.
Example use-cases:
  * Custom curves, opt-out of the concentrated liquidity curve in favor of an entirely independent pricing mechanism
  * Hook swap fees, charge and collect fees on swaps
  * Liquidity withdrawal fees, penalize and/or redistribute fee revenue


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/overview.mdx)
Was this helpful?
[PreviousGlossary](https://docs.uniswap.org/concepts/glossary)[NextOverview](https://docs.uniswap.org/contracts/v4/overview)
On this page
  * [Hooks](https://docs.uniswap.org/contracts/v4/overview#hooks)
  * [Dynamic Fees](https://docs.uniswap.org/contracts/v4/overview#dynamic-fees)
  * [Singleton Design](https://docs.uniswap.org/contracts/v4/overview#singleton-design)
  * [Flash Accounting](https://docs.uniswap.org/contracts/v4/overview#flash-accounting)
  * [Native ETH](https://docs.uniswap.org/contracts/v4/overview#native-eth)
  * [Custom Accounting](https://docs.uniswap.org/contracts/v4/overview#custom-accounting)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/overview.mdx)
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
