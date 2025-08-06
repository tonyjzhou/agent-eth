# https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
          * [Set Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
          * [Mint a New Position](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
          * [Collecting Fees](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees)
          * [Decrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
          * [Increase Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity)
          * [The Full Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
    * [Permit2](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Guides
  * Providing Liquidity
  * [Mint a New Position](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)


On this page
# Mint a New Position
## Input Parameters[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#input-parameters "Direct link to Input Parameters")
To mint a new position, we use the `nonFungiblePositionManager` and call `mint`.
For the sake of this example, we're hard coding the token amounts to be minted. In production, this would be a user-configurable function argument.
```
/// @notice Calls the mint function defined in periphery, mints the same amount of each token. For this example we are providing 1000 DAI and 1000 USDC in liquidity/// @return tokenId The id of the newly minted ERC721/// @return liquidity The amount of liquidity for the position/// @return amount0 The amount of token0/// @return amount1 The amount of token1functionmintNewPosition()externalreturns(uint256 tokenId,uint128 liquidity,uint256 amount0,uint256 amount1){// For this example, we will provide equal amounts of liquidity in both assets.// Providing liquidity in both assets means liquidity will be earning fees and is considered in-range.uint256 amount0ToMint =1000;uint256 amount1ToMint =1000;
```

## Calling Mint[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#calling-mint "Direct link to Calling Mint")
Here we approve the `nonfungiblePositionManager` to use the contracts' tokens, then populate the `MintParams` struct and assign it to a local variable `params` that will be passed to the `nonfungiblePositionManager` when we call `mint`.
  * By using `TickMath.MIN_TICK` and `TickMath.MAX_TICK`, we are providing liquidity across the whole range of the pool. In production you may want to specify a more concentrated position.
  * We set `amount0Min` and `amount1Min` to zero for the example - but this would be a vulnerability in production. A function calling `mint` with no slippage protection would be vulnerable to a frontrunning attack designed to execute the `mint` call at an inaccurate price.
  * For a more secure practice the developer would need to implement a slippage estimation process.
  * Note that this function will not initialize a pool where one does not yet exist.


```
// Approve the position manager    TransferHelper.safeApprove(DAI,address(nonfungiblePositionManager), amount0ToMint);    TransferHelper.safeApprove(USDC,address(nonfungiblePositionManager), amount1ToMint);    INonfungiblePositionManager.MintParams memory params =      INonfungiblePositionManager.MintParams({        token0: DAI,        token1: USDC,        fee: poolFee,        tickLower: TickMath.MIN_TICK,        tickUpper: TickMath.MAX_TICK,        amount0Desired: amount0ToMint,        amount1Desired: amount1ToMint,        amount0Min:0,        amount1Min:0,        recipient:address(this),        deadline: block.timestamp});// Note that the pool defined by DAI/USDC and fee tier 0.3% must already be created and initialized in order to mint(tokenId, liquidity, amount0, amount1)= nonfungiblePositionManager.mint(params);
```

## Updating The Deposit Mapping And Refunding The Calling Address[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#updating-the-deposit-mapping-and-refunding-the-calling-address "Direct link to Updating The Deposit Mapping And Refunding The Calling Address")
Now we can call the internal function we previously wrote in [Setting Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up). After that, we can take any liquidity leftover from minting and refund it to `msg.sender`.
```
// Create a deposit_createDeposit(msg.sender, tokenId);// Remove allowance and refund in both assets.if(amount0 < amount0ToMint){      TransferHelper.safeApprove(DAI,address(nonfungiblePositionManager),0);uint256 refund0 = amount0ToMint - amount0;      TransferHelper.safeTransfer(DAI, msg.sender, refund0);}if(amount1 < amount1ToMint){      TransferHelper.safeApprove(USDC,address(nonfungiblePositionManager),0);uint256 refund1 = amount1ToMint - amount1;      TransferHelper.safeTransfer(USDC, msg.sender, refund1);}}
```

## The Full Example[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#the-full-example "Direct link to The Full Example")
```
/// @notice Calls the mint function defined in periphery, mints the same amount of each token. For this example we are providing 1000 DAI and 1000 USDC in liquidity/// @return tokenId The id of the newly minted ERC721/// @return liquidity The amount of liquidity for the position/// @return amount0 The amount of token0/// @return amount1 The amount of token1functionmintNewPosition()externalreturns(uint256 tokenId,uint128 liquidity,uint256 amount0,uint256 amount1){// For this example, we will provide equal amounts of liquidity in both assets.// Providing liquidity in both assets means liquidity will be earning fees and is considered in-range.uint256 amount0ToMint =1000;uint256 amount1ToMint =1000;// Approve the position manager    TransferHelper.safeApprove(DAI,address(nonfungiblePositionManager), amount0ToMint);    TransferHelper.safeApprove(USDC,address(nonfungiblePositionManager), amount1ToMint);    INonfungiblePositionManager.MintParams memory params =      INonfungiblePositionManager.MintParams({        token0: DAI,        token1: USDC,        fee: poolFee,        tickLower: TickMath.MIN_TICK,        tickUpper: TickMath.MAX_TICK,        amount0Desired: amount0ToMint,        amount1Desired: amount1ToMint,        amount0Min:0,        amount1Min:0,        recipient:address(this),        deadline: block.timestamp});// Note that the pool defined by DAI/USDC and fee tier 0.3% must already be created and initialized in order to mint(tokenId, liquidity, amount0, amount1)= nonfungiblePositionManager.mint(params);// Create a deposit_createDeposit(msg.sender, tokenId);// Remove allowance and refund in both assets.if(amount0 < amount0ToMint){      TransferHelper.safeApprove(DAI,address(nonfungiblePositionManager),0);uint256 refund0 = amount0ToMint - amount0;      TransferHelper.safeTransfer(DAI, msg.sender, refund0);}if(amount1 < amount1ToMint){      TransferHelper.safeApprove(USDC,address(nonfungiblePositionManager),0);uint256 refund1 = amount1ToMint - amount1;      TransferHelper.safeTransfer(USDC, msg.sender, refund1);}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/mint-a-new-position.md)
Was this helpful?
[PreviousSet Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)[NextCollecting Fees](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees)
On this page
  * [Input Parameters](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#input-parameters)
  * [Calling Mint](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#calling-mint)
  * [Updating The Deposit Mapping And Refunding The Calling Address](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#updating-the-deposit-mapping-and-refunding-the-calling-address)
  * [The Full Example](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position#the-full-example)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/mint-a-new-position.md)
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
