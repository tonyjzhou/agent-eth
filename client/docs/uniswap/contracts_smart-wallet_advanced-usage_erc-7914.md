# https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7914

[Skip to main content](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7914#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/contracts/smart-wallet/overview)
      * [Deployments](https://docs.uniswap.org/contracts/smart-wallet/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
      * [Advanced Usage](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/alternative-signers)
        * [Alternative Signers](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/alternative-signers)
        * [Hooks](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
        * [ERC-7739](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7739)
        * [ERC-7914](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7914)
      * [Technical Reference](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Smart Wallet
  * Advanced Usage
  * [ERC-7914](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7914)


# ERC-7914
ERC-7914 is a standard which adds native Ether “transfer from” functionality to smart contract wallets. It allows for our Smart Wallet to approve other accounts and contracts to spend ETH, which is not possible today with Externally Owned Accounts (EOAs).
We are contributors to the [ERC](https://github.com/ethereum/ERCs/blob/8380220418521ff1995445cff5ca1d0e496a3d2d/ERCS/erc-7914.md) and welcome any feedback from the community on the spec or our implementation.
Calibur’s reference implementation is [here](https://github.com/Uniswap/calibur/blob/main/src/ERC7914.sol).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/advanced-usage/04-erc-7914.md)
Was this helpful?
[PreviousERC-7739](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7739)[NextTechnical Reference](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/advanced-usage/04-erc-7914.md)
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
