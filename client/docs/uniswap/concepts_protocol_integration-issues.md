# https://docs.uniswap.org/concepts/protocol/integration-issues

[Skip to main content](https://docs.uniswap.org/concepts/protocol/integration-issues#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
    * [Uniswap Overview](https://docs.uniswap.org/concepts/overview)
    * [The Uniswap Protocol](https://docs.uniswap.org/concepts/uniswap-protocol)
    * [Protocol Concepts](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Hooks](https://docs.uniswap.org/concepts/protocol/hooks)
      * [Swaps](https://docs.uniswap.org/concepts/protocol/swaps)
      * [Fees](https://docs.uniswap.org/concepts/protocol/fees)
      * [Oracle](https://docs.uniswap.org/concepts/protocol/oracle)
      * [Range Orders](https://docs.uniswap.org/concepts/protocol/range-orders)
      * [Concentrated Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)
      * [Token Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)
    * [Governance](https://docs.uniswap.org/concepts/governance/overview)
    * [Research](https://docs.uniswap.org/concepts/research)
    * [Resources](https://docs.uniswap.org/concepts/resources)
    * [Glossary](https://docs.uniswap.org/concepts/glossary)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * Protocol Concepts
  * [Token Integration Issues](https://docs.uniswap.org/concepts/protocol/integration-issues)


On this page
# Token Integration Issues
Fee-on-transfer and rebasing tokens will not function correctly on v3.
## Fee-on-transfer Tokens[​](https://docs.uniswap.org/concepts/protocol/integration-issues#fee-on-transfer-tokens "Direct link to Fee-on-transfer Tokens")
Fee-on-transfer tokens will not function with our router contracts. As a workaround, the token creators may create a token wrapper or a customized router. We will not be making a router that supports fee-on-transfer tokens in the future.
## Rebasing Tokens[​](https://docs.uniswap.org/concepts/protocol/integration-issues#rebasing-tokens "Direct link to Rebasing Tokens")
Rebasing tokens will succeed in pool creation and swapping, but liquidity providers will bear the loss of a negative rebase when their position becomes active, with no way to recover the loss.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/integration-issues.md)
Was this helpful?
[PreviousConcentrated Liquidity](https://docs.uniswap.org/concepts/protocol/concentrated-liquidity)[NextOverview](https://docs.uniswap.org/concepts/governance/overview)
On this page
  * [Fee-on-transfer Tokens](https://docs.uniswap.org/concepts/protocol/integration-issues#fee-on-transfer-tokens)
  * [Rebasing Tokens](https://docs.uniswap.org/concepts/protocol/integration-issues#rebasing-tokens)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/concepts/protocol/integration-issues.md)
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
