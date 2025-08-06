# https://docs.uniswap.org/contracts/universal-router/overview

[Skip to main content](https://docs.uniswap.org/contracts/universal-router/overview#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/contracts/universal-router/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/universal-router/technical-reference)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Universal Router
  * [Overview](https://docs.uniswap.org/contracts/universal-router/overview)


On this page
# Overview
The `UniversalRouter` is an ETH and ERC20 swap router, designed to aggregate trades across Uniswap protocols (including v2, v3, and v4) and provide users with highly flexible and composable transactions. The contract is unowned and non-upgradeable.
The flexible command-based architecture enables:
  * Splitting and interleaving of Uniswap v2/v3/v4 swaps
  * Partial fills of trades
  * Wrapping and unwrapping of ETH (via WETH)
  * Time-bound, signature-controlled token approvals using [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
  * v3 and v4 position manager interactions (e.g., permit, liquidity modification, pool initialization)
  * Sub-plan execution and balance checks


Transactions are encoded as a sequence of byte-sized commands, each with structured inputs. These commands can be chained within a single transaction to express highly customized workflows, including multi-hop swaps, liquidity migration from v3 to v4, and complex value routing—all without the need for prior token approvals.
> **Note:** The `UniversalRouter` integrates with `Permit2` to eliminate the need for direct token approvals. See the [Permit2 documentation](https://docs.uniswap.org/contracts/permit2/overview) for details.
## Resources[​](https://docs.uniswap.org/contracts/universal-router/overview#resources "Direct link to Resources")
  * [UniversalRouter GitHub Repository](https://github.com/Uniswap/universal-router)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/universal-router/01-overview.md)
Was this helpful?
[PreviousWebhook Support](https://docs.uniswap.org/contracts/uniswapx/guides/webhooks)[NextTechnical Reference](https://docs.uniswap.org/contracts/universal-router/technical-reference)
On this page
  * [Resources](https://docs.uniswap.org/contracts/universal-router/overview#resources)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/universal-router/01-overview.md)
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
