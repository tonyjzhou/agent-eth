# https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks

[Skip to main content](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
      * [Quickstart](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
      * [Technical Reference](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
    * [v3 Protocol](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
      * [Overview](https://docs.uniswap.org/contracts/smart-wallet/overview)
      * [Deployments](https://docs.uniswap.org/contracts/smart-wallet/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
      * [Advanced Usage](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
        * [Alternative Signers](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/alternative-signers)
        * [Hooks](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
        * [ERC-7739](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7739)
        * [ERC-7914](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7914)
      * [Technical Reference](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [UniswapX](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
    * [Universal Router](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
    * [Permit2](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
    * [v2 Protocol](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
    * [v1 Protocol](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Smart Wallet
  * Advanced Usage
  * [Hooks](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/hooks)


# Hooks
Hooks are powerful add-ons to keys which can perform arbitrary validation on signatures, and/or actions during execution time.
There are two subtypes of hooks: `ValidationHook` and `ExecutionHook`. A hook can implement either, or both interfaces.
Validation hooks have three call sites:
  * `afterVerifySignature`
  * `afterIsValidSignature`
  * `afterValidateUserOp`


Execution hooks have two call sites:
  * `beforeExecute`
  * `afterExecute`


Hooks must revert to indicate that the given action should revert.
Example functionality which can be implemented in hooks includes:
  * Spending limits
  * Restricting keys from calling certain contracts and methods
  * Turning a key into a multisig, effectively requiring additional signatures for verification
  * Automated actions pre/post swaps


There are a few example hooks referenced in the repo. Be aware that these example hooks are not production code and may contain bugs. We do not recommend you to deploy these hooks or use them as reference implementations for productionized code. They are proof of concepts.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/advanced-usage/02-hooks.md)
Was this helpful?
[PreviousAlternative Signers](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/alternative-signers)[NextERC-7739](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7739)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/advanced-usage/02-hooks.md)
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
