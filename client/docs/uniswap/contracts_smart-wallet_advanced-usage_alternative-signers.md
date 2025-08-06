# https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/alternative-signers

[Skip to main content](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/alternative-signers#__docusaurus_skipToContent_fallback)
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
  * [Alternative Signers](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/alternative-signers)


# Alternative Signers
Calibur allows for a user to add any number of signers to their account. Known as `keys`, the following types are supported: `Secp256k1`, `Secp256r1 (P256)`, and `WebAuthn P256`.
This is an **advanced** feature. Please be aware of the following:
  * By default, added keys do not expire and have no hooks.
  * A malicious key could steal all of your ETH and tokens


Registering external signers on your account changes the security model. Proceed with caution!
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/advanced-usage/01-alternative-signer.md)
Was this helpful?
[PreviousGas Abstraction](https://docs.uniswap.org/contracts/smart-wallet/concepts/gas-abstraction)[NextHooks](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/advanced-usage/01-alternative-signer.md)
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
