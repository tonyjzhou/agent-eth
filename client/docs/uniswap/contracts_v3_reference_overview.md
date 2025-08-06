# https://docs.uniswap.org/contracts/v3/reference/overview

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/overview#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/periphery/base/BlockTimestamp)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)


On this page
# Overview
Uniswap v3 is a binary smart contract system comprised of many libraries, which together make the Core and Periphery.
Core contracts provide fundamental safety guarantees for all parties interacting with Uniswap. They define the logic of pool generation, the pools themselves, and the interactions involving the respective assets therein.
Periphery contracts interact with one or more Core contracts but are not part of the core. They are designed to provide methods of interacting with the core that increase clarity and user safety.
External calls will primarily call the periphery interfaces. Externally available functions are all viewable in the reference documentation. Internal functions are viewable on the Uniswap v3 GitHub repo.
## Core[​](https://docs.uniswap.org/contracts/v3/reference/overview#core "Direct link to Core")
> [**Core Source Code**](https://github.com/Uniswap/uniswap-v3-core)
The core consists of a single factory, a pool deployer, and the many pools the factory will create.
A significant amount of care and attention has been given to gas optimization in the core contracts. The result is a substantial reduction in gas costs for all protocol interactions compared to V2, at the cost of a reduction in code clarity.
### Factory[​](https://docs.uniswap.org/contracts/v3/reference/overview#factory "Direct link to Factory")
> [**Factory Reference**](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
The factory defines the logic for generating pools. A pool is defined by two tokens, which make up the asset pair, and a fee. There can be multiple pools of the same asset pair, distinguished only by their swap fee.
### Pools[​](https://docs.uniswap.org/contracts/v3/reference/overview#pools "Direct link to Pools")
> [**Pool Reference**](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool).
Pools primarily serve as automated market makers for the paired assets. Additionally, they expose price oracle data and may be used as an asset source for flash transactions.
## Periphery[​](https://docs.uniswap.org/contracts/v3/reference/overview#periphery "Direct link to Periphery")
The periphery is a constellation of smart contracts designed to support domain-specific interactions with the core. As the Uniswap protocol is a permissionless system, the contracts described below have no special privileges and are only a small subset of possible periphery-like contracts.
### SwapRouter[​](https://docs.uniswap.org/contracts/v3/reference/overview#swaprouter "Direct link to SwapRouter")
> [**Swap Router Reference**](https://docs.uniswap.org/contracts/v3/reference/periphery/SwapRouter)
> [**Swap Router Interface**](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/ISwapRouter)
The swap router supports all the basic requirements of a front-end offering trading. It natively supports single trades (x to y) and multihop trades (e.g. x to y to z).
### Nonfungible Position Manager[​](https://docs.uniswap.org/contracts/v3/reference/overview#nonfungible-position-manager "Direct link to Nonfungible Position Manager")
> [**Nonfungible Position Manager Reference**](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager)
> [**Nonfungible Position Manager Interface**](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/INonfungiblePositionManager)
The position manager handles the logic transactions involving the creation, adjustment, or exiting of positions.
### Oracle[​](https://docs.uniswap.org/contracts/v3/reference/overview#oracle "Direct link to Oracle")
> [**Oracle Reference**](https://docs.uniswap.org/contracts/v3/reference/core/libraries/Oracle)
The oracle provides price and liquidity data useful for a wide variety of system designs, and is available in every deployed pool.
### Periphery Libraries[​](https://docs.uniswap.org/contracts/v3/reference/overview#periphery-libraries "Direct link to Periphery Libraries")
> [**Periphery Libraries**](https://docs.uniswap.org/contracts/v3/reference/periphery/libraries/Base64)
The libraries provide a variety of helper functions developers may need, like calculating pool addresses, safe transfer functions, and more.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/overview.md)
Was this helpful?
[PreviousLicense Modifications](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)[NextUniswapV3Factory](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
On this page
  * [Core](https://docs.uniswap.org/contracts/v3/reference/overview#core)
    * [Factory](https://docs.uniswap.org/contracts/v3/reference/overview#factory)
    * [Pools](https://docs.uniswap.org/contracts/v3/reference/overview#pools)
  * [Periphery](https://docs.uniswap.org/contracts/v3/reference/overview#periphery)
    * [SwapRouter](https://docs.uniswap.org/contracts/v3/reference/overview#swaprouter)
    * [Nonfungible Position Manager](https://docs.uniswap.org/contracts/v3/reference/overview#nonfungible-position-manager)
    * [Oracle](https://docs.uniswap.org/contracts/v3/reference/overview#oracle)
    * [Periphery Libraries](https://docs.uniswap.org/contracts/v3/reference/overview#periphery-libraries)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/overview.md)
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
