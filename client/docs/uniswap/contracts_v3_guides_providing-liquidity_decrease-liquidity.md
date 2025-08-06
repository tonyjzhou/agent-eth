# https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
          * [Set Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
          * [Mint a New Position](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
          * [Collecting Fees](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees)
          * [Decrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
          * [Increase Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity)
          * [The Full Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
    * [Permit2](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Guides
  * Providing Liquidity
  * [Decrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)


On this page
# Decrease Liquidity
Make sure to go through the [Setting Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up) before continuing to this section
Here we decrease the liquidity of our position without withdrawing all of it.
  * This example assumes the contract already has possession of the position NFT, and requires the calling address to be the same address that deposited the position NFT to our contract.
  * In production, `amount0Min` and `amount1Min` should be adjusted to create slippage protections.


## Decrease Liquidity[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity#decrease-liquidity "Direct link to Decrease Liquidity")
```
/// @notice A function that decreases the current liquidity by half. An example to show how to call the `decreaseLiquidity` function defined in periphery./// @param tokenId The id of the erc721 token/// @return amount0 The amount received back in token0/// @return amount1 The amount returned back in token1functiondecreaseLiquidityInHalf(uint256 tokenId)externalreturns(uint256 amount0,uint256 amount1){// caller must be the owner of the NFTrequire(msg.sender == deposits[tokenId].owner,'Not the owner');// get liquidity data for tokenIduint128 liquidity = deposits[tokenId].liquidity;uint128 halfLiquidity = liquidity /2;// amount0Min and amount1Min are price slippage checks// if the amount received after burning is not greater than these minimums, transaction will fail    INonfungiblePositionManager.DecreaseLiquidityParams memory params =      INonfungiblePositionManager.DecreaseLiquidityParams({        tokenId: tokenId,        liquidity: halfLiquidity,        amount0Min:0,        amount1Min:0,        deadline: block.timestamp});    nonfungiblePositionManager.decreaseLiquidity(params);(amount0, amount1)= nonfungiblePositionManager.collect(      INonfungiblePositionManager.CollectParams({        tokenId: tokenId,        recipient:address(this),        amount0Max:type(uint128).max,        amount1Max:type(uint128).max}));//send liquidity back to owner_sendToOwner(tokenId, amount0, amount1);}
```

## Sending Fees To The Calling Address[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity#sending-fees-to-the-calling-address "Direct link to Sending Fees To The Calling Address")
This internal helper function sends any tokens, in the form of fees or position tokens, to the owner of an NFT.
In `_sendToOwner`, we pass the amount of fees due, previously populated in the last function, as arguments to `safeTransfer`, which transfers the fees to `owner`.
```
/// @notice Transfers funds to owner of NFT/// @param tokenId The id of the erc721/// @param amount0 The amount of token0/// @param amount1 The amount of token1function_sendToOwner(uint256 tokenId,uint256 amount0,uint256 amount1)internal{// get owner of contractaddress owner = deposits[tokenId].owner;address token0 = deposits[tokenId].token0;address token1 = deposits[tokenId].token1;// send collected fees to owner    TransferHelper.safeTransfer(token0, owner, amount0);    TransferHelper.safeTransfer(token1, owner, amount1);}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/decrease-liquidity.md)
Was this helpful?
[PreviousCollecting Fees](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees)[NextIncrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity)
On this page
  * [Decrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity#decrease-liquidity)
  * [Sending Fees To The Calling Address](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity#sending-fees-to-the-calling-address)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/decrease-liquidity.md)
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
