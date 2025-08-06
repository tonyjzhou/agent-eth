# https://docs.uniswap.org/contracts/v4/concepts/erc6909

[Skip to main content](https://docs.uniswap.org/contracts/v4/concepts/erc6909#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
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
      * [Quickstart](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
    * [Permit2](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/concepts/erc6909)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Concepts
  * [ERC-6909](https://docs.uniswap.org/contracts/v4/concepts/erc6909)


On this page
# ERC-6909
Uniswap v4 uses [ERC-6909](https://eips.ethereum.org/EIPS/eip-6909) to further improve gas-efficiency on token claims and redemptions.
ERC-6909 is a minimal and gas-efficient standard for managing multiple ERC-20 tokens from a single contract. It provides a simplified alternative to the more complex ERC-1155 multi-token standard.
### ERC-6909 vs ERC-1155[​](https://docs.uniswap.org/contracts/v4/concepts/erc6909#erc-6909-vs-erc-1155 "Direct link to ERC-6909 vs ERC-1155")
ERC-6909 offers several advantages over ERC-1155:
  1. Simplified interface: ERC-6909 removes unnecessary safe transfer callbacks and batching constraints presented in ERC-1155.
  2. Improved transfer delegation: ERC-6909 provides a more efficient system for transfer delegation.
  3. Gas efficiency: ERC-6909 reduces gas costs for deployment, transfers, and burning operations.
  4. Reduced code size: Implementing ERC-6909 results in smaller contract sizes compared to ERC-1155.


However, it's worth noting that ERC-6909 does introduce a `totalSupply` variable, which leads to an additional disk write on mint and burn operations.
# How it works
Instead of choosing to move tokens in/out of the `PoolManager`, developers can opt-in and leave the ERC-20 tokens within the `PoolManager`. In exchange, the `PoolManager` can **mint them an ERC-6909 token representing their claim**. In subsequent interactions requiring _paying_ tokens, users will not need to transfer ERC-20 tokens into the `PoolManager` - users can simply _burn_ some (or all) of their claim tokens they have
Doing _real_ ERC-20 token transfers requires calls to external smart contracts - incurring gas overhead compared to internal accounting. Secondly, these external smart contracts have their own custom logic within their `transfer` functions - for example USDC's blocked-address list - which is a further gas overhead. Thus, minting and burning ERC-6909 tokens are more gas-efficient because they don't require external function calls and have a constant-size gas overhead regardless of the underlying ERC-20 token.
This mechanism therefore helps further reduce gas costs. All these gas cost reductions overall make pools much more competitive based on the fees they charge.
# Examples
## High-frequency traders / MEV bots[​](https://docs.uniswap.org/contracts/v4/concepts/erc6909#high-frequency-traders--mev-bots "Direct link to High-frequency traders / MEV bots")
These users are often conducting a lot of swaps in relatively short durations of time, while staying within the Uniswap Protocol. These power-users can trade using ERC-6909 tokens for improved gas-efficiency.
## Liquidity management[​](https://docs.uniswap.org/contracts/v4/concepts/erc6909#liquidity-management "Direct link to Liquidity management")
ERC-6909 does not only benefit swappers. For power-users that may be opening and closing liquidity positions frequently, liquidity managers can opt-in and receive their capital as ERC-6909.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/03-erc6909.mdx)
Was this helpful?
[PreviousFlash Accounting](https://docs.uniswap.org/contracts/v4/concepts/flash-accounting)[NextHooks](https://docs.uniswap.org/contracts/v4/concepts/hooks)
On this page
  * [ERC-6909 vs ERC-1155](https://docs.uniswap.org/contracts/v4/concepts/erc6909#erc-6909-vs-erc-1155)
  * [High-frequency traders / MEV bots](https://docs.uniswap.org/contracts/v4/concepts/erc6909#high-frequency-traders--mev-bots)
  * [Liquidity management](https://docs.uniswap.org/contracts/v4/concepts/erc6909#liquidity-management)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/03-erc6909.mdx)
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
