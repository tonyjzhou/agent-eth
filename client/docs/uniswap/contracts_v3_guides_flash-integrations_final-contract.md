# https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract#__docusaurus_skipToContent_fallback)
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
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
          * [Getting Started](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
          * [Calling Flash](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash)
          * [The Flash Callback](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
          * [The Final Contract](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract)
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
  * Implement Flash Swaps
  * [The Final Contract](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract)


On this page
# The Final Contract
## The Full Contract[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract#the-full-contract "Direct link to The Full Contract")
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragmasolidity=0.7.6;pragma abicoder v2;import'@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3FlashCallback.sol';import'@uniswap/v3-core/contracts/libraries/LowGasSafeMath.sol';import'@uniswap/v3-periphery/contracts/base/PeripheryPayments.sol';import'@uniswap/v3-periphery/contracts/base/PeripheryImmutableState.sol';import'@uniswap/v3-periphery/contracts/libraries/PoolAddress.sol';import'@uniswap/v3-periphery/contracts/libraries/CallbackValidation.sol';import'@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';import'@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol';/// @title Flash contract implementation/// @notice An example contract using the Uniswap V3 flash functioncontractPairFlashis IUniswapV3FlashCallback, PeripheryImmutableState, PeripheryPayments {usingLowGasSafeMathforuint256;usingLowGasSafeMathforint256;  ISwapRouter public immutable swapRouter;constructor(    ISwapRouter _swapRouter,address _factory,address _WETH9)PeripheryImmutableState(_factory, _WETH9){    swapRouter = _swapRouter;}/// @param fee0 The fee from calling flash for token0/// @param fee1 The fee from calling flash for token1/// @param data The data needed in the callback passed as FlashCallbackData from `initFlash`/// @notice implements the callback called from flash/// @dev fails if the flash is not profitable, meaning the amountOut from the flash is less than the amount borrowedfunctionuniswapV3FlashCallback(uint256 fee0,uint256 fee1,bytescalldata data)external override {    FlashCallbackData memory decoded = abi.decode(data,(FlashCallbackData));    CallbackValidation.verifyCallback(factory, decoded.poolKey);address token0 = decoded.poolKey.token0;address token1 = decoded.poolKey.token1;    TransferHelper.safeApprove(token0,address(swapRouter), decoded.amount0);    TransferHelper.safeApprove(token1,address(swapRouter), decoded.amount1);// profitable check// exactInputSingle will fail if this amount not metuint256 amount1Min = LowGasSafeMath.add(decoded.amount1, fee1);uint256 amount0Min = LowGasSafeMath.add(decoded.amount0, fee0);// call exactInputSingle for swapping token1 for token0 in pool w/fee2uint256 amountOut0 =      swapRouter.exactInputSingle(        ISwapRouter.ExactInputSingleParams({          tokenIn: token1,          tokenOut: token0,          fee: decoded.poolFee2,          recipient:address(this),          deadline: block.timestamp,          amountIn: decoded.amount1,          amountOutMinimum: amount0Min,          sqrtPriceLimitX96:0}));// call exactInputSingle for swapping token0 for token 1 in pool w/fee3uint256 amountOut1 =      swapRouter.exactInputSingle(        ISwapRouter.ExactInputSingleParams({          tokenIn: token0,          tokenOut: token1,          fee: decoded.poolFee3,          recipient:address(this),          deadline: block.timestamp,          amountIn: decoded.amount0,          amountOutMinimum: amount1Min,          sqrtPriceLimitX96:0}));// end up with amountOut0 of token0 from first swap and amountOut1 of token1 from second swapuint256 amount0Owed = LowGasSafeMath.add(decoded.amount0, fee0);uint256 amount1Owed = LowGasSafeMath.add(decoded.amount1, fee1);    TransferHelper.safeApprove(token0,address(this), amount0Owed);    TransferHelper.safeApprove(token1,address(this), amount1Owed);if(amount0Owed >0)pay(token0,address(this), msg.sender, amount0Owed);if(amount1Owed >0)pay(token1,address(this), msg.sender, amount1Owed);// if profitable pay profits to payerif(amountOut0 > amount0Owed){uint256 profit0 = LowGasSafeMath.sub(amountOut0, amount0Owed);      TransferHelper.safeApprove(token0,address(this), profit0);pay(token0,address(this), decoded.payer, profit0);}if(amountOut1 > amount1Owed){uint256 profit1 = LowGasSafeMath.sub(amountOut1, amount1Owed);      TransferHelper.safeApprove(token0,address(this), profit1);pay(token1,address(this), decoded.payer, profit1);}}//fee1 is the fee of the pool from the initial borrow//fee2 is the fee of the first pool to arb from//fee3 is the fee of the second pool to arb fromstructFlashParams{address token0;address token1;uint24 fee1;uint256 amount0;uint256 amount1;uint24 fee2;uint24 fee3;}// fee2 and fee3 are the two other fees associated with the two other pools of token0 and token1structFlashCallbackData{uint256 amount0;uint256 amount1;address payer;    PoolAddress.PoolKey poolKey;uint24 poolFee2;uint24 poolFee3;}/// @param params The parameters necessary for flash and the callback, passed in as FlashParams/// @notice Calls the pools flash function with data needed in `uniswapV3FlashCallback`functioninitFlash(FlashParams memory params)external{    PoolAddress.PoolKey memory poolKey =      PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee1});    IUniswapV3Pool pool =IUniswapV3Pool(PoolAddress.computeAddress(factory, poolKey));// recipient of borrowed amounts// amount of token0 requested to borrow// amount of token1 requested to borrow// need amount 0 and amount1 in callback to pay back pool// recipient of flash should be THIS contract    pool.flash(address(this),      params.amount0,      params.amount1,      abi.encode(FlashCallbackData({          amount0: params.amount0,          amount1: params.amount1,          payer: msg.sender,          poolKey: poolKey,          poolFee2: params.fee2,          poolFee3: params.fee3})));}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/flash-integrations/final-contract.md)
Was this helpful?
[PreviousThe Flash Callback](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)[NextLicense Modifications](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
On this page
  * [The Full Contract](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract#the-full-contract)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/flash-integrations/final-contract.md)
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
