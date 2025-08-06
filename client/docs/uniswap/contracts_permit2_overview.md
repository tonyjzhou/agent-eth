# https://docs.uniswap.org/contracts/permit2/overview

[Skip to main content](https://docs.uniswap.org/contracts/permit2/overview#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/permit2/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/permit2/overview)
      * [Quickstart](https://docs.uniswap.org/contracts/permit2/overview)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/permit2/overview)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/permit2/overview)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/permit2/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/permit2/overview)
    * [v3 Protocol](https://docs.uniswap.org/contracts/permit2/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/permit2/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/permit2/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/permit2/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
      * [Overview](https://docs.uniswap.org/contracts/permit2/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/permit2/overview)
        * [SignatureTransfer](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
        * [AllowanceTransfer](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer)
    * [v2 Protocol](https://docs.uniswap.org/contracts/permit2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/permit2/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Permit2
  * [Overview](https://docs.uniswap.org/contracts/permit2/overview)


On this page
# Overview
[`Permit2`](https://github.com/Uniswap/permit2) is a unification of 2 contracts, [`SignatureTransfer`](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer) and [`AllowanceTransfer`](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer). The `SignatureTransfer` contract handles all signature-based transfers, meaning that an allowance on the token is bypassed and permissions to the spender only last for the duration of the transaction that the one-time signature is spent. The `AllowanceTransfer` contract handles setting allowances on tokens, giving permissions to spenders on a specified amount for a specified duration of time. Any transfers that then happen through the `AllowanceTransfer` contract will only succeed if the proper permissions have been set.
## Resources[​](https://docs.uniswap.org/contracts/permit2/overview#resources "Direct link to Resources")
A great [explanation](https://github.com/dragonfly-xyz/useful-solidity-patterns/tree/main/patterns/permit2) of the Permit2 contract and example usage.
## Approving Permit2[​](https://docs.uniswap.org/contracts/permit2/overview#approving-permit2 "Direct link to Approving Permit2")
Before integrating contracts can request users’ tokens through Permit2, users must approve the Permit2 contract through the specific token contract by calling something like:
```
USDC.approve(permit2Address, totalAmount);
```

To get the maximal benefits from Permit2, users should do a max approval on the contract where:
```
totalAmount =type(uint256).max;
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/permit2/overview.md)
Was this helpful?
[PreviousTechnical Reference](https://docs.uniswap.org/contracts/universal-router/technical-reference)[NextSignatureTransfer](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
On this page
  * [Resources](https://docs.uniswap.org/contracts/permit2/overview#resources)
  * [Approving Permit2](https://docs.uniswap.org/contracts/permit2/overview#approving-permit2)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/permit2/overview.md)
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
