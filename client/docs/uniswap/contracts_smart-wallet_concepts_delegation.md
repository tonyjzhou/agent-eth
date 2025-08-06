# https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation

[Skip to main content](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
      * [Quickstart](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
      * [Technical Reference](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
    * [v3 Protocol](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
      * [Overview](https://docs.uniswap.org/contracts/smart-wallet/overview)
      * [Deployments](https://docs.uniswap.org/contracts/smart-wallet/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
        * [Delegation](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
        * [Batched Transactions](https://docs.uniswap.org/contracts/smart-wallet/concepts/batched-transactions)
        * [Gas Abstraction](https://docs.uniswap.org/contracts/smart-wallet/concepts/gas-abstraction)
      * [Advanced Usage](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
      * [Technical Reference](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [UniswapX](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
    * [Universal Router](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
    * [Permit2](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
    * [v2 Protocol](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
    * [v1 Protocol](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Smart Wallet
  * Concepts
  * [Delegation](https://docs.uniswap.org/contracts/smart-wallet/concepts/delegation)


# Delegation
This contract is meant to be used with [EIP-7702](https://eips.ethereum.org/EIPS/eip-7702): Set Code for EOAs. After the Ethereum Pectra fork, Externally Owned Accounts (EOAs) can now internalize code at a remotely deployed smart contract address. This process is called **delegation.**
For simplicity, we will refer to these EOAs as _users_ and these remote smart contracts as _implementations._ The term _Smart Wallet_ will refer to the Uniswap smart wallet product.
Users can only be delegated to one contract at a time but can have other delegations on different chains.
After a user is delegated, they can execute transactions using the logic defined in the contract implementation, enabling advanced features like transaction batching, gas-less transactions, and custom permission controls - all while keeping their original address and on-chain history.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/concepts/01-delegation.md)
Was this helpful?
[PreviousDeployments](https://docs.uniswap.org/contracts/smart-wallet/deployments)[NextBatched Transactions](https://docs.uniswap.org/contracts/smart-wallet/concepts/batched-transactions)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/concepts/01-delegation.md)
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
