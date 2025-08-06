# https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees#__docusaurus_skipToContent_fallback)
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
  * [Collecting Fees](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees)


On this page
# Collecting Fees
## Collect Fees[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees#collect-fees "Direct link to Collect Fees")
  * Make sure to go through the [first guide](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up) before continuing to this section.
  * For each of these liquidity interaction examples, our contract must be in possession of the liquidity position NFT. Therefore, in any example where the NFT deposit is not coded into a function, the contract is assumed to already be in possession of it.


To collect the fees of an owner position, transfer the NFT from the calling address, assign the relevant variables from the NFT to local variables within our function, and pass those variables to the`nonfungiblePositionManager` to call `collect`.
This function collects all fees, sending them to the original owner of the NFT, while maintaining custody of the position NFT.
```
/// @notice Collects the fees associated with provided liquidity/// @dev The contract must hold the erc721 token before it can collect fees/// @param tokenId The id of the erc721 token/// @return amount0 The amount of fees collected in token0/// @return amount1 The amount of fees collected in token1functioncollectAllFees(uint256 tokenId)externalreturns(uint256 amount0,uint256 amount1){// Caller must own the ERC721 position// Call to safeTransfer will trigger `onERC721Received` which must return the selector else transfer will fail    nonfungiblePositionManager.safeTransferFrom(msg.sender,address(this), tokenId);// set amount0Max and amount1Max to uint256.max to collect all fees// alternatively can set recipient to msg.sender and avoid another transaction in `sendToOwner`    INonfungiblePositionManager.CollectParams memory params =      INonfungiblePositionManager.CollectParams({        tokenId: tokenId,        recipient:address(this),        amount0Max:type(uint128).max,        amount1Max:type(uint128).max});(amount0, amount1)= nonfungiblePositionManager.collect(params);// send collected feed back to owner_sendToOwner(tokenId, amount0, amount1);}
```

## Sending Fees To The Calling Address[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees#sending-fees-to-the-calling-address "Direct link to Sending Fees To The Calling Address")
This internal helper function sends any tokens, in the form of fees or position tokens, to the owner of an NFT.
In `_sendToOwner`, we pass the amount of fees due, previously populated in the last function, as arguments to `safeTransfer`, which transfers the fees to `owner`.
```
/// @notice Transfers funds to owner of NFT/// @param tokenId The id of the erc721/// @param amount0 The amount of token0/// @param amount1 The amount of token1function_sendToOwner(uint256 tokenId,uint256 amount0,uint256 amount1)internal{// get owner of contractaddress owner = deposits[tokenId].owner;address token0 = deposits[tokenId].token0;address token1 = deposits[tokenId].token1;// send collected fees to owner    TransferHelper.safeTransfer(token0, owner, amount0);    TransferHelper.safeTransfer(token1, owner, amount1);}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/collect-fees.md)
Was this helpful?
[PreviousMint a New Position](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)[NextDecrease Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/decrease-liquidity)
On this page
  * [Collect Fees](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees#collect-fees)
  * [Sending Fees To The Calling Address](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/collect-fees#sending-fees-to-the-calling-address)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/collect-fees.md)
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
