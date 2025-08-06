# https://docs.uniswap.org/contracts/smart-wallet/technical-reference

[Skip to main content](https://docs.uniswap.org/contracts/smart-wallet/technical-reference#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
      * [Quickstart](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
      * [Technical Reference](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [v3 Protocol](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
      * [Overview](https://docs.uniswap.org/contracts/smart-wallet/overview)
      * [Deployments](https://docs.uniswap.org/contracts/smart-wallet/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
      * [Advanced Usage](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
      * [Technical Reference](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [UniswapX](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [Universal Router](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [Permit2](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [v2 Protocol](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
    * [v1 Protocol](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Smart Wallet
  * [Technical Reference](https://docs.uniswap.org/contracts/smart-wallet/technical-reference)


On this page
# Technical Reference
For more details, see the [technical reference](https://github.com/Uniswap/calibur/tree/main/docs).
### Important Integration Callouts[​](https://docs.uniswap.org/contracts/smart-wallet/technical-reference#important-integration-callouts "Direct link to Important Integration Callouts")
The Calibur contracts are open sourced and MIT licensed. While anyone is free to build off of them, it is extremely important that integrators protect against certain known integration pitfalls.
  * By default, registered keys do not expire and they do not have any hooks attached to them. While they are not admin keys and thus cannot self-call (callback into the Calibur contract), they can still spend any token and ETH balances.
  * An admin key added to the account can add other keys with any expiration or admin status
  * It is possible to pass along hookData to hooks for extra verification. This data is arbitrary and hook developers MUST verify its uniqueness and integrity.
  * The contract does not enforce that the User Verification flag is set in Webauthn signatures
  * Signatures from the private key of the EOA are always valid on the Calibur contracts. Those from alternative signers can be invalidated if desired by calling `updateSalt`, which will update the account’s EIP712 domain separator.
  * It is completely valid to not execute the ERC7739 rehash code path on a signature originating from the root EOA. This is because this contract makes an assumption that IF the root key is registered on another wallet, that wallet is responsible for rehashing.
  * Undelegating and redelegating may leave your account exposed to dirtied storage. Proceed with caution when redelegating to another contract.


For a comprehensive list of best practices, reference the documentation in the Github repository, past audit reports, and inline comments in the contract code.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/03-technical-reference.md)
Was this helpful?
[PreviousERC-7914](https://docs.uniswap.org/contracts/smart-wallet/advanced-usage/erc-7914)[NextOverview](https://docs.uniswap.org/contracts/uniswapx/overview)
On this page
  * [Important Integration Callouts](https://docs.uniswap.org/contracts/smart-wallet/technical-reference#important-integration-callouts)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/smart-wallet/03-technical-reference.md)
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
