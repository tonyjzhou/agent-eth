# https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
          * [UniswapV3Factory](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
          * [UniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
          * [UniswapV3PoolDeployer](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3PoolDeployer)
          * [Interfaces](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
          * [Libraries](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
    * [Permit2](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * Core
  * [UniswapV3Factory](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)


On this page
# UniswapV3Factory
Deploys Uniswap V3 pools and manages ownership and control over pool protocol fees
## Functions[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#functions "Direct link to Functions")
### createPool[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#createpool "Direct link to createPool")
```
functioncreatePool(address tokenA,address tokenB,uint24 fee)externalreturns(address pool)
```

Creates a pool for the given two tokens and fee
tokenA and tokenB may be passed in either order: token0/token1 or token1/token0. tickSpacing is retrieved from the fee. The call will revert if the pool already exists, the fee is invalid, or the token arguments are invalid.
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#parameters "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`tokenA`| address| One of the two tokens in the desired pool  
`tokenB`| address| The other of the two tokens in the desired pool  
`fee`| uint24| The desired fee for the pool  
#### Return Values:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#return-values "Direct link to Return Values:")
Name| Type| Description  
---|---|---  
`pool`| address| The address of the newly created pool  
### setOwner[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#setowner "Direct link to setOwner")
```
functionsetOwner(address _owner)external
```

Updates the owner of the factory
Must be called by the current owner
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#parameters-1 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`_owner`| address| The new owner of the factory  
### enableFeeAmount[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#enablefeeamount "Direct link to enableFeeAmount")
```
functionenableFeeAmount(uint24 fee,int24 tickSpacing)public
```

Enables a fee amount with the given tickSpacing
Fee amounts may never be removed once enabled
#### Parameters:[​](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#parameters-2 "Direct link to Parameters:")
Name| Type| Description  
---|---|---  
`fee`| uint24| The fee amount to enable, denominated in hundredths of a bip (i.e. 1e-6)  
`tickSpacing`| int24| The spacing between ticks to be enforced for all pools created with the given fee amount  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/core/UniswapV3Factory.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/contracts/v3/reference/overview)[NextUniswapV3Pool](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Pool)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#functions)
    * [createPool](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#createpool)
    * [setOwner](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#setowner)
    * [enableFeeAmount](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory#enablefeeamount)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/core/UniswapV3Factory.md)
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
