# https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer#__docusaurus_skipToContent_fallback)
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
          * [UniswapV3Factory](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
          * [UniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
          * [UniswapV3PoolDeployer](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
          * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IERC20Minimal)
            * [IERC20Minimal](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IERC20Minimal)
            * [IUniswapV3Factory](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3Factory)
            * [IUniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3Pool)
            * [IUniswapV3PoolDeployer](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer)
            * [callback](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/callback/IUniswapV3FlashCallback)
            * [Pool](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/pool/IUniswapV3PoolActions)
          * [Libraries](https://docs.uniswap.org/contracts/v3/reference/core/libraries/BitMath)
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
  * Core
  * Interfaces
  * [IUniswapV3PoolDeployer](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer)


On this page
# IUniswapV3PoolDeployer
A contract that constructs a pool must implement this to pass arguments to the pool
This is used to avoid having constructor arguments in the pool contract, which results in the init code hash of the pool being constant allowing the CREATE2 address of the pool to be cheaply computed on-chain
## Functions[​](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer#functions "Direct link to Functions")
### parameters[​](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer#parameters "Direct link to parameters")
```
functionparameters()externalviewreturns(address factory,address token0,address token1,uint24 fee,int24 tickSpacing)
```

Get the parameters to be used in constructing the pool, set transiently during pool creation.
#### Return Values :[​](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer#return-values- "Direct link to Return Values :")
Name| Type| Description  
---|---|---  
`factory `| address| The factory address  
`token0`| address| The first token of the pool by address sort order  
`token1`| address| The second token of the pool by address sort order  
`fee `| uint24| The fee collected upon every swap in the pool, denominated in hundredths of a bip  
`tickSpacing`| int24| The minimum number of ticks between initialized ticks  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer.md)
Was this helpful?
[PreviousIUniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3Pool)[NextIUniswapV3FlashCallback](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/callback/IUniswapV3FlashCallback)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer#functions)
    * [parameters](https://docs.uniswap.org/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer#parameters)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/core/interfaces/IUniswapV3PoolDeployer.md)
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
