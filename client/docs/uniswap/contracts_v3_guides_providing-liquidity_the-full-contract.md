# https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
          * [Set Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
          * [Mint a New Position](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
          * [Collecting Fees](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees)
          * [Decrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
          * [Increase Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity)
          * [The Full Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
    * [Permit2](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Guides
  * Providing Liquidity
  * [The Full Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)


# The Full Contract
Below we have the complete functioning code example: a contract that can custody Uniswap V3 position NFT's and manipulate the positions and liquidity therein by collecting fees, increasing or decreasing liquidity, and minting new positions. View on github [here](https://github.com/Uniswap/uniswap-docs/blob/main/examples/smart-contracts/LiquidityExamples.sol).
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragmasolidity=0.7.6;pragma abicoder v2;import'@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol';import'@uniswap/v3-core/contracts/libraries/TickMath.sol';import'@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol';import'../libraries/TransferHelper.sol';import'../interfaces/INonfungiblePositionManager.sol';import'../base/LiquidityManagement.sol';contractLiquidityExamplesis IERC721Receiver {addresspublicconstant DAI =0x6B175474E89094C44Da98b954EedeAC495271d0F;addresspublicconstant USDC =0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;uint24publicconstant poolFee =3000;  INonfungiblePositionManager public immutable nonfungiblePositionManager;/// @notice Represents the deposit of an NFTstructDeposit{address owner;uint128 liquidity;address token0;address token1;}/// @dev deposits[tokenId] => Depositmapping(uint256=> Deposit)public deposits;constructor(    INonfungiblePositionManager _nonfungiblePositionManager){    nonfungiblePositionManager = _nonfungiblePositionManager;}// Implementing `onERC721Received` so this contract can receive custody of erc721 tokensfunctiononERC721Received(address operator,address,uint256 tokenId,bytescalldata)external override returns(bytes4){// get position information_createDeposit(operator, tokenId);returnthis.onERC721Received.selector;}function_createDeposit(address owner,uint256 tokenId)internal{(,,address token0,address token1,,,,uint128 liquidity,,,,)=      nonfungiblePositionManager.positions(tokenId);// set the owner and data for position// operator is msg.sender    deposits[tokenId]=Deposit({owner: owner, liquidity: liquidity, token0: token0, token1: token1});}/// @notice Calls the mint function defined in periphery, mints the same amount of each token./// For this example we are providing 1000 DAI and 1000 USDC in liquidity/// @return tokenId The id of the newly minted ERC721/// @return liquidity The amount of liquidity for the position/// @return amount0 The amount of token0/// @return amount1 The amount of token1functionmintNewPosition()externalreturns(uint256 tokenId,uint128 liquidity,uint256 amount0,uint256 amount1){// For this example, we will provide equal amounts of liquidity in both assets.// Providing liquidity in both assets means liquidity will be earning fees and is considered in-range.uint256 amount0ToMint =1000;uint256 amount1ToMint =1000;// transfer tokens to contract    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amount0ToMint);    TransferHelper.safeTransferFrom(USDC, msg.sender,address(this), amount1ToMint);// Approve the position manager    TransferHelper.safeApprove(DAI,address(nonfungiblePositionManager), amount0ToMint);    TransferHelper.safeApprove(USDC,address(nonfungiblePositionManager), amount1ToMint);    INonfungiblePositionManager.MintParams memory params =      INonfungiblePositionManager.MintParams({        token0: DAI,        token1: USDC,        fee: poolFee,        tickLower: TickMath.MIN_TICK,        tickUpper: TickMath.MAX_TICK,        amount0Desired: amount0ToMint,        amount1Desired: amount1ToMint,        amount0Min:0,        amount1Min:0,        recipient:address(this),        deadline: block.timestamp});// Note that the pool defined by DAI/USDC and fee tier 0.3% must already be created and initialized in order to mint(tokenId, liquidity, amount0, amount1)= nonfungiblePositionManager.mint(params);// Create a deposit_createDeposit(msg.sender, tokenId);// Remove allowance and refund in both assets.if(amount0 < amount0ToMint){      TransferHelper.safeApprove(DAI,address(nonfungiblePositionManager),0);uint256 refund0 = amount0ToMint - amount0;      TransferHelper.safeTransfer(DAI, msg.sender, refund0);}if(amount1 < amount1ToMint){      TransferHelper.safeApprove(USDC,address(nonfungiblePositionManager),0);uint256 refund1 = amount1ToMint - amount1;      TransferHelper.safeTransfer(USDC, msg.sender, refund1);}}/// @notice Collects the fees associated with provided liquidity/// @dev The contract must hold the erc721 token before it can collect fees/// @param tokenId The id of the erc721 token/// @return amount0 The amount of fees collected in token0/// @return amount1 The amount of fees collected in token1functioncollectAllFees(uint256 tokenId)externalreturns(uint256 amount0,uint256 amount1){// Caller must own the ERC721 position, meaning it must be a deposit// set amount0Max and amount1Max to uint256.max to collect all fees// alternatively can set recipient to msg.sender and avoid another transaction in `sendToOwner`    INonfungiblePositionManager.CollectParams memory params =      INonfungiblePositionManager.CollectParams({        tokenId: tokenId,        recipient:address(this),        amount0Max:type(uint128).max,        amount1Max:type(uint128).max});(amount0, amount1)= nonfungiblePositionManager.collect(params);// send collected feed back to owner_sendToOwner(tokenId, amount0, amount1);}/// @notice A function that decreases the current liquidity by half. An example to show how to call the `decreaseLiquidity` function defined in periphery./// @param tokenId The id of the erc721 token/// @return amount0 The amount received back in token0/// @return amount1 The amount returned back in token1functiondecreaseLiquidityInHalf(uint256 tokenId)externalreturns(uint256 amount0,uint256 amount1){// caller must be the owner of the NFTrequire(msg.sender == deposits[tokenId].owner,'Not the owner');// get liquidity data for tokenIduint128 liquidity = deposits[tokenId].liquidity;uint128 halfLiquidity = liquidity /2;// amount0Min and amount1Min are price slippage checks// if the amount received after burning is not greater than these minimums, transaction will fail    INonfungiblePositionManager.DecreaseLiquidityParams memory params =      INonfungiblePositionManager.DecreaseLiquidityParams({        tokenId: tokenId,        liquidity: halfLiquidity,        amount0Min:0,        amount1Min:0,        deadline: block.timestamp});(amount0, amount1)= nonfungiblePositionManager.decreaseLiquidity(params);//send liquidity back to owner_sendToOwner(tokenId, amount0, amount1);}/// @notice Increases liquidity in the current range/// @dev Pool must be initialized already to add liquidity/// @param tokenId The id of the erc721 token/// @param amount0 The amount to add of token0/// @param amount1 The amount to add of token1functionincreaseLiquidityCurrentRange(uint256 tokenId,uint256 amountAdd0,uint256 amountAdd1)externalreturns(uint128 liquidity,uint256 amount0,uint256 amount1){    TransferHelper.safeTransferFrom(deposits[tokenId].token0, msg.sender,address(this), amountAdd0);    TransferHelper.safeTransferFrom(deposits[tokenId].token1, msg.sender,address(this), amountAdd1);    TransferHelper.safeApprove(deposits[tokenId].token0,address(nonfungiblePositionManager), amountAdd0);    TransferHelper.safeApprove(deposits[tokenId].token1,address(nonfungiblePositionManager), amountAdd1);    INonfungiblePositionManager.IncreaseLiquidityParams memory params = INonfungiblePositionManager.IncreaseLiquidityParams({      tokenId: tokenId,      amount0Desired: amountAdd0,      amount1Desired: amountAdd1,      amount0Min:0,      amount1Min:0,      deadline: block.timestamp});(liquidity, amount0, amount1)= nonfungiblePositionManager.increaseLiquidity(params);}/// @notice Transfers funds to owner of NFT/// @param tokenId The id of the erc721/// @param amount0 The amount of token0/// @param amount1 The amount of token1function_sendToOwner(uint256 tokenId,uint256 amount0,uint256 amount1)internal{// get owner of contractaddress owner = deposits[tokenId].owner;address token0 = deposits[tokenId].token0;address token1 = deposits[tokenId].token1;// send collected fees to owner    TransferHelper.safeTransfer(token0, owner, amount0);    TransferHelper.safeTransfer(token1, owner, amount1);}/// @notice Transfers the NFT to the owner/// @param tokenId The id of the erc721functionretrieveNFT(uint256 tokenId)external{// must be the owner of the NFTrequire(msg.sender == deposits[tokenId].owner,'Not the owner');// transfer ownership to original owner    nonfungiblePositionManager.safeTransferFrom(address(this), msg.sender, tokenId);//remove information related to tokenIddelete deposits[tokenId];}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/the-full-contract.md)
Was this helpful?
[PreviousIncrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/increase-liquidity)[NextOverview](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/the-full-contract.md)
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
