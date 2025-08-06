# https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/security

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/security#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
          * [Fees](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
          * [Pricing](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing)
          * [Understanding Returns](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/understanding-returns)
          * [Security](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/security)
          * [Math](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/math)
          * [Research](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/research)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/Governance/governance-reference)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Concepts
  * Advanced Topics
  * [Security](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/security)


# Audit & Formal Verification
Between January 8 and April 30, a team of six engineers reviewed and formally verified crucial components of the smart contracts for Uniswap V2.
Their past work includes smart contract development on and formal verification of multi-collateral DAI.
The scope of work includes:
  * Formal verification of the core smart contracts
  * Code review of core smart contracts
  * Numerical error analysis
  * Code review of periphery smart contracts (during ongoing development)


The report also has a "Design Comments" section that we highly recommend for gaining a deep technical understanding of some of the choices made in Uniswap V2.
> [Read the report](https://uniswap.org/audit.html)
# Bug Bounty
Uniswap has an open and ongoing bug [bounty program](https://uniswap.org/bug-bounty/).
# Considerations when building on Uniswap
When integrating Uniswap V2 into another on-chain system, particular care must be taken to avoid security vulnerabilities, avenues for manipulations, and the potential loss of funds.
As a preliminary note: smart contract integrations can happen at two levels: directly with [Pair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair) contracts, or through the [Router](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02). Direct interactions offer maximal flexibility but require the most work to get right. Mediated interactions offer more limited capabilities but stronger safety guarantees.
There are two primary categories of risk associated with Uniswap V2. The first involves so-called "static" errors. These can include sending too many tokens to a pair during a swap (or requesting too few tokens back) or allowing transactions to linger in the mempool long enough for the sender's expectations about prices to no longer be accurate.
One may address these errors with fairly straightforward logic checks. Executing these logic checks is the primary purpose of routers. Those who interact directly with pairs must perform these checks themselves (with the help of the [Library](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library)).
"Dynamic" risk, the second category, involves runtime pricing. Because Ethereum transactions occur in an adversarial environment, naively written smart contracts can, and will, be exploited for profit. For example, suppose a smart contract checks the asset ratio in a Uniswap pool at runtime and trades against it, assuming that the ratio represents the "fair" or "market" price of these assets. In that case, it is highly vulnerable to manipulation. A malicious actor could, e.g., trivially insert transactions before and after the naive transaction (a so-called "sandwich" attack), causing the smart contract to trade at a radically worse price, profit from this at the trader's expense, and then return the contracts to their original state, all at a low cost. (One important caveat is that these types of attacks are mitigated by trading in highly liquid pools, or at low values.)
The best way to protect against these attacks is to introduce a price oracle. An oracle is any device that returns desired information, in this case, a pair's spot price. The best "oracle" is simply a traders' off-chain observation of the prevailing price, which can be passed into the trade as a safety check. This strategy is best suited to retail trading venues where users initiate transactions on their own behalf. However, it is often the case that a trusted price observation is not available (e.g., in multi-step, programmatic interactions involving Uniswap). Without a price oracle, these interactions will be forced to trade at whatever the (potentially manipulated) rate on Uniswap is. For details on the Uniswap V2 approach to oracles, see [Oracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/03-advanced-topics/04-security.md)
Was this helpful?
[PreviousUnderstanding Returns](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/understanding-returns)[NextMath](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/math)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/03-advanced-topics/04-security.md)
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
