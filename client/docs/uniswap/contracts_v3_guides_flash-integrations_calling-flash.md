# https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash#__docusaurus_skipToContent_fallback)
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
  * [Calling Flash](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash)


On this page
# Calling Flash
## Parameter Structs[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash#parameter-structs "Direct link to Parameter Structs")
In order to call `flash`, we will need the flash parameters for the initial call, as well as any parameters we want to pass through to the callback.
The `FlashParams` struct will contain the token addresses and amounts we wish to pull out of the pool, as well as the three fee tiers used to determine which pool we are withdrawing from, and which we will be swapping with.
```
structFlashParams{address token0;address token1;uint24 fee1;uint256 amount0;uint256 amount1;uint24 fee2;uint24 fee3;}
```

The `FlashCallbackData` struct will contain the data we want to send to the callback. This includes `poolKey`, which expresses the sorted tokens with the matched fee tier, returned by the [**PoolAddress**](https://github.com/Uniswap/uniswap-v3-periphery/blob/main/contracts/libraries/PoolAddress.sol) library.
```
structFlashCallbackData{uint256 amount0;uint256 amount1;address payer;    PoolAddress.PoolKey poolKey;uint24 poolFee2;uint24 poolFee3;}
```

## Pool Key[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash#pool-key "Direct link to Pool Key")
Now we'll start our function by assigning the relevant parameters from the `Flashparams` (which we have declared in memory as `params`) to our variable `poolKey`
```
functioninitFlash(FlashParams memory params)external{    PoolAddress.PoolKey memory poolKey =      PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee1});}
```

Next we will declare `pool` as type [**IUniswapV3Pool**], which allows us to call `flash` on our desired pool contract.
```
    IUniswapV3Pool pool =IUniswapV3Pool(PoolAddress.computeAddress(factory, poolKey));
```

## Calling Flash[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash#calling-flash "Direct link to Calling Flash")
Finally, we call `flash` on our previously declared `pool`. In the last parameter, we abi.encode the `FlashCallbackData`, which will be decoded in the callback and used to inform the next steps of the transaction.
```
    pool.flash(address(this),      params.amount0,      params.amount1,      abi.encode(FlashCallbackData({          amount0: params.amount0,          amount1: params.amount1,          payer: msg.sender,          poolKey: poolKey,          poolFee2: params.fee2,          poolFee3: params.fee3})));
```

The full function:
```
//fee1 is the fee of the pool from the initial borrow//fee2 is the fee of the first pool to arb from//fee3 is the fee of the second pool to arb fromstructFlashParams{address token0;address token1;uint24 fee1;uint256 amount0;uint256 amount1;uint24 fee2;uint24 fee3;}// fee2 and fee3 are the two other fees associated with the two other pools of token0 and token1structFlashCallbackData{uint256 amount0;uint256 amount1;address payer;    PoolAddress.PoolKey poolKey;uint24 poolFee2;uint24 poolFee3;}functioninitFlash(FlashParams memory params)external{    PoolAddress.PoolKey memory poolKey =      PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee1});    IUniswapV3Pool pool =IUniswapV3Pool(PoolAddress.computeAddress(factory, poolKey));    pool.flash(address(this),      params.amount0,      params.amount1,      abi.encode(FlashCallbackData({          amount0: params.amount0,          amount1: params.amount1,          payer: msg.sender,          poolKey: poolKey,          poolFee2: params.fee2,          poolFee3: params.fee3})));}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/flash-integrations/calling-flash.md)
Was this helpful?
[PreviousGetting Started](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)[NextThe Flash Callback](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/flash-callback)
On this page
  * [Parameter Structs](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash#parameter-structs)
  * [Pool Key](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash#pool-key)
  * [Calling Flash](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash#calling-flash)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/flash-integrations/calling-flash.md)
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
