# https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
          * [Getting Started](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
          * [Calling Flash](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash)
          * [The Flash Callback](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
          * [The Final Contract](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
    * [Permit2](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Guides
  * Implement Flash Swaps
  * [The Flash Callback](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)


On this page
# The Flash Callback
## Setting Up The Callback[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback#setting-up-the-callback "Direct link to Setting Up The Callback")
Here we will override the flash callback with our custom logic to execute the desired swaps and pay the profits to the original `msg.sender`.
Declare the `uniswapV3FlashCallback` function and override it.
```
functionuniswapV3FlashCallback(uint256 fee0,uint256 fee1,bytescalldata data)external override {
```

Declare a variable `decoded` in memory and assign it to the [**decoded data**](https://docs.soliditylang.org/en/v0.7.6/units-and-global-variables.html?highlight=abi.decode#abi-encoding-and-decoding-functions) previously encoded into the calldata.
```
    FlashCallbackData memory decoded = abi.decode(data,(FlashCallbackData));
```

Each callback must be validated to verify that the call originated from a genuine V3 pool. Otherwise, the pool contract would be vulnerable to attack via an EOA manipulating the callback function.
```
    CallbackValidation.verifyCallback(factory, decoded.poolKey);
```

Assign local variables of type `address` as `token0` and `token1` to approve the router to interact with the tokens from the flash.
```
address token0 = decoded.poolKey.token0;address token1 = decoded.poolKey.token1;    TransferHelper.safeApprove(token0,address(swapRouter), decoded.amount0);    TransferHelper.safeApprove(token1,address(swapRouter), decoded.amount1);
```

Code in a minimum amount out for both of the upcoming swaps, such that the following swaps will revert if we do not receive a profitable trade.
```
uint256 amount1Min = LowGasSafeMath.add(decoded.amount1, fee1);uint256 amount0Min = LowGasSafeMath.add(decoded.amount0, fee0);
```

## Initiating A Swap[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback#initiating-a-swap "Direct link to Initiating A Swap")
Call the first of two swaps, calling `exactInputSingle` on the [**router interface**](https://docs.uniswap.org/contracts/v3/reference/periphery/interfaces/ISwapRouter) contract. In this call, we are using the previously declared `amount0In` as the minimum amount out, and assigning the returned balance of the swap to `amountOut0`.
Most of These function arguments have already been discussed, except for two new introductions:
`sqrtPriceLimitX96`: This value limits the price that the swap can change the pool to. Remember that price is always expressed in the pool contract as `token1` in terms of `token0`. This is useful for circumstances where the user wants to swap _up until_ a specific price. For this example, we will set it to 0, which makes to make the argument inactive.
`deadline`: this is the timestamp after which the transaction will revert, to protect the transaction from dramatic changes in price environment that can happen if the transaction is pending for too long. For this example, we will set it far in the future for the sake of simplicity.
The first swap takes the `amount1` that we withdrew from the original pool, and passes that amount as the input amount for a single swap that trades a fixed input for the maximum amount of possible output. It calls this function on the pool determined by our previous token pair, but with the next fee tier in our list of three.
```
uint256 amountOut0 =      swapRouter.exactInputSingle(        ISwapRouter.ExactInputSingleParams({          tokenIn: token1,          tokenOut: token0,          fee: decoded.poolFee2,          recipient:address(this),          deadline: block.timestamp +200,          amountIn: decoded.amount1,          amountOutMinimum: amount0Min,          sqrtPriceLimitX96:0}));
```

Populate the second of two swaps, this time with the last fee tier and with the `amount0` that we withdrew from the original pool.
```
uint256 amountOut1 =      swapRouter.exactInputSingle(        ISwapRouter.ExactInputSingleParams({          tokenIn: token0,          tokenOut: token1,          fee: decoded.poolFee3,          recipient:address(this),          deadline: block.timestamp +200,          amountIn: decoded.amount0,          amountOutMinimum: amount1Min,          sqrtPriceLimitX96:0}));
```

## Paying back the pool[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback#paying-back-the-pool "Direct link to Paying back the pool")
To pay the original pool back for the flash transaction, first calculate the balance due to it and approve the router to transfer the tokens in our contract back to the pool.
```
uint256 amount0Owed = LowGasSafeMath.add(decoded.amount0, fee0);uint256 amount1Owed = LowGasSafeMath.add(decoded.amount1, fee1);TransferHelper.safeApprove(token0,address(this), amount0Owed);TransferHelper.safeApprove(token1,address(this), amount1Owed);
```

If there is any balance due to the token, use simple logic to call [pay](https://docs.uniswap.org/contracts/v3/reference/periphery/base/PeripheryPayments#pay). Remember that the callback function is being called by the pool itself, which is why we can call `pay` despite the function being marked `internal`.
```
if(amount0Owed >0)pay(token0,address(this), msg.sender, amount0Owed);if(amount1Owed >0)pay(token1,address(this), msg.sender, amount1Owed);
```

Send the profits to the `payer`: the original `msg.sender` of the `initFlash` function, which executed the flash transaction and in turn triggered the callback.
```
if(amountOut0 > amount0Owed){uint256 profit0 = LowGasSafeMath.sub(amountOut0, amount0Owed);      TransferHelper.safeApprove(token0,address(this), profit0);pay(token0,address(this), decoded.payer, profit0);}if(amountOut1 > amount1Owed){uint256 profit1 = LowGasSafeMath.sub(amountOut1, amount1Owed);      TransferHelper.safeApprove(token0,address(this), profit1);pay(token1,address(this), decoded.payer, profit1);}
```

# The full function
```
functionuniswapV3FlashCallback(uint256 fee0,uint256 fee1,bytescalldata data)external override {    FlashCallbackData memory decoded = abi.decode(data,(FlashCallbackData));    CallbackValidation.verifyCallback(factory, decoded.poolKey);address token0 = decoded.poolKey.token0;address token1 = decoded.poolKey.token1;    TransferHelper.safeApprove(token0,address(swapRouter), decoded.amount0);    TransferHelper.safeApprove(token1,address(swapRouter), decoded.amount1);// profitable check// exactInputSingle will fail if this amount not metuint256 amount1Min = LowGasSafeMath.add(decoded.amount1, fee1);uint256 amount0Min = LowGasSafeMath.add(decoded.amount0, fee0);// call exactInputSingle for swapping token1 for token0 in pool w/fee2uint256 amountOut0 =      swapRouter.exactInputSingle(        ISwapRouter.ExactInputSingleParams({          tokenIn: token1,          tokenOut: token0,          fee: decoded.poolFee2,          recipient:address(this),          deadline: block.timestamp +200,          amountIn: decoded.amount1,          amountOutMinimum: amount0Min,          sqrtPriceLimitX96:0}));// call exactInputSingle for swapping token0 for token 1 in pool w/fee3uint256 amountOut1 =      swapRouter.exactInputSingle(        ISwapRouter.ExactInputSingleParams({          tokenIn: token0,          tokenOut: token1,          fee: decoded.poolFee3,          recipient:address(this),          deadline: block.timestamp +200,          amountIn: decoded.amount0,          amountOutMinimum: amount1Min,          sqrtPriceLimitX96:0}));// end up with amountOut0 of token0 from first swap and amountOut1 of token1 from second swapuint256 amount0Owed = LowGasSafeMath.add(decoded.amount0, fee0);uint256 amount1Owed = LowGasSafeMath.add(decoded.amount1, fee1);    TransferHelper.safeApprove(token0,address(this), amount0Owed);    TransferHelper.safeApprove(token1,address(this), amount1Owed);if(amount0Owed >0)pay(token0,address(this), msg.sender, amount0Owed);if(amount1Owed >0)pay(token1,address(this), msg.sender, amount1Owed);// if profitable pay profits to payerif(amountOut0 > amount0Owed){uint256 profit0 = LowGasSafeMath.sub(amountOut0, amount0Owed);      TransferHelper.safeApprove(token0,address(this), profit0);pay(token0,address(this), decoded.payer, profit0);}if(amountOut1 > amount1Owed){uint256 profit1 = LowGasSafeMath.sub(amountOut1, amount1Owed);      TransferHelper.safeApprove(token0,address(this), profit1);pay(token1,address(this), decoded.payer, profit1);}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/flash-integrations/flash-callback.md)
Was this helpful?
[PreviousCalling Flash](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash)[NextThe Final Contract](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/final-contract)
On this page
  * [Setting Up The Callback](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback#setting-up-the-callback)
  * [Initiating A Swap](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback#initiating-a-swap)
  * [Paying back the pool](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback#paying-back-the-pool)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/flash-integrations/flash-callback.md)
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
