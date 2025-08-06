# https://docs.uniswap.org/contracts/v3/reference/error-codes

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/error-codes#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/reference/error-codes)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/reference/error-codes)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/error-codes)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/reference/error-codes)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/reference/error-codes)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/error-codes)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Permit2](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/reference/error-codes)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)


# Error Codes
LiquidityMath.sol
  * `LS`: Liquidity Sub
  * `LA`: Liquidity Add


Oracle.sol
  * `OLD`: The target must be chronologically after the oldest observation
  * `I`: The pool has not been initialized


Position.sol
  * `NP`: Burn cannot be called for a position with 0 liquidity


Tick.sol
  * `LO`: LiquidityGrossAfter must be less than MaxLiquidity


TickMath.sol
  * `T`: The given tick must be less than, or equal to, the maximum tick
  * `R`: second inequality must be < because the price can never reach the price at the max tick


TransferHelper.sol
  * `TF`: Transfer Failed : errors with TF if transfer fails


UniswapV3Pool.sol
  * `LOK`: The reentrancy guard. A transaction cannot re-enter the pool mid-swap
  * `TLU`: The lower tick must be below the upper tick
  * `TLM`: The lower tick must be greater, or equal to, the minimum tick
  * `TUM`: The upper tick must be lesser than, or equal to, the maximum tick
  * `AI`: The pool is already initialized
  * `M0`: Mint 0, The balance of token0 in the given pool before minting must be less than, or equal to, the balance after minting
  * `M1`: Mint 1, The balance of token1 in the given pool before minting must be less than, or equal to, the balance after minting
  * `AS`: `amountSpecified` cannot be zero
  * `SPL`: Square root price limit
  * `IIA`: Insufficient input amount, an insufficient amount of input token was sent during the callback
  * `L`: Liquidity in the pool must be greater than zero for a flash to be executed
  * `F0`: The balance of token0 in the given pool before the flash transaction must be less than, or equal to, the balance of token0 after the flash plus the fee
  * `F1`: The balance of token1 in the given pool before the flash transaction must be less than, or equal to, the balance of token1 after the flash plus the fee


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/error-codes.md)
Was this helpful?
[PreviousZora Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/Zora-deployments)[NextOverview](https://docs.uniswap.org/contracts/smart-wallet/overview)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/error-codes.md)
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
