# https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
          * [Single Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)
          * [Multihop Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
    * [Permit2](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Guides
  * Implement A Swap
  * [Multihop Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)


On this page
# Multihop Swaps
## Introduction[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#introduction "Direct link to Introduction")
The examples below are implementations of the two styles of multi-hop swapping available on v3. The examples below are not production ready code, and are implemented in a simplistic manner for the purpose of learning.
## Setting up the Contract[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#setting-up-the-contract "Direct link to Setting up the Contract")
Declare the solidity version that will be used to compile the contract, and the `abicoder v2` to allow arbitrary nested arrays and structs to be encoded and decoded in calldata, a feature we use when executing a swap.
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragmasolidity=0.7.6;pragma abicoder v2;
```

Import the two needed contracts from the npm package installation.
```
import'@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol';import'@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';
```

Create a contract called `SwapExamples`, and declare an immutable public variable `swapRouter` of type `ISwapRouter`. This allows us to call functions in the `ISwapRouter` interface.
```
contractSwapExamples{// For the scope of these swap examples,// we will detail the design considerations when using `exactInput`, `exactInputSingle`, `exactOutput`, and `exactOutputSingle`.// It should be noted that for the sake of these examples we pass in the swap router as a constructor argument instead of inheriting it.// More advanced example contracts will detail how to inherit the swap router safely.// This example swaps DAI/WETH9 for single path swaps and DAI/USDC/WETH9 for multi path swaps.  ISwapRouter public immutable swapRouter;
```

Hardcode the token contract addresses and pool fee tiers for the example. In production, you would likely use an input parameter for this and pass the input into a memory variable, allowing the contract to change the pools and tokens it interacts with on a per transaction basis, but for conceptual simplicity, we are hardcoding them here.
```
addresspublicconstant DAI =0x6B175474E89094C44Da98b954EedeAC495271d0F;addresspublicconstant WETH9 =0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;addresspublicconstant USDC =0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;// For this example, we will set the pool fee to 0.3%.uint24publicconstant poolFee =3000;constructor(ISwapRouter _swapRouter){    swapRouter = _swapRouter;}
```

## Exact Input Multi Hop Swaps[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#exact-input-multi-hop-swaps "Direct link to Exact Input Multi Hop Swaps")
Exact input multi hop swaps will swap a fixed amount on a given input token for the maximum amount possible for a given output, and can include an arbitrary number of intermediary swaps.
### Input Parameters[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#input-parameters "Direct link to Input Parameters")
  * `path`: The path is a sequence of (`tokenAddress` - `fee` - `tokenAddress`), which are the variables needed to compute each pool contract address in our sequence of swaps. The multihop swap router code will automatically find the correct pool with these variables, and execute the swap needed within each pool in our sequence.
  * `recipient`: the destination address of the outbound asset.
  * `deadline`: the unix time after which a transaction will be reverted, to protect against long delays and the increased chance of large price swings therein.
  * `amountIn`: the amount of the inbound asset
  * `amountOutMin`: the minimum amount of the outbound asset, less than which will cause the transaction to revert. For the sake of this example we will set it to `0`, in production one will need to use the SDK to quote an expected price, or an on chain price oracle for more advanced manipulation resistant systems.


### Calling the function[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#calling-the-function "Direct link to Calling the function")
```
/// @notice swapExactInputMultihop swaps a fixed amount of DAI for a maximum possible amount of WETH9 through an intermediary pool./// For this example, we will swap DAI to USDC, then USDC to WETH9 to achieve our desired output./// @dev The calling address must approve this contract to spend at least `amountIn` worth of its DAI for this function to succeed./// @param amountIn The amount of DAI to be swapped./// @return amountOut The amount of WETH9 received after the swap.functionswapExactInputMultihop(uint256 amountIn)externalreturns(uint256 amountOut){// Transfer `amountIn` of DAI to this contract.    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amountIn);// Approve the router to spend DAI.    TransferHelper.safeApprove(DAI,address(swapRouter), amountIn);// Multiple pool swaps are encoded through bytes called a `path`. A path is a sequence of token addresses and poolFees that define the pools used in the swaps.// The format for pool encoding is (tokenIn, fee, tokenOut/tokenIn, fee, tokenOut) where tokenIn/tokenOut parameter is the shared token across the pools.// Since we are swapping DAI to USDC and then USDC to WETH9 the path encoding is (DAI, 0.3%, USDC, 0.3%, WETH9).    ISwapRouter.ExactInputParams memory params =      ISwapRouter.ExactInputParams({        path: abi.encodePacked(DAI, poolFee, USDC, poolFee, WETH9),        recipient: msg.sender,        deadline: block.timestamp,        amountIn: amountIn,        amountOutMinimum:0});// Executes the swap.    amountOut = swapRouter.exactInput(params);}
```

## Exact Output Multihop Swap[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#exact-output-multihop-swap "Direct link to Exact Output Multihop Swap")
An exact output swap will swap a variable amount of the input token for a fixed amount of the outbound token. This is the less common technique for multihop swaps. The code for swapping is largely the same except for one notable difference, the `Path` is encoded backwards, as an exact output swap is executed in reverse order to pass down the necessary variables for the chain of transactions
### Input Parameters[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#input-parameters-1 "Direct link to Input Parameters")
  * `path`: The path is a sequence of `tokenAddress` `Fee` `tokenAddress`, _encoded in reverse order_ , which are the variables needed to compute each pool contract address in our sequence of swaps. The multihop swap router code will automatically find the correct pool with these variables, and execute the swap needed within each pool in our sequence.
  * `recipient`: the destination address of the outbound asset.
  * `deadline`: the unix time after which a transaction will be reverted, to protect against long delays and the increased chance of large price swings therein.
  * `amountOut`: The desired amount of WETH9.
  * `amountInMaximum`: The maximum amount of DAI willing to be swapped for the specified amountOut of WETH9.


### Calling the function[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#calling-the-function-1 "Direct link to Calling the function")
```
/// @notice swapExactOutputMultihop swaps a minimum possible amount of DAI for a fixed amount of WETH through an intermediary pool./// For this example, we want to swap DAI for WETH9 through a USDC pool but we specify the desired amountOut of WETH9. Notice how the path encoding is slightly different in for exact output swaps./// @dev The calling address must approve this contract to spend its DAI for this function to succeed. As the amount of input DAI is variable,/// the calling address will need to approve for a slightly higher amount, anticipating some variance./// @param amountOut The desired amount of WETH9./// @param amountInMaximum The maximum amount of DAI willing to be swapped for the specified amountOut of WETH9./// @return amountIn The amountIn of DAI actually spent to receive the desired amountOut.functionswapExactOutputMultihop(uint256 amountOut,uint256 amountInMaximum)externalreturns(uint256 amountIn){// Transfer the specified `amountInMaximum` to this contract.    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amountInMaximum);// Approve the router to spend `amountInMaximum`.    TransferHelper.safeApprove(DAI,address(swapRouter), amountInMaximum);// The parameter path is encoded as (tokenOut, fee, tokenIn/tokenOut, fee, tokenIn)// The tokenIn/tokenOut field is the shared token between the two pools used in the multiple pool swap. In this case USDC is the "shared" token.// For an exactOutput swap, the first swap that occurs is the swap which returns the eventual desired token.// In this case, our desired output token is WETH9 so that swap happens first, and is encoded in the path accordingly.    ISwapRouter.ExactOutputParams memory params =      ISwapRouter.ExactOutputParams({        path: abi.encodePacked(WETH9, poolFee, USDC, poolFee, DAI),        recipient: msg.sender,        deadline: block.timestamp,        amountOut: amountOut,        amountInMaximum: amountInMaximum});// Executes the swap, returning the amountIn actually spent.    amountIn = swapRouter.exactOutput(params);// If the swap did not require the full amountInMaximum to achieve the exact amountOut then we refund msg.sender and approve the router to spend 0.if(amountIn < amountInMaximum){      TransferHelper.safeApprove(DAI,address(swapRouter),0);      TransferHelper.safeTransferFrom(DAI,address(this), msg.sender, amountInMaximum - amountIn);}}
```

## The Full Contract[​](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#the-full-contract "Direct link to The Full Contract")
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragmasolidity=0.7.6;pragma abicoder v2;import'@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';import'@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol';contractSwapExamples{// For the scope of these swap examples,// we will detail the design considerations when using// `exactInput`, `exactInputSingle`, `exactOutput`, and `exactOutputSingle`.// It should be noted that for the sake of these examples, we purposefully pass in the swap router instead of inherit the swap router for simplicity.// More advanced example contracts will detail how to inherit the swap router safely.  ISwapRouter public immutable swapRouter;// This example swaps DAI/WETH9 for single path swaps and DAI/USDC/WETH9 for multi path swaps.addresspublicconstant DAI =0x6B175474E89094C44Da98b954EedeAC495271d0F;addresspublicconstant WETH9 =0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;addresspublicconstant USDC =0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;// For this example, we will set the pool fee to 0.3%.uint24publicconstant poolFee =3000;constructor(ISwapRouter _swapRouter){    swapRouter = _swapRouter;}/// @notice swapInputMultiplePools swaps a fixed amount of DAI for a maximum possible amount of WETH9 through an intermediary pool./// For this example, we will swap DAI to USDC, then USDC to WETH9 to achieve our desired output./// @dev The calling address must approve this contract to spend at least `amountIn` worth of its DAI for this function to succeed./// @param amountIn The amount of DAI to be swapped./// @return amountOut The amount of WETH9 received after the swap.functionswapExactInputMultihop(uint256 amountIn)externalreturns(uint256 amountOut){// Transfer `amountIn` of DAI to this contract.    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amountIn);// Approve the router to spend DAI.    TransferHelper.safeApprove(DAI,address(swapRouter), amountIn);// Multiple pool swaps are encoded through bytes called a `path`. A path is a sequence of token addresses and poolFees that define the pools used in the swaps.// The format for pool encoding is (tokenIn, fee, tokenOut/tokenIn, fee, tokenOut) where tokenIn/tokenOut parameter is the shared token across the pools.// Since we are swapping DAI to USDC and then USDC to WETH9 the path encoding is (DAI, 0.3%, USDC, 0.3%, WETH9).    ISwapRouter.ExactInputParams memory params =      ISwapRouter.ExactInputParams({        path: abi.encodePacked(DAI, poolFee, USDC, poolFee, WETH9),        recipient: msg.sender,        deadline: block.timestamp,        amountIn: amountIn,        amountOutMinimum:0});// Executes the swap.    amountOut = swapRouter.exactInput(params);}/// @notice swapExactOutputMultihop swaps a minimum possible amount of DAI for a fixed amount of WETH through an intermediary pool./// For this example, we want to swap DAI for WETH9 through a USDC pool but we specify the desired amountOut of WETH9. Notice how the path encoding is slightly different in for exact output swaps./// @dev The calling address must approve this contract to spend its DAI for this function to succeed. As the amount of input DAI is variable,/// the calling address will need to approve for a slightly higher amount, anticipating some variance./// @param amountOut The desired amount of WETH9./// @param amountInMaximum The maximum amount of DAI willing to be swapped for the specified amountOut of WETH9./// @return amountIn The amountIn of DAI actually spent to receive the desired amountOut.functionswapExactOutputMultihop(uint256 amountOut,uint256 amountInMaximum)externalreturns(uint256 amountIn){// Transfer the specified `amountInMaximum` to this contract.    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amountInMaximum);// Approve the router to spend `amountInMaximum`.    TransferHelper.safeApprove(DAI,address(swapRouter), amountInMaximum);// The parameter path is encoded as (tokenOut, fee, tokenIn/tokenOut, fee, tokenIn)// The tokenIn/tokenOut field is the shared token between the two pools used in the multiple pool swap. In this case USDC is the "shared" token.// For an exactOutput swap, the first swap that occurs is the swap which returns the eventual desired token.// In this case, our desired output token is WETH9 so that swap happpens first, and is encoded in the path accordingly.    ISwapRouter.ExactOutputParams memory params =      ISwapRouter.ExactOutputParams({        path: abi.encodePacked(WETH9, poolFee, USDC, poolFee, DAI),        recipient: msg.sender,        deadline: block.timestamp,        amountOut: amountOut,        amountInMaximum: amountInMaximum});// Executes the swap, returning the amountIn actually spent.    amountIn = swapRouter.exactOutput(params);// If the swap did not require the full amountInMaximum to achieve the exact amountOut then we refund msg.sender and approve the router to spend 0.if(amountIn < amountInMaximum){      TransferHelper.safeApprove(DAI,address(swapRouter),0);      TransferHelper.safeTransferFrom(DAI,address(this), msg.sender, amountInMaximum - amountIn);}}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/swaps/multihop-swaps.md)
Was this helpful?
[PreviousSingle Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)[NextSet Up Your Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
On this page
  * [Introduction](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#introduction)
  * [Setting up the Contract](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#setting-up-the-contract)
  * [Exact Input Multi Hop Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#exact-input-multi-hop-swaps)
    * [Input Parameters](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#input-parameters)
    * [Calling the function](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#calling-the-function)
  * [Exact Output Multihop Swap](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#exact-output-multihop-swap)
    * [Input Parameters](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#input-parameters-1)
    * [Calling the function](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#calling-the-function-1)
  * [The Full Contract](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps#the-full-contract)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/swaps/multihop-swaps.md)
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
