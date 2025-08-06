# https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#__docusaurus_skipToContent_fallback)
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
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
        * [Unlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)
        * [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)
        * [Custom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Swap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Position Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)
        * [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Access msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
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
  * v4 Protocol
  * Guides
  * [Access msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)


On this page
# Access msg.sender Inside a Hook
## Introduction[​](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#introduction "Direct link to Introduction")
In Uniswap v4, when a hook is triggered, `msg.sender` is always the PoolManager contract, not the EOA or smart-account that initiated the swap. This behavior occurs because the PoolManager acts as an intermediary, executing the swap logic on behalf of the user.
## Securely Retrieving the Original `msg.sender` address in a Hook[​](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#securely-retrieving-the-original-msgsender-address-in-a-hook "Direct link to securely-retrieving-the-original-msgsender-address-in-a-hook")
Since a smart contract executes the swap, the `sender` parameter passed to `beforeSwap()` represents the caller of `PoolManager.swap()`.
This is typically a router contract, such as a custom swap router or the Universal Router. The challenge is distinguishing between different routers and securely obtaining the original msg.sender.
This guide explains how to securely retrieve the EOA or smart account in a hook.
## Hook Overview[​](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#hook-overview "Direct link to Hook Overview")
To identify the true sender of a swap:
  * Maintain a trusted list of swap routers in the hook.
  * When a swap is initiated, check if the sender is a trusted router.
  * If trusted, call `msgSender()` view function on the router to retrieve the original EOA.


# Implementing a Trusted Router Mechanism
## Implement the Hook[​](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#implement-the-hook "Direct link to Implement the Hook")
Lets start with a simple hook that wants to access `msg.sender` in `beforeSwap()`
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.24;// Import statements for a hookcontractAccessSenderHookis BaseHook {// constructor, state, interface, etc// ...function_beforeSwap(address sender, PoolKey calldata, IPoolManager.SwapParams calldata,bytescalldata)internal    overridereturns(bytes4, BeforeSwapDelta,uint24){// read msg.sender// ...return(BaseHook.beforeSwap.selector, BeforeSwapDelta.wrap(0),0);}...}
```

> [Refer to Building your first hook](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#setting-things-up)
## Define an Interface for Trusted Routers[​](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#define-an-interface-for-trusted-routers "Direct link to Define an Interface for Trusted Routers")
Each trusted router should implement a standard function that exposes the original `msg.sender`
```
interfaceIMsgSender{functionmsgSender()externalviewreturns(address);}
```

This function allows hooks to query the router for the actual sender.
## Store a List of Trusted Routers[​](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#store-a-list-of-trusted-routers "Direct link to Store a List of Trusted Routers")
The hook should keep track of which router contracts can be trusted to return a valid `msgSender()`
This can be done with the help of add and remove functions implemented in the hook.
```
mapping(address swapRouter => bool approved) public verifiedRouters;
```

note
Make sure you include an address mapping in your hook for the routers before adding the functions.
```
functionaddRouter(address _router)external{  verifiedRouters[_router]=true;  console.log("Router added:", _router);}
```

This function allows hook to add the router to the list of trusted routers.
```
functionremoveRouter(address _router)external{  verifiedRouters[_router]=false;  console.log("Router removed:", _router);}
```

This function allows the hook to remove the router from the list of trusted routers if it's no longer needed.
## Implementing `beforeSwap`[​](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#implementing-beforeswap "Direct link to implementing-beforeswap")
Now that we have implemented a basic hook and have added necessary functions, let us implement the beforeSwap function:
```
function_beforeSwap(address sender, PoolKey calldata, IPoolManager.SwapParams calldata,bytescalldata)internal  overridereturns(bytes4, BeforeSwapDelta,uint24){  try IMsgSender(sender).msgSender()returns(address swapper){    console.log("Swap initiated by account:", swapper);} catch {revert("Router does not implement msgSender()");}return(BaseHook.beforeSwap.selector, BeforeSwapDelta.wrap(0),0);}
```

note
While developing, make sure that you verify the contracts are valid before adding them to the list of trusted routers.
> **Here are some examples of trusted routers:**
  * <https://github.com/Uniswap/universal-router/tree/main>
  * <https://github.com/z0r0z/v4-router>


**Here is the full working code sample:**
```
// SPDX-License-Identifier: MITpragma solidity ^0.8.24;import {BaseHook} from "v4-periphery/src/utils/BaseHook.sol";import {Hooks} from "v4-core/src/libraries/Hooks.sol";import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";import {PoolKey} from "v4-core/src/types/PoolKey.sol";import {PoolId, PoolIdLibrary} from "v4-core/src/types/PoolId.sol";import {BalanceDelta} from "v4-core/src/types/BalanceDelta.sol";import {BeforeSwapDelta, BeforeSwapDeltaLibrary} from "v4-core/src/types/BeforeSwapDelta.sol";import "forge-std/console.sol";interface IMsgSender {  function msgSender() external view returns (address);}contract AccessSenderHook is BaseHook {  constructor(IPoolManager _poolManager) BaseHook(_poolManager) {  }  function _beforeSwap(    address sender,    PoolKey calldata,    IPoolManager.SwapParams calldata,    bytes calldata  ) internal override returns (bytes4, BeforeSwapDelta, uint24) {    try IMsgSender(sender).msgSender() returns (address swapper) {      console.log("Swap initiated by account:", swapper);    } catch {      revert("Router does not implement msgSender()");    }    return (BaseHook.beforeSwap.selector, BeforeSwapDelta.wrap(0), 0);  }  function getHookPermissions()    public    pure    override    returns (Hooks.Permissions memory)  {    return      Hooks.Permissions({        beforeInitialize: false,        afterInitialize: false,        beforeAddLiquidity: false,        afterAddLiquidity: false,        beforeRemoveLiquidity: false,        afterRemoveLiquidity: false,        beforeSwap: true,        afterSwap: false,        beforeDonate: false,        afterDonate: false,        beforeSwapReturnDelta: false,        afterSwapReturnDelta: false,        afterAddLiquidityReturnDelta: false,        afterRemoveLiquidityReturnDelta: false      });  }}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/14-accessing-msg.sender-using-hook.mdx)
Was this helpful?
[PreviousFlash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)[NextErrors](https://docs.uniswap.org/contracts/v4/reference/errors/)
On this page
  * [Introduction](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#introduction)
  * [Securely Retrieving the Original `msg.sender` address in a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#securely-retrieving-the-original-msgsender-address-in-a-hook)
  * [Hook Overview](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#hook-overview)
  * [Implement the Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#implement-the-hook)
  * [Define an Interface for Trusted Routers](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#define-an-interface-for-trusted-routers)
  * [Store a List of Trusted Routers](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#store-a-list-of-trusted-routers)
  * [Implementing `beforeSwap`](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook#implementing-beforeswap)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/14-accessing-msg.sender-using-hook.mdx)
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
