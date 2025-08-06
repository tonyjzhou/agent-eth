# https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#__docusaurus_skipToContent_fallback)
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
          * [Single Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)
          * [Multihop Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
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
  * Implement A Swap
  * [Single Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)


On this page
# Single Swaps
Swaps are the most common interaction with the Uniswap protocol. The following example shows you how to implement a single-path swap contract that uses two functions that you create:
  * `swapExactInputSingle`
  * `swapExactOutputSingle`


The `swapExactInputSingle` function is for performing _exact input_ swaps, which swap a fixed amount of one token for a maximum possible amount of another token. This function uses the `ExactInputSingleParams` struct and the `exactInputSingle` function from the [ISwapRouter](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/ISwapRouter) interface.
The `swapExactOutputSingle` function is for performing _exact output_ swaps, which swap a minimum possible amount of one token for a fixed amount of another token. This function uses the `ExactOutputSingleParams` struct and the `exactOutputSingle` function from the [ISwapRouter](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/ISwapRouter) interface.
For simplification, the example hardcodes the token contract addresses, but as explained further below the contract could be modified to change pools and tokens on a per transaction basis.
When trading from a smart contract, the most important thing to keep in mind is that access to an external price source is required. Without this, trades can be frontrun for considerable loss.
**Note:** The swap examples are not production ready code, and are implemented in a simplistic manner for the purpose of learning.
## Set Up the Contract[​](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#set-up-the-contract "Direct link to Set Up the Contract")
Declare the solidity version used to compile the contract, and `abicoder v2` to allow arbitrary nested arrays and structs to be encoded and decoded in calldata, a feature used when executing a swap.
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragmasolidity=0.7.6;pragma abicoder v2;
```

Import the two relevant contracts from the npm package installation
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

## Exact Input Swaps[​](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#exact-input-swaps "Direct link to Exact Input Swaps")
The caller must `approve` the contract to withdraw the tokens from the calling address's account to execute a swap. Remember that because our contract is a contract itself and not an extension of the caller (us); we must also approve the Uniswap protocol router contract to use the tokens that our contract will be in possession of after they have been withdrawn from the calling address (us).
Then, transfer the `amount` of Dai from the calling address into our contract, and use `amount` as the value passed to the second `approve`.
```
/// @notice swapExactInputSingle swaps a fixed amount of DAI for a maximum possible amount of WETH9/// using the DAI/WETH9 0.3% pool by calling `exactInputSingle` in the swap router./// @dev The calling address must approve this contract to spend at least `amountIn` worth of its DAI for this function to succeed./// @param amountIn The exact amount of DAI that will be swapped for WETH9./// @return amountOut The amount of WETH9 received.functionswapExactInputSingle(uint256 amountIn)externalreturns(uint256 amountOut){// msg.sender must approve this contract// Transfer the specified amount of DAI to this contract.    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amountIn);// Approve the router to spend DAI.    TransferHelper.safeApprove(DAI,address(swapRouter), amountIn);
```

### Swap Input Parameters[​](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#swap-input-parameters "Direct link to Swap Input Parameters")
To execute the swap function, we need to populate the `ExactInputSingleParams` with the necessary swap data. These parameters are found in the smart contract interfaces, which can be browsed [here](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/ISwapRouter).
A brief overview of the parameters:
  * `tokenIn` The contract address of the inbound token
  * `tokenOut` The contract address of the outbound token
  * `fee` The fee tier of the pool, used to determine the correct pool contract in which to execute the swap
  * `recipient` the destination address of the outbound token
  * `deadline`: the unix time after which a swap will fail, to protect against long-pending transactions and wild swings in prices
  * `amountOutMinimum`: we are setting to zero, but this is a significant risk in production. For a real deployment, this value should be calculated using our SDK or an onchain price oracle - this helps protect against getting an unusually bad price for a trade due to a front running sandwich or another type of price manipulation
  * `sqrtPriceLimitX96`: We set this to zero - which makes this parameter inactive. In production, this value can be used to set the limit for the price the swap will push the pool to, which can help protect against price impact or for setting up logic in a variety of price-relevant mechanisms.


### Call the function[​](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#call-the-function "Direct link to Call the function")
```
// Naively set amountOutMinimum to 0. In production, use an oracle or other data source to choose a safer value for amountOutMinimum.// We also set the sqrtPriceLimitx96 to be 0 to ensure we swap our exact input amount.    ISwapRouter.ExactInputSingleParams memory params =      ISwapRouter.ExactInputSingleParams({        tokenIn: DAI,        tokenOut: WETH9,        fee: poolFee,        recipient: msg.sender,        deadline: block.timestamp,        amountIn: amountIn,        amountOutMinimum:0,        sqrtPriceLimitX96:0});// The call to `exactInputSingle` executes the swap.    amountOut = swapRouter.exactInputSingle(params);}
```

## Exact Output Swaps[​](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#exact-output-swaps "Direct link to Exact Output Swaps")
Exact Output swaps a minimum possible amount of the input token for a fixed amount of the outbound token. This is the less common swap style - but useful in a variety of circumstances.
Because this example transfers in the inbound asset in anticipation of the swap - its possible that some of the inbound token will be left over after the swap is executed, which is why we pay it back to the calling address at the end of the swap.
### Call the function[​](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#call-the-function-1 "Direct link to Call the function")
```
/// @notice swapExactOutputSingle swaps a minimum possible amount of DAI for a fixed amount of WETH./// @dev The calling address must approve this contract to spend its DAI for this function to succeed. As the amount of input DAI is variable,/// the calling address will need to approve for a slightly higher amount, anticipating some variance./// @param amountOut The exact amount of WETH9 to receive from the swap./// @param amountInMaximum The amount of DAI we are willing to spend to receive the specified amount of WETH9./// @return amountIn The amount of DAI actually spent in the swap.functionswapExactOutputSingle(uint256 amountOut,uint256 amountInMaximum)externalreturns(uint256 amountIn){// Transfer the specified amount of DAI to this contract.    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amountInMaximum);// Approve the router to spend the specified `amountInMaximum` of DAI.// In production, you should choose the maximum amount to spend based on oracles or other data sources to achieve a better swap.    TransferHelper.safeApprove(DAI,address(swapRouter), amountInMaximum);    ISwapRouter.ExactOutputSingleParams memory params =      ISwapRouter.ExactOutputSingleParams({        tokenIn: DAI,        tokenOut: WETH9,        fee: poolFee,        recipient: msg.sender,        deadline: block.timestamp,        amountOut: amountOut,        amountInMaximum: amountInMaximum,        sqrtPriceLimitX96:0});// Executes the swap returning the amountIn needed to spend to receive the desired amountOut.    amountIn = swapRouter.exactOutputSingle(params);// For exact output swaps, the amountInMaximum may not have all been spent.// If the actual amount spent (amountIn) is less than the specified maximum amount, we must refund the msg.sender and approve the swapRouter to spend 0.if(amountIn < amountInMaximum){      TransferHelper.safeApprove(DAI,address(swapRouter),0);      TransferHelper.safeTransfer(DAI, msg.sender, amountInMaximum - amountIn);}}
```

## A Complete Single Swap Contract[​](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#a-complete-single-swap-contract "Direct link to A Complete Single Swap Contract")
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragmasolidity=0.7.6;pragma abicoder v2;import'@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';import'@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol';contractSwapExamples{// For the scope of these swap examples,// we will detail the design considerations when using// `exactInput`, `exactInputSingle`, `exactOutput`, and `exactOutputSingle`.// It should be noted that for the sake of these examples, we purposefully pass in the swap router instead of inherit the swap router for simplicity.// More advanced example contracts will detail how to inherit the swap router safely.  ISwapRouter public immutable swapRouter;// This example swaps DAI/WETH9 for single path swaps and DAI/USDC/WETH9 for multi path swaps.addresspublicconstant DAI =0x6B175474E89094C44Da98b954EedeAC495271d0F;addresspublicconstant WETH9 =0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;addresspublicconstant USDC =0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;// For this example, we will set the pool fee to 0.3%.uint24publicconstant poolFee =3000;constructor(ISwapRouter _swapRouter){    swapRouter = _swapRouter;}/// @notice swapExactInputSingle swaps a fixed amount of DAI for a maximum possible amount of WETH9/// using the DAI/WETH9 0.3% pool by calling `exactInputSingle` in the swap router./// @dev The calling address must approve this contract to spend at least `amountIn` worth of its DAI for this function to succeed./// @param amountIn The exact amount of DAI that will be swapped for WETH9./// @return amountOut The amount of WETH9 received.functionswapExactInputSingle(uint256 amountIn)externalreturns(uint256 amountOut){// msg.sender must approve this contract// Transfer the specified amount of DAI to this contract.    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amountIn);// Approve the router to spend DAI.    TransferHelper.safeApprove(DAI,address(swapRouter), amountIn);// Naively set amountOutMinimum to 0. In production, use an oracle or other data source to choose a safer value for amountOutMinimum.// We also set the sqrtPriceLimitx96 to be 0 to ensure we swap our exact input amount.    ISwapRouter.ExactInputSingleParams memory params =      ISwapRouter.ExactInputSingleParams({        tokenIn: DAI,        tokenOut: WETH9,        fee: poolFee,        recipient: msg.sender,        deadline: block.timestamp,        amountIn: amountIn,        amountOutMinimum:0,        sqrtPriceLimitX96:0});// The call to `exactInputSingle` executes the swap.    amountOut = swapRouter.exactInputSingle(params);}/// @notice swapExactOutputSingle swaps a minimum possible amount of DAI for a fixed amount of WETH./// @dev The calling address must approve this contract to spend its DAI for this function to succeed. As the amount of input DAI is variable,/// the calling address will need to approve for a slightly higher amount, anticipating some variance./// @param amountOut The exact amount of WETH9 to receive from the swap./// @param amountInMaximum The amount of DAI we are willing to spend to receive the specified amount of WETH9./// @return amountIn The amount of DAI actually spent in the swap.functionswapExactOutputSingle(uint256 amountOut,uint256 amountInMaximum)externalreturns(uint256 amountIn){// Transfer the specified amount of DAI to this contract.    TransferHelper.safeTransferFrom(DAI, msg.sender,address(this), amountInMaximum);// Approve the router to spend the specifed `amountInMaximum` of DAI.// In production, you should choose the maximum amount to spend based on oracles or other data sources to acheive a better swap.    TransferHelper.safeApprove(DAI,address(swapRouter), amountInMaximum);    ISwapRouter.ExactOutputSingleParams memory params =      ISwapRouter.ExactOutputSingleParams({        tokenIn: DAI,        tokenOut: WETH9,        fee: poolFee,        recipient: msg.sender,        deadline: block.timestamp,        amountOut: amountOut,        amountInMaximum: amountInMaximum,        sqrtPriceLimitX96:0});// Executes the swap returning the amountIn needed to spend to receive the desired amountOut.    amountIn = swapRouter.exactOutputSingle(params);// For exact output swaps, the amountInMaximum may not have all been spent.// If the actual amount spent (amountIn) is less than the specified maximum amount, we must refund the msg.sender and approve the swapRouter to spend 0.if(amountIn < amountInMaximum){      TransferHelper.safeApprove(DAI,address(swapRouter),0);      TransferHelper.safeTransfer(DAI, msg.sender, amountInMaximum - amountIn);}}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/swaps/single-swaps.md)
Was this helpful?
[PreviousSet Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)[NextMultihop Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps)
On this page
  * [Set Up the Contract](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#set-up-the-contract)
  * [Exact Input Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#exact-input-swaps)
    * [Swap Input Parameters](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#swap-input-parameters)
    * [Call the function](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#call-the-function)
  * [Exact Output Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#exact-output-swaps)
    * [Call the function](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#call-the-function-1)
  * [A Complete Single Swap Contract](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps#a-complete-single-swap-contract)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/swaps/single-swaps.md)
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
