# https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors#__docusaurus_skipToContent_fallback)
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
  * [Getting Started](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)


On this page
# Getting Started
In this guide, we will write a smart contract that calls `flash` on a V3 pool and swaps the full amount withdrawn of `token0` and `token1` in the corresponding pools with the same token pair - but different fee tiers. After the swap, the contract will pay back the first pool and transfer profits to the original calling address.
## Flash Transactions Overview[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors#flash-transactions-overview "Direct link to Flash Transactions Overview")
Flash transactions are an approach to transferring tokens on Ethereum that transfer token balances _before_ the necessary conditions are met for those balances to be transferred. In the context of a swap, this would mean the output is sent from the swap before the input is received.
Uniswap V3 introduces a new function, `flash`, within the Pool contract. `Flash` withdraws a specified amount of both `token0` and `token1` to the `recipient` address. The withdrawn amount, plus the swap fees, will be due to the pool at the end of the transaction. `flash` includes a fourth parameter, `data`, which allows the caller to abi.encode any necessary data to be passed through the function and decoded later.
```
functionflash(address recipient,uint256 amount0,uint256 amount1,bytescalldata data)external override lock noDelegateCall {
```

## The Flash Callback[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors#the-flash-callback "Direct link to The Flash Callback")
`flash` will withdraw the tokens, but how are they paid back? To understand this, we must look inside the flash function code. midway through the [**flash**](https://github.com/Uniswap/uniswap-v3-core/blob/main/contracts/UniswapV3Pool.sol#L791) function, we see this:
```
IUniswapV3FlashCallback(msg.sender).uniswapV3FlashCallback(fee0, fee1, data);
```

This step calls the `FlashCallback` function on `msg.sender` - which passes the fee data needed to calculate the balances due to the pool, as well as any data encoded into the `data` parameter.
In V3 there are three separate callback functions, `uniswapV3SwapCallback`, `uniswapV3MintCallback`, and `uniswapV3FlashCallback`, each available to be overridden with custom logic. To write our arbitrage contract, we'll be calling `flash` and overriding the `uniswapV3FlashCallback` with the steps needed to finish executing our transaction.
## Inheriting The V3 Contracts[​](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors#inheriting-the-v3-contracts "Direct link to Inheriting The V3 Contracts")
Inherit `IUniswapV3FlashCallback` and `PeripheryPayments`, as we will use each in our program. Note these two inherited contracts already extend many other contracts that we will be using, such as [LowGasSafeMath](https://docs.uniswap.org/contracts/v3/reference/core/libraries/LowGasSafeMath) which we [attach](https://docs.soliditylang.org/en/v0.7.6/contracts.html?highlight=using#using-for), to types `uint256` and `int256`.
```
contractPairFlashis IUniswapV3FlashCallback, PeripheryPayments {usingLowGasSafeMathforuint256;usingLowGasSafeMathforint256;
```

Declare an immutable public variable `swapRouter` of type `ISwapRouter`:
```
  ISwapRouter public Immutable swapRouter;
```

Declare the constructor here, which is executed once when the contract is deployed. Our constructor hardcodes the address of the V3 router, factory, and the address of weth9, the [ERC-20 wrapper](https://weth.io/) for ether.
```
constructor(    ISwapRouter _swapRouter,address _factory,address _WETH9)PeripheryImmutableState(_factory, _WETH9){    swapRouter = _swapRouter;}
```

The full import section and contract declaration:
```
pragmasolidity=0.7.6;pragma abicoder v2;import'@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3FlashCallback.sol';import'@uniswap/v3-core/contracts/libraries/LowGasSafeMath.sol';import'@uniswap/v3-periphery/contracts/base/PeripheryPayments.sol';import'@uniswap/v3-periphery/contracts/base/PeripheryImmutableState.sol';import'@uniswap/v3-periphery/contracts/libraries/PoolAddress.sol';import'@uniswap/v3-periphery/contracts/libraries/CallbackValidation.sol';import'@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';import'@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol';contractPairFlashis IUniswapV3FlashCallback, PeripheryPayments {usingLowGasSafeMathforuint256;usingLowGasSafeMathforint256;  ISwapRouter public immutable swapRouter;constructor(    ISwapRouter _swapRouter,address _factory,address _WETH9)PeripheryImmutableState(_factory, _WETH9){    swapRouter = _swapRouter;}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/flash-integrations/Inheritance-constructors.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)[NextCalling Flash](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/calling-flash)
On this page
  * [Flash Transactions Overview](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors#flash-transactions-overview)
  * [The Flash Callback](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors#the-flash-callback)
  * [Inheriting The V3 Contracts](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors#inheriting-the-v3-contracts)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/flash-integrations/Inheritance-constructors.md)
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
