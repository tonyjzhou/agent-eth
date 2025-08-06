# https://docs.uniswap.org/contracts/v4/quickstart/subscriber

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/subscriber#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
    * [Permit2](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Quickstart
  * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)


On this page
# Context
For developers looking to support custom _liquidity mining_ , Subscriber contracts can be used to receive notifications about position modifications or transfers.
# Guide
## 1. Implement the [`ISubscriber`](https://github.com/Uniswap/v4-periphery/blob/main/src/interfaces/ISubscriber.sol) interface[​](https://docs.uniswap.org/contracts/v4/quickstart/subscriber#1-implement-the-isubscriber-interface "Direct link to 1-implement-the-isubscriber-interface")
Can also refer to [MockSubscriber](https://github.com/Uniswap/v4-periphery/blob/main/test/mocks/MockSubscriber.sol) for an actual implementation example.
```
import{ISubscriber}from"v4-periphery/src/interfaces/ISubscriber.sol";contractMySubscriberis ISubscriber {uint256public notifySubscribeCount;uint256public notifyUnsubscribeCount;uint256public notifyModifyLiquidityCount;uint256public notifyBurnCount;// other implementations...functionnotifySubscribe(uint256,bytesmemory)external onlyByPosm {    notifySubscribeCount++;}functionnotifyUnsubscribe(uint256)external onlyByPosm {    notifyUnsubscribeCount++;}functionnotifyModifyLiquidity(uint256,int256, BalanceDelta)external onlyByPosm {    notifyModifyLiquidityCount++;}functionnotifyBurn(uint256,address, PositionInfo,uint256, BalanceDelta)external    onlyByPosm{    notifyBurnCount++;}}
```

## 2. A caveat on `unsubscribe()`[​](https://docs.uniswap.org/contracts/v4/quickstart/subscriber#2-a-caveat-on-unsubscribe "Direct link to 2-a-caveat-on-unsubscribe")
To prevent gas griefing during unsubscription, Uniswap v4 sets a fixed variable [`unsubscribeGasLimit`](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/INotifier#unsubscribegaslimit) when calling a subscriber’s `notifyUnsubscribe()` function.
Without this limit, malicious subscribers could prevent liquidity providers from unsubscribing. If `notifyUnsubscribe()` were to consume too much gas, it would cause the unsubscription transaction to revert, thus leading to a denial-of-service
With the gas limit in place, if the subscriber’s notification fails, the unsubscription will still succeed and only the notification to the subscriber is skipped.
From [`_unsubscribe()`](https://github.com/Uniswap/v4-periphery/blob/main/src/base/Notifier.sol#L80) on `Notifier`:
```
if(address(_subscriber).code.length >0){// require that the remaining gas is sufficient to notify the subscriber// otherwise, users can select a gas limit where .notifyUnsubscribe hits OutOfGas yet the// transaction/unsubscription can still succeeif(gasleft()< unsubscribeGasLimit) GasLimitTooLow.selector.revertWith();  try _subscriber.notifyUnsubscribe{gas: unsubscribeGasLimit}(tokenId){} catch {}}
```

## 3. Opt-in to a subscriber contract[​](https://docs.uniswap.org/contracts/v4/quickstart/subscriber#3-opt-in-to-a-subscriber-contract "Direct link to 3. Opt-in to a subscriber contract")
To opt-in to a subscriber contract, call [`subscribe()`](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/INotifier#subscribe) on `PositionManager`.
```
import{IPositionManager}from"v4-periphery/src/interfaces/IPositionManager.sol";IPositionManager posm =IPositionManager(<address>);ISubscriber mySubscriber =ISubscriber(<address>);bytesmemory optionalData =...;posm.subscribe(tokenId, mySubscriber, optionalData);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/05-subscriber.mdx)
Was this helpful?
[PreviousAsyncSwap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/async-swap)[NextBuilding Your First Hook](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
On this page
  * [1. Implement the `ISubscriber` interface](https://docs.uniswap.org/contracts/v4/quickstart/subscriber#1-implement-the-isubscriber-interface)
  * [2. A caveat on `unsubscribe()`](https://docs.uniswap.org/contracts/v4/quickstart/subscriber#2-a-caveat-on-unsubscribe)
  * [3. Opt-in to a subscriber contract](https://docs.uniswap.org/contracts/v4/quickstart/subscriber#3-opt-in-to-a-subscriber-contract)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/05-subscriber.mdx)
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
