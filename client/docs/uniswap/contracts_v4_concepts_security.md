# https://docs.uniswap.org/contracts/v4/concepts/security

[Skip to main content](https://docs.uniswap.org/contracts/v4/concepts/security#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/concepts/security)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/security)
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
      * [Quickstart](https://docs.uniswap.org/contracts/v4/concepts/security)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/concepts/security)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/concepts/security)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/concepts/security)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/concepts/security)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/concepts/security)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/concepts/security)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/concepts/security)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/concepts/security)
    * [Permit2](https://docs.uniswap.org/contracts/v4/concepts/security)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/concepts/security)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/concepts/security)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Concepts
  * [Security](https://docs.uniswap.org/contracts/v4/concepts/security)


On this page
# Security Considerations
When building on Uniswap v4, security should be a primary consideration. This section covers emergency response resources and security best practices specific to v4 implementations.
## Emergency Response[​](https://docs.uniswap.org/contracts/v4/concepts/security#emergency-response "Direct link to Emergency Response")
### SEAL 911 Emergency Hotline[​](https://docs.uniswap.org/contracts/v4/concepts/security#seal-911-emergency-hotline "Direct link to SEAL 911 Emergency Hotline")
If you encounter a security incident (exploit, vulnerability, or other urgent security matter) while working with Uniswap v4, the SEAL 911 Emergency Hotline provides immediate access to security experts.
**Emergency Contact** : <https://t.me/seal_911_bot>
SEAL 911 is a community-operated Telegram bot that connects you directly with vetted security responders who can provide immediate assistance during security incidents.
#### How It Works[​](https://docs.uniswap.org/contracts/v4/concepts/security#how-it-works "Direct link to How It Works")
  * Send a message through the bot during a security emergency
  * Automatic alert routing to a vetted group of white hat security professionals
  * Immediate response from trusted security experts in the space


#### Additional Resources[​](https://docs.uniswap.org/contracts/v4/concepts/security#additional-resources "Direct link to Additional Resources")
  * [SEAL 911 GitHub Repository](https://github.com/security-alliance/seal-911)
  * [Security Alliance Website](https://www.securityalliance.org/seal-911)


note
SEAL 911 is a third-party service operated by the Security Alliance. Exercise appropriate judgment when sharing sensitive information during emergency situations.
## v4-Specific Security Considerations[​](https://docs.uniswap.org/contracts/v4/concepts/security#v4-specific-security-considerations "Direct link to v4-Specific Security Considerations")
### Hook Security[​](https://docs.uniswap.org/contracts/v4/concepts/security#hook-security "Direct link to Hook Security")
When developing custom hooks for v4, ensure proper validation and access controls. Malicious or poorly implemented hooks can compromise pool security.
### Flash Accounting[​](https://docs.uniswap.org/contracts/v4/concepts/security#flash-accounting "Direct link to Flash Accounting")
v4's flash accounting system requires careful implementation to prevent exploitation. Always ensure proper settlement of deltas.
### Pool Manager Interactions[​](https://docs.uniswap.org/contracts/v4/concepts/security#pool-manager-interactions "Direct link to Pool Manager Interactions")
Direct interactions with the PoolManager require thorough understanding of the locking mechanism and callback patterns.
## Bug Bounty Program[​](https://docs.uniswap.org/contracts/v4/concepts/security#bug-bounty-program "Direct link to Bug Bounty Program")
For non-emergency security issues, report vulnerabilities through Uniswap's official [bug bounty program](https://uniswap.org/bug-bounty/).
## Additional Security Resources[​](https://docs.uniswap.org/contracts/v4/concepts/security#additional-security-resources "Direct link to Additional Security Resources")
  * Review the [v4 Core contracts](https://docs.uniswap.org/contracts/v4/reference/core/) for implementation details
  * Follow security best practices outlined in the [Hooks documentation](https://docs.uniswap.org/contracts/v4/concepts/hooks)
  * Test thoroughly using the provided [test contracts](https://docs.uniswap.org/contracts/v4/reference/core/test/)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/10-security.mdx)
Was this helpful?
[Previousv4 Fee Structure Guide](https://docs.uniswap.org/contracts/v4/concepts/fees)[NextCreate Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
On this page
  * [Emergency Response](https://docs.uniswap.org/contracts/v4/concepts/security#emergency-response)
    * [SEAL 911 Emergency Hotline](https://docs.uniswap.org/contracts/v4/concepts/security#seal-911-emergency-hotline)
  * [v4-Specific Security Considerations](https://docs.uniswap.org/contracts/v4/concepts/security#v4-specific-security-considerations)
    * [Hook Security](https://docs.uniswap.org/contracts/v4/concepts/security#hook-security)
    * [Flash Accounting](https://docs.uniswap.org/contracts/v4/concepts/security#flash-accounting)
    * [Pool Manager Interactions](https://docs.uniswap.org/contracts/v4/concepts/security#pool-manager-interactions)
  * [Bug Bounty Program](https://docs.uniswap.org/contracts/v4/concepts/security#bug-bounty-program)
  * [Additional Security Resources](https://docs.uniswap.org/contracts/v4/concepts/security#additional-security-resources)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/concepts/10-security.mdx)
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
