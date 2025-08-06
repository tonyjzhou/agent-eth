# https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity#__docusaurus_skipToContent_fallback)
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
          * [Set Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
          * [Mint a New Position](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
          * [Collecting Fees](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees)
          * [Decrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
          * [Increase Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity)
          * [The Full Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/overview)
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
  * Guides
  * Providing Liquidity
  * [Increase Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity)


On this page
# Increase Liquidity
## Increase Liquidity Within The Current Range[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity#increase-liquidity-within-the-current-range "Direct link to Increase Liquidity Within The Current Range")
Make sure to go through the [first guide](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up) before continuing to this section
  * This example assumes the contract already has custody of the NFT.
  * We cannot change the boundaries of a given liquidity position using the Uniswap v3 protocol; `increaseLiquidity` can only increase the liquidity of a position.
  * In production, `amount0Min` and `amount1Min` should be adjusted to create slippage protections.


```
/// @notice Increases liquidity in the current range/// @dev Pool must be initialized already to add liquidity/// @param tokenId The id of the erc721 token/// @param amount0 The amount to add of token0/// @param amount1 The amount to add of token1functionincreaseLiquidityCurrentRange(uint256 tokenId,uint256 amountAdd0,uint256 amountAdd1)externalreturns(uint128 liquidity,uint256 amount0,uint256 amount1){    INonfungiblePositionManager.IncreaseLiquidityParams memory params =      INonfungiblePositionManager.IncreaseLiquidityParams({        tokenId: tokenId,        amount0Desired: amountAdd0,        amount1Desired: amountAdd1,        amount0Min:0,        amount1Min:0,        deadline: block.timestamp});(liquidity, amount0, amount1)= nonfungiblePositionManager.increaseLiquidity(params);}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/increase-liquidity.md)
Was this helpful?
[PreviousDecrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)[NextThe Full Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
On this page
  * [Increase Liquidity Within The Current Range](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity#increase-liquidity-within-the-current-range)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/increase-liquidity.md)
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
