# https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary

[Skip to main content](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#__docusaurus_skipToContent_fallback)
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
          * [How Uniswap works](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
          * [Ecosystem Participants](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/ecosystem-participants)
          * [Smart contracts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts)
          * [Glossary](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
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
  * Protocol Overview
  * [Glossary](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary)


On this page
# Glossary
### Automated market maker[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#automated-market-maker "Direct link to Automated market maker")
An automated market maker is a smart contract on Ethereum that holds on-chain liquidity reserves. Users can trade against these reserves at prices set by an automated market making formula.
### Constant product formula[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#constant-product-formula "Direct link to Constant product formula")
The automated market making algorithm used by Uniswap. See [x*y=k](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#x--y--k).
### ERC20[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#erc20 "Direct link to ERC20")
ERC20 tokens are fungible tokens on Ethereum. Uniswap supports all standard ERC20 implementations.
### Factory[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#factory "Direct link to Factory")
A smart contract that deploys a unique smart contract for any ERC20/ERC20 trading pair.
### Pair[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#pair "Direct link to Pair")
A smart contract deployed from the Uniswap V2 Factory that enables trading between two ERC20 tokens.
### Pool[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#pool "Direct link to Pool")
Liquidity within a pair is pooled across all liquidity providers.
### Liquidity provider / LP[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#liquidity-provider--lp "Direct link to Liquidity provider / LP")
A liquidity provider is someone who deposits an equivalent value of two ERC20 tokens into the liquidity pool within a pair. Liquidity providers take on price risk and are compensated with fees.
### Mid price[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#mid-price "Direct link to Mid price")
The price between what users can buy and sell tokens at a given moment. In Uniswap this is the ratio of the two ERC20 token reserves.
### Price impact[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#price-impact "Direct link to Price impact")
The difference between the mid-price and the execution price of a trade.
### Slippage[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#slippage "Direct link to Slippage")
The amount the price moves in a trading pair between when a transaction is submitted and when it is executed.
### Core[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#core "Direct link to Core")
Smart contracts that are essential for Uniswap to exist. Upgrading to a new version of core would require a liquidity migration.
### Periphery[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#periphery "Direct link to Periphery")
External smart contracts that are useful, but not required for Uniswap to exist. New periphery contracts can always be deployed without migrating liquidity.
### Flash swap[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#flash-swap "Direct link to Flash swap")
A trade that uses the tokens being purchased before paying for them.
### `x * y = k`[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#x--y--k "Direct link to x--y--k")
The constant product formula.
### Invariant[​](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#invariant "Direct link to Invariant")
The "k" value in the constant product formula
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/01-protocol-overview/04-glossary.md)
Was this helpful?
[PreviousSmart contracts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/smart-contracts)[NextSwaps](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
On this page
  * [Automated market maker](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#automated-market-maker)
  * [Constant product formula](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#constant-product-formula)
  * [ERC20](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#erc20)
  * [Factory](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#factory)
  * [Pair](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#pair)
  * [Pool](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#pool)
  * [Liquidity provider / LP](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#liquidity-provider--lp)
  * [Mid price](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#mid-price)
  * [Price impact](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#price-impact)
  * [Slippage](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#slippage)
  * [Core](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#core)
  * [Periphery](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#periphery)
  * [Flash swap](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#flash-swap)
  * [`x * y = k`](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#x--y--k)
  * [Invariant](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/glossary#invariant)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/concepts/01-protocol-overview/04-glossary.md)
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
