# https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#__docusaurus_skipToContent_fallback)
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
  * [Set Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)


On this page
# Set Up Your Contract
## Setting up the Contract[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#setting-up-the-contract "Direct link to Setting up the Contract")
This guide is an example of a custodial contract Uniswap V3 positions, which allows interaction with the Uniswap V3 Periphery by minting a position, adding liquidity to a position, decreasing liquidity, and collecting fees.
First, declare the solidity version used to compile the contract and `abicoder v2` to allow arbitrary nested arrays and structs to be encoded and decoded in calldata, a feature we use when transacting with a pool.
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragmasolidity=0.7.6;pragma abicoder v2;
```

Import the contracts needed from the npm package installation.
```
import'@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol';import'@uniswap/v3-core/contracts/libraries/TickMath.sol';import'@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol';import'@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol';import'@uniswap/v3-periphery/contracts/interfaces/INonfungiblePositionManager.sol';import'@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';import'@uniswap/v3-periphery/contracts/base/LiquidityManagement.sol';
```

Create a contract called `LiquidityExamples` and inherit both `IERC721Receiver` and `LiquidityManagement`.
We've chosen to hardcode the token contract addresses and pool fee tiers for our example. In production, you would likely use an input parameter for this, allowing you to change the pools and tokens you are interacting with on a per transaction basis.
```
contractLiquidityExamplesis IERC721Receiver {addresspublicconstant DAI =0x6B175474E89094C44Da98b954EedeAC495271d0F;addresspublicconstant USDC =0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;uint24publicconstant poolFee =3000;
```

Declare an immutable public variable `nonfungiblePositionManager` of type `INonfungiblePositionManager`.
```
  INonfungiblePositionManager public immutable nonfungiblePositionManager;
```

## Allowing ERC721 Interactions[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#allowing-erc721-interactions "Direct link to Allowing ERC721 Interactions")
Every [NFT](https://ethereum.org/en/nft/) is identified by a unique uint256 ID inside the ERC-721 smart contract, declared as the `tokenId`
To allow deposits of ERC721 expressions of liquidity, create a struct called `Deposit`, a mapping of `uint256` to the `Deposit` struct, then declare that mapping as a public variable `deposits`.
```
structDeposit{address owner;uint128 liquidity;address token0;address token1;}mapping(uint256=> Deposit)public deposits;
```

## The Constructor[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#the-constructor "Direct link to The Constructor")
Declare the constructor here, which is executed once when the contract is deployed. Our constructor hard codes the address of the nonfungible position manager interface, V3 router, and the periphery immutable state constructor, which requires the factory and the address of weth9 (the [ERC-20 wrapper](https://weth.io/) for ether).
```
constructor(    INonfungiblePositionManager _nonfungiblePositionManager,address _factory,address _WETH9)PeripheryImmutableState(_factory, _WETH9){    nonfungiblePositionManager = _nonfungiblePositionManager;}
```

## Allowing custody of ERC721 tokens[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#allowing-custody-of-erc721-tokens "Direct link to Allowing custody of ERC721 tokens")
To allow the contract to custody ERC721 tokens, implement the `onERC721Received` function within the inherited `IERC721Receiver.sol` [contract](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC721/IERC721Receiver.sol).
The `from` identifier may be omitted because it is not used.
```
functiononERC721Received(address operator,address,uint256 tokenId,bytescalldata)external override returns(bytes4){// get position information_createDeposit(operator, tokenId);returnthis.onERC721Received.selector;}
```

## Creating a Deposit[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#creating-a-deposit "Direct link to Creating a Deposit")
To add a `Deposit` instance to the `deposits` mapping, create an internal function called `_createDeposit` that destructures the `positions` struct returned by `positions` in `nonfungiblePositionManager.sol`. Pass the relevant variables `token0` `token1` and `liquidity` to the `deposits` mapping.
```
function_createDeposit(address owner,uint256 tokenId)internal{(,,address token0,address token1,,,,uint128 liquidity,,,,)=      nonfungiblePositionManager.positions(tokenId);// set the owner and data for position// operator is msg.sender    deposits[tokenId]=Deposit({owner: owner, liquidity: liquidity, token0: token0, token1: token1});}
```

## The Full Contract Setup[​](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#the-full-contract-setup "Direct link to The Full Contract Setup")
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragmasolidity=0.7.6;pragma abicoder v2;import'@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol';import'@uniswap/v3-core/contracts/libraries/TickMath.sol';import'@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol';import'../libraries/TransferHelper.sol';import'../interfaces/INonfungiblePositionManager.sol';import'../base/LiquidityManagement.sol';contractLiquidityExamplesis IERC721Receiver {addresspublicconstant DAI =0x6B175474E89094C44Da98b954EedeAC495271d0F;addresspublicconstant USDC =0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;uint24publicconstant poolFee =3000;  INonfungiblePositionManager public immutable nonfungiblePositionManager;/// @notice Represents the deposit of an NFTstructDeposit{address owner;uint128 liquidity;address token0;address token1;}/// @dev deposits[tokenId] => Depositmapping(uint256=> Deposit)public deposits;constructor(    INonfungiblePositionManager _nonfungiblePositionManager){    nonfungiblePositionManager = _nonfungiblePositionManager;}// Implementing `onERC721Received` so this contract can receive custody of erc721 tokensfunctiononERC721Received(address operator,address,uint256 tokenId,bytescalldata)external override returns(bytes4){// get position information_createDeposit(operator, tokenId);returnthis.onERC721Received.selector;}function_createDeposit(address owner,uint256 tokenId)internal{(,,address token0,address token1,,,,uint128 liquidity,,,,)=      nonfungiblePositionManager.positions(tokenId);// set the owner and data for position// operator is msg.sender    deposits[tokenId]=Deposit({owner: owner, liquidity: liquidity, token0: token0, token1: token1});}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/setting-up-your-contract.md)
Was this helpful?
[PreviousMultihop Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)[NextMint a New Position](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/mint-a-position)
On this page
  * [Setting up the Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#setting-up-the-contract)
  * [Allowing ERC721 Interactions](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#allowing-erc721-interactions)
  * [The Constructor](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#the-constructor)
  * [Allowing custody of ERC721 tokens](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#allowing-custody-of-erc721-tokens)
  * [Creating a Deposit](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#creating-a-deposit)
  * [The Full Contract Setup](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up#the-full-contract-setup)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/providing-liquidity/setting-up-your-contract.md)
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
