# https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
          * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
          * [Mint Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position)
          * [Increase Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/increase-liquidity)
          * [Decrease Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/decrease-liquidity)
          * [Collect Fees](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/collect)
          * [Burn Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/burn-liquidity)
          * [Batch Modify](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/batch-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
    * [Permit2](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Quickstart
  * Manage Liquidity
  * [Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)


On this page
# Setup
For users looking to interact with the canonical Uniswap v4 `PositionManager`, _v4-periphery_ is a required dependency
Currently, developing with Uniswap v4 _requires[foundry](https://book.getfoundry.sh)_
## Quickstart[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity#quickstart "Direct link to Quickstart")
_Use[v4-template](https://github.com/new?template_name=v4-template&template_owner=uniswapfoundation)_, which has pre-configured dependencies and tests for Uniswap v4
Clone the repository made from _v4-template_
```
git clone https://github.com/<your_username>/<your_repo>
```

Install dependencies
```
forge install
```

## Manual Setup[​](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity#manual-setup "Direct link to Manual Setup")
After cloning the repository, and installing foundry, developers can manually set up their Uniswap v4 environment:
  1. Initialize a foundry project
```
forge init . --force
```

  2. Install dependencies
```
forge install uniswap/v4-coreforge install uniswap/v4-periphery
```

  3. Set the `remappings.txt` to:
```
@uniswap/v4-core/=lib/v4-core/forge-gas-snapshot/=lib/v4-core/lib/forge-gas-snapshot/src/forge-std/=lib/v4-core/lib/forge-std/src/permit2/=lib/v4-periphery/lib/permit2/solmate/=lib/v4-core/lib/solmate/v4-periphery/=lib/v4-periphery/
```



[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/00-setup-liquidity.mdx)
Was this helpful?
[PreviousCreate Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)[NextMint Position](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/mint-position)
On this page
  * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity#quickstart)
  * [Manual Setup](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity#manual-setup)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/02-manage-liquidity/00-setup-liquidity.mdx)
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
