# https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
          * [UniswapV3Factory](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
          * [UniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
          * [UniswapV3PoolDeployer](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
          * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
          * [Libraries](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
    * [Permit2](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * Core
  * [UniswapV3PoolDeployer](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)


On this page
# UniswapV3PoolDeployer
## Functions[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer#functions "Direct link to Functions")
### deploy[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer#deploy "Direct link to deploy")
```
functiondeploy(address factory,address token0,address token1,uint24 fee,int24 tickSpacing)internalreturns(address pool)
```

Deploys a pool with the given parameters by transiently setting the parameters storage slot and then clearing it after deploying the pool.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer#parameters "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`factory`| address| The contract address of the Uniswap V3 factory  
`token0`| address| The first token of the pool by address sort order  
`token1`| address| The second token of the pool by address sort order  
`fee`| uint24| The fee collected upon every swap in the pool, denominated in hundredths of a bip  
`tickSpacing`| int24| The spacing between usable ticks  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/core/UniswapV3PoolDeployer.md)
Was this helpful?
[PreviousUniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)[NextIERC20Minimal](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IERC20Minimal)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer#functions)
    * [deploy](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer#deploy)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/core/UniswapV3PoolDeployer.md)
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
